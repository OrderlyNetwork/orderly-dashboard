import { DatePicker } from '@mantine/dates';
import {
  ChevronLeftIcon,
  ChevronRightIcon,
  DoubleArrowLeftIcon,
  DoubleArrowRightIcon,
  MixerHorizontalIcon
} from '@radix-ui/react-icons';
import { Button, Popover, Table } from '@radix-ui/themes';
import {
  ExpandedState,
  GroupColumnDef,
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
import { FC, useState, useEffect, useMemo, useCallback } from 'react';

import { Spinner } from '~/components';
import { EventTableData, EventType, useSymbols, getSymbolName } from '~/hooks';

interface EventsTableProps {
  events: EventTableData[] | undefined;
  columns: GroupColumnDef<EventTableData, unknown>[];
  isLoading: boolean;
  isLoadingMore: boolean;
  hasMore: boolean;
  tradesCount: number;
  loadMore: () => void;
  eventType: EventType | 'ALL';
  setEventType: (type: EventType | 'ALL') => void;
  dateRange: [string | null, string | null];
  setDateRange: (range: [string | null, string | null]) => void;
  aggregateTrades?: boolean;
  setAggregateTrades?: (value: boolean) => void;
  rawEventsCount?: number;
  symbolFilter?: string;
  setSymbolFilter?: (value: string) => void;
}

const getDefaultVisibility = (aggregateTrades?: boolean, eventType?: EventType | 'ALL') => {
  const baseVisibility = {
    block_number: false,
    'data_Transaction.account_id': false,
    'data_Transaction.broker_hash': false,
    'data_Transaction.fail_reason': false,
    'data_Transaction.withdraw_nonce': false,
    data_ProcessedTrades_batch_id: false,
    trade_timestamp: false,
    trade_account_id: false,
    trade_match_id: false,
    trade_sum_unitary_fundings: false,
    trade_trade_id: false,
    trade_fee: false,
    trade_fee_asset_hash: false,
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

  if (aggregateTrades && eventType === 'PERPTRADE') {
    return {
      ...baseVisibility,
      trade_match_id: true,
      trade_trade_id: true
    };
  }

  return baseVisibility;
};

export const EventsTable: FC<EventsTableProps> = ({
  events,
  columns,
  isLoading,
  isLoadingMore,
  hasMore,
  tradesCount,
  loadMore,
  eventType,
  setEventType,
  dateRange,
  setDateRange,
  aggregateTrades,
  setAggregateTrades,
  rawEventsCount,
  symbolFilter: externalSymbolFilter,
  setSymbolFilter: setExternalSymbolFilter
}) => {
  const [sorting, setSorting] = useState<SortingState>([
    {
      id: 'block_timestamp',
      desc: true
    }
  ]);

  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 10
  });

  const [internalSymbolFilter, setInternalSymbolFilter] = useState<string>('');
  const symbolFilter =
    externalSymbolFilter !== undefined ? externalSymbolFilter : internalSymbolFilter;
  const setSymbolFilter = setExternalSymbolFilter || setInternalSymbolFilter;

  const symbols = useSymbols();

  const getShortSymbolName = useCallback(
    (symbolHash: string) => {
      const symbolName = getSymbolName(symbolHash, symbols);
      const parts = symbolName ? symbolName.split('_') : [];
      return parts.length >= 2 ? parts[1] : symbolName || symbolHash;
    },
    [symbols]
  );

  const uniqueSymbols = useMemo(() => {
    if (!events || eventType !== 'PERPTRADE') return [];

    const symbols = new Set<string>();
    events.forEach((event) => {
      if (event.type === 'trade') {
        symbols.add(event.trade.symbol_hash);
      }
    });

    return Array.from(symbols).sort();
  }, [events, eventType]);

  const filteredEvents = useMemo(() => {
    if (!events || eventType !== 'PERPTRADE' || !symbolFilter) {
      return events;
    }

    return events.filter((event) => {
      if (event.type === 'trade') {
        return event.trade.symbol_hash === symbolFilter;
      }
      return true;
    });
  }, [events, eventType, symbolFilter]);

  const table = useReactTable<EventTableData>({
    data: filteredEvents ?? [],
    columns,
    state: {
      expanded: (eventType !== 'ALL') as ExpandedState,
      pagination,
      sorting
    },
    initialState: {
      columnVisibility: getDefaultVisibility(aggregateTrades, eventType)
    },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getExpandedRowModel: getExpandedRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    onPaginationChange: setPagination
  });

  useEffect(() => {
    const newVisibility = getDefaultVisibility(aggregateTrades, eventType);
    table.setColumnVisibility(newVisibility);
  }, [aggregateTrades, eventType, table]);

  useEffect(() => {
    if (eventType !== 'PERPTRADE') {
      setSymbolFilter('');
    }
  }, [eventType, setSymbolFilter]);

  useEffect(() => {
    table.setPageIndex(0);
  }, [symbolFilter, table]);

  const renderPagination = () => (
    <div className="flex flex-col sm:flex-row items-center justify-between gap-3 sm:gap-4 p-3 sm:p-4 bg-bg-primary rounded-xl border border-border-primary">
      <div className="flex items-center gap-2">
        <button
          className="btn btn-secondary p-2 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick={() => table.firstPage()}
          disabled={!table.getCanPreviousPage()}
        >
          <DoubleArrowLeftIcon className="h-4 w-4" />
        </button>
        <button
          className="btn btn-secondary p-2 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick={() => table.previousPage()}
          disabled={!table.getCanPreviousPage()}
        >
          <ChevronLeftIcon className="h-4 w-4" />
        </button>
        <button
          className="btn btn-secondary p-2 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick={() => table.nextPage()}
          disabled={!table.getCanNextPage()}
        >
          <ChevronRightIcon className="h-4 w-4" />
        </button>
        <button
          className="btn btn-secondary p-2 disabled:opacity-50 disabled:cursor-not-allowed"
          onClick={() => table.lastPage()}
          disabled={!table.getCanNextPage()}
        >
          <DoubleArrowRightIcon className="h-4 w-4" />
        </button>
      </div>

      <div className="flex flex-col sm:flex-row items-center gap-2 sm:gap-4 text-sm">
        <span className="flex items-center gap-2 text-gray-300">
          <span>Page</span>
          <strong className="text-white">
            {table.getState().pagination.pageIndex + 1} of {table.getPageCount().toLocaleString()}
          </strong>
        </span>

        <div className="flex items-center gap-2">
          <span className="text-gray-300">Go to:</span>
          <input
            type="number"
            defaultValue={table.getState().pagination.pageIndex + 1}
            onChange={(e) => {
              const page = e.target.value ? Number(e.target.value) - 1 : 0;
              table.setPageIndex(page);
            }}
            className="w-16 px-2 py-1 text-center"
            min="1"
          />
        </div>

        <div className="flex items-center gap-2">
          <span className="text-gray-300">Show:</span>
          <select
            value={table.getState().pagination.pageSize}
            onChange={(e) => {
              table.setPageSize(Number(e.target.value));
            }}
            className="px-2 py-1"
          >
            {[10, 20, 30, 40, 50].map((pageSize) => (
              <option key={pageSize} value={pageSize}>
                {pageSize}
              </option>
            ))}
          </select>
        </div>
      </div>
    </div>
  );

  const renderLoadMore = () => {
    if (!hasMore) return null;

    const loadedEvents = rawEventsCount || events?.length || 0;
    const totalAvailable = tradesCount;
    const remainingEvents = Math.max(0, totalAvailable - loadedEvents);

    return (
      <div className="flex flex-col sm:flex-row items-center justify-between gap-3 sm:gap-4 p-3 sm:p-4 bg-bg-secondary rounded-lg border border-border-primary">
        <div className="flex flex-col sm:flex-row items-center gap-3 sm:gap-4">
          <Button
            onClick={loadMore}
            disabled={isLoadingMore}
            className="btn btn-primary flex items-center gap-2"
          >
            {isLoadingMore ? (
              <>
                <Spinner size="1rem" />
                Loading...
              </>
            ) : (
              'Load More Events'
            )}
          </Button>
          <div className="text-sm text-gray-300 text-center">
            {remainingEvents > 0
              ? `${remainingEvents.toLocaleString()} more events available (${loadedEvents.toLocaleString()}/${totalAvailable.toLocaleString()} loaded)`
              : 'All events loaded'}
          </div>
        </div>
      </div>
    );
  };

  return (
    <>
      {/* Filters Section */}
      <div className="card p-4 sm:p-6 space-y-4 sm:space-y-6 max-w-2xl">
        <div className="space-y-3 sm:space-y-4">
          <h3 className="text-xl font-semibold text-white">Filters</h3>
          <div className="flex flex-col gap-3 sm:gap-4">
            <div>
              <label htmlFor="date-range" className="block text-sm text-gray-300 mb-2">
                Date Range
              </label>
              <DatePicker
                id="date-range"
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
            <div>
              <label htmlFor="event-type" className="block text-sm text-gray-300 mb-2">
                Event Type
              </label>
              <select
                id="event-type"
                value={eventType}
                onChange={(e) => {
                  setEventType(e.target.value as EventType);
                  const newVisibility = getDefaultVisibility(
                    aggregateTrades,
                    e.target.value as EventType
                  );
                  table.setColumnVisibility(newVisibility);
                }}
                className="w-full bg-bg-primary text-white border border-border-primary rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent transition-all duration-200"
              >
                <option value="ALL">All Events</option>
                <option value="TRANSACTION">Transactions</option>
                <option value="PERPTRADE">Trades</option>
                <option value="SETTLEMENT">Pnl Settlements</option>
                <option value="LIQUIDATIONV2">Liquidations</option>
                <option value="LIQUIDATION">Liquidations (old)</option>
                <option value="ADLV2">ADL</option>
                <option value="ADL">ADL (old)</option>
              </select>
            </div>
            {eventType === 'PERPTRADE' && (
              <div>
                <label htmlFor="symbol-filter" className="block text-sm text-gray-300 mb-2">
                  Symbol Filter
                </label>
                <div className="relative">
                  <select
                    id="symbol-filter"
                    value={symbolFilter}
                    onChange={(e) => setSymbolFilter(e.target.value)}
                    className="w-full bg-bg-primary text-white border border-border-primary rounded-lg px-3 py-2 pr-8 text-sm focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent transition-all duration-200"
                  >
                    <option value="">All Symbols</option>
                    {uniqueSymbols.map((symbolHash) => (
                      <option key={symbolHash} value={symbolHash}>
                        {getShortSymbolName(symbolHash)}
                      </option>
                    ))}
                  </select>
                </div>
              </div>
            )}
            {eventType === 'PERPTRADE' && setAggregateTrades && (
              <div>
                <label className="flex items-center gap-2 text-sm text-gray-300">
                  <input
                    type="checkbox"
                    checked={aggregateTrades}
                    onChange={(e) => setAggregateTrades(e.target.checked)}
                    className="w-4 h-4 text-primary bg-bg-primary border border-border-primary rounded focus:ring-2 focus:ring-primary focus:ring-offset-0"
                  />
                  Aggregate Trade Data
                </label>
                <p className="text-xs text-gray-500 mt-1">
                  Combine trades with the same transaction ID and batch ID into a single row.
                  Numeric fields are summed, executed price uses weighted average by trade quantity.
                </p>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Data Table */}
      <div className="card p-4 sm:p-6 space-y-2 w-full max-w-full">
        <div className="flex flex-col sm:flex-row items-start justify-between gap-4">
          <h3 className="text-xl font-semibold text-white">Event Data</h3>
          {eventType === 'PERPTRADE' && symbolFilter && (
            <div className="text-sm text-gray-300">
              Showing {filteredEvents?.length || 0} of {events?.length || 0} events
              {symbolFilter && ` (filtered by ${getShortSymbolName(symbolFilter)})`}
            </div>
          )}
        </div>

        {!events || isLoading ? (
          <div className="flex justify-center py-12">
            <Spinner size="2.5rem" />
          </div>
        ) : (
          <>
            {renderLoadMore()}

            <div className="flex justify-start px-3 sm:px-4">
              <Popover.Root>
                <Popover.Trigger className="w-auto">
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
                          const newVisibility = getDefaultVisibility(aggregateTrades, eventType);
                          table.setColumnVisibility(newVisibility);
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

            {renderPagination()}

            <div className="w-full overflow-x-auto">
              <Table.Root className="max-w-full min-w-[600px] bg-bg-primary rounded-lg border border-border-primary overflow-hidden">
                <Table.Header>
                  {table.getHeaderGroups().map((headerGroup) => (
                    <Table.Row
                      key={headerGroup.id}
                      className="bg-bg-secondary border-b border-border-primary"
                    >
                      {headerGroup.headers.map((header) => (
                        <Table.ColumnHeaderCell
                          key={header.id}
                          colSpan={header.colSpan}
                          className="p-4"
                        >
                          {header.isPlaceholder ? null : (
                            <div
                              className={
                                header.column.getCanSort()
                                  ? 'cursor-pointer select-none hover:bg-bg-tertiary text-sm font-medium text-white p-2 rounded transition-colors duration-200'
                                  : 'text-sm font-medium text-white'
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
                      className={`border-b border-border-primary hover:bg-bg-tertiary transition-colors duration-200 ${
                        index % 2 === 0 ? 'bg-bg-primary' : 'bg-bg-secondary'
                      }`}
                    >
                      {row.getVisibleCells().map((cell) => (
                        <Table.Cell
                          key={cell.id}
                          className="align-middle text-sm p-4 text-gray-300"
                        >
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
    </>
  );
};
