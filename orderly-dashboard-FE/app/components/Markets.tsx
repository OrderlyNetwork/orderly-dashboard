import { MixerHorizontalIcon } from '@radix-ui/react-icons';
import { Button, Popover, Table, Tooltip } from '@radix-ui/themes';
import { useNavigate } from '@remix-run/react';
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  SortingState,
  useReactTable
} from '@tanstack/react-table';
import { useMemo, useState, FC, useCallback } from 'react';

import { Spinner } from '~/components';
import { fmtBps, fmtPct, fmtUsd } from '~/components/analytics/shared/formatters';
import {
  useMarketSummary,
  usePriceChanges,
  useTradersOpenInterests,
  useFuturesMarket
} from '~/hooks/usePublicInfo';
import { formatPriceByTick } from '~/utils/format';

type MarketRow = {
  symbol: string;
  baseToken: string;
  markPrice: number;
  change1h: number | null;
  change4h: number | null;
  change24h: number | null;
  change7d: number | null;
  volume24h: number;
  openInterest: number;
  fundingRate: number;
  estFundingRate: number | null;
  longOI: number;
  shortOI: number;
  longRatio: number | null;
};

const defaultVisibility = {
  symbol: true,
  markPrice: true,
  change1h: true,
  change4h: true,
  change24h: true,
  change7d: true,
  volume24h: true,
  openInterest: true,
  fundingRate: true,
  estFundingRate: false,
  longShort: true
};

function computeChange(current: number, previous: number | null): number | null {
  if (previous == null || previous === 0) return null;
  return ((current - previous) / previous) * 100;
}

