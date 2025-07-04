import { DatePicker } from '@mui/x-date-pickers';
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
  const [eventType, setEventType] = useState<EventType | 'ALL'>('ALL');

  const [from, setFrom] = useState<Dayjs | null>(dayjs(new Date()).subtract(2, 'weeks'));
  const [until, setUntil] = useState<Dayjs | null>(dayjs(new Date()));
  const [sorting, setSorting] = useState<SortingState>([
    {
      id: 'block_timestamp',
      desc: true
    }
  ]);

  const { address: rawAddress } = useLoaderData<typeof loader>();
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
    }
    throw new Error(`Could not match address ${rawAddress}`);
  }, [rawAddress]);

  const [searchParams] = useSearchParams();
  const broker_id = searchParams.get('broker_id');

  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 10
  });

  const eventsParams = useMemo(
    () =>
      broker_id != null
        ? ({
            address,
            broker_id,
            event_type: match(eventType)
              .with('ALL', () => undefined)
              .with('LIQUIDATIONV2', () => 'LIQUIDATION')
              .with('ADLV2', () => 'ADL')
              .otherwise((value) => value) as EventType,
            from_time: from,
            to_time: until
          } satisfies EventsParams)
        : null,
    [broker_id, address, eventType, from, until]
  );

  const { evmApiUrl } = useAppState();
  const { data: accountId } = useSWR<string>(
    broker_id != null
      ? `${evmApiUrl}/v1/get_account?address=${address.address}&broker_id=${broker_id}&chain_type=${address.chain_namespace.toUpperCase()}`
      : undefined,
    (url: string) =>
      fetch(url)
        .then((r) => r.json())
        .then((val) => {
          return val.data.account_id;
        })
  );

  const { columns, events, error, isLoading, mutate } = useRenderColumns(
    eventsParams,
    eventType,
    setEventType
  );

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
      <h2 className="mb-2">{address.address}</h2>

      {accountId != null && (
        <div className="flex flex-col [&>*:first-child]:font-bold">
          <div>Account ID:</div>
          <div>
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

      <div className="flex flex-col [&>*:first-child]:font-bold">
        <div>Chain Namespace:</div>
        <div>
          {match(address.chain_namespace)
            .with('evm', () => 'EVM')
            .with('sol', () => 'Solana')
            .exhaustive()}
        </div>
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

      <Tabs.Root
        value={eventType}
        defaultValue="ALL"
        onValueChange={(value) => {
          setEventType(value as EventType);
          table.resetColumnVisibility();
        }}
      >
        <Tabs.List>
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
