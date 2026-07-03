import { FC, useEffect, useMemo, useState } from 'react';

import { fmtCompact } from '../shared/formatters';
import { Empty, Skeleton } from '../shared/primitives';

import { useSymbolWeekly } from '~/hooks/useOrderlyMetrics';
import { usePlatformPositions, type PlatformPosition } from '~/hooks/usePublicInfo';

const NUM_BINS = 48;
// Keep only liquidation prices within this factor of the mark price in both
// directions (mark/FACTOR .. mark*FACTOR). Filters far outliers (e.g. a 60x
// liq price) so the axis hugs the meaningful cluster near current price.
const LIQ_BAND_FACTOR = 2;

const LONG_COLOR = '#f6465d';
const SHORT_COLOR = '#0ecb81';

type Bin = {
  long: number;
  short: number;
  count: number;
  priceLo: number;
  priceHi: number;
};

type Heatmap = {
  bins: Bin[];
  markPrice: number;
  lo: number;
  hi: number;
  maxNotional: number;
  withLiqCount: number;
  noLiqCount: number;
  outOfBandCount: number;
  liqLong: number;
  liqShort: number;
};

function computeHeatmap(rows: PlatformPosition[]): Heatmap {
  const markPrice = parseFloat(rows[0]?.mark_price ?? '0') || 0;
  const loBand = markPrice / LIQ_BAND_FACTOR;
  const hiBand = markPrice * LIQ_BAND_FACTOR;

  let noLiqCount = 0;
  let outOfBandCount = 0;
  // Keep positions that have a real liquidation price within ±100% of mark.
  const withLiq = rows.filter((r) => {
    const p = parseFloat(r.est_liq_price ?? '');
    if (!Number.isFinite(p) || p <= 0) {
      noLiqCount += 1;
      return false;
    }
    if (p < loBand || p > hiBand) {
      outOfBandCount += 1;
      return false;
    }
    return true;
  });
  // at-risk notional = only positions that actually have a liquidation price in range
  let liqLong = 0;
  let liqShort = 0;
  withLiq.forEach((r) => {
    const n = parseFloat(r.notional) || 0;
    if (r.side === 'LONG') liqLong += n;
    else liqShort += n;
  });
  const empty: Heatmap = {
    bins: [],
    markPrice,
    lo: 0,
    hi: 0,
    maxNotional: 0,
    withLiqCount: withLiq.length,
    noLiqCount,
    outOfBandCount,
    liqLong,
    liqShort
  };

  if (withLiq.length === 0 || markPrice === 0) return empty;

  const liqPrices = withLiq.map((r) => parseFloat(r.est_liq_price as string)).sort((a, b) => a - b);

  // Range = min/max of the in-band liq prices, always including mark.
  let lo = liqPrices[0];
  let hi = liqPrices[liqPrices.length - 1];
  lo = Math.min(lo, markPrice);
  hi = Math.max(hi, markPrice);
  if (hi <= lo) hi = lo * 1.01;

  // pad so extremes aren't flush against the edges
  const pad = (hi - lo) * 0.04;
  lo -= pad;
  hi += pad;

  const binWidth = (hi - lo) / NUM_BINS;
  const bins: Bin[] = Array.from({ length: NUM_BINS }, (_, i) => ({
    long: 0,
    short: 0,
    count: 0,
    priceLo: lo + i * binWidth,
    priceHi: lo + (i + 1) * binWidth
  }));

  withLiq.forEach((r) => {
    const p = parseFloat(r.est_liq_price as string);
    const n = parseFloat(r.notional) || 0;
    let idx = Math.floor((p - lo) / binWidth);
    idx = Math.max(0, Math.min(NUM_BINS - 1, idx));
    if (r.side === 'LONG') bins[idx].long += n;
    else bins[idx].short += n;
    bins[idx].count += 1;
  });

  const maxNotional = Math.max(1, ...bins.map((b) => Math.max(b.long, b.short)));
  return {
    bins,
    markPrice,
    lo,
    hi,
    maxNotional,
    withLiqCount: withLiq.length,
    noLiqCount,
    outOfBandCount,
    liqLong,
    liqShort
  };
}

