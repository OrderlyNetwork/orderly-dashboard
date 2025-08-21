import {
  ChevronLeftIcon,
  ChevronRightIcon,
  ClipboardCopyIcon,
  DoubleArrowLeftIcon,
  MixerHorizontalIcon,
  MagnifyingGlassIcon,
  Cross2Icon
} from '@radix-ui/react-icons';
import { Button, Popover, Table, Tooltip, Dialog } from '@radix-ui/themes';
import { Link } from '@remix-run/react';
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  getPaginationRowModel,
  PaginationState,
  useReactTable
} from '@tanstack/react-table';
import { useMemo, useState, FC, useCallback } from 'react';

import { Spinner } from '~/components';
import {
  useBrokers,
  useSymbols,
  getSymbolName,
  getSymbolBaseTick,
  getMaxFractionDigits,
  useSearchAddress
} from '~/hooks';
import { usePositions, PositionsParams } from '~/hooks/usePositions';
import { PositionEntry, PositionsResponse } from '~/types/leaderboard';
import { base64UrlSafeEncode } from '~/util';

const defaultVisibility = {
  account_id: false,
  address: true,
  broker_id: false,
  symbol: true,
  holding: true,
  total_realized_pnl: true,
  index_price: false,
  mark_price: false,
  holding_value: true,
  opening_cost: false,
  average_entry_price: false,
  un_realized_pnl: true
};

const addressPageVisibility = {
  rank: false,
  account_id: false,
  address: false,
  broker_id: false,
  symbol: true,
  holding: true,
  total_realized_pnl: true,
  index_price: false,
  mark_price: false,
  holding_value: true,
  opening_cost: false,
  average_entry_price: false,
  un_realized_pnl: true
};

interface PositionsProps {
  accountId?: string;
  hideFilters?: boolean;
  hideTitle?: boolean;
  hideQuickActions?: boolean;
}

