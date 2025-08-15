import { DatePicker } from '@mantine/dates';
import {
  ChevronLeftIcon,
  ChevronRightIcon,
  CopyIcon,
  DoubleArrowLeftIcon,
  DoubleArrowRightIcon,
  MixerHorizontalIcon
} from '@radix-ui/react-icons';
import { Button, IconButton, Popover, Table, Tabs } from '@radix-ui/themes';
import { LoaderFunctionArgs } from '@remix-run/node';
import { json, useLoaderData, useSearchParams, useNavigate } from '@remix-run/react';
import {
  ExpandedState,
  PaginationState,
  SortingState,
  flexRender,
  getCoreRowModel,
  getExpandedRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable
} from '@tanstack/react-table';
import dayjs from 'dayjs';
import { FC, useMemo, useState, useEffect } from 'react';
import useSWR from 'swr';
import { match } from 'ts-pattern';

import { useRenderColumns } from './address';

import { useAppState } from '~/App';
import { Spinner } from '~/components';
import { ChainAddress, EventsParams, EventTableData, EventType } from '~/hooks';

export function loader({ params }: LoaderFunctionArgs) {
  return json({ address: params.address ?? '' });
}

const defaultVisibility = {
  block_number: false,
  'data_Transaction.account_id': false,
  'data_Transaction.broker_hash': false,
  'data_Transaction.fail_reason': false,
  'data_Transaction.withdraw_nonce': false,
  'data_ProcessedTrades.batch_id': false,
  trade_timestamp: false,
  trade_account_id: false,
  trade_match_id: false,
  trade_sum_unitary_fundings: false,
  trade_trade_id: false,
  'data_SettlementResult.account_id': false,
  'data_SettlementResult.settled_amount': false,
  'data_SettlementResult.insurance_transfer_amount': false,
  'data_SettlementResult.insurance_account_id': false,
  settlement_sum_unitary_fundings: false,
  'data_LiquidationResult.liquidated_account_id': false,
  'data_LiquidationResult.insurance_account_id': false,
  'data_LiquidationResult.insurance_transfer_amount': false,
  liquidation_cost_position_transfer: false,
  liquidation_insurance_fee: false,
  liquidation_liquidation_transfer_id: false,
  liquidation_liquidator_fee: false,
  liquidation_sum_unitary_fundings: false,
  'data_LiquidationResultV2.account_id': false,
  'data_LiquidationResultV2.insurance_transfer_amount': false,
  liquidationv2_cost_position_transfer: false,
  liquidationv2_account_id: false,
  liquidationv2_sum_unitary_fundings: false,
  'data_AdlResult.account_id': false,
  'data_AdlResult.insurance_account_id': false,
  'data_AdlResult.sum_unitary_fundings': false
};