function fmtPrice(p: number): string {
  if (!Number.isFinite(p)) return '—';
  if (p >= 1000) return p.toLocaleString('en', { maximumFractionDigits: 0 });
  if (p >= 1) return p.toFixed(2);
  return p.toFixed(4);
}

function fmtAgo(ts: number | null): string {
  if (!ts) return '';
  const s = Math.round((Date.now() - ts) / 1000);
  if (s < 60) return `${s}s ago`;
  const m = Math.floor(s / 60);
  if (m < 60) return `${m}m ago`;
  const h = Math.floor(m / 60);
  return `${h}h ago`;
}

const selectStyle: React.CSSProperties = {
  background: '#130E1D',
  border: '1px solid rgba(156,117,255,0.18)',
  color: '#fff',
  borderRadius: 6,
  padding: '4px 8px',
  fontSize: 12,
  outline: 'none',
  cursor: 'pointer'
};

export const LiquidationHeatmapWidget: FC = () => {
  const { data: volData } = useSymbolWeekly();
  const symbolOptions = useMemo(() => {
    const totals = new Map<string, number>();
    volData?.rows.forEach((r) => {
      if (r.volume_usd && r.symbol !== 'ALL') {
        totals.set(r.symbol, (totals.get(r.symbol) ?? 0) + r.volume_usd);
      }
    });
    return [...totals.entries()]
      .sort((a, b) => b[1] - a[1])
      .map(([sym, vol]) => ({
        symbol: sym,
        label: `${sym.replace('PERP_', '').replace('_USDC', '')} · ${fmtCompact(vol)}`
      }));
  }, [volData]);
  const [symbol, setSymbol] = useState('PERP_BTC_USDC');
  const [hovered, setHovered] = useState<number | null>(null);
  const [lastUpdated, setLastUpdated] = useState<number | null>(null);

  const { data, isLoading, error, mutate } = usePlatformPositions(symbol, 1000);

  useEffect(() => {
    if (data) setLastUpdated(Date.now());
  }, [data]);

  const rows = data?.rows;
  const heatmap = useMemo(() => computeHeatmap(rows ?? []), [rows]);
  const {
    bins,
    markPrice,
    lo,
    hi,
    maxNotional,
    withLiqCount,
    noLiqCount,
    outOfBandCount,
    liqLong,
    liqShort
  } = heatmap;

  const lsRatio = liqShort > 0 ? liqLong / liqShort : 0;
  const totalPositions = data?.total_positions ?? 0;

  // price axis labels (high -> low, top -> bottom)
  const numLabels = 5;
  const priceLabels = useMemo(() => {
    if (hi <= lo) return [];
    return Array.from({ length: numLabels }, (_, i) => {
      const frac = i / (numLabels - 1);
      const price = hi - frac * (hi - lo);
      return { price, top: frac * 100 };
    });
  }, [lo, hi]);

  const markFrac =
    hi > lo && markPrice > 0 && markPrice >= lo && markPrice <= hi
      ? ((hi - markPrice) / (hi - lo)) * 100
      : null;

  const hoveredBin = hovered != null ? bins[hovered] : null;

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
      {/* controls */}
      <div className="flex items-center gap-2 flex-wrap">
        <select value={symbol} onChange={(e) => setSymbol(e.target.value)} style={selectStyle}>
          {symbolOptions.map((o) => (
            <option key={o.symbol} value={o.symbol}>
              {o.label}
            </option>
          ))}
        </select>

        <button
          onClick={() => mutate()}
          title="Refresh"
          className="flex items-center justify-center rounded-md cursor-pointer"
          style={{
            width: 28,
            height: 28,
            background: '#221E30',
            border: 'none',
            color: 'rgba(255,255,255,0.5)'
          }}
        >
          <svg
            width="13"
            height="13"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <polyline points="23 4 23 10 17 10" />
            <polyline points="1 20 1 14 7 14" />
            <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
          </svg>
        </button>

        <span className="text-[11px]" style={{ color: 'rgba(255,255,255,0.3)' }}>
          {fmtAgo(lastUpdated)}
        </span>
      </div>

      {isLoading ? (
        <Skeleton height={440} />
      ) : error ? (
        <Empty msg="Failed to load" />
      ) : bins.length === 0 ? (
        <Empty msg="No liquidation data for this market" />
      ) : (
        <>
          {/* summary strip */}
          <div className="flex gap-4 flex-wrap text-[12px]">
            <Stat label="Mark" value={fmtPrice(markPrice)} />
            <Stat label="Long liq." value={fmtCompact(liqLong)} color={LONG_COLOR} />
            <Stat label="Short liq." value={fmtCompact(liqShort)} color={SHORT_COLOR} />
            <Stat label="L/S Ratio" value={lsRatio ? lsRatio.toFixed(2) : '—'} />
            <Stat label="Liq. levels" value={`${withLiqCount}/${totalPositions}`} />
          </div>

          {/* legend */}
          <div
            className="flex items-center gap-4 text-[11px] flex-wrap"
            style={{ color: 'rgba(255,255,255,0.45)' }}
          >
            <span className="flex items-center gap-1.5">
              <span
                style={{
                  width: 10,
                  height: 10,
                  borderRadius: 2,
                  background: LONG_COLOR,
                  display: 'inline-block'
                }}
              />
              Long liquidations (below)
            </span>
            <span className="flex items-center gap-1.5">
              <span
                style={{
                  width: 10,
                  height: 10,
                  borderRadius: 2,
                  background: SHORT_COLOR,
                  display: 'inline-block'
                }}
              />
              Short liquidations (above)
            </span>
            {(noLiqCount > 0 || outOfBandCount > 0) && (
              <span style={{ color: 'rgba(255,255,255,0.3)' }}>
                {[
                  noLiqCount > 0 && `${noLiqCount} no liq price`,
                  outOfBandCount > 0 && `${outOfBandCount} out of range`
                ]
                  .filter(Boolean)
                  .join(' · ')}{' '}
                excluded
              </span>
            )}
          </div>

          {/* ladder */}
          <div style={{ position: 'relative', display: 'flex', gap: 8, height: 440 }}>
            {/* price axis */}
            <div style={{ width: 58, position: 'relative', flexShrink: 0 }}>
              {priceLabels.map((l) => (
                <div
                  key={l.top}
                  className="text-[10px]"
                  style={{
                    position: 'absolute',
                    right: 0,
                    top: `${l.top}%`,
                    transform: 'translateY(-50%)',
                    color: 'rgba(255,255,255,0.3)',
                    whiteSpace: 'nowrap'
                  }}
                >
                  {fmtPrice(l.price)}
                </div>
              ))}
            </div>

            {/* chart */}
            <div style={{ flex: 1, position: 'relative', minWidth: 0 }}>
              {/* center vertical line */}
              <div
                style={{
                  position: 'absolute',
                  left: '50%',
                  top: 0,
                  bottom: 0,
                  width: 1,
                  background: 'rgba(255,255,255,0.08)'
                }}
              />

              {/* bins, reversed so high price is on top */}
              {[...bins].reverse().map((bin, i) => {
                const realIdx = NUM_BINS - 1 - i;
                const longPct = maxNotional > 0 ? (bin.long / maxNotional) * 100 : 0;
                const shortPct = maxNotional > 0 ? (bin.short / maxNotional) * 100 : 0;
                const longOpacity = bin.long > 0 ? 0.3 + 0.65 * (bin.long / maxNotional) : 0;
                const shortOpacity = bin.short > 0 ? 0.3 + 0.65 * (bin.short / maxNotional) : 0;
                return (
                  <div
                    key={realIdx}
                    onMouseEnter={() => setHovered(realIdx)}
                    onMouseLeave={() => setHovered(null)}
                    style={{
                      height: `${100 / NUM_BINS}%`,
                      display: 'flex',
                      alignItems: 'center',
                      cursor: 'default'
                    }}
                  >
                    <div
                      style={{
                        flex: 1,
                        display: 'flex',
                        justifyContent: 'flex-end',
                        height: '100%',
                        alignItems: 'center'
                      }}
                    >
                      {longPct > 0 && (
                        <div
                          style={{
                            width: `${longPct}%`,
                            height: '70%',
                            background: LONG_COLOR,
                            opacity: longOpacity,
                            borderRadius: '2px 0 0 2px'
                          }}
                        />
                      )}
                    </div>
                    <div
                      style={{
                        flex: 1,
                        display: 'flex',
                        justifyContent: 'flex-start',
                        height: '100%',
                        alignItems: 'center'
                      }}
                    >
                      {shortPct > 0 && (
                        <div
                          style={{
                            width: `${shortPct}%`,
                            height: '70%',
                            background: SHORT_COLOR,
                            opacity: shortOpacity,
                            borderRadius: '0 2px 2px 0'
                          }}
                        />
                      )}
                    </div>
                  </div>
                );
              })}

              {/* mark price line */}
              {markFrac != null && (
                <div
                  style={{
                    position: 'absolute',
                    left: 0,
                    right: 0,
                    top: `${markFrac}%`,
                    borderTop: '1px dashed rgba(156,117,255,0.7)',
                    pointerEvents: 'none'
                  }}
                >
                  <span
                    className="text-[10px]"
                    style={{
                      position: 'absolute',
                      right: 4,
                      top: -8,
                      background: '#6700CE',
                      color: '#fff',
                      padding: '1px 5px',
                      borderRadius: 4,
                      whiteSpace: 'nowrap'
                    }}
                  >
                    {fmtPrice(markPrice)}
                  </span>
                </div>
              )}

              {/* hover tooltip */}
              {hoveredBin && (
                <div
                  style={{
                    position: 'absolute',
                    top: 4,
                    left: '50%',
                    transform: 'translateX(-50%)',
                    background: '#130E1D',
                    border: '1px solid rgba(156,117,255,0.25)',
                    borderRadius: 8,
                    padding: '6px 10px',
                    fontSize: 11,
                    color: '#fff',
                    pointerEvents: 'none',
                    zIndex: 5,
                    whiteSpace: 'nowrap',
                    boxShadow: '0 4px 16px rgba(0,0,0,0.4)'
                  }}
                >
                  <div style={{ color: 'rgba(255,255,255,0.5)', marginBottom: 2 }}>
                    {fmtPrice(hoveredBin.priceLo)} – {fmtPrice(hoveredBin.priceHi)}
                    {markPrice > 0 && (
                      <span style={{ marginLeft: 8 }}>
                        {(
                          (((hoveredBin.priceLo + hoveredBin.priceHi) / 2 - markPrice) /
                            markPrice) *
                          100
                        ).toFixed(1)}
                        %
                      </span>
                    )}
                  </div>
                  <div style={{ display: 'flex', gap: 12 }}>
                    <span style={{ color: LONG_COLOR }}>Long {fmtCompact(hoveredBin.long)}</span>
                    <span style={{ color: SHORT_COLOR }}>Short {fmtCompact(hoveredBin.short)}</span>
                    <span style={{ color: 'rgba(255,255,255,0.5)' }}>{hoveredBin.count} pos</span>
                  </div>
                </div>
              )}
            </div>
          </div>
        </>
      )}
    </div>
  );
};

const Stat: FC<{ label: string; value: string; color?: string }> = ({ label, value, color }) => (
  <div className="flex flex-col">
    <span
      className="uppercase tracking-wider"
      style={{ color: 'rgba(255,255,255,0.35)', fontSize: 10, letterSpacing: '0.06em' }}
    >
      {label}
    </span>
    <span style={{ color: color ?? 'rgba(255,255,255,0.85)', fontWeight: 600 }}>{value}</span>
  </div>
);
