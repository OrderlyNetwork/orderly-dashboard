import { Select, Table, Tooltip } from '@radix-ui/themes';
import { LoaderFunctionArgs } from '@remix-run/node';
import { json, useLoaderData, useSearchParams } from '@remix-run/react';
import {
  GroupColumnDef,
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  getExpandedRowModel,
  useReactTable
} from '@tanstack/react-table';
import { FixedNumber } from '@tarnadas/fixed-number';
import { FC, useMemo, useState } from 'react';
import { P, match } from 'ts-pattern';

import { Spinner } from '~/components';
import { EventTableData, EventType, useEvents } from '~/hooks';

export function loader({ params }: LoaderFunctionArgs) {
  return json({ address: params.address });
}

export const Address: FC = () => {
  const [eventType, setEventType] = useState<EventType | 'ALL'>('ALL');
  const { address }: { address: string } = useLoaderData<typeof loader>();
  const [searchParams] = useSearchParams();
  const broker_id = searchParams.get('broker_id');
  const {
    data: events,
    error,
    isLoading
  } = useEvents(
    broker_id != null
      ? {
          address,
          broker_id,
          event_type: match(eventType)
            .with('ALL', () => undefined)
            .otherwise((value) => value)
        }
      : null
  );

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
                  return new FixedNumber(info.getValue(), 8).format({
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
                  // FIXME why API returns as 4 decimals?
                  return new FixedNumber(info.getValue(), 4).format({
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
                  // FIXME why API returns as 8 decimals?
                  return new FixedNumber(info.getValue(), 11).format({
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
      });
    return columns;
  }, [columnHelper, eventType, events]);

  const table = useReactTable<EventTableData>({
    data: events ?? [],
    columns,
    state: { expanded: true },
    getSubRows: (row) =>
      row.type === 'event'
        ? match(row.event.data)
            .with({ ProcessedTrades: P.select() }, (data) =>
              data.trades.map((trade) => ({
                type: 'trade' as const,
                trade
              }))
            )
            .otherwise(() => undefined)
        : undefined,
    getCoreRowModel: getCoreRowModel(),
    getExpandedRowModel: getExpandedRowModel()
  });

  if (error) {
    return error.message ?? '';
  }
  if (!events || isLoading) {
    return <Spinner size="2.5rem" />;
  }

  console.log('EVENTS', events);
  console.log('TABLE', table);

  return (
    <div className="flex flex-col gap-4 flex-items-start">
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
    </div>
  );
};
export default Address;