export const Markets: FC = () => {
  const navigate = useNavigate();
  const [sorting, setSorting] = useState<SortingState>([{ id: 'volume24h', desc: true }]);
  const [searchFilter, setSearchFilter] = useState('');

  const handleRowClick = useCallback(
    (baseToken: string) => {
      navigate(`/markets/${baseToken}`);
    },
    [navigate]
  );

  const { data: priceData, isLoading: priceLoading } = usePriceChanges();
  const { data: oiData, isLoading: oiLoading } = useTradersOpenInterests();
  const { data: futuresData, isLoading: futuresLoading } = useFuturesMarket();
  const { data: marketSummary } = useMarketSummary();

  const quoteTickFor = useMemo(() => {
    return (symbol: string): number | null => {
      const tick = marketSummary?.markets.find((m) => m.symbol === symbol)?.quote_tick;
      const parsed = tick != null ? parseFloat(tick) : NaN;
      return Number.isFinite(parsed) ? parsed : null;
    };
  }, [marketSummary]);

  const isLoading = priceLoading || oiLoading || futuresLoading;

  const rows = useMemo<MarketRow[]>(() => {
    if (!futuresData?.rows) return [];

    const priceMap = new Map<
      string,
      {
        last_price: number;
        '1h': number | null;
        '4h': number | null;
        '24h': number | null;
        '7d': number | null;
      }
    >();
    if (priceData?.rows) {
      for (const p of priceData.rows) {
        priceMap.set(p.symbol, {
          last_price: p.last_price,
          '1h': p['1h'],
          '4h': p['4h'],
          '24h': p['24h'],
          '7d': p['7d']
        });
      }
    }

    const oiMap = new Map<string, { long_oi: number; short_oi: number }>();
    if (oiData?.rows) {
      for (const o of oiData.rows) {
        oiMap.set(o.symbol, { long_oi: o.long_oi, short_oi: o.short_oi });
      }
    }

    return futuresData.rows.map((m) => {
      const markPrice = m.mark_price;
      const prices = priceMap.get(m.symbol);
      const oi = oiMap.get(m.symbol);

      const change1h = prices ? computeChange(markPrice, prices['1h']) : null;
      const change4h = prices ? computeChange(markPrice, prices['4h']) : null;
      const change24h = prices ? computeChange(markPrice, prices['24h']) : null;
      const change7d = prices ? computeChange(markPrice, prices['7d']) : null;

      const volume24h = m['24h_amount'];
      const openInterest = m.open_interest * markPrice;

      const fundingRate = m.last_funding_rate;
      const estFundingRate = m.est_funding_rate;

      const longOI = Math.abs(oi?.long_oi || 0);
      const shortOI = Math.abs(oi?.short_oi || 0);
      const totalOI = longOI + shortOI;
      const longRatio = totalOI > 0 ? longOI / totalOI : null;

      const parts = m.symbol.split('_');
      const baseToken = parts.length >= 2 ? parts[1] : m.symbol;

      return {
        symbol: m.symbol,
        baseToken,
        markPrice,
        change1h,
        change4h,
        change24h,
        change7d,
        volume24h,
        openInterest,
        fundingRate,
        estFundingRate,
        longOI,
        shortOI,
        longRatio
      };
    });
  }, [futuresData, priceData, oiData]);

  const filteredRows = useMemo(() => {
    if (!searchFilter.trim()) return rows;
    const q = searchFilter.toLowerCase();
    return rows.filter(
      (r) => r.baseToken.toLowerCase().includes(q) || r.symbol.toLowerCase().includes(q)
    );
  }, [rows, searchFilter]);

  const columns = useMemo<ColumnDef<MarketRow>[]>(
    () => [
      {
        accessorKey: 'symbol',
        header: 'Symbol',
        cell: ({ row }) => (
          <span className="font-mono text-sm font-semibold text-white">
            {row.original.baseToken}
          </span>
        ),
        sortingFn: 'alphanumeric'
      },
      {
        accessorKey: 'markPrice',
        header: 'Mark Price',
        cell: ({ row }) => (
          <span className="font-mono text-sm text-white">
            {formatPriceByTick(row.original.markPrice, quoteTickFor(row.original.symbol))}
          </span>
        )
      },
      {
        accessorKey: 'change1h',
        header: '1h',
        cell: ({ row }) => {
          const v = row.original.change1h;
          if (v == null) return <span className="text-sm text-gray-500">—</span>;
          return (
            <span className={`text-sm font-medium ${v >= 0 ? 'text-[#00dea3]' : 'text-[#FF6390]'}`}>
              {fmtPct(v)}
            </span>
          );
        }
      },
      {
        accessorKey: 'change4h',
        header: '4h',
        cell: ({ row }) => {
          const v = row.original.change4h;
          if (v == null) return <span className="text-sm text-gray-500">—</span>;
          return (
            <span className={`text-sm font-medium ${v >= 0 ? 'text-[#00dea3]' : 'text-[#FF6390]'}`}>
              {fmtPct(v)}
            </span>
          );
        }
      },
      {
        accessorKey: 'change24h',
        header: '24h',
        cell: ({ row }) => {
          const v = row.original.change24h;
          if (v == null) return <span className="text-sm text-gray-500">—</span>;
          return (
            <span className={`text-sm font-medium ${v >= 0 ? 'text-[#00dea3]' : 'text-[#FF6390]'}`}>
              {fmtPct(v)}
            </span>
          );
        }
      },
      {
        accessorKey: 'change7d',
        header: '7d',
        cell: ({ row }) => {
          const v = row.original.change7d;
          if (v == null) return <span className="text-sm text-gray-500">—</span>;
          return (
            <span className={`text-sm font-medium ${v >= 0 ? 'text-[#00dea3]' : 'text-[#FF6390]'}`}>
              {fmtPct(v)}
            </span>
          );
        }
      },
      {
        accessorKey: 'volume24h',
        header: '24h Volume',
        cell: ({ row }) => (
          <span className="text-sm text-white">{fmtUsd(row.original.volume24h)}</span>
        )
      },
      {
        accessorKey: 'openInterest',
        header: 'Open Interest',
        cell: ({ row }) => (
          <span className="text-sm text-white">{fmtUsd(row.original.openInterest)}</span>
        )
      },
      {
        accessorKey: 'fundingRate',
        header: (
          <Tooltip content="Last settled funding rate (8h)">
            <span className="cursor-help">Funding</span>
          </Tooltip>
        ) as unknown as string,
        cell: ({ row }) => (
          <span
            className={`text-sm font-medium ${row.original.fundingRate >= 0 ? 'text-[#00dea3]' : 'text-[#FF6390]'}`}
          >
            {fmtBps(row.original.fundingRate)}
          </span>
        )
      },
      {
        accessorKey: 'estFundingRate',
        header: (
          <Tooltip content="Estimated funding rate for next settlement">
            <span className="cursor-help">Est. Funding</span>
          </Tooltip>
        ) as unknown as string,
        cell: ({ row }) => {
          const v = row.original.estFundingRate;
          if (v == null) return <span className="text-sm text-gray-500">—</span>;
          return (
            <span className={`text-sm font-medium ${v >= 0 ? 'text-[#00dea3]' : 'text-[#FF6390]'}`}>
              {fmtBps(v)}
            </span>
          );
        }
      },
      {
        id: 'longShort',
        accessorKey: 'longRatio',
        header: (
          <Tooltip content="Long vs Short ratio (market makers excluded)">
            <span className="cursor-help">Long / Short</span>
          </Tooltip>
        ) as unknown as string,
        cell: ({ row }) => {
          const ratio = row.original.longRatio;
          if (ratio == null) return <span className="text-sm text-gray-500">—</span>;
          const longPct = (ratio * 100).toFixed(1);
          return (
            <div className="flex items-center gap-2 min-w-[100px]">
              <div
                className="flex-1 h-2 rounded-full overflow-hidden flex"
                style={{ background: '#221E30' }}
              >
                <div style={{ width: `${ratio * 100}%`, background: '#00dea3' }} />
                <div style={{ width: `${(1 - ratio) * 100}%`, background: '#FF6390' }} />
              </div>
              <span className="text-xs text-[#00dea3] w-10 text-right">{longPct}%</span>
            </div>
          );
        }
      }
    ],
    [quoteTickFor]
  );

  const table = useReactTable<MarketRow>({
    data: filteredRows,
    columns,
    state: { sorting },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    initialState: { columnVisibility: defaultVisibility }
  });

  if (isLoading && !futuresData) {
    return (
      <div className="flex justify-center py-12">
        <Spinner size="2.5rem" />
      </div>
    );
  }

  return (
    <div className="space-y-4 sm:space-y-6 animate-fade-in">
      <div className="card w-full space-y-4" style={{ background: '#130E1D', border: 'none' }}>
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
          <div className="flex items-center gap-3">
            <input
              type="text"
              placeholder="Search symbol..."
              value={searchFilter}
              onChange={(e) => setSearchFilter(e.target.value)}
              className="lb-input w-48 px-3 py-2 text-sm"
              style={{ background: '#221E30', border: 'none' }}
            />
            <span className="text-xs text-gray-500">{filteredRows.length} markets</span>
          </div>
          <Popover.Root>
            <Popover.Trigger className="w-auto">
              <Button variant="soft" className="btn btn-secondary">
                <MixerHorizontalIcon width="16" height="16" />
                Columns
              </Button>
            </Popover.Trigger>
            <Popover.Content width="18rem" maxHeight="26rem" className="max-w-[90vw] card">
              <div className="flex flex-col gap-2">
                <Button onClick={() => table.resetColumnVisibility()} className="btn btn-primary">
                  Reset
                </Button>
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={table.getIsAllColumnsVisible()}
                    onChange={table.getToggleAllColumnsVisibilityHandler()}
                    className="rounded"
                  />
                  <span className="text-sm text-white">Toggle All</span>
                </label>
                <hr className="w-full border-border-primary" />
                {table.getAllLeafColumns().map((column) => (
                  <label key={column.id} className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={column.getIsVisible()}
                      onChange={column.getToggleVisibilityHandler()}
                      className="rounded"
                    />
                    <span className="text-sm text-white">
                      {typeof column.columnDef.header === 'string'
                        ? column.columnDef.header
                        : column.id}
                    </span>
                  </label>
                ))}
              </div>
            </Popover.Content>
          </Popover.Root>
        </div>

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
                      className="py-3 px-3"
                    >
                      {header.isPlaceholder ? null : (
                        <div
                          className={
                            header.column.getCanSort()
                              ? 'cursor-pointer select-none hover:text-primary-light transition-colors duration-150 text-xs font-medium uppercase tracking-wider'
                              : 'text-xs font-medium uppercase tracking-wider'
                          }
                          onClick={header.column.getToggleSortingHandler()}
                          onKeyDown={(ev) => {
                            if (ev.key === 'Enter') header.column.getToggleSortingHandler();
                          }}
                          role="button"
                          tabIndex={0}
                        >
                          {flexRender(header.column.columnDef.header, header.getContext())}
                          {{
                            asc: ' ↑',
                            desc: ' ↓'
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
                  onClick={() => handleRowClick(row.original.baseToken)}
                  className={`border-b border-border-primary hover:bg-[rgba(156,117,255,0.12)] transition-colors duration-150 cursor-pointer ${
                    index % 2 === 0 ? 'bg-bg-secondary' : 'bg-bg-primary'
                  }`}
                >
                  {row.getVisibleCells().map((cell) => (
                    <Table.Cell key={cell.id} className="align-middle py-2.5 px-3">
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </Table.Cell>
                  ))}
                </Table.Row>
              ))}
            </Table.Body>
          </Table.Root>
        </div>

        {filteredRows.length === 0 && (
          <div className="text-center py-8 text-gray-400">
            <p>No markets found{searchFilter ? ` for "${searchFilter}"` : ''}.</p>
          </div>
        )}
      </div>
    </div>
  );
};
