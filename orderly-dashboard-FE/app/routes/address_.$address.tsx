import { DatePicker } from '@mui/x-date-pickers';
import {
  ChevronLeftIcon,
  ChevronRightIcon,
  DoubleArrowLeftIcon,
  DoubleArrowRightIcon,
  MixerHorizontalIcon
} from '@radix-ui/react-icons';
import { Button, Popover, Select, Table } from '@radix-ui/themes';
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
import useSWR from 'swr';
import { P, match } from 'ts-pattern';

import { useRenderColumns } from './address';

import { useAppState } from '~/App';
import { Spinner } from '~/components';
import { EventTableData, EventType } from '~/hooks';

export function loader({ params }: LoaderFunctionArgs) {
  return json({ address: params.address });
}

const defaultVisibility = {
  event_block_number: false,
  'event_data.Transaction.account_id': false,
  'event_data.Transaction.broker_hash': false,
  'event_data.Transaction.fail_reason': false,
  'event_data.Transaction.withdraw_nonce': false,
  'event_data.Transaction.token_hash': false,
  'event_data.ProcessedTrades.batch_id': false,
  trade_account_id: false,
  trade_match_id: false,
  trade_fee_asset_hash: false,
  trade_sum_unitary_fundings: false,
  trade_trade_id: false,
  trade_symbol_hash: false,
  'event_data.SettlementResult.account_id': false,
  'event_data.SettlementResult.settled_amount': false,
  'event_data.SettlementResult.insurance_transfer_amount': false,
  'event_data.SettlementResult.insurance_account_id': false,
  'event_data.SettlementResult.settled_asset_hash': false,
  settlement_sum_unitary_fundings: false,
  settlement_symbol_hash: false,
  'event_data.LiquidationResult.liquidated_account_id': false,
  'event_data.LiquidationResult.insurance_account_id': false,
  'event_data.LiquidationResult.insurance_transfer_amount': false,
  liquidation_cost_position_transfer: false,
  liquidation_insurance_fee: false,
  liquidation_liquidation_transfer_id: false,
  liquidation_liquidator_fee: false,
  liquidation_sum_unitary_fundings: false,
  'event_data.AdlResult.account_id': false,
  'event_data.AdlResult.insurance_account_id': false,
  'event_data.AdlResult.sum_unitary_fundings': false,
  'event_data.AdlResult.symbol_hash': false
};

export const Address: FC = () => {
  const [eventType, setEventType] = useState<EventType | 'ALL'>('ALL');

  const [from, setFrom] = useState<Dayjs | null>(dayjs(new Date()).subtract(2, 'weeks'));
  const [until, setUntil] = useState<Dayjs | null>(dayjs(new Date()));
  const [sorting, setSorting] = useState<SortingState>([
    {
      id: 'event_block_timestamp',
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

  const { evmApiUrl } = useAppState();
  const { data: accountId } = useSWR<string>(
    broker_id != null
      ? `${evmApiUrl}/v1/get_account?address=${address}&broker_id=${broker_id}`
      : undefined,
    (url: string) =>
      fetch(url)
        .then((r) => r.json())
        .then((val) => {
          return val.data.account_id;
        })
  );

  const { columns, events, error, isLoading, mutate } = useRenderColumns(eventsParams, eventType);

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

  const renderPagination = () => (
    <div className="flex items-center gap-3">
      <button
        className="border rounded p-1"
        onClick={() => table.firstPage()}
        disabled={!table.getCanPreviousPage()}
      >
        <DoubleArrowLeftIcon />
      </button>
      <button
        className="border rounded p-1"
        onClick={() => table.previousPage()}
        disabled={!table.getCanPreviousPage()}
      >
        <ChevronLeftIcon />
      </button>
      <button
        className="border rounded p-1"
        onClick={() => table.nextPage()}
        disabled={!table.getCanNextPage()}
      >
        <ChevronRightIcon />
      </button>
      <button
        className="border rounded p-1"
        onClick={() => table.lastPage()}
        disabled={!table.getCanNextPage()}
      >
        <DoubleArrowRightIcon />
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

      {accountId != null && (
        <div className="flex flex-col [&>*:first-child]:font-bold">
          <div>Account ID:</div>
          <div>
            {accountId.substring(0, 7)}...{accountId.substr(-7)}
          </div>
        </div>
      )}

      <div className="flex flex-col flex-items-start gap-1">
        <span className="font-bold font-size-5">Events:</span>
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

      <div>
        <Popover.Root>
          <Popover.Trigger className="w-auto flex-self-start">
            <Button variant="soft">
              <MixerHorizontalIcon width="16" height="16" />
              Column Filters
            </Button>
          </Popover.Trigger>
          <Popover.Content width="20rem" maxHeight="26rem">
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
                    <label>
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
