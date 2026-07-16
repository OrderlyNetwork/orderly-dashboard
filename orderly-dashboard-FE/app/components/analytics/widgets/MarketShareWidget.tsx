import { ArcElement, Chart as ChartJS, Tooltip, type ChartData, type ChartOptions } from 'chart.js';
import { FC, useEffect, useMemo, useRef, useState } from 'react';
import { Doughnut } from 'react-chartjs-2';

import { CHART_COLORS, useChartReady } from '../shared/chartConfig';
import { fmtCompact } from '../shared/formatters';
import { Empty, Skeleton, TD, TH_STICKY, tdSticky } from '../shared/primitives';

import { useMarketShare } from '~/hooks/useMarketShare';
import { useIsMobile } from '~/hooks/useMediaQuery';

ChartJS.register(ArcElement, Tooltip);

export const MarketShareWidget: FC = () => {
  const { data, isLoading, error } = useMarketShare();
  const chartRef = useRef<ChartJS<'doughnut'>>(null);
  const leftColRef = useRef<HTMLDivElement>(null);
  const [leftHeight, setLeftHeight] = useState<number | undefined>(undefined);
  useChartReady(chartRef);
  const isMobile = useIsMobile();
  const [hidden, setHidden] = useState<Set<number>>(() => new Set());
  const [search, setSearch] = useState('');
  const [sortOrder, setSortOrder] = useState<'default' | 'az'>('default');

  useEffect(() => {
    if (isMobile || !leftColRef.current) return;
    const observer = new ResizeObserver((entries) => {
      setLeftHeight(entries[0].contentRect.height);
    });
    observer.observe(leftColRef.current);
    return () => observer.disconnect();
  }, [isMobile, data]);

  const allProtocols = data?.dexProtocols ?? [];
  const orderlyIndex = allProtocols.findIndex((p) => p.isOrderly);
  const orderlyProto = orderlyIndex >= 0 ? allProtocols[orderlyIndex] : null;

  const topProtocols = useMemo(() => {
    if (!data) return [];
    const protocols = data.dexProtocols;
    const orderlyIdx = protocols.findIndex((p) => p.isOrderly);
    if (orderlyIdx < 0) return protocols.slice(0, 15);
    if (orderlyIdx < 15) return protocols.slice(0, 15);
    const top14 = protocols.slice(0, 14);
    const orderly = protocols[orderlyIdx];
    return [...top14, orderly];
  }, [data]);

  const chips = topProtocols.map((p, i) => ({
    label: p.name,
    color: p.isOrderly ? '#9C75FF' : CHART_COLORS[i % CHART_COLORS.length],
    visible: !hidden.has(i),
    originalIndex: i
  }));

  const displayChips = useMemo(() => {
    let result = [...chips];
    if (search)
      result = result.filter((chip) => chip.label.toLowerCase().includes(search.toLowerCase()));
    if (sortOrder === 'az') result = [...result].sort((a, b) => a.label.localeCompare(b.label));
    return result;
  }, [chips, search, sortOrder]);

  if (isLoading) return <Skeleton height={300} />;
  if (error || !data) return <Empty msg="Failed to load market share data" />;
  if (data.dexProtocols.length === 0) return <Empty msg="No DEX volume data" />;

  const topProtocolSlugs = new Set(topProtocols.map((p) => p.slug));
  const othersVolume = allProtocols
    .filter((p) => !topProtocolSlugs.has(p.slug))
    .reduce((sum, p) => sum + p.volume24h, 0);

  const visible = topProtocols.map((p, i) => ({ p, i })).filter(({ i }) => !hidden.has(i));

  const data_chart: ChartData<'doughnut'> = {
    labels: visible.map(({ p }) => p.name),
    datasets: [
      {
        data: visible.map(({ p }) => p.volume24h),
        backgroundColor: visible.map(({ p, i }) => {
          if (p.isOrderly) return '#9C75FFCC';
          return CHART_COLORS[i % CHART_COLORS.length] + 'CC';
        }),
        hoverBackgroundColor: visible.map(({ p, i }) => {
          if (p.isOrderly) return '#9C75FF';
          return CHART_COLORS[i % CHART_COLORS.length];
        }),
        borderWidth: 0,
        spacing: 2
      }
    ]
  };

  const options: ChartOptions<'doughnut'> = {
    responsive: true,
    maintainAspectRatio: false,
    cutout: '55%',
    plugins: {
      legend: { display: false },
      tooltip: {
        callbacks: {
          label: (ctx) => {
            const total = ctx.dataset.data.reduce((s, v) => s + (v as number), 0);
            const pct = (((ctx.raw as number) / total) * 100).toFixed(2);
            return ` ${fmtCompact(ctx.raw as number)} (${pct}%)`;
          }
        }
      }
    }
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16, width: '100%' }}>
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: isMobile ? '1fr' : '340px 1fr',
          gap: 32,
          width: '100%',
          alignItems: 'start'
        }}
      >
        <div ref={leftColRef} style={{ minWidth: 300 }}>
          <div
            style={{
              position: 'relative',
              height: 220,
              marginBottom: 8
            }}
          >
            <Doughnut ref={chartRef} data={data_chart} options={options} />
          </div>
          <div style={{ display: 'flex', gap: 8, marginBottom: 8 }}>
            <div
              style={{
                flex: 1,
                background: 'rgba(156,117,255,0.08)',
                borderRadius: 8,
                padding: '6px 10px'
              }}
            >
              <div
                style={{
                  fontSize: 10,
                  color: 'rgba(255,255,255,0.45)',
                  textTransform: 'uppercase',
                  letterSpacing: '0.06em',
                  marginBottom: 2
                }}
              >
                Total DEX Volume
              </div>
              <div style={{ fontSize: 16, fontWeight: 700, color: '#fff' }}>
                {fmtCompact(data.totalDexVolume)}
              </div>
            </div>
            <div
              style={{
                flex: 1,
                background: 'rgba(156,117,255,0.08)',
                borderRadius: 8,
                padding: '6px 10px'
              }}
            >
              <div
                style={{
                  fontSize: 10,
                  color: 'rgba(255,255,255,0.45)',
                  textTransform: 'uppercase',
                  letterSpacing: '0.06em',
                  marginBottom: 2
                }}
              >
                Total DEX OI
              </div>
              <div style={{ fontSize: 16, fontWeight: 700, color: '#fff' }}>
                {fmtCompact(data.totalDexOI)}
              </div>
            </div>
          </div>
          {orderlyProto && (
            <div
              style={{
                background: 'rgba(156,117,255,0.15)',
                borderRadius: 8,
                padding: '6px 10px',
                border: '1px solid rgba(156,117,255,0.3)',
                marginBottom: 8
              }}
            >
              <div
                style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                  marginBottom: 2
                }}
              >
                <div
                  style={{
                    fontSize: 10,
                    color: 'rgba(255,255,255,0.45)',
                    textTransform: 'uppercase',
                    letterSpacing: '0.06em'
                  }}
                >
                  Orderly Market Share
                </div>
                <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.35)' }}>
                  Rank #{data.orderlyRank} · {fmtCompact(orderlyProto.volume24h)}
                </div>
              </div>
              <div style={{ fontSize: 20, fontWeight: 700, color: '#9C75FF' }}>
                {orderlyProto.marketShare.toFixed(2)}%
              </div>
            </div>
          )}
          <div>
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 6,
                marginBottom: 6,
                flexWrap: 'wrap'
              }}
            >
              <button
                onClick={() => setHidden(new Set())}
                style={{
                  padding: '4px 10px',
                  borderRadius: 5,
                  fontSize: 10,
                  cursor: 'pointer',
                  background: '#221E30',
                  border: 'none',
                  color: '#fff'
                }}
              >
                Select all
              </button>
              <button
                onClick={() => setHidden(new Set(topProtocols.map((_, i) => i)))}
                style={{
                  padding: '4px 10px',
                  borderRadius: 5,
                  fontSize: 10,
                  cursor: 'pointer',
                  background: '#221E30',
                  border: 'none',
                  color: '#fff'
                }}
              >
                Deselect all
              </button>
              <div
                style={{
                  display: 'flex',
                  gap: 2,
                  borderRadius: 5,
                  padding: 2,
                  background: '#130E1D'
                }}
              >
                {(['default', 'az'] as const).map((s) => (
                  <button
                    key={s}
                    onClick={() => setSortOrder(s)}
                    style={{
                      padding: '3px 8px',
                      borderRadius: 4,
                      fontSize: 10,
                      cursor: 'pointer',
                      background: sortOrder === s ? '#6700CE' : 'transparent',
                      border: 'none',
                      color: sortOrder === s ? '#E9DEFF' : 'rgba(255,255,255,0.45)',
                      fontWeight: sortOrder === s ? 600 : 400
                    }}
                  >
                    {s === 'default' ? 'Qty' : 'A–Z'}
                  </button>
                ))}
              </div>
              <input
                type="text"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                placeholder="Search…"
                style={{
                  width: 100,
                  padding: '4px 8px',
                  borderRadius: 5,
                  fontSize: 10,
                  background: '#130E1D',
                  border: 'none',
                  color: '#fff',
                  outline: 'none'
                }}
              />
            </div>
            <div
              style={{
                display: 'flex',
                flexWrap: 'wrap',
                gap: 5,
                maxHeight: 120,
                overflowY: 'auto',
                scrollbarWidth: 'thin',
                scrollbarColor: 'rgba(156,117,255,0.25) #130E1D'
              }}
            >
              {displayChips.map((chip) => (
                <button
                  key={chip.label}
                  onClick={() =>
                    setHidden((prev) => {
                      const next = new Set(prev);
                      if (next.has(chip.originalIndex)) next.delete(chip.originalIndex);
                      else next.add(chip.originalIndex);
                      return next;
                    })
                  }
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: 5,
                    padding: '4px 10px',
                    borderRadius: 5,
                    fontSize: 10,
                    cursor: 'pointer',
                    background: chip.visible ? `${chip.color}18` : '#130E1D',
                    border: 'none',
                    color: '#fff',
                    textDecoration: chip.visible ? 'none' : 'line-through'
                  }}
                >
                  <span
                    style={{
                      width: 7,
                      height: 7,
                      borderRadius: '50%',
                      background: chip.visible ? chip.color : 'rgba(255,255,255,0.15)'
                    }}
                  />
                  {chip.label}
                </button>
              ))}
            </div>
          </div>
        </div>
        <div
          style={{
            minWidth: 300,
            maxHeight: isMobile ? 400 : leftHeight,
            overflowY: 'auto',
            scrollbarWidth: 'thin'
          }}
        >
          <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 12 }}>
            <thead>
              <tr>
                <th style={{ ...TH_STICKY, width: 40 }}>#</th>
                <th style={TH_STICKY}>Protocol</th>
                <th style={{ ...TH_STICKY, textAlign: 'right' }}>24h Volume</th>
                <th style={{ ...TH_STICKY, textAlign: 'right' }}>Open Interest</th>
                <th style={{ ...TH_STICKY, textAlign: 'right' }}>Market Share</th>
              </tr>
            </thead>
            <tbody>
              {data.dexProtocols.map((p, i) => (
                <tr key={p.slug}>
                  <td
                    style={{
                      ...tdSticky(i),
                      width: 40,
                      color: 'rgba(255,255,255,0.4)',
                      background: p.isOrderly ? 'rgba(156,117,255,0.1)' : undefined
                    }}
                  >
                    {i + 1}
                  </td>
                  <td
                    style={{
                      ...tdSticky(i),
                      color: p.isOrderly ? '#9C75FF' : '#fff',
                      fontWeight: p.isOrderly ? 700 : 400,
                      background: p.isOrderly ? 'rgba(156,117,255,0.1)' : undefined
                    }}
                  >
                    <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                      <img
                        src={p.logo}
                        alt={p.name}
                        style={{ width: 16, height: 16, borderRadius: 4 }}
                        onError={(e) => {
                          (e.target as HTMLImageElement).style.display = 'none';
                        }}
                      />
                      {p.name}
                    </div>
                  </td>
                  <td
                    style={{
                      ...TD,
                      textAlign: 'right',
                      color: '#fff',
                      background: p.isOrderly ? 'rgba(156,117,255,0.1)' : undefined
                    }}
                  >
                    {fmtCompact(p.volume24h)}
                  </td>
                  <td
                    style={{
                      ...TD,
                      textAlign: 'right',
                      color: 'rgba(255,255,255,0.7)',
                      background: p.isOrderly ? 'rgba(156,117,255,0.1)' : undefined
                    }}
                  >
                    {fmtCompact(p.openInterest)}
                  </td>
                  <td
                    style={{
                      ...TD,
                      textAlign: 'right',
                      color: p.isOrderly ? '#9C75FF' : 'rgba(255,255,255,0.6)',
                      fontWeight: p.isOrderly ? 600 : 400,
                      background: p.isOrderly ? 'rgba(156,117,255,0.1)' : undefined
                    }}
                  >
                    {p.marketShare.toFixed(2)}%
                  </td>
                </tr>
              ))}
              {othersVolume > 0 && (
                <tr>
                  <td
                    style={{
                      ...tdSticky(data.dexProtocols.length),
                      width: 40,
                      color: 'rgba(255,255,255,0.3)'
                    }}
                  >
                    —
                  </td>
                  <td
                    style={{
                      ...tdSticky(data.dexProtocols.length),
                      color: 'rgba(255,255,255,0.4)'
                    }}
                  >
                    Others
                  </td>
                  <td style={{ ...TD, textAlign: 'right', color: 'rgba(255,255,255,0.5)' }}>
                    {fmtCompact(othersVolume)}
                  </td>
                  <td style={{ ...TD, textAlign: 'right', color: 'rgba(255,255,255,0.3)' }}>—</td>
                  <td style={{ ...TD, textAlign: 'right', color: 'rgba(255,255,255,0.3)' }}>
                    {((othersVolume / data.totalDexVolume) * 100).toFixed(2)}%
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
      <div
        style={{
          fontSize: 11,
          color: 'rgba(255,255,255,0.35)',
          fontStyle: 'italic'
        }}
      >
        Volume data represents a 24-hour rolling window. Powered by CoinGecko.
      </div>
    </div>
  );
};
