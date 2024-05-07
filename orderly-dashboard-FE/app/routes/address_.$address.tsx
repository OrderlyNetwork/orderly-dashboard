import { DatePicker } from '@mui/x-date-pickers';
import { Select, Table, Tooltip } from '@radix-ui/themes';
import { LoaderFunctionArgs } from '@remix-run/node';
import { json, useLoaderData, useSearchParams } from '@remix-run/react';
import {
  ExpandedState,
  GroupColumnDef,
  PaginationState,
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  getExpandedRowModel,
  getPaginationRowModel,
  useReactTable
} from '@tanstack/react-table';
import { FixedNumber } from '@tarnadas/fixed-number';
import dayjs, { Dayjs } from 'dayjs';
import { FC, useMemo, useState } from 'react';
import { P, match } from 'ts-pattern';

import { Spinner } from '~/components';
import { EventTableData, EventType, useEvents } from '~/hooks';

export function loader({ params }: LoaderFunctionArgs) {
  return json({ address: params.address });
}

export const Address: FC = () => {
  const [eventType, setEventType] = useState<EventType | 'ALL'>('ALL');

  const [from, setFrom] = useState<Dayjs | null>(dayjs(new Date()).subtract(2, 'weeks'));
  const [until, setUntil] = useState<Dayjs | null>(dayjs(new Date()));

  const { address }: { address: string } = useLoaderData<typeof loader>();
  const [searchParams] = useSearchParams();
  const broker_id = searchParams.get('broker_id');

  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 10
  });

  const eventsParams = useMemo(
    () =>
      broker_id != null
        ? {
            address,
            broker_id,
            event_type: match(eventType)
              .with('ALL', () => undefined)
              .otherwise((value) => value),
            from_time: from,
            to_time: until
          }
        : null,
    [broker_id, address, eventType, from, until]
  );
  const { data, error, isLoading, setSize } = useEvents(eventsParams);
  const events = useMemo(() => data?.flat(), [data]);

  const columnHelper = createColumnHelper<EventTableData>();

  const columns = useMemo<GroupColumnDef<EventTableData, unknown>[]>(() => {
    const columns = [
      columnHelper.group({
        id: 'general',
        header: 'General',
        columns: [
          columnHelper.accessor('event.block_number', {
            header: 'Block height',
            cell: (info) => info.getValue()
          }),
          columnHelper.accessor('event.block_timestamp', {
            header: 'Block timestamp',
            cell: (info) => info.getValue()
          }),
          columnHelper.accessor('event.transaction_id', {
            header: 'Transaction ID',
            cell: (info) => {
              const value = info.getValue();
              if (value == null) return '';
              return (
                <Tooltip content={`${value} (click to copy)`}>
                  <span>
                    {value.substring(0, 4)}...{value.substr(-4)}
                  </span>
                </Tooltip>
              );
            }
          })
        ]
      })
    ];
    match(eventType)
      .with('ALL', () => {
        columns.push(
          columnHelper.display({
            header: 'Event Type',
            cell: (info) => {
              if (events == null) return '';
              return match(events![info.row.index])
                .with({ type: 'event', event: P.select() }, (event) =>
                  match(event.data)
                    .with(
                      {
                        Transaction: P.any
                      },
                      () => 'Transaction'
                    )
                    .with(
                      {
                        ProcessedTrades: P.any
                      },
                      () => 'Trade'
                    )
                    .with(
                      {
                        LiquidationResult: P.any
                      },
                      () => 'Liqudation'
                    )
                    .with(
                      {
                        SettlementResult: P.any
                      },
                      () => 'Pnl Settlement'
                    )
                    .with(
                      {
                        AdlResult: P.any
                      },
                      () => 'Adl'
                    )
                    .exhaustive()
                )
                .otherwise(() => undefined);
            }
          })
        );
      })
      .with('TRANSACTION', () => {
        columns.push(
          columnHelper.group({
            id: 'transaction',
            header: 'Transaction',
            columns: [
              columnHelper.accessor('event.data.Transaction.chain_id', {
                header: 'Chain ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.side', {
                header: 'Side',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.token_amount', {
                header: 'Token amount',
                cell: (info) =>
                  new FixedNumber(info.getValue(), 6).format({
                    maximumFractionDigits: 2
                  })
              }),
              columnHelper.accessor('event.data.Transaction.status', {
                header: 'Status',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.fail_reason', {
                header: 'Fail Reason',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.fee', {
                header: 'Fee',
                cell: (info) =>
                  new FixedNumber(info.getValue(), 6).format({
                    maximumFractionDigits: 2
                  })
              }),
              columnHelper.accessor('event.data.Transaction.withdraw_nonce', {
                header: 'Withdraw Nonce',
                cell: (info) => info.getValue()
              })
            ]
          })
        );
      })
      .with('PERPTRADE', () => {
        columns.push(
          columnHelper.group({
            id: 'perptrade',
            header: 'Trades',
            columns: [
              columnHelper.accessor('event.data.ProcessedTrades.batch_id', {
                header: 'Batch ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.executed_price', {
                header: 'Executed Price',
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  // FIXME why API returns as 8 decimals?
                  return new FixedNumber(value, 8).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('trade.fee', {
                header: 'Fee',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.match_id', {
                header: 'Match ID',
                cell: (info) => {
                  let value = info.getValue();
                  if (value == null) return;
                  value = String(value);
                  return (
                    <Tooltip content={`${value} (click to copy)`}>
                      <span>
                        {value.substring(0, 3)}...{value.substr(-3)}
                      </span>
                    </Tooltip>
                  );
                }
              }),
              columnHelper.accessor('trade.notional', {
                header: 'Notional',
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('trade.side', {
                header: 'Side',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.sum_unitary_fundings', {
                header: 'Sum Uni. Funding',
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  // FIXME how many decimals?
                  return new FixedNumber(value, 8).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('trade.timestamp', {
                header: 'Timestamp',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.trade_id', {
                header: 'Trade ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.trade_qty', {
                header: 'Trade Qty',
                cell: (info) => info.getValue()
              })
            ]
          })
        );
      })
      .with('SETTLEMENT', () => {
        columns.push(
          columnHelper.group({
            id: 'settlement',
            header: 'PnL Settlement',
            columns: [
              columnHelper.accessor('event.data.SettlementResult.settled_amount', {
                header: 'Settled Amount',
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('event.data.SettlementResult.insurance_transfer_amount', {
                header: 'Insurance Transfer Amount',
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('event.data.SettlementResult.insurance_account_id', {
                header: 'Insurance Account ID',
                cell: (info) => {
                  let value = info.getValue();
                  if (value == null) return;
                  value = String(value);
                  return (
                    <Tooltip content={`${value} (click to copy)`}>
                      <span>
                        {value.substring(0, 4)}...{value.substr(-4)}
                      </span>
                    </Tooltip>
                  );
                }
              }),
              columnHelper.accessor('settlement.mark_price', {
                header: 'Mark Price',
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  // FIXMe why 8 decimals?
                  return new FixedNumber(value, 8).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('settlement.settled_amount', {
                header: 'Settled Amount',
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('settlement.sum_unitary_fundings', {
                header: 'Sum Uni. Funding',
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  // FIXME how many decimals?
                  return new FixedNumber(value, 11).format({
                    maximumFractionDigits: 2
                  });
                }
              })
            ]
          })
        );
      })
      .with('LIQUIDATION', () => {
        columns.push(
          columnHelper.group({
            id: 'liquidation',
            header: 'Liquidation',
            columns: [
              columnHelper.accessor('event.data.LiquidationResult.liquidated_account_id', {
                header: 'Liquidated Account ID',
                cell: (info) => {
                  let value = info.getValue();
                  if (value == null) return;
                  value = String(value);
                  return (
                    <Tooltip content={`${value} (click to copy)`}>
                      <span>
                        {value.substring(0, 4)}...{value.substr(-4)}
                      </span>
                    </Tooltip>
                  );
                }
              }),
              columnHelper.accessor('event.data.LiquidationResult.insurance_account_id', {
                header: 'Insurance Account ID',
                cell: (info) => {
                  let value = info.getValue();
                  if (value == null) return;
                  value = String(value);
                  return (
                    <Tooltip content={`${value} (click to copy)`}>
                      <span>
                        {value.substring(0, 4)}...{value.substr(-4)}
                      </span>
                    </Tooltip>
                  );
                }
              }),
              columnHelper.accessor('event.data.LiquidationResult.insurance_transfer_amount', {
                header: 'Insurance Transfer Amount',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.cost_position_transfer', {
                header: 'Cost Position Transfer',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.insurance_fee', {
                header: 'Insurance Fee',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidation_fee', {
                header: 'Liquidation Fee',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidation_transfer_id', {
                header: 'Liquidation Transfer ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidator_account_id', {
                header: 'Liquidator Account ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidator_fee', {
                header: 'Liquidator Fee',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.mark_price', {
                header: 'Mark Price',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.position_qty_transfer', {
                header: 'Position Qty Transfer',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.sum_unitary_fundings', {
                header: 'Sum Uni. Funding',
                cell: (info) => info.getValue()
              })
            ]
          })
        );
      })
      .with('ADL', () => {
        columns.push(
          columnHelper.group({
            id: 'adl',
            header: 'Adl',
            columns: [
              columnHelper.accessor('event.data.AdlResult.account_id', {
                header: 'Account ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.adl_price', {
                header: 'Adl Price',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.cost_position_transfer', {
                header: 'Cost Position Transfer',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.insurance_account_id', {
                header: 'Insurance Account ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.position_qty_transfer', {
                header: 'Position Qty Transfer',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.sum_unitary_fundings', {
                header: 'Sum Uni. Funding',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.symbol_hash', {
                header: 'Symbol Hash',
                cell: (info) => info.getValue()
              })
            ]
          })
        );
      });
    return columns;
  }, [columnHelper, eventType, events]);

  const table = useReactTable<EventTableData>({
    data: events ?? [],
    columns,
    state: { expanded: (eventType !== 'ALL') as ExpandedState, pagination },
    getSubRows: (row) =>
      row.type === 'event'
        ? match(row.event.data)
            .with({ ProcessedTrades: P.select() }, (data) =>
              data.trades.map((trade) => ({
                type: 'trade' as const,
                trade
              }))
            )
            .with({ SettlementResult: P.select() }, (data) =>
              data.settlement_executions.map((settlement) => ({
                type: 'settlement' as const,
                settlement
              }))
            )
            .with({ LiquidationResult: P.select() }, (data) =>
              data.liquidation_transfers.map((liquidation) => ({
                type: 'liquidation' as const,
                liquidation
              }))
            )
            .otherwise(() => undefined)
        : undefined,
    getCoreRowModel: getCoreRowModel(),
    getExpandedRowModel: getExpandedRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    onPaginationChange: setPagination
  });

  if (error) {
    return error.message ?? '';
  }

  console.log('EVENTS', events);

  const renderPagination = () => (
    <div className="flex items-center gap-2">
      <button
        className="border rounded p-1"
        onClick={() => table.firstPage()}
        disabled={!table.getCanPreviousPage()}
      >
        {'<<'}
      </button>
      <button
        className="border rounded p-1"
        onClick={() => table.previousPage()}
        disabled={!table.getCanPreviousPage()}
      >
        {'<'}
      </button>
      <button
        className="border rounded p-1"
        onClick={() => table.nextPage()}
        disabled={!table.getCanNextPage()}
      >
        {'>'}
      </button>
      <button
        className="border rounded p-1"
        onClick={() => table.lastPage()}
        disabled={!table.getCanNextPage()}
      >
        {'>>'}
      </button>
      <span className="flex items-center gap-1">
        <div>Page</div>
        <strong>
          {table.getState().pagination.pageIndex + 1} of {table.getPageCount().toLocaleString()}
        </strong>
      </span>
      <span className="flex items-center gap-1">
        | Go to page:
        <input
          type="number"
          defaultValue={table.getState().pagination.pageIndex + 1}
          onChange={(e) => {
            const page = e.target.value ? Number(e.target.value) - 1 : 0;
            table.setPageIndex(page);
          }}
          className="border p-1 rounded w-16"
        />
      </span>
      <select
        value={table.getState().pagination.pageSize}
        onChange={(e) => {
          table.setPageSize(Number(e.target.value));
        }}
      >
        {[10, 20, 30, 40, 50].map((pageSize) => (
          <option key={pageSize} value={pageSize}>
            Show {pageSize}
          </option>
        ))}
      </select>
    </div>
  );

  return (
    <div className="flex flex-col gap-4 flex-items-start">
      <h2>{address}</h2>

      <div className="flex flex-col gap-1">
        <span className="font-bold font-size-5">Filter:</span>
        <Select.Root
          defaultValue={undefined}
          onValueChange={(value) => {
            setEventType(value as EventType);
          }}
          value={eventType}
        >
          <Select.Trigger />
          <Select.Content>
            <Select.Item value="ALL">All</Select.Item>
            <Select.Item value="TRANSACTION">Transactions</Select.Item>
            <Select.Item value="PERPTRADE">Trades</Select.Item>
            <Select.Item value="SETTLEMENT">Pnl Settlements</Select.Item>
            <Select.Item value="LIQUIDATION">Liquidations</Select.Item>
            <Select.Item value="ADL">ADL</Select.Item>
          </Select.Content>
        </Select.Root>
      </div>

      <div className="flex flex-items-center gap-2">
        <DatePicker
          label="From"
          value={from}
          onChange={(date) => {
            setFrom(date);
            setSize(100);
          }}
          maxDate={until ?? undefined}
        />
        <span>-</span>
        <DatePicker
          label="Until"
          value={until}
          onChange={(date) => {
            setUntil(date);
            setSize(100);
          }}
          minDate={from ?? undefined}
        />
      </div>

      {!events || isLoading ? (
        <Spinner size="2.5rem" />
      ) : (
        <>
          {renderPagination()}

          <Table.Root>
            <Table.Header>
              {table.getHeaderGroups().map((headerGroup) => (
                <Table.Row key={headerGroup.id}>
                  {headerGroup.headers.map((header) => (
                    <Table.ColumnHeaderCell key={header.id} colSpan={header.colSpan}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(header.column.columnDef.header, header.getContext())}
                    </Table.ColumnHeaderCell>
                  ))}
                </Table.Row>
              ))}
            </Table.Header>

            <Table.Body>
              {table.getRowModel().rows.map((row) => (
                <Table.Row key={row.id}>
                  {row.getVisibleCells().map((cell) => (
                    <Table.Cell key={cell.id}>
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </Table.Cell>
                  ))}
                </Table.Row>
              ))}
            </Table.Body>
          </Table.Root>

          {renderPagination()}
        </>
      )}
    </div>
  );
};
export default Address;
