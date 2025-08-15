import { DatePicker } from '@mantine/dates';
import {
  ChevronLeftIcon,
  ChevronRightIcon,
  ClipboardCopyIcon,
  DoubleArrowLeftIcon,
  DoubleArrowRightIcon,
  MixerHorizontalIcon
} from '@radix-ui/react-icons';
import { Button, Popover, Table, Tooltip } from '@radix-ui/themes';
import { Link } from '@remix-run/react';
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  PaginationState,
  SortingState,
  useReactTable
} from '@tanstack/react-table';
import dayjs from 'dayjs';
import { useMemo, useState, useEffect, FC } from 'react';

import { Spinner } from '~/components';
import { useBrokers } from '~/hooks';
import { useLeaderboard, LeaderboardParams, LeaderboardSortOption } from '~/hooks/useLeaderboard';
import { LeaderboardEntry, LeaderboardResponse } from '~/types/leaderboard';

const defaultVisibility = {
  date: false,
  account_id: false,
  address: true,
  perp_taker_volume: false,
  perp_maker_volume: false,
  broker_fee: false,
  broker_id: false
};

export const Leaderboard: FC = () => {
  const [queryParams, setQueryParams] = useState<LeaderboardParams>({
    start_date: dayjs().subtract(30, 'days').format('YYYY-MM-DD'),
    end_date: dayjs().format('YYYY-MM-DD'),
    page: 1,
    size: 20,
    broker_id: '',
    sort: 'descending_perp_volume',
    aggregateBy: 'address'
  });

  const [addressInput, setAddressInput] = useState<string>('');

  const [dateRange, setDateRange] = useState<[string | null, string | null]>([
    queryParams.start_date,
    queryParams.end_date
  ]);

  const [sorting, setSorting] = useState<SortingState>([
    {
      id: 'perp_volume',
      desc: true
    }
  ]);

  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 20
  });

  const { data, error, isLoading } = useLeaderboard(queryParams);
  const { data: brokers } = useBrokers();

  const [previousData, setPreviousData] = useState<LeaderboardResponse | null>(null);

  useMemo(() => {
    if (data) {
      setPreviousData(data);
    }
  }, [data]);

  const displayData = isLoading && previousData ? previousData : data;

  const currentRequestedPage = queryParams.page || 1;

  const emptySorting = useMemo(() => [], []);

  const handleDateChange = (value: [string | null, string | null]) => {
    setDateRange(value);
    if (value[0] && value[1]) {
      setQueryParams((prev) => ({
        ...prev,
        start_date: value[0]!,
        end_date: value[1]!,
        page: 1
      }));
    }
  };

  const handleInputChange = (field: keyof LeaderboardParams, value: string | number) => {
    setQueryParams((prev) => ({
      ...prev,
      [field]: value,
      ...(field !== 'page' && field !== 'size' && { page: 1 })
    }));
  };

  const handleAddressChange = (value: string) => {
    setAddressInput(value);

    const isValidAddress =
      value === '' || value.match(/^0x[0-9a-fA-F]{40}$/) || value.match(/^[0-9a-zA-Z]{43,44}$/);

    if (isValidAddress) {
      handleInputChange('address', value);
    }
  };

  const handleAggregateByChange = (
    aggregateBy: 'address' | 'address_per_builder' | 'date' | 'account' | ''
  ) => {
    setQueryParams((prev) => ({
      ...prev,
      aggregateBy: aggregateBy || undefined,
      page: 1
    }));
  };

  const formatNumber = (value: number) => {
    return new Intl.NumberFormat('en-US', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2
    }).format(value);
  };

  const formatAddress = (address: string | undefined) => {
    if (!address) return '-';
    return `${address.substring(0, 6)}...${address.substring(address.length - 4)}`;
  };

  const columns = useMemo<ColumnDef<LeaderboardEntry>[]>(
    () => [
      {
        accessorKey: 'rank',
        header: 'Rank',
        cell: ({ row }) => {
          const index = row.index;
          const pageSize = displayData?.meta.records_per_page || 20;
          return (currentRequestedPage - 1) * pageSize + index + 1;
        },
        enableSorting: false
      },
      {
        accessorKey: 'address',
        header: 'Address',
        cell: ({ row }) =>
          row.original.address ? (
            <div className="flex items-center gap-1">
              <Link
                to={
                  row.original.broker_id
                    ? `/address/${row.original.address}?broker_id=${row.original.broker_id}`
                    : `/search?q=${row.original.address}&chain_namespace=${row.original.address.match(/^0x[0-9a-fA-F]{40}$/) ? 'evm' : 'sol'}`
                }
                className="font-mono text-sm text-blue-400 hover:text-blue-300 hover:underline"
              >
                {formatAddress(row.original.address)}
              </Link>
              <Tooltip content="Copy address">
                <ClipboardCopyIcon
                  className="w-3 h-3 text-gray-400 hover:text-white cursor-pointer"
                  onClick={() => navigator.clipboard.writeText(row.original.address!)}
                />
              </Tooltip>
            </div>
          ) : (
            <span className="font-mono text-sm">-</span>
          ),
        enableSorting: false
      },
      {
        accessorKey: 'account_id',
        header: 'Account ID',
        cell: ({ row }) =>
          row.original.account_id ? (
            <div className="flex items-center gap-1">
              <Link
                to={`/address/${row.original.account_id}`}
                className="font-mono text-sm text-blue-400 hover:text-blue-300 hover:underline"
              >
                {formatAddress(row.original.account_id)}
              </Link>
              <Tooltip content="Copy account ID">
                <ClipboardCopyIcon
                  className="w-3 h-3 text-gray-400 hover:text-white cursor-pointer"
                  onClick={() => navigator.clipboard.writeText(row.original.account_id!)}
                />
              </Tooltip>
            </div>
          ) : (
            <span className="font-mono text-sm">-</span>
          ),
        enableSorting: false
      },
      {
        accessorKey: 'perp_volume',
        header: 'Perp Volume',
        cell: ({ row }) => formatNumber(row.original.perp_volume),
        enableSorting: true
      },
      {
        accessorKey: 'perp_taker_volume',
        header: 'Taker Volume',
        cell: ({ row }) => formatNumber(row.original.perp_taker_volume),
        enableSorting: false
      },
      {
        accessorKey: 'perp_maker_volume',
        header: 'Maker Volume',
        cell: ({ row }) => formatNumber(row.original.perp_maker_volume),
        enableSorting: false
      },
      {
        accessorKey: 'total_fee',
        header: 'Total Fee',
        cell: ({ row }) => formatNumber(row.original.total_fee),
        enableSorting: false
      },
      {
        accessorKey: 'realized_pnl',
        header: 'Realized PnL',
        cell: ({ row }) => (
          <span className={row.original.realized_pnl >= 0 ? 'text-green-400' : 'text-red-400'}>
            {formatNumber(row.original.realized_pnl)}
          </span>
        ),
        enableSorting: true
      },
      {
        accessorKey: 'broker_id',
        header: 'Broker ID',
        cell: ({ row }) => row.original.broker_id || '-',
        enableSorting: false
      },
      {
        accessorKey: 'date',
        header: 'Date',
        cell: ({ row }) => dayjs(row.original.date).format('YYYY-MM-DD'),
        enableSorting: false
      }
    ],
    [currentRequestedPage, displayData?.meta.records_per_page]
  );

  const table = useReactTable<LeaderboardEntry>({
    data: displayData?.rows ?? [],
    columns,
    state: {
      pagination,
      sorting: queryParams.aggregateBy === 'date' ? emptySorting : sorting
    },
    initialState: {
      columnVisibility: (() => {
        const baseVisibility = { ...defaultVisibility };

        switch (queryParams.aggregateBy) {
          case 'date':
            baseVisibility.address = false;
            baseVisibility.date = true;
            break;
          case 'address':
            break;
          case 'address_per_builder':
            baseVisibility.broker_id = true;
            break;
          case 'account':
            baseVisibility.broker_id = true;
            break;
          default:
            baseVisibility.account_id = true;
            baseVisibility.broker_id = true;
            baseVisibility.date = true;
            break;
        }

        return baseVisibility;
      })()
    },
    onSortingChange: (updater) => {
      if (queryParams.aggregateBy === 'date') {
        return;
      }

      const newSorting = typeof updater === 'function' ? updater(sorting) : updater;

      if (newSorting.length === 0) {
        const currentSort = sorting[0];
        if (currentSort) {
          const toggledSort = {
            id: currentSort.id,
            desc: !currentSort.desc
          };
          setSorting([toggledSort]);

          let sortParam: string;
          if (toggledSort.id === 'realized_pnl') {
            sortParam = toggledSort.desc ? 'descending_realized_pnl' : 'ascending_realized_pnl';
          } else if (toggledSort.id === 'perp_volume') {
            sortParam = toggledSort.desc ? 'descending_perp_volume' : 'ascending_perp_volume';
          } else {
            return;
          }

          setQueryParams((prev) => ({
            ...prev,
            sort: sortParam as LeaderboardSortOption,
            page: 1
          }));
        }
        return;
      }

      setSorting(newSorting);

      if (newSorting.length > 0) {
        const sort = newSorting[0];
        let sortParam: string;

        if (sort.id === 'realized_pnl') {
          sortParam = sort.desc ? 'descending_realized_pnl' : 'ascending_realized_pnl';
        } else if (sort.id === 'perp_volume') {
          sortParam = sort.desc ? 'descending_perp_volume' : 'ascending_perp_volume';
        } else {
          return;
        }

        setQueryParams((prev) => ({
          ...prev,
          sort: sortParam as LeaderboardSortOption,
          page: 1
        }));
      }
    },
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    onPaginationChange: setPagination
  });

  useEffect(() => {
    const newVisibility = (() => {
      const baseVisibility = { ...defaultVisibility };

      switch (queryParams.aggregateBy) {
        case 'date':
          baseVisibility.address = false;
          baseVisibility.date = true;
          break;
        case 'address':
          break;
        case 'address_per_builder':
          baseVisibility.broker_id = true;
          break;
        case 'account':
          baseVisibility.broker_id = true;
          baseVisibility.account_id = true;
          break;
        default:
          baseVisibility.account_id = true;
          baseVisibility.broker_id = true;
          baseVisibility.date = true;
          break;
      }

      return baseVisibility;
    })();

    table.setColumnVisibility(newVisibility);
  }, [queryParams.aggregateBy, table]);

  if (error) {
    return (
      <div className="flex flex-col gap-4 flex-items-center [&>*]:w-full [&>*]:max-w-full lg:max-w-[50rem] px-2 sm:px-0">
        <h2 className="mb-2 text-lg sm:text-xl">Leaderboard</h2>
        <div className="text-red-500">Error: {error.message}</div>
      </div>
    );
  }

  const renderPagination = () => (
    <div className="flex flex-col sm:flex-row items-center gap-2 sm:gap-3 text-xs sm:text-sm">
      <div className="flex items-center gap-1 sm:gap-2">
        <button
          className="border rounded p-1 hover:bg-gray-700 disabled:opacity-50 text-xs"
          onClick={() => handleInputChange('page', 1)}
          disabled={!displayData || displayData.meta.current_page <= 1}
        >
          <DoubleArrowLeftIcon />
        </button>
        <button
          className="border rounded p-1 hover:bg-gray-700 disabled:opacity-50 text-xs"
          onClick={() => handleInputChange('page', (displayData?.meta.current_page || 1) - 1)}
          disabled={!displayData || displayData.meta.current_page <= 1}
        >
          <ChevronLeftIcon />
        </button>
        <button
          className="border rounded p-1 hover:bg-gray-700 disabled:opacity-50 text-xs"
          onClick={() => handleInputChange('page', (displayData?.meta.current_page || 1) + 1)}
          disabled={
            !displayData ||
            displayData.meta.current_page >=
              Math.ceil((displayData?.meta.total || 0) / (displayData?.meta.records_per_page || 1))
          }
        >
          <ChevronRightIcon />
        </button>
        <button
          className="border rounded p-1 hover:bg-gray-700 disabled:opacity-50 text-xs"
          onClick={() =>
            handleInputChange(
              'page',
              Math.ceil((displayData?.meta.total || 0) / (displayData?.meta.records_per_page || 1))
            )
          }
          disabled={
            !displayData ||
            displayData.meta.current_page >=
              Math.ceil((displayData?.meta.total || 0) / (displayData?.meta.records_per_page || 1))
          }
        >
          <DoubleArrowRightIcon />
        </button>
      </div>
      <div className="flex flex-row items-center gap-1 sm:gap-2">
        <span className="flex items-center gap-1 text-xs">
          <div>Page</div>
          <strong>
            {displayData?.meta.current_page || 1} of{' '}
            {displayData
              ? Math.ceil(displayData.meta.total / displayData.meta.records_per_page)
              : 1}
          </strong>
        </span>
        <span className="flex items-center gap-1 text-xs">
          | Go to page:
          <input
            type="number"
            defaultValue={displayData?.meta.current_page || 1}
            onChange={(e) => {
              const page = e.target.value ? Number(e.target.value) : 1;
              handleInputChange('page', page);
            }}
            className="border p-1 rounded w-12 sm:w-16 text-xs"
          />
        </span>
        <select
          value={displayData?.meta.records_per_page || 20}
          onChange={(e) => {
            handleInputChange('size', Number(e.target.value));
          }}
          className="text-xs"
        >
          {[10, 20, 30, 40, 50].map((pageSize) => (
            <option key={pageSize} value={pageSize}>
              Show {pageSize}
            </option>
          ))}
        </select>
      </div>
    </div>
  );

  return (
    <div className="flex flex-col gap-4 flex-items-center [&>*]:w-full px-2 sm:px-0 max-w-full">
      <h2 className="mb-2 text-lg sm:text-xl">Leaderboard</h2>
      <p className="text-sm text-gray-400 mb-4 max-w-2xl text-center">
        Track trading performance across different addresses, accounts, and brokers. View perp
        volume, fees, and realized PnL with customizable date ranges and aggregation options.
      </p>

      {/* Query Form */}
      <div className="flex flex-col sm:flex-row flex-items-center gap-2 w-full max-w-2xl">
        <DatePicker
          type="range"
          value={dateRange}
          maxLevel="year"
          allowSingleDateInRange={true}
          maxDate={
            dateRange[0] && dateRange[1]
              ? dayjs().format('YYYY-MM-DD')
              : dateRange[0]
                ? (() => {
                    const today = dayjs();
                    const maxRangeDate = dayjs(dateRange[0]).add(89, 'day');
                    return today.isBefore(maxRangeDate)
                      ? today.format('YYYY-MM-DD')
                      : maxRangeDate.format('YYYY-MM-DD');
                  })()
                : dayjs().format('YYYY-MM-DD')
          }
          onChange={handleDateChange}
          highlightToday={true}
        />
      </div>

      {/* Aggregate By */}
      <div className="flex flex-col text-left max-w-2xl">
        <label htmlFor="aggregate-by" className="text-sm font-medium mb-1">
          Aggregate By:
        </label>
        <select
          id="aggregate-by"
          value={queryParams.aggregateBy || ''}
          onChange={(e) => {
            const value = e.target.value;
            handleAggregateByChange(
              value as 'address' | 'address_per_builder' | 'date' | 'account' | ''
            );
          }}
          className="bg-gray-700 text-white border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500"
        >
          <option value="">No aggregation</option>
          <option value="address">Address (Sum across all builders)</option>
          <option value="address_per_builder">
            Address per builder (Sum separately for each builder)
          </option>
          <option value="date">Date</option>
          <option value="account">Account</option>
        </select>
      </div>

      {/* Additional Filters */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 w-full max-w-2xl">
        <div className="text-left">
          <label htmlFor="broker-id" className="text-sm font-medium mb-1 block">
            Broker ID
          </label>
          <select
            id="broker-id"
            value={queryParams.broker_id || ''}
            onChange={(e) => handleInputChange('broker_id', e.target.value)}
            className="w-full bg-gray-700 text-white border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500"
          >
            <option value="">All brokers</option>
            {brokers?.map((broker) => (
              <option key={broker.broker_id} value={broker.broker_id}>
                {broker.broker_name}
              </option>
            ))}
          </select>
        </div>
        <div className="text-left">
          <label htmlFor="address-input" className="text-sm font-medium mb-1 block">
            Address
          </label>
          <input
            id="address-input"
            type="text"
            placeholder="Enter EVM or Solana address"
            value={addressInput}
            onChange={(e) => {
              handleAddressChange(e.target.value);
            }}
            className="w-full bg-gray-700 text-white border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500"
          />
        </div>
      </div>

      {/* Column Filters */}
      <div className="max-w-2xl flex">
        <Popover.Root>
          <Popover.Trigger className="w-auto flex-self-start">
            <Button variant="soft" className="text-sm">
              <MixerHorizontalIcon width="16" height="16" />
              Column Filters
            </Button>
          </Popover.Trigger>
          <Popover.Content width="20rem" maxHeight="26rem" className="max-w-[90vw]">
            <div className="flex flex-col [&>*]:text-size-4 gap-2">
              <div className="px-1">
                <Button
                  onClick={() => {
                    table.resetColumnVisibility();
                  }}
                >
                  Reset to default
                </Button>
              </div>
              <div className="px-1">
                <label>
                  <input
                    {...{
                      type: 'checkbox',
                      checked: table.getIsAllColumnsVisible(),
                      onChange: table.getToggleAllColumnsVisibilityHandler()
                    }}
                  />{' '}
                  Toggle All
                </label>
              </div>
              <hr className="w-full" />
              {table.getAllLeafColumns().map((column) => {
                return (
                  <div key={column.id} className="px-1">
                    <label className="text-sm">
                      <input
                        {...{
                          type: 'checkbox',
                          checked: column.getIsVisible(),
                          onChange: column.getToggleVisibilityHandler()
                        }}
                      />{' '}
                      {
                        // eslint-disable-next-line @typescript-eslint/no-explicit-any
                        flexRender(column.columnDef.header, undefined as any)
                      }{' '}
                      ({column.id})
                    </label>
                  </div>
                );
              })}
            </div>
          </Popover.Content>
        </Popover.Root>
      </div>

      {!displayData ? (
        <Spinner size="2.5rem" />
      ) : (
        <>
          {renderPagination()}

          <div className="w-full overflow-x-auto relative">
            {isLoading && (
              <div className="absolute inset-0 bg-black bg-opacity-50 flex items-center justify-center z-10">
                <Spinner size="2rem" />
              </div>
            )}
            <Table.Root className="max-w-full min-w-[800px]">
              <Table.Header>
                {table.getHeaderGroups().map((headerGroup) => (
                  <Table.Row key={headerGroup.id}>
                    {headerGroup.headers.map((header) => (
                      <Table.ColumnHeaderCell key={header.id} colSpan={header.colSpan}>
                        {header.isPlaceholder ? null : (
                          <div
                            className={
                              header.column.getCanSort()
                                ? 'cursor-pointer select-none hover:bg-[--accent-3] text-sm'
                                : 'text-sm'
                            }
                            onClick={header.column.getToggleSortingHandler()}
                            onKeyDown={(ev) => {
                              if (ev.key === 'Enter') {
                                header.column.getToggleSortingHandler();
                              }
                            }}
                            role="button"
                            tabIndex={0}
                          >
                            {flexRender(header.column.columnDef.header, header.getContext())}
                            {{
                              asc: ' 🔼',
                              desc: ' 🔽'
                            }[header.column.getIsSorted() as string] ?? null}
                          </div>
                        )}
                      </Table.ColumnHeaderCell>
                    ))}
                  </Table.Row>
                ))}
              </Table.Header>

              <Table.Body>
                {table.getRowModel().rows.map((row) => (
                  <Table.Row key={row.id}>
                    {row.getVisibleCells().map((cell) => (
                      <Table.Cell key={cell.id} className="align-middle text-sm">
                        {flexRender(cell.column.columnDef.cell, cell.getContext())}
                      </Table.Cell>
                    ))}
                  </Table.Row>
                ))}
              </Table.Body>
            </Table.Root>
          </div>

          {renderPagination()}
        </>
      )}
    </div>
  );
};
