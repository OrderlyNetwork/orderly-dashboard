import { Table } from '@radix-ui/themes';
import { useNavigate } from '@remix-run/react';
import { ColumnDef, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import dayjs from 'dayjs';
import { FC, useCallback, useMemo } from 'react';

import { Spinner } from '~/components';
import { getMaxFractionDigits, useSymbols } from '~/hooks';
import { useAccountState, useMarketSummary, AccountStatePosition } from '~/hooks/usePublicInfo';
import { formatPriceByTick } from '~/utils/format';

interface AddressPositionsProps {
  address: string;
  brokerId?: string;
  accountId?: string;
}

const POSITIVE_COLOR = '#00dea3';
const NEGATIVE_COLOR = '#FF6390';

const formatBaseToken = (symbol: string) => {
  const parts = symbol.split('_');
  return parts.length >= 2 ? parts[1] : symbol;
};

export const AddressPositions: FC<AddressPositionsProps> = ({ address, brokerId, accountId }) => {
  const navigate = useNavigate();

  const handleRowClick = useCallback(
    (symbol: string) => {
      navigate(`/markets/${formatBaseToken(symbol)}`);
    },
    [navigate]
  );

  const { data, error, isLoading } = useAccountState({
    address,
    broker_id: brokerId,
    account_id: accountId
  });

  const symbols = useSymbols();
  const { data: marketSummary } = useMarketSummary();

  const baseTickFor = useMemo(() => {
    return (symbol: string) => symbols?.find((s) => s.symbol === symbol)?.base_tick ?? 0.01;
  }, [symbols]);

  const quoteTickFor = useMemo(() => {
    return (symbol: string): number | null => {
      const tick = marketSummary?.markets.find((m) => m.symbol === symbol)?.quote_tick;
      const parsed = tick != null ? parseFloat(tick) : NaN;
      return Number.isFinite(parsed) ? parsed : null;
    };
  }, [marketSummary]);

  const formatQty = useMemo(() => {
    return (value: string | number | null, symbol: string) => {
      if (value === null) return '-';
      const numValue = typeof value === 'string' ? parseFloat(value) : value;
      const maxFractionDigits = getMaxFractionDigits(baseTickFor(symbol));
      return new Intl.NumberFormat('en-US', {
        minimumFractionDigits: 0,
        maximumFractionDigits: maxFractionDigits
      }).format(numValue);
    };
  }, [baseTickFor]);

  const formatPriceForSymbol = useMemo(() => {
    return (value: string | number | null | undefined, symbol: string) =>
      formatPriceByTick(value, quoteTickFor(symbol));
  }, [quoteTickFor]);

  const formatUsdCompact = useMemo(() => {
    return (value: string | number | null) => {
      if (value === null) return '-';
      const numValue = typeof value === 'string' ? parseFloat(value) : value;
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD',
        notation: 'compact',
        maximumFractionDigits: 3,
        minimumFractionDigits: 0
      }).format(numValue);
    };
  }, []);

  const formatOpenedAt = (ts: number | null) => {
    if (ts === null) return '-';
    return dayjs(ts).format('YYYY-MM-DD HH:mm:ss');
  };

  const columns = useMemo<ColumnDef<AccountStatePosition>[]>(
    () => [
      {
        accessorKey: 'symbol',
        header: 'Symbol',
        cell: ({ row }) => (
          <span className="font-mono text-sm">{formatBaseToken(row.original.symbol)}</span>
        ),
        enableSorting: false
      },
      {
        accessorKey: 'side',
        header: 'Side',
        cell: ({ row }) => (
          <span
            className="text-xs font-semibold px-2 py-0.5 rounded-full"
            style={{
              color: row.original.side === 'LONG' ? POSITIVE_COLOR : NEGATIVE_COLOR,
              background:
                row.original.side === 'LONG'
                  ? 'rgba(0, 222, 163, 0.12)'
                  : 'rgba(255, 99, 144, 0.12)'
            }}
          >
            {row.original.side}
          </span>
        ),
        enableSorting: false
      },
      {
        accessorKey: 'position_qty',
        header: 'Holding',
        cell: ({ row }) => (
          <span
            style={{
              color: parseFloat(row.original.position_qty) >= 0 ? POSITIVE_COLOR : NEGATIVE_COLOR
            }}
          >
            {formatQty(row.original.position_qty, row.original.symbol)}
          </span>
        ),
        enableSorting: false
      },
      {
        accessorKey: 'notional',
        header: 'Holding Value',
        cell: ({ row }) => formatUsdCompact(row.original.notional),
        enableSorting: false
      },
      {
        accessorKey: 'mark_price',
        header: 'Mark Price',
        cell: ({ row }) => formatPriceForSymbol(row.original.mark_price, row.original.symbol),
        enableSorting: false
      },
      {
        accessorKey: 'average_open_price',
        header: 'Avg Entry Price',
        cell: ({ row }) =>
          formatPriceForSymbol(row.original.average_open_price, row.original.symbol),
        enableSorting: false
      },
      {
        accessorKey: 'unrealized_pnl',
        header: 'Unrealized PnL',
        cell: ({ row }) => {
          const value = row.original.unrealized_pnl;
          if (value === null) return <span>-</span>;
          const num = parseFloat(value);
          return (
            <span style={{ color: num >= 0 ? POSITIVE_COLOR : NEGATIVE_COLOR }}>
              {formatUsdCompact(value)}
            </span>
          );
        },
        enableSorting: false
      },
      {
        accessorKey: 'pnl_24_h',
        header: '24h PnL',
        cell: ({ row }) => {
          const value = row.original.pnl_24_h;
          if (value === null) return <span>-</span>;
          const num = parseFloat(value);
          return (
            <span style={{ color: num >= 0 ? POSITIVE_COLOR : NEGATIVE_COLOR }}>
              {formatUsdCompact(value)}
            </span>
          );
        },
        enableSorting: false
      },
      {
        accessorKey: 'est_liq_price',
        header: 'Est. Liq. Price',
        cell: ({ row }) => formatPriceForSymbol(row.original.est_liq_price, row.original.symbol),
        enableSorting: false
      },
      {
        accessorKey: 'opened_at',
        header: 'Opened At',
        cell: ({ row }) => (
          <span className="text-gray-300 font-mono text-xs">
            {formatOpenedAt(row.original.opened_at)}
          </span>
        ),
        enableSorting: false
      }
    ],
    [formatQty, formatPriceForSymbol, formatUsdCompact]
  );

  const positions = data?.positions ?? [];

  const table = useReactTable<AccountStatePosition>({
    data: positions,
    columns,
    getCoreRowModel: getCoreRowModel()
  });

  if (error) {
    return <div className="text-red-500 px-4 py-8">Error loading positions: {error.message}</div>;
  }

  return (
    <div className="space-y-4 sm:space-y-8 animate-fade-in flex flex-col align-center">
      <div
        className="card w-full space-y-4 sm:space-y-6"
        style={{ background: '#130E1D', border: 'none' }}
      >
        <div className="w-full">
          {isLoading && !data ? (
            <div className="flex justify-center py-12 w-full">
              <Spinner size="2.5rem" />
            </div>
          ) : positions.length === 0 ? (
            <div className="text-center py-12 text-gray-400">
              <p>No open positions for this account.</p>
            </div>
          ) : (
            <div className="w-full overflow-x-auto relative">
              {isLoading && (
                <div className="absolute inset-0 bg-bg-overlay flex items-center justify-center z-10 rounded-xl">
                  <Spinner size="2rem" />
                </div>
              )}
              <Table.Root className="w-full">
                <Table.Header>
                  {table.getHeaderGroups().map((headerGroup) => (
                    <Table.Row key={headerGroup.id} className="border-b border-border-primary">
                      {headerGroup.headers.map((header) => (
                        <Table.ColumnHeaderCell
                          key={header.id}
                          colSpan={header.colSpan}
                          className="py-2 px-2 sm:py-4 sm:px-4 text-sm font-medium"
                        >
                          {header.isPlaceholder
                            ? null
                            : flexRender(header.column.columnDef.header, header.getContext())}
                        </Table.ColumnHeaderCell>
                      ))}
                    </Table.Row>
                  ))}
                </Table.Header>

                <Table.Body>
                  {table.getRowModel().rows.map((row, index) => (
                    <Table.Row
                      key={row.id}
                      onClick={() => handleRowClick(row.original.symbol)}
                      className={`border-b border-border-primary hover:bg-[rgba(156,117,255,0.12)] transition-colors duration-150 cursor-pointer ${
                        index % 2 === 0 ? 'bg-bg-secondary' : 'bg-bg-primary'
                      }`}
                    >
                      {row.getVisibleCells().map((cell) => (
                        <Table.Cell
                          key={cell.id}
                          className="align-middle text-sm py-2 px-2 sm:py-3 sm:px-4"
                        >
                          {flexRender(cell.column.columnDef.cell, cell.getContext())}
                        </Table.Cell>
                      ))}
                    </Table.Row>
                  ))}
                </Table.Body>
              </Table.Root>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
