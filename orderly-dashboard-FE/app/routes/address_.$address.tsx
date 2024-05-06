import { Select, Table, Tooltip } from '@radix-ui/themes';
import { LoaderFunctionArgs } from '@remix-run/node';
import { json, useLoaderData, useSearchParams } from '@remix-run/react';
import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  useReactTable
} from '@tanstack/react-table';
import { FixedNumber } from '@tarnadas/fixed-number';
import { FC, useState } from 'react';
import { P, match } from 'ts-pattern';

import { Spinner } from '~/components';
import { EventType, useEvents } from '~/hooks';
import types from '~/types/api';

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

  const columnHelper = createColumnHelper<types.TradingEvent>();

  const columns = [
    columnHelper.group({
      id: 'general',
      header: 'General',
      columns: [
        columnHelper.accessor('block_number', {
          header: 'Block height',
          cell: (info) => info.getValue()
        }),
        columnHelper.accessor('block_timestamp', {
          header: 'Block timestamp',
          cell: (info) => info.getValue()
        }),
        columnHelper.accessor('transaction_id', {
          header: 'Transaction ID',
          cell: (info) => (
            <Tooltip content={`${info.getValue()} (click to copy)`}>
              <span>
                {info.getValue().substring(0, 4)}...{info.getValue().substr(-4)}
              </span>
            </Tooltip>
          )
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
            console.log('INFO', info);
            return match(events![info.row.index].data)
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
              .exhaustive();
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
            columnHelper.accessor('data.Transaction.side', {
              header: 'Side',
              cell: (info) => info.getValue()
            }),
            columnHelper.accessor('data.Transaction.token_amount', {
              header: 'Token amount',
              cell: (info) => {
                return new FixedNumber(info.getValue(), 6).format({
                  maximumFractionDigits: 2
                });
              }
            }),
            columnHelper.accessor('data.Transaction.status', {
              header: 'Status',
              cell: (info) => info.getValue()
            })
          ]
        })
      );
    });

  const table = useReactTable({
    data: events ?? [],
    columns,
    getCoreRowModel: getCoreRowModel()
  });

  if (error) {
    return error.message ?? '';
  }
  if (!events || isLoading) {
    return <Spinner size="2.5rem" />;
  }

  console.log('EVENTS', events);

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
