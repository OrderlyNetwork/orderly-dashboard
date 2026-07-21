import { FC, useMemo, useState } from 'react';

import { Empty, TableSkeleton, TH, TD, TH_STICKY, tdSticky } from '../shared/primitives';

import {
  useFundingRatesRange,
  useSymbolWeekly,
  type FundingRateRow
} from '~/hooks/useOrderlyMetrics';
import {
  useFundingComparison,
  type FundingComparisonExchange,
  type FundingComparisonRow
} from '~/hooks/usePublicInfo';

const WINDOWS = ['last', '1d', '7d', '30d'] as const;
type Window = (typeof WINDOWS)[number];
const WINDOW_LABELS: Record<Window, string> = {
  last: 'Last',
  '1d': '1D avg',
  '7d': '7D avg',
  '30d': '30D avg'
};

const ORDERLY = 'orderly';
const SCROLL_HEIGHT = 440;
const ROW_HOVER_OVERLAY = 'inset 0 0 0 9999px rgba(156,117,255,0.12)';

const isOrderly = (name: string) => name.toLowerCase() === ORDERLY;
const baseToken = (symbol: string) => symbol.split('_')[1] ?? symbol;

function rateOf(e: FundingComparisonExchange, w: Window): string | undefined {
  return w === 'last' ? e.last : e[w];
}

function toNum(v: number | string | undefined): number {
  if (v == null) return NaN;
  return typeof v === 'number' ? v : Number(v);
}

function fmtRate(v: number | string | undefined): string {
  const n = toNum(v);
  if (!isFinite(n)) return '—';
  return `${n >= 0 ? '+' : ''}${(n * 100).toFixed(3)}%`;
}

function cellBg(v: number | string | undefined): string {
  const n = toNum(v);
  if (!isFinite(n)) return 'transparent';
  const alpha = Math.min(0.35, (Math.abs(n) * 100) / 0.1);
  return n >= 0 ? `rgba(46,160,67,${alpha})` : `rgba(244,63,94,${alpha})`;
}

function cellStyle(
  name: string,
  v: number | string | undefined,
  hovered: boolean
): React.CSSProperties {
  const base: React.CSSProperties = { ...TD };
  if (isOrderly(name)) {
    base.borderLeft = '1px solid rgba(156,117,255,0.4)';
    base.borderRight = '1px solid rgba(156,117,255,0.4)';
    base.fontWeight = 600;
    base.color = '#fff';
  }
  base.backgroundColor = cellBg(v);
  if (hovered) base.boxShadow = ROW_HOVER_OVERLAY;
  return base;
}

// Orderly is not part of the comparison `exchanges` array (it's the reference
// venue). Compute its last + 1d/7d/30d averages from the DATA_API funding-rate
// history (8h epochs).
function computeOrderlyRates(
  rows: FundingRateRow[] | undefined
): Map<string, Record<Window, number | undefined>> {
  const out = new Map<string, Record<Window, number | undefined>>();
  if (!rows?.length) return out;
  const bySym = new Map<string, { ts: number; rate: number }[]>();
  const now = Date.now();
  for (const r of rows) {
    if (!r.symbol || r.symbol === 'ALL' || r.funding_rate == null) continue;
    const ts = new Date(`${r.funding_time}Z`).getTime();
    if (!isFinite(ts)) continue;
    let arr = bySym.get(r.symbol);
    if (!arr) {
      arr = [];
      bySym.set(r.symbol, arr);
    }
    arr.push({ ts, rate: r.funding_rate });
  }
  for (const [sym, arr] of bySym) {
    arr.sort((a, b) => a.ts - b.ts);
    const latest = arr[arr.length - 1];
    const avg = (days: number) => {
      const cutoff = now - days * 86400000;
      const within = arr.filter((x) => x.ts >= cutoff);
      if (!within.length) return undefined;
      return within.reduce((s, x) => s + x.rate, 0) / within.length;
    };
    out.set(sym, { last: latest?.rate, '1d': avg(1), '7d': avg(7), '30d': avg(30) });
  }
  return out;
}