export const Address: FC = () => {
  const navigate = useNavigate();
  const [eventType, setEventType] = useState<EventType | 'ALL'>('ALL');

  const [dateRange, setDateRange] = useState<[string | null, string | null]>([
    dayjs(new Date()).subtract(30, 'days').format('YYYY-MM-DD'),
    dayjs(new Date()).format('YYYY-MM-DD')
  ]);
  const [validDateRange, setValidDateRange] = useState<[string | null, string | null]>([
    dayjs(new Date()).subtract(30, 'days').format('YYYY-MM-DD'),
    dayjs(new Date()).format('YYYY-MM-DD')
  ]);
  const [sorting, setSorting] = useState<SortingState>([
    {
      id: 'block_timestamp',
      desc: true
    }
  ]);

  const { address: rawAddress } = useLoaderData<typeof loader>();

  const isAccountId = useMemo(() => rawAddress.match(/^0x[0-9a-fA-F]{64}$/), [rawAddress]);

  const address: ChainAddress = useMemo(() => {
    if (rawAddress.match(/^0x[0-9a-fA-F]{40}$/)) {
      return {
        address: rawAddress,
        chain_namespace: 'evm'
      };
    } else if (rawAddress.match(/^[0-9a-zA-Z]{43,44}$/)) {
      return {
        address: rawAddress,
        chain_namespace: 'sol'
      };
    } else if (isAccountId) {
      return {
        address: rawAddress,
        chain_namespace: 'evm'
      };
    }
    throw new Error(`Could not match address ${rawAddress}`);
  }, [rawAddress, isAccountId]);

  const [searchParams] = useSearchParams();
  const broker_id = searchParams.get('broker_id');
  const user_id = searchParams.get('user_id');

  useEffect(() => {
    if (dateRange[0] && dateRange[1]) {
      setValidDateRange([dateRange[0], dateRange[1]]);
    }
  }, [dateRange]);

  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 10
  });

  const { evmApiUrl } = useAppState();

  const { data: accountIdData } = useSWR<{
    address: string;
    broker_id: string;
  }>(
    isAccountId ? `${evmApiUrl}/v1/public/account?account_id=${rawAddress}` : undefined,
    (url: string) =>
      fetch(url)
        .then((r) => r.json())
        .then((val) => {
          if (!val.success) {
            throw new Error(val.message || 'Failed to fetch account data');
          }
          return val.data;
        })
  );

  const { data: accountIdSubaccounts } = useSWR<{
    rows: Array<{
      user_id: number;
      account_id: string;
      broker_id: string;
      chain_type: string;
      user_type: string;
    }>;
  }>(
    isAccountId && accountIdData
      ? `${evmApiUrl}/v1/get_all_accounts?address=${accountIdData.address}&broker_id=${accountIdData.broker_id}`
      : undefined,
    (url: string) =>
      fetch(url)
        .then((r) => r.json())
        .then((val) => {
          if (!val.success) {
            throw new Error(val.message || 'Failed to fetch subaccounts');
          }
          return val.data;
        })
  );

  useEffect(() => {
    if (isAccountId && accountIdData && accountIdSubaccounts?.rows) {
      const matchingSubaccount = accountIdSubaccounts.rows.find(
        (account) => account.account_id === rawAddress
      );

      const searchParams = new URLSearchParams();
      searchParams.set('broker_id', accountIdData.broker_id);
      if (matchingSubaccount) {
        searchParams.set('user_id', matchingSubaccount.user_id.toString());
      }
      navigate({
        pathname: `/address/${accountIdData.address}`,
        search: `?${searchParams.toString()}`
      });
    }
  }, [isAccountId, accountIdData, accountIdSubaccounts, rawAddress, navigate]);

  const { data: accountsData } = useSWR<{
    rows: Array<{
      user_id: number;
      account_id: string;
      broker_id: string;
      chain_type: string;
      user_type: string;
    }>;
  }>(
    broker_id != null
      ? `${evmApiUrl}/v1/get_all_accounts?address=${address.address}&broker_id=${broker_id}&chain_type=${address.chain_namespace.toUpperCase()}`
      : undefined,
    (url: string) =>
      fetch(url)
        .then((r) => r.json())
        .then((val) => {
          return val.data;
        })
  );

  const selectedAccount = useMemo(() => {
    if (!accountsData?.rows || accountsData.rows.length === 0) return null;

    if (user_id) {
      const targetUserId = parseInt(user_id);
      return (
        accountsData.rows.find((account) => account.user_id === targetUserId) ||
        accountsData.rows[0]
      );
    }

    return accountsData.rows[0];
  }, [accountsData, user_id]);

  const accountId = selectedAccount?.account_id;

  const eventsParams = useMemo(
    () =>
      broker_id != null && selectedAccount != null
        ? ({
            account_id: selectedAccount.account_id,
            event_type: match(eventType)
              .with('ALL', () => undefined)
              .with('LIQUIDATIONV2', () => 'LIQUIDATION')
              .with('ADLV2', () => 'ADL')
              .otherwise((value) => value) as EventType,
            from_time: validDateRange[0] ? dayjs(validDateRange[0]) : null,
            to_time: validDateRange[1] ? dayjs(validDateRange[1]).endOf('day') : null
          } satisfies EventsParams)
        : null,
    [broker_id, selectedAccount, eventType, validDateRange]
  );

  const { columns, events, error, isLoading, isLoadingMore, loadMore, hasMore, tradesCount } =
    useRenderColumns(eventsParams, eventType, setEventType);

  const table = useReactTable<EventTableData>({
    data: events ?? [],
    columns,
    state: {
      expanded: (eventType !== 'ALL') as ExpandedState,
      pagination,
      sorting
    },
    initialState: {
      columnVisibility: defaultVisibility
    },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getExpandedRowModel: getExpandedRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    onPaginationChange: setPagination
  });

  if (error) {
    return error.message ?? '';
  }

  if (accountsData?.rows && accountsData.rows.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center gap-4 p-8 text-center">
        <div className="text-xl font-semibold text-gray-300">Account Not Found</div>
        <div className="text-gray-500 max-w-md">
          No accounts found for address{' '}
          <code className="bg-gray-800 px-2 py-1 rounded text-sm">{address.address}</code>
          {broker_id && (
            <>
              {' '}
              with broker ID{' '}
              <code className="bg-gray-800 px-2 py-1 rounded text-sm">{broker_id}</code>
            </>
          )}
        </div>
        <Button onClick={() => navigate(`/search?q=${address.address}`)} className="mt-4">
          Search All Accounts for This Address
        </Button>
      </div>
    );
  }

  if (isAccountId) {
    return <Spinner size="2.5rem" />;
  }

  const renderPagination = () => (
    <div className="flex flex-col sm:flex-row items-center gap-2 sm:gap-3 text-xs sm:text-sm">
      <div className="flex items-center gap-1 sm:gap-2">
        <button
          className="border rounded p-1 hover:bg-gray-700 disabled:opacity-50 text-xs"
          onClick={() => table.firstPage()}
          disabled={!table.getCanPreviousPage()}
        >
          <DoubleArrowLeftIcon />
        </button>
        <button
          className="border rounded p-1 hover:bg-gray-700 disabled:opacity-50 text-xs"
          onClick={() => table.previousPage()}
          disabled={!table.getCanPreviousPage()}
        >
          <ChevronLeftIcon />
        </button>
        <button
          className="border rounded p-1 hover:bg-gray-700 disabled:opacity-50 text-xs"
          onClick={() => table.nextPage()}
          disabled={!table.getCanNextPage()}
        >
          <ChevronRightIcon />
        </button>
        <button
          className="border rounded p-1 hover:bg-gray-700 disabled:opacity-50 text-xs"
          onClick={() => table.lastPage()}
          disabled={!table.getCanNextPage()}
        >
          <DoubleArrowRightIcon />
        </button>
      </div>
      <div className="flex flex-row items-center gap-1 sm:gap-2">
        <span className="flex items-center gap-1 text-xs">
          <div>Page</div>
          <strong>
            {table.getState().pagination.pageIndex + 1} of {table.getPageCount().toLocaleString()}
          </strong>
        </span>
        <span className="flex items-center gap-1 text-xs">
          | Go to page:
          <input
            type="number"
            defaultValue={table.getState().pagination.pageIndex + 1}
            onChange={(e) => {
              const page = e.target.value ? Number(e.target.value) - 1 : 0;
              table.setPageIndex(page);
            }}
            className="border p-1 rounded w-12 sm:w-16 text-xs"
          />
        </span>
        <select
          value={table.getState().pagination.pageSize}
          onChange={(e) => {
            table.setPageSize(Number(e.target.value));
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

  const renderLoadMore = () => {
    if (!hasMore) return null;

    const loadedEvents = events.length;
    const totalAvailable = tradesCount;
    const remainingEvents = Math.max(0, totalAvailable - loadedEvents);

    return (
      <div className="flex flex-col sm:flex-row items-center gap-3 mb-4">
        <div className="flex flex-col sm:flex-row items-center gap-3">
          <Button onClick={loadMore} disabled={isLoadingMore} className="flex items-center gap-2">
            {isLoadingMore ? (
              <>
                <Spinner size="1rem" />
                Loading...
              </>
            ) : (
              'Load More'
            )}
          </Button>
          <span className="text-xs sm:text-sm text-gray-600 text-center">
            {remainingEvents > 0
              ? `${remainingEvents.toLocaleString()} more events available (${loadedEvents.toLocaleString()}/${totalAvailable.toLocaleString()} loaded)`
              : 'All events loaded'}
          </span>
        </div>
      </div>
    );
  };

  return (
    <div className="flex flex-col gap-4 flex-items-center [&>*]:w-full [&>*]:max-w-full lg:max-w-[50rem] px-2 sm:px-0">
      <h2 className="mb-2 text-lg sm:text-xl break-all">{address.address}</h2>

      {accountId != null && (
        <div className="flex flex-col [&>*:first-child]:font-bold text-sm sm:text-base">
          <div>Account ID:</div>
          <div className="break-all">
            {accountId.substring(0, 7)}...{accountId.substr(-7)}
            <IconButton
              className="ml-1"
              size="1"
              variant="soft"
              onClick={async () => {
                if (accountId == null) return;
                navigator.clipboard.writeText(accountId);
              }}
            >
              <CopyIcon height="12" />
            </IconButton>
          </div>
        </div>
      )}

      {accountsData?.rows && accountsData.rows.length > 1 && (
        <div className="flex flex-col [&>*:first-child]:font-bold text-sm sm:text-base">
          <div>Account Type:</div>
          <select
            value={selectedAccount?.user_id || ''}
            onChange={(e) => {
              const selectedUserId = e.target.value;
              if (selectedUserId) {
                const searchParams = new URLSearchParams();
                searchParams.set('broker_id', broker_id!);
                searchParams.set('user_id', selectedUserId);
                navigate({
                  pathname: `/address/${address.address}`,
                  search: `?${searchParams.toString()}`
                });
              }
            }}
            className="bg-gray-700 text-white border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-blue-500"
          >
            {accountsData.rows.map((account) => (
              <option key={account.user_id} value={account.user_id}>
                {account.user_type === 'MAIN'
                  ? 'Main'
                  : account.user_type === 'SUB'
                    ? 'Sub'
                    : account.user_type}
                {account.user_type === 'SUB' ? ` (ID: ${account.user_id})` : ''}
              </option>
            ))}
          </select>
        </div>
      )}

      <div className="flex flex-col [&>*:first-child]:font-bold text-sm sm:text-base">
        <div>Chain Namespace:</div>
        <div>
          {match(address.chain_namespace)
            .with('evm', () => 'EVM')
            .with('sol', () => 'Solana')
            .exhaustive()}
        </div>
      </div>

      <div className="flex flex-col sm:flex-row flex-items-center gap-2 w-full">
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
                    const maxRangeDate = dayjs(dateRange[0]).add(30, 'day');
                    return today.isBefore(maxRangeDate)
                      ? today.format('YYYY-MM-DD')
                      : maxRangeDate.format('YYYY-MM-DD');
                  })()
                : dayjs().format('YYYY-MM-DD')
          }
          onChange={(value) => {
            setDateRange(value);
          }}
          highlightToday={true}
        />
      </div>

      <div className="w-full overflow-x-auto">
        <Tabs.Root
          value={eventType}
          defaultValue="ALL"
          onValueChange={(value) => {
            setEventType(value as EventType);
            table.resetColumnVisibility();
          }}
        >
          <Tabs.List className="flex-wrap">
            <Tabs.Trigger value="ALL">All</Tabs.Trigger>
            <Tabs.Trigger value="TRANSACTION">Transactions</Tabs.Trigger>
            <Tabs.Trigger value="PERPTRADE">Trades</Tabs.Trigger>
            <Tabs.Trigger value="SETTLEMENT">Pnl Settlements</Tabs.Trigger>
            <Tabs.Trigger value="LIQUIDATIONV2">Liquidations</Tabs.Trigger>
            <Tabs.Trigger value="LIQUIDATION">Liquidations (old)</Tabs.Trigger>
            <Tabs.Trigger value="ADLV2">ADL</Tabs.Trigger>
            <Tabs.Trigger value="ADL">ADL (old)</Tabs.Trigger>
          </Tabs.List>
        </Tabs.Root>
      </div>

      <div>
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

      {!events || isLoading ? (
        <Spinner size="2.5rem" />
      ) : (
        <>
          {renderLoadMore()}
          {renderPagination()}

          <div className="w-full overflow-x-auto">
            <Table.Root className="max-w-full min-w-[600px]">
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
export default Address;
