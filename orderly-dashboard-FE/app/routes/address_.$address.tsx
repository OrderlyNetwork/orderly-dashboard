import { DatePicker } from '@mui/x-date-pickers';
import { Select, Table } from '@radix-ui/themes';
import { LoaderFunctionArgs } from '@remix-run/node';
import { json, useLoaderData, useSearchParams } from '@remix-run/react';
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
import dayjs, { Dayjs } from 'dayjs';
import { FC, useMemo, useState } from 'react';
import { P, match } from 'ts-pattern';

import { useRenderColumns } from './address';

import { Spinner } from '~/components';
import { EventTableData, EventType } from '~/hooks';

export function loader({ params }: LoaderFunctionArgs) {
  return json({ address: params.address });
}

export const Address: FC = () => {
  const [eventType, setEventType] = useState<EventType | 'ALL'>('ALL');

  const [from, setFrom] = useState<Dayjs | null>(dayjs(new Date()).subtract(2, 'weeks'));
  const [until, setUntil] = useState<Dayjs | null>(dayjs(new Date()));
  const [sorting, setSorting] = useState<SortingState>([
    {
      id: 'block_timestamp',
      desc: true
    }
  ]);

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

  const { columns, events, error, isLoading, mutate } = useRenderColumns(eventsParams, eventType);

  const table = useReactTable<EventTableData>({
    data: events ?? [],
    columns,
    state: { expanded: (eventType !== 'ALL') as ExpandedState, pagination, sorting },
    onSortingChange: setSorting,
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
    getSortedRowModel: getSortedRowModel(),
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
    <div className="flex flex-col gap-4 flex-items-center [&>*]:w-full [&>*]:max-w-[50rem]">
      <h2>{address}</h2>

      <div className="flex flex-col flex-items-start gap-1">
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
            mutate();
          }}
          maxDate={until ?? undefined}
        />
        <span>-</span>
        <DatePicker
          label="Until"
          value={until}
          onChange={(date) => {
            setUntil(date);
            mutate();
          }}
          minDate={from ?? undefined}
        />
      </div>

      {!events || isLoading ? (
        <Spinner size="2.5rem" />
      ) : (
        <>
          {renderPagination()}

          <Table.Root className="max-w-full">
            <Table.Header>
              {table.getHeaderGroups().map((headerGroup) => (
                <Table.Row key={headerGroup.id}>
                  {headerGroup.headers.map((header) => (
                    <Table.ColumnHeaderCell key={header.id} colSpan={header.colSpan}>
                      {header.isPlaceholder ? null : (
                        <div
                          className={
                            header.column.getCanSort()
                              ? 'cursor-pointer select-none hover:bg-[--accent-3]'
                              : ''
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
                    <Table.Cell key={cell.id} className="align-middle">
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