const thTop: React.CSSProperties = {
  ...TH,
  position: 'sticky',
  top: 0,
  zIndex: 2,
  background: '#140F1D'
};

const orderlyHeaderStyle: React.CSSProperties = {
  ...TH,
  position: 'sticky',
  top: 0,
  zIndex: 4,
  color: '#E9DEFF',
  background: '#4c0099',
  borderLeft: '1px solid rgba(156,117,255,0.4)',
  borderRight: '1px solid rgba(156,117,255,0.4)'
};

const WindowToggle: FC<{ value: Window; onChange: (w: Window) => void }> = ({
  value,
  onChange
}) => (
  <div className="flex gap-1 rounded-lg p-1" style={{ background: '#130E1D' }}>
    {WINDOWS.map((w) => (
      <button
        key={w}
        type="button"
        onClick={() => onChange(w)}
        className="px-2.5 py-1 rounded-md border-none cursor-pointer text-[12px] transition-all duration-150"
        style={{
          background: value === w ? '#6700CE' : 'transparent',
          color: value === w ? '#E9DEFF' : 'rgba(255,255,255,0.45)',
          fontWeight: value === w ? 600 : 400
        }}
      >
        {WINDOW_LABELS[w]}
      </button>
    ))}
  </div>
);

const MultiSymbolView: FC<{ rows: FundingComparisonRow[] }> = ({ rows }) => {
  const [windowSel, setWindowSel] = useState<Window>('last');
  const [hovered, setHovered] = useState<number | null>(null);
  const { data: volData } = useSymbolWeekly();
  const { data: frHistory } = useFundingRatesRange(30);

  const volumeBySymbol = useMemo(() => {
    const totals = new Map<string, number>();
    for (const r of volData?.rows ?? []) {
      if (r.volume_usd && r.symbol !== 'ALL') {
        totals.set(r.symbol, (totals.get(r.symbol) ?? 0) + r.volume_usd);
      }
    }
    return totals;
  }, [volData]);

  const orderlyRates = useMemo(() => computeOrderlyRates(frHistory?.rows), [frHistory]);

  const sortedRows = useMemo(
    () =>
      [...rows].sort(
        (a, b) => (volumeBySymbol.get(b.symbol) ?? 0) - (volumeBySymbol.get(a.symbol) ?? 0)
      ),
    [rows, volumeBySymbol]
  );

  const exchanges = useMemo(() => {
    const seen = new Set<string>();
    const rest: string[] = [];
    for (const r of rows) {
      for (const e of r.exchanges) {
        if (isOrderly(e.name) || seen.has(e.name)) continue;
        seen.add(e.name);
        rest.push(e.name);
      }
    }
    return [ORDERLY, ...rest];
  }, [rows]);

  return (
    <div>
      <div className="flex items-center justify-between mb-3 gap-2">
        <span className="text-[11px]" style={{ color: 'rgba(255,255,255,0.35)' }}>
          {sortedRows.length} symbols · sorted by 7d volume
        </span>
        <WindowToggle value={windowSel} onChange={setWindowSel} />
      </div>
      <div
        className="overflow-auto w-full"
        style={{ maxHeight: SCROLL_HEIGHT, scrollbarWidth: 'thin' }}
      >
        <table className="w-full border-collapse text-[13px]">
          <thead>
            <tr>
              <th style={TH_STICKY}>Symbol</th>
              {exchanges.map((name) => (
                <th
                  key={name}
                  style={isOrderly(name) ? orderlyHeaderStyle : thTop}
                  className="capitalize"
                >
                  {name}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {sortedRows.map((r, i) => {
              const exchangeMap = new Map(r.exchanges.map((e) => [e.name, e]));
              const isHovered = hovered === i;
              return (
                <tr
                  key={r.symbol}
                  onMouseEnter={() => setHovered(i)}
                  onMouseLeave={() => setHovered(null)}
                  style={{ cursor: 'pointer', position: 'relative' }}
                >
                  <td style={tdSticky(i)}>
                    <span style={{ color: '#fff' }} className={isHovered ? 'underline' : undefined}>
                      {baseToken(r.symbol)}
                    </span>
                  </td>
                  {exchanges.map((name, exIdx) => {
                    let v: number | string | undefined;
                    if (isOrderly(name)) {
                      v = orderlyRates.get(r.symbol)?.[windowSel];
                    } else {
                      const e = exchangeMap.get(name);
                      v = e ? rateOf(e, windowSel) : undefined;
                    }
                    return (
                      <td key={name} style={cellStyle(name, v, isHovered)}>
                        {exIdx === 0 && (
                          <a
                            href={`/markets/${baseToken(r.symbol)}`}
                            aria-label={`${baseToken(r.symbol)} market`}
                            style={{ position: 'absolute', inset: 0, zIndex: 1 }}
                          />
                        )}
                        {fmtRate(v)}
                      </td>
                    );
                  })}
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
};

type SingleRow = {
  name: string;
  values: Record<Window, number | string | undefined>;
};

const SingleSymbolView: FC<{ symbol: string; row?: FundingComparisonRow }> = ({ symbol, row }) => {
  const { data: frHistory } = useFundingRatesRange(30);

  const competitors = useMemo(() => row?.exchanges ?? [], [row]);

  const rowData: SingleRow[] = useMemo(() => {
    const orderlyRates = computeOrderlyRates(frHistory?.rows?.filter((r) => r.symbol === symbol));
    const o = orderlyRates.get(symbol);
    const orderly: SingleRow = {
      name: ORDERLY,
      values: {
        last: o?.last,
        '1d': o?.['1d'],
        '7d': o?.['7d'],
        '30d': o?.['30d']
      }
    };
    const others: SingleRow[] = competitors.map((e) => ({
      name: e.name,
      values: { last: e.last, '1d': e['1d'], '7d': e['7d'], '30d': e['30d'] }
    }));
    return [orderly, ...others];
  }, [frHistory, symbol, competitors]);

  if (!row || competitors.length === 0) {
    return <Empty msg="No comparison data for this market" />;
  }

  return (
    <div className="overflow-x-auto w-full">
      <table className="w-full border-collapse text-[13px]">
        <thead>
          <tr>
            <th style={TH_STICKY}>Exchange</th>
            {WINDOWS.map((w) => (
              <th key={w} style={TH}>
                {WINDOW_LABELS[w]}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rowData.map((r, i) => (
            <tr key={r.name}>
              <td style={tdSticky(i)}>
                <span
                  style={{
                    fontWeight: isOrderly(r.name) ? 600 : 400,
                    color: isOrderly(r.name) ? '#E9DEFF' : undefined
                  }}
                  className="capitalize"
                >
                  {r.name}
                </span>
              </td>
              {WINDOWS.map((w) => {
                const v = r.values[w];
                return (
                  <td key={w} style={cellStyle(r.name, v, false)}>
                    {fmtRate(v)}
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export const FundingComparisonWidget: FC<{ symbol?: string }> = ({ symbol: fixedSymbol }) => {
  const { data, isLoading, error } = useFundingComparison(fixedSymbol);

  if (isLoading) {
    return <TableSkeleton rows={fixedSymbol ? 6 : 11} height={fixedSymbol ? 240 : 420} />;
  }
  if (error || !data?.rows?.length) {
    return <Empty msg={error ? 'Failed to load' : 'No data'} />;
  }

  if (fixedSymbol) {
    return <SingleSymbolView symbol={fixedSymbol} row={data.rows[0]} />;
  }
  return <MultiSymbolView rows={data.rows} />;
};