export const Positions: FC<PositionsProps> = ({
  accountId,
  hideFilters = false,
  hideTitle = false,
  hideQuickActions = false
}) => {
  const [queryParams, setQueryParams] = useState<PositionsParams>({
    offset: 0,
    limit: 30,
    order_by: 'DESC',
    ...(accountId && { account_id: accountId })
  });

  const [accountIdInput, setAccountIdInput] = useState<string>('');
  const [addressInput, setAddressInput] = useState<string>('');
  const [showAddressModal, setShowAddressModal] = useState<boolean>(false);
  const [searchedAddress, setSearchedAddress] = useState<string>('');

  const { addressData, loading: addressLoading } = useSearchAddress(searchedAddress || null);

  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 30
  });

  const { data, error, isLoading } = usePositions(queryParams);
  const { data: brokers } = useBrokers();
  const symbols = useSymbols();

  const [previousData, setPreviousData] = useState<PositionsResponse | null>(null);

  useMemo(() => {
    if (data) {
      setPreviousData(data);
    }
  }, [data]);

  const displayData = isLoading && previousData ? previousData : data;

  // Auto-show modal when valid accounts are found
  useMemo(() => {
    if (
      addressData &&
      addressData.length > 0 &&
      addressData.filter((account) => (account.perp_volume || 0) > 0).length > 0
    ) {
      setShowAddressModal(true);
    }
  }, [addressData]);

  const handleInputChange = useCallback((field: keyof PositionsParams, value: string | number) => {
    setQueryParams((prev) => ({
      ...prev,
      [field]: value,
      ...(field !== 'offset' && field !== 'limit' && { offset: 0 })
    }));
  }, []);

  const handleAccountIdChange = useCallback(
    (value: string) => {
      setAccountIdInput(value);
      handleInputChange('account_id', value);
    },
    [handleInputChange]
  );

  const formatNumber = useCallback(
    (value: string | number, symbolHash?: string, maxFractionDigitsOverride?: number) => {
      const numValue = typeof value === 'string' ? parseFloat(value) : value;
      const maxFractionDigits =
        maxFractionDigitsOverride ??
        (symbolHash ? getMaxFractionDigits(getSymbolBaseTick(symbolHash, symbols)) : 2);
      return new Intl.NumberFormat('en-US', {
        minimumFractionDigits: 0,
        maximumFractionDigits: maxFractionDigits
      }).format(numValue);
    },
    [symbols]
  );

  const formatNumberShort = (value: string | number) => {
    const numValue = typeof value === 'string' ? parseFloat(value) : value;
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      notation: 'compact',
      maximumFractionDigits: 3,
      minimumFractionDigits: 0
    }).format(numValue);
  };

  const formatAddress = (address: string | undefined) => {
    if (!address) return '-';
    return `${address.substring(0, 6)}...${address.substring(address.length - 4)}`;
  };

  const formatSymbol = useCallback(
    (symbolHash: string) => getSymbolName(symbolHash, symbols),
    [symbols]
  );

  const isValidAddress = (address: string): boolean => {
    // EVM address: 0x followed by 40 hex characters
    const evmPattern = /^0x[a-fA-F0-9]{40}$/;
    // Solana address: 32-44 base58 characters
    const solPattern = /^[1-9A-HJ-NP-Za-km-z]{32,44}$/;
    // Account ID: 0x followed by 64 hex characters
    const accountIdPattern = /^0x[a-fA-F0-9]{64}$/;

    return evmPattern.test(address) || solPattern.test(address) || accountIdPattern.test(address);
  };

  const isAccountId = (address: string): boolean => {
    return /^0x[a-fA-F0-9]{64}$/.test(address);
  };

  const handleAddressChange = (value: string) => {
    setAddressInput(value);
    if (isValidAddress(value)) {
      if (isAccountId(value)) {
        // If it's an account ID, populate the hidden account ID field directly
        handleAccountIdChange(value);
        setSearchedAddress('');
      } else {
        // If it's an address, search for accounts
        setSearchedAddress(value);
      }
    } else {
      setSearchedAddress('');
    }
  };

  const columns = useMemo<ColumnDef<PositionEntry>[]>(
    () => [
      {
        accessorKey: 'rank',
        header: 'Rank',
        cell: ({ row }) => {
          const index = row.index;
          const offset = queryParams.offset || 0;
          return offset + index + 1;
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
                    : (() => {
                        const isSol = row.original.address.match(/^[0-9a-zA-Z]{43,44}$/);
                        if (isSol) {
                          return `/search?q=${base64UrlSafeEncode(row.original.address)}&chain_namespace=sol`;
                        } else {
                          return `/search?q=${row.original.address}&chain_namespace=evm`;
                        }
                      })()
                }
                className="font-mono text-sm text-blue-400 hover:text-blue-300 hover:underline"
              >
                {formatAddress(row.original.address)}
              </Link>
              {!hideQuickActions && (
                <>
                  <Tooltip content="Copy address">
                    <ClipboardCopyIcon
                      className="w-3 h-3 text-gray-400 hover:text-white cursor-pointer"
                      onClick={() => navigator.clipboard.writeText(row.original.address)}
                    />
                  </Tooltip>
                  <Tooltip content="Filter by this address">
                    <MagnifyingGlassIcon
                      className="w-3 h-3 text-gray-400 hover:text-blue-400 cursor-pointer"
                      onClick={() => handleAccountIdChange(row.original.account_id)}
                    />
                  </Tooltip>
                </>
              )}
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
              {!hideQuickActions && (
                <>
                  <Tooltip content="Copy account ID">
                    <ClipboardCopyIcon
                      className="w-3 h-3 text-gray-400 hover:text-white cursor-pointer"
                      onClick={() => navigator.clipboard.writeText(row.original.account_id)}
                    />
                  </Tooltip>
                  <Tooltip content="Filter by this account ID">
                    <MagnifyingGlassIcon
                      className="w-3 h-3 text-gray-400 hover:text-blue-400 cursor-pointer"
                      onClick={() => handleAccountIdChange(row.original.account_id)}
                    />
                  </Tooltip>
                </>
              )}
            </div>
          ) : (
            <span className="font-mono text-sm">-</span>
          ),
        enableSorting: false
      },
      {
        accessorKey: 'symbol',
        header: 'Symbol',
        cell: ({ row }) => {
          const symbol = formatSymbol(row.original.symbol_hash) || row.original.symbol;
          const parts = symbol.split('_');
          const baseToken = parts.length >= 2 ? parts[1] : symbol;
          return (
            <div className="flex items-center gap-1">
              <span className="font-mono text-sm">{baseToken}</span>
              {!hideQuickActions && (
                <Tooltip content="Filter by this symbol">
                  <MagnifyingGlassIcon
                    className="w-3 h-3 text-gray-400 hover:text-blue-400 cursor-pointer"
                    onClick={() => handleInputChange('symbol', row.original.symbol)}
                  />
                </Tooltip>
              )}
            </div>
          );
        },
        enableSorting: false
      },
      {
        accessorKey: 'holding',
        header: 'Holding',
        cell: ({ row }) => (
          <span
            className={parseFloat(row.original.holding) >= 0 ? 'text-green-400' : 'text-red-400'}
          >
            {formatNumber(row.original.holding, row.original.symbol_hash)}
          </span>
        ),
        enableSorting: false
      },
      {
        accessorKey: 'holding_value',
        header: 'Holding Value',
        cell: ({ row }) => formatNumberShort(row.original.holding_value),
        enableSorting: false
      },
      {
        accessorKey: 'total_realized_pnl',
        header: (
          <Tooltip content="Aggregated realized PnL across all historical positions for this symbol">
            <span className="cursor-help">Realized PnL</span>
          </Tooltip>
        ) as unknown as string,
        cell: ({ row }) => (
          <span
            className={
              parseFloat(row.original.total_realized_pnl) >= 0 ? 'text-green-400' : 'text-red-400'
            }
          >
            {formatNumberShort(row.original.total_realized_pnl)}
          </span>
        ),
        enableSorting: false
      },
      {
        accessorKey: 'un_realized_pnl',
        header: 'Unrealized PnL',
        cell: ({ row }) => (
          <span
            className={
              parseFloat(row.original.un_realized_pnl) >= 0 ? 'text-green-400' : 'text-red-400'
            }
          >
            {formatNumberShort(row.original.un_realized_pnl)}
          </span>
        ),
        enableSorting: false
      },
      {
        accessorKey: 'index_price',
        header: 'Index Price',
        cell: ({ row }) => formatNumber(row.original.index_price, undefined, 10),
        enableSorting: false
      },
      {
        accessorKey: 'mark_price',
        header: 'Mark Price',
        cell: ({ row }) => formatNumber(row.original.mark_price, undefined, 10),
        enableSorting: false
      },
      {
        accessorKey: 'average_entry_price',
        header: 'Avg Entry Price',
        cell: ({ row }) => formatNumber(row.original.average_entry_price, undefined, 10),
        enableSorting: false
      },
      {
        accessorKey: 'broker_id',
        header: 'Broker ID',
        cell: ({ row }) => (
          <div className="flex items-center gap-1">
            <span>{row.original.broker_id || '-'}</span>
            {!hideQuickActions && (
              <Tooltip content="Filter by this broker">
                <MagnifyingGlassIcon
                  className="w-3 h-3 text-gray-400 hover:text-blue-400 cursor-pointer"
                  onClick={() => handleInputChange('broker_id', row.original.broker_id)}
                />
              </Tooltip>
            )}
          </div>
        ),
        enableSorting: false
      }
    ],
    [
      queryParams.offset,
      hideQuickActions,
      handleAccountIdChange,
      formatSymbol,
      handleInputChange,
      formatNumber
    ]
  );

  const table = useReactTable<PositionEntry>({
    data: displayData?.rows ?? [],
    columns,
    state: {
      pagination
    },
    initialState: {
      columnVisibility: hideQuickActions ? addressPageVisibility : defaultVisibility
    },
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    onPaginationChange: setPagination
  });

  const handlePageChange = (newOffset: number) => {
    setQueryParams((prev) => ({
      ...prev,
      offset: newOffset
    }));
  };

  const currentPage = Math.floor((queryParams.offset || 0) / (queryParams.limit || 30)) + 1;
  const hasMoreData = displayData?.rows.length === (queryParams.limit || 30);

  if (error) {
    return (
      <div className="flex flex-col gap-4 flex-items-center [&>*]:w-full [&>*]:max-w-full lg:max-w-[50rem] px-2 sm:px-0">
        <h2 className="mb-2 text-lg sm:text-xl">Positions</h2>
        <div className="text-red-500">Error: {error.message}</div>
      </div>
    );
  }

  const renderPagination = () => (
    <div className="flex flex-col sm:flex-row items-center justify-between gap-4 p-4 bg-bg-primary rounded-xl border border-border-primary">
      <div className="flex items-center gap-2">
        <button
          className="btn btn-secondary p-2 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick={() => handlePageChange(0)}
          disabled={!displayData || (queryParams.offset || 0) <= 0}
        >
          <DoubleArrowLeftIcon className="h-4 w-4" />
        </button>
        <button
          className="btn btn-secondary p-2 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick={() =>
            handlePageChange(Math.max(0, (queryParams.offset || 0) - (queryParams.limit || 30)))
          }
          disabled={!displayData || (queryParams.offset || 0) <= 0}
        >
          <ChevronLeftIcon className="h-4 w-4" />
        </button>
        <button
          className="btn btn-secondary p-2 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick={() => handlePageChange((queryParams.offset || 0) + (queryParams.limit || 30))}
          disabled={!displayData || !hasMoreData}
        >
          <ChevronRightIcon className="h-4 w-4" />
        </button>
      </div>

      <div className="flex flex-col sm:flex-row items-center gap-2 sm:gap-4 text-sm">
        <span className="flex items-center gap-2 text-gray-300">
          <span>Page</span>
          <strong className="text-white">{currentPage}</strong>
        </span>

        <div className="flex items-center gap-2">
          <span className="text-gray-300">Show:</span>
          <select
            value={queryParams.limit || 30}
            onChange={(e) => {
              handleInputChange('limit', Number(e.target.value));
            }}
            className="px-2 py-1"
          >
            {[10, 20, 30].map((pageSize) => (
              <option key={pageSize} value={pageSize}>
                {pageSize}
              </option>
            ))}
          </select>
        </div>
      </div>
    </div>
  );

  return (
    <div className="space-y-8 animate-fade-in flex flex-col align-center">
      {!hideTitle && (
        <div className="text-center space-y-4">
          <h2 className="text-2xl font-bold text-white">Positions</h2>
          <p className="text-gray-300 max-w-3xl mx-auto">
            Track positions across different addresses, accounts, and brokers. View holding values,
            realized and unrealized PnL with customizable filters. Shows current position data for
            each symbol, including closed positions. Realized PnL is aggregated across all
            historical positions for each symbol.
          </p>
        </div>
      )}

      {/* Filters Section */}
      {!hideFilters && (
        <div className="card space-y-6 max-w-2xl mxa min-w-[min-content]">
          {/* Additional Filters */}
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
            <div className="space-y-2">
              <label htmlFor="broker-id" className="text-sm font-medium text-white">
                Broker ID
              </label>
              <select
                id="broker-id"
                value={queryParams.broker_id || ''}
                onChange={(e) => handleInputChange('broker_id', e.target.value)}
                className="w-full"
              >
                <option value="">All brokers</option>
                {brokers?.map((broker) => (
                  <option key={broker.broker_id} value={broker.broker_id}>
                    {broker.broker_name}
                  </option>
                ))}
              </select>
            </div>
            <div className="space-y-2">
              <label htmlFor="symbol-input" className="text-sm font-medium text-white">
                Symbol
              </label>
              <select
                id="symbol-input"
                value={queryParams.symbol || ''}
                onChange={(e) => handleInputChange('symbol', e.target.value)}
                className="w-full"
              >
                <option value="">All symbols</option>
                {symbols?.map((symbol) => {
                  const parts = symbol.symbol.split('_');
                  const baseToken = parts.length >= 2 ? parts[1] : symbol.symbol;
                  return (
                    <option key={symbol.symbol} value={symbol.symbol}>
                      {baseToken}
                    </option>
                  );
                })}
              </select>
            </div>
            <div className="space-y-2">
              <label htmlFor="address-input" className="text-sm font-medium text-white">
                Address
              </label>
              <div className="relative">
                <input
                  id="address-input"
                  type="text"
                  placeholder="Enter EVM/Solana address or Account ID"
                  value={addressInput}
                  onChange={(e) => handleAddressChange(e.target.value)}
                  className="w-full pr-10"
                />
                <div className="absolute right-3 top-1/2 transform -translate-y-1/2">
                  {addressLoading && <Spinner size="1rem" />}
                  {!addressLoading &&
                    searchedAddress &&
                    addressData &&
                    addressData.filter((account) => (account.perp_volume || 0) > 0).length ===
                      0 && <Cross2Icon className="w-4 h-4 text-red-400" />}
                </div>
              </div>
            </div>
            {accountIdInput && (
              <div className="space-y-2">
                <div className="text-sm font-medium text-white">Account ID Filter</div>
                <div className="flex items-center gap-2">
                  <input
                    type="text"
                    value={accountIdInput}
                    readOnly
                    className="flex-1 bg-bg-secondary text-gray-300 cursor-not-allowed"
                  />
                  <button
                    onClick={() => handleAccountIdChange('')}
                    className="btn btn-secondary px-3"
                  >
                    Clear
                  </button>
                </div>
              </div>
            )}
            <div className="space-y-2" style={{ display: 'none' }}>
              <label htmlFor="account-id-input" className="text-sm font-medium text-white">
                Account ID
              </label>
              <input
                id="account-id-input"
                type="text"
                placeholder="Enter account ID"
                value={accountIdInput}
                onChange={(e) => {
                  handleAccountIdChange(e.target.value);
                }}
                className="w-full"
              />
            </div>
          </div>
        </div>
      )}

      {/* Table Section */}
      <div className="card">
        {!displayData ? (
          <div className="flex justify-center py-12 w-full">
            <Spinner size="2.5rem" />
          </div>
        ) : displayData.rows.length === 0 ? (
          <div className="text-center py-8 text-gray-400">
            <p>No positions found for the selected criteria.</p>
          </div>
        ) : (
          <div className="space-y-2">
            {/* Column Filters */}
            <div className="flex justify-start pb-0! p-3 sm:p-4">
              <Popover.Root>
                <Popover.Trigger className="w-auto">
                  <Button variant="soft" className="btn btn-secondary">
                    <MixerHorizontalIcon width="16" height="16" />
                    Column Filters
                  </Button>
                </Popover.Trigger>
                <Popover.Content width="20rem" maxHeight="26rem" className="max-w-[90vw] card">
                  <div className="flex flex-col gap-2">
                    <div className="flex gap-2">
                      <Button
                        onClick={() => {
                          table.resetColumnVisibility();
                        }}
                        className="btn btn-primary"
                      >
                        Reset to default
                      </Button>
                    </div>
                    <div className="flex items-center gap-2">
                      <input
                        type="checkbox"
                        checked={table.getIsAllColumnsVisible()}
                        onChange={table.getToggleAllColumnsVisibilityHandler()}
                        className="rounded"
                      />
                      <label htmlFor="toggle-all" className="text-sm text-white">
                        Toggle All
                      </label>
                    </div>
                    <hr className="w-full border-border-primary" />
                    {table.getAllLeafColumns().map((column) => {
                      return (
                        <div key={column.id} className="flex items-center gap-2">
                          <input
                            type="checkbox"
                            checked={column.getIsVisible()}
                            onChange={column.getToggleVisibilityHandler()}
                            className="rounded"
                          />
                          <label className="text-sm text-white">
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

            {renderPagination()}

            <div className="w-full overflow-x-auto relative">
              {isLoading && (
                <div className="absolute inset-0 bg-bg-overlay flex items-center justify-center z-10 rounded-xl">
                  <Spinner size="2rem" />
                </div>
              )}
              <Table.Root className="max-w-full min-w-[800px]">
                <Table.Header>
                  {table.getHeaderGroups().map((headerGroup) => (
                    <Table.Row key={headerGroup.id} className="border-b border-border-primary">
                      {headerGroup.headers.map((header) => (
                        <Table.ColumnHeaderCell
                          key={header.id}
                          colSpan={header.colSpan}
                          className="py-4 px-4"
                        >
                          {header.isPlaceholder ? null : (
                            <div
                              className={
                                header.column.getCanSort()
                                  ? 'cursor-pointer select-none hover:text-primary-light transition-colors duration-150 text-sm font-medium'
                                  : 'text-sm font-medium'
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
                  {table.getRowModel().rows.map((row, index) => (
                    <Table.Row
                      key={row.id}
                      className={`border-b border-border-primary hover:bg-bg-tertiary transition-colors duration-150 ${
                        index % 2 === 0 ? 'bg-bg-secondary' : 'bg-bg-primary'
                      }`}
                    >
                      {row.getVisibleCells().map((cell) => (
                        <Table.Cell key={cell.id} className="align-middle text-sm py-3 px-4">
                          {flexRender(cell.column.columnDef.cell, cell.getContext())}
                        </Table.Cell>
                      ))}
                    </Table.Row>
                  ))}
                </Table.Body>
              </Table.Root>
            </div>

            {renderPagination()}
          </div>
        )}
      </div>

      {/* Address Search Modal */}
      <Dialog.Root open={showAddressModal} onOpenChange={setShowAddressModal}>
        <Dialog.Content className="card max-w-4xl w-full sm:w-[95vw] max-h-[90vh] mx-0 sm:mx-auto flex flex-col">
          <Dialog.Title className="text-lg sm:text-xl font-semibold text-white mb-4 break-all flex-shrink-0">
            Search Results for {addressInput}
          </Dialog.Title>

          {addressLoading ? (
            <div className="flex justify-center py-8 flex-1">
              <Spinner size="2rem" />
            </div>
          ) : addressData && addressData.length > 0 ? (
            <div className="flex flex-col flex-1 min-h-0">
              <p className="text-sm sm:text-base text-gray-300 mb-4 flex-shrink-0">
                Found {addressData.filter((account) => (account.perp_volume || 0) > 0).length}{' '}
                account(s) with trading volume
              </p>
              <div className="grid grid-cols-1 gap-3 overflow-y-auto flex-1">
                {addressData
                  .filter((account) => (account.perp_volume || 0) > 0)
                  .map((data) => (
                    <button
                      key={`${data.broker_id}-${data.account_id}`}
                      className="card p-3 sm:p-4 hover:bg-bg-tertiary transition-colors cursor-pointer text-left w-full"
                      onClick={() => {
                        handleAccountIdChange(data.account_id);
                        setShowAddressModal(false);
                      }}
                    >
                      <div className="space-y-2 sm:space-y-3">
                        <div className="flex items-center justify-between">
                          <span className="text-xs sm:text-sm text-gray-400">Broker ID</span>
                          <span className="text-xs px-2 py-1 bg-primary rounded-full text-white">
                            {data.broker_id}
                          </span>
                        </div>
                        <div className="text-base sm:text-lg font-semibold text-white">
                          {data.broker_id}
                          {(() => {
                            const accountsForThisBroker = addressData.filter(
                              (account) => account.broker_id === data.broker_id
                            );
                            return accountsForThisBroker.length > 1 && data.user_type ? (
                              <span className="text-sm text-gray-400 ml-2 font-normal">
                                (
                                {data.user_type === 'MAIN'
                                  ? 'main'
                                  : data.user_type === 'SUB'
                                    ? 'sub'
                                    : data.user_type.toLowerCase()}
                                )
                              </span>
                            ) : null;
                          })()}
                        </div>
                        <div className="space-y-2">
                          <span className="text-xs sm:text-sm text-gray-400">Account ID</span>
                          <div className="font-mono text-xs sm:text-sm bg-bg-primary p-2 rounded border border-border-primary break-all">
                            {data.account_id.substring(0, 7)}...{data.account_id.substr(-7)}
                          </div>
                        </div>
                        <div className="space-y-2 pt-2 border-t border-border-primary">
                          <div className="flex justify-between items-center">
                            <span className="text-xs sm:text-sm text-gray-400">Trading Volume</span>
                            <span className="text-xs sm:text-sm font-semibold text-white">
                              {formatNumberShort(data.perp_volume || 0)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center">
                            <span className="text-xs sm:text-sm text-gray-400">Realized PnL</span>
                            <span
                              className={`text-xs sm:text-sm font-semibold ${
                                (data.realized_pnl || 0) >= 0 ? 'text-green-400' : 'text-red-400'
                              }`}
                            >
                              {formatNumberShort(data.realized_pnl || 0)}
                            </span>
                          </div>
                        </div>
                      </div>
                    </button>
                  ))}
              </div>
            </div>
          ) : (
            <div className="text-center py-8 flex-1">
              <div className="text-lg sm:text-xl text-gray-300 mb-2">No Results Found</div>
              <div className="text-sm sm:text-base text-gray-400">
                No accounts with trading volume found for this address
              </div>
            </div>
          )}

          <div className="flex justify-end gap-2 mt-6 flex-shrink-0">
            <button onClick={() => setShowAddressModal(false)} className="btn btn-secondary">
              Close
            </button>
          </div>
        </Dialog.Content>
      </Dialog.Root>
    </div>
  );
};
