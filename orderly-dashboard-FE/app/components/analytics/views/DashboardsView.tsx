import {
  BarElement,
  CategoryScale,
  Chart as ChartJS,
  Filler,
  LinearScale,
  LineElement,
  PointElement,
  Tooltip,
  type ChartData,
  type ChartOptions
} from 'chart.js';
import { FC, useState } from 'react';
import { Bar, Line } from 'react-chartjs-2';

import { KPICard } from '../widgets/KPICard';

import type { Role } from '~/components/analytics/Sidebar';
import {
  useDexUsers,
  useDistributorInvitees,
  useDistributorStats,
  useMetricsOverview,
  useOmnivaultTvl,
  useStakeUsers,
  useStakeVsSupply,
  useVolumeSegments
} from '~/hooks/useOrderlyMetrics';
import type { DuneData } from '~/types/dune';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

// ── Helpers ───────────────────────────────────────────────────────────────────

function fmtCompact(n: number): string {
  if (n >= 1e12) return `$${(n / 1e12).toFixed(2)}T`;
  if (n >= 1e9) return `$${(n / 1e9).toFixed(2)}B`;
  if (n >= 1e6) return `$${(n / 1e6).toFixed(2)}M`;
  if (n >= 1e3) return `$${(n / 1e3).toFixed(1)}K`;
  return `$${n.toFixed(0)}`;
}

function fmtUsd(n: number | undefined | null): string {
  if (n == null || isNaN(n)) return '—';
  if (n >= 1e9) return `$${(n / 1e9).toFixed(2)}B`;
  if (n >= 1e6) return `$${(n / 1e6).toFixed(2)}M`;
  if (n >= 1e3) return `$${(n / 1e3).toFixed(1)}K`;
  return `$${n.toFixed(0)}`;
}

function fmtNum(n: number | undefined | null): string {
  if (n == null || isNaN(n)) return '—';
  if (n >= 1e6) return `${(n / 1e6).toFixed(2)}M`;
  if (n >= 1e3) return `${(n / 1e3).toFixed(1)}K`;
  return n.toLocaleString();
}

function fmtPct(n: number | undefined | null): string {
  if (n == null || isNaN(n)) return '—';
  return `${n > 0 ? '+' : ''}${n.toFixed(1)}%`;
}

function fmtDeltaPct(current: number, previous: number): number | undefined {
  if (!previous) return undefined;
  return ((current - previous) / Math.abs(previous)) * 100;
}

function labelFromDate(dateStr: string): string {
  const d = new Date(dateStr);
  return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
}

function weekLabel(dateStr: string | undefined): string {
  if (!dateStr) return '';
  const d = new Date(dateStr);
  if (isNaN(d.getTime())) return dateStr;
  return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
}

function capitalize(s: string): string {
  return s.charAt(0).toUpperCase() + s.slice(1);
}

type Period = '30D' | '90D' | 'ALL';

const CHAIN_COLORS: Record<string, string> = {
  solana: '#9945FF',
  arbitrum: '#28A0F0',
  ethereum: '#627EEA',
  bnb: '#F3BA2F',
  optimism: '#FF0420',
  base: '#0052FF',
  polygon: '#8247E5',
  avalanche: '#E84142',
  abstract: '#A0F0A0',
  berachain: '#F5A623',
  ceffu: '#34d399',
  story: '#FF6B6B',
  sonic: '#60a5fa',
  mode: '#9C75FF',
  sei: '#F87171',
  mantle: '#C084FC',
  plume: '#FCD34D',
  monad: '#6EE7B7',
  morph: '#A78BFA'
};

const CHART_COLORS = ['#9C75FF', '#34d399', '#60a5fa', '#f59e0b', '#f87171', '#a78bfa', '#fb923c'];

// ── Base chart options ─────────────────────────────────────────────────────────

const baseLineOpts: ChartOptions<'line'> = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: { legend: { display: false }, tooltip: { mode: 'index', intersect: false } },
  scales: {
    x: {
      grid: { color: 'rgba(255,255,255,0.04)' },
      ticks: {
        color: 'rgba(255,255,255,0.3)',
        font: { size: 10 },
        maxTicksLimit: 8,
        maxRotation: 0
      }
    },
    y: {
      grid: { color: 'rgba(255,255,255,0.04)' },
      ticks: { color: 'rgba(255,255,255,0.3)', font: { size: 10 } }
    }
  }
};

const baseBarOpts: ChartOptions<'bar'> = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: { legend: { display: false }, tooltip: { mode: 'index', intersect: false } },
  scales: {
    x: {
      grid: { color: 'rgba(255,255,255,0.04)' },
      ticks: {
        color: 'rgba(255,255,255,0.3)',
        font: { size: 10 },
        maxTicksLimit: 8,
        maxRotation: 0
      }
    },
    y: {
      grid: { color: 'rgba(255,255,255,0.04)' },
      ticks: { color: 'rgba(255,255,255,0.3)', font: { size: 10 } }
    }
  }
};

// ── Panel + UI primitives ──────────────────────────────────────────────────────

function Panel({
  title,
  subtitle,
  height = 240,
  controls,
  children
}: {
  title: string;
  subtitle?: string;
  height?: number;
  controls?: React.ReactNode;
  children: React.ReactNode;
}) {
  return (
    <div
      style={{
        background: 'rgba(20,15,35,.9)',
        border: '1px solid rgba(156,117,255,0.15)',
        borderRadius: 16,
        overflow: 'hidden'
      }}
    >
      <div
        style={{
          padding: '16px 20px',
          borderBottom: '1px solid rgba(156,117,255,0.08)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between'
        }}
      >
        <div>
          <div style={{ fontSize: 14, fontWeight: 600, color: '#fff' }}>{title}</div>
          {subtitle && (
            <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.35)', marginTop: 2 }}>
              {subtitle}
            </div>
          )}
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>{controls}</div>
      </div>
      <div style={{ padding: '12px 16px 16px', height }}>{children}</div>
    </div>
  );
}

function PeriodSelector({ period, onChange }: { period: Period; onChange: (p: Period) => void }) {
  return (
    <div
      style={{
        display: 'flex',
        background: 'rgba(156,117,255,0.06)',
        border: '1px solid rgba(156,117,255,0.15)',
        borderRadius: 8,
        padding: 3,
        gap: 2
      }}
    >
      {(['30D', '90D', 'ALL'] as Period[]).map((p) => (
        <button
          key={p}
          onClick={() => onChange(p)}
          style={{
            padding: '4px 12px',
            borderRadius: 6,
            border: 'none',
            background: period === p ? '#9C75FF' : 'transparent',
            color: period === p ? '#fff' : 'rgba(255,255,255,0.45)',
            fontSize: 12,
            fontWeight: period === p ? 600 : 400,
            cursor: 'pointer',
            transition: 'all 0.15s'
          }}
        >
          {p}
        </button>
      ))}
    </div>
  );
}

function StatCard({
  label,
  value,
  sub,
  color = '#9C75FF'
}: {
  label: string;
  value: string;
  sub?: string;
  color?: string;
}) {
  return (
    <div
      style={{
        background: 'rgba(20,15,35,.9)',
        border: `1px solid ${color}22`,
        borderLeft: `3px solid ${color}`,
        borderRadius: 12,
        padding: '14px 18px'
      }}
    >
      <div
        style={{
          fontSize: 10,
          fontWeight: 600,
          color: 'rgba(255,255,255,0.38)',
          textTransform: 'uppercase',
          letterSpacing: '0.08em',
          marginBottom: 6
        }}
      >
        {label}
      </div>
      <div style={{ fontSize: 20, fontWeight: 700, color: '#fff' }}>{value}</div>
      {sub && (
        <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)', marginTop: 4 }}>{sub}</div>
      )}
    </div>
  );
}

function Skeleton({ height = 40 }: { height?: number }) {
  return <div style={{ height, borderRadius: 8, background: 'rgba(156,117,255,0.07)' }} />;
}

function Empty({ msg }: { msg: string }) {
  return (
    <div
      style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100%',
        color: 'rgba(255,255,255,0.25)',
        fontSize: 13,
        gap: 8
      }}
    >
      <svg
        width="15"
        height="15"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
      >
        <circle cx="12" cy="12" r="10" />
        <line x1="12" y1="8" x2="12" y2="12" />
        <line x1="12" y1="16" x2="12.01" y2="16" />
      </svg>
      {msg}
    </div>
  );
}

function SectionHeading({ children }: { children: React.ReactNode }) {
  return (
    <div
      style={{
        fontSize: 11,
        fontWeight: 600,
        color: 'rgba(255,255,255,0.3)',
        textTransform: 'uppercase',
        letterSpacing: '0.1em',
        marginBottom: 12
      }}
    >
      {children}
    </div>
  );
}

function TableWrap({ children }: { children: React.ReactNode }) {
  return (
    <div
      style={{
        background: 'rgba(20,15,35,.9)',
        border: '1px solid rgba(156,117,255,0.15)',
        borderRadius: 16,
        overflow: 'hidden'
      }}
    >
      {children}
    </div>
  );
}

const TH: React.CSSProperties = {
  padding: '10px 16px',
  textAlign: 'left',
  color: 'rgba(255,255,255,0.3)',
  fontWeight: 600,
  fontSize: 10,
  textTransform: 'uppercase',
  letterSpacing: '0.06em',
  borderBottom: '1px solid rgba(255,255,255,0.07)',
  whiteSpace: 'nowrap'
};
const TD: React.CSSProperties = {
  padding: '9px 16px',
  borderBottom: '1px solid rgba(255,255,255,0.04)',
  fontSize: 12
};

// ── Dune chart components ──────────────────────────────────────────────────────

function VolumeBarChart({ rows, period }: { rows: DuneData['volumeRows']; period: Period }) {
  const count = period === '30D' ? 30 : period === '90D' ? 90 : rows.length;
  const reversed = [...rows.slice(0, count)].reverse();
  const data: ChartData<'bar'> = {
    labels: reversed.map((r) => labelFromDate(r.volume_date)),
    datasets: [
      {
        data: reversed.map((r) => r.volume),
        backgroundColor: 'rgba(156,117,255,0.65)',
        hoverBackgroundColor: 'rgba(156,117,255,0.9)',
        borderRadius: 3,
        borderSkipped: false
      }
    ]
  };
  const options: ChartOptions<'bar'> = {
    ...baseBarOpts,
    plugins: {
      legend: { display: false },
      tooltip: { callbacks: { label: (ctx) => ` ${fmtCompact(ctx.raw as number)}` } }
    },
    scales: {
      x: {
        ...baseBarOpts.scales?.x,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          maxTicksLimit: period === '30D' ? 10 : 8,
          maxRotation: 0
        }
      },
      y: {
        ...baseBarOpts.scales?.y,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          callback: (v) => fmtCompact(v as number)
        }
      }
    }
  };
  return <Bar data={data} options={options} />;
}

function TvlBarChart({ chains }: { chains: DuneData['tvlChains'] }) {
  const sorted = [...chains].sort((a, b) => b.tvl - a.tvl).slice(0, 12);
  const data: ChartData<'bar'> = {
    labels: sorted.map((c) => capitalize(c.chain)),
    datasets: [
      {
        data: sorted.map((c) => c.tvl),
        backgroundColor: sorted.map((c) => (CHAIN_COLORS[c.chain] ?? '#9C75FF') + 'CC'),
        hoverBackgroundColor: sorted.map((c) => CHAIN_COLORS[c.chain] ?? '#9C75FF'),
        borderRadius: 4,
        borderSkipped: false
      }
    ]
  };
  const options: ChartOptions<'bar'> = {
    indexAxis: 'y',
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: { display: false },
      tooltip: { callbacks: { label: (ctx) => ` ${fmtCompact(ctx.raw as number)}` } }
    },
    scales: {
      x: {
        grid: { color: 'rgba(255,255,255,0.04)' },
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          callback: (v) => fmtCompact(v as number)
        }
      },
      y: { grid: { display: false }, ticks: { color: 'rgba(255,255,255,0.5)', font: { size: 11 } } }
    }
  };
  return <Bar data={data} options={options} />;
}

function NetFeesLineChart({ rows }: { rows: DuneData['feeRows'] }) {
  const sliced = [...rows].slice(0, 90).reverse();
  const data: ChartData<'line'> = {
    labels: sliced.map((r) => labelFromDate(r.volume_date)),
    datasets: [
      {
        data: sliced.map((r) => r.cum_rev),
        fill: true,
        backgroundColor: 'rgba(52,211,153,0.12)',
        borderColor: '#34d399',
        borderWidth: 2,
        pointRadius: 0,
        tension: 0.35
      }
    ]
  };
  const options: ChartOptions<'line'> = {
    ...baseLineOpts,
    plugins: {
      legend: { display: false },
      tooltip: { callbacks: { label: (ctx) => ` ${fmtCompact(ctx.raw as number)}` } }
    },
    scales: {
      x: {
        ...baseLineOpts.scales?.x,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          maxTicksLimit: 8,
          maxRotation: 0
        }
      },
      y: {
        ...baseLineOpts.scales?.y,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          callback: (v) => fmtCompact(v as number)
        }
      }
    }
  };
  return <Line data={data} options={options} />;
}

// ── Orderly API section components ─────────────────────────────────────────────

function OverviewSection() {
  const { data, isLoading, error } = useMetricsOverview();
  const weeks = data?.weekly ?? [];
  const latest = weeks[weeks.length - 1];

  const chartData: ChartData<'bar'> = {
    labels: weeks.map((w) => weekLabel(w.week_start_date)),
    datasets: [
      {
        label: 'Avg Weekly Volume',
        data: weeks.map((w) => w.avg_trading_volume ?? 0),
        backgroundColor: 'rgba(156,117,255,0.55)',
        borderRadius: 3,
        borderSkipped: false
      }
    ]
  };
  const options: ChartOptions<'bar'> = {
    ...baseBarOpts,
    plugins: {
      legend: { display: false },
      tooltip: { callbacks: { label: (ctx) => ` ${fmtUsd(ctx.raw as number)}` } }
    },
    scales: {
      ...baseBarOpts.scales,
      y: {
        ...baseBarOpts.scales?.y,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          callback: (v) => fmtUsd(v as number)
        }
      }
    }
  };

  if (isLoading)
    return (
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(160px, 1fr))',
          gap: 12
        }}
      >
        {Array.from({ length: 4 }).map((_, i) => (
          <Skeleton key={i} height={76} />
        ))}
      </div>
    );
  if (error)
    return (
      <div style={{ color: 'rgba(248,113,113,0.8)', fontSize: 13 }}>
        Failed to load overview metrics
      </div>
    );

  return (
    <>
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(160px, 1fr))',
          gap: 12,
          marginBottom: 20
        }}
      >
        <StatCard
          label="Avg New Users / wk"
          value={fmtNum(latest?.avg_new_user)}
          color="#60a5fa"
          sub={`week of ${weekLabel(latest?.week_start_date)}`}
        />
        <StatCard
          label="Avg Active Users / wk"
          value={fmtNum(latest?.avg_active_user)}
          color="#34d399"
        />
        <StatCard
          label="Avg Volume / wk"
          value={fmtUsd(latest?.avg_trading_volume)}
          color="#9C75FF"
        />
        <StatCard
          label="Avg Revenue / wk"
          value={fmtUsd(latest?.avg_orderly_revenue)}
          color="#f59e0b"
        />
      </div>
      <Panel title="Weekly Volume Trend" subtitle="avg trading volume per week" height={220}>
        {weeks.length === 0 ? <Empty msg="No data" /> : <Bar data={chartData} options={options} />}
      </Panel>
    </>
  );
}

function DexUsersSection() {
  const { data, isLoading, error } = useDexUsers();
  const brokers = (data?.data ?? []).sort((a, b) => (b.total_users ?? 0) - (a.total_users ?? 0));

  return (
    <div>
      <SectionHeading>DEX Users by Broker</SectionHeading>
      <TableWrap>
        <div style={{ overflowX: 'auto', maxHeight: 340, overflowY: 'auto' }}>
          {isLoading ? (
            <div style={{ padding: 20 }}>
              <Skeleton height={160} />
            </div>
          ) : error ? (
            <div style={{ padding: 24 }}>
              <Empty msg="Failed to load" />
            </div>
          ) : brokers.length === 0 ? (
            <div style={{ padding: 24 }}>
              <Empty msg="No data" />
            </div>
          ) : (
            <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 12 }}>
              <thead>
                <tr>
                  {['Broker', 'DAU', 'DoD%', 'WAU', 'WoW%', 'MAU', 'MoM%', 'Total', 'New 30d'].map(
                    (h) => (
                      <th key={h} style={TH}>
                        {h}
                      </th>
                    )
                  )}
                </tr>
              </thead>
              <tbody>
                {brokers.map((b, i) => (
                  <tr key={i}>
                    <td style={{ ...TD, color: '#9C75FF', fontWeight: 600 }}>
                      {b.broker_name ?? b.broker_id ?? '—'}
                    </td>
                    <td style={{ ...TD, color: '#fff' }}>{fmtNum(b.dau)}</td>
                    <td style={{ ...TD, color: (b.dau_dod_pct ?? 0) >= 0 ? '#34d399' : '#f87171' }}>
                      {fmtPct(b.dau_dod_pct)}
                    </td>
                    <td style={{ ...TD, color: '#fff' }}>{fmtNum(b.wau)}</td>
                    <td style={{ ...TD, color: (b.wau_wow_pct ?? 0) >= 0 ? '#34d399' : '#f87171' }}>
                      {fmtPct(b.wau_wow_pct)}
                    </td>
                    <td style={{ ...TD, color: '#fff' }}>{fmtNum(b.mau)}</td>
                    <td style={{ ...TD, color: (b.mau_mom_pct ?? 0) >= 0 ? '#34d399' : '#f87171' }}>
                      {fmtPct(b.mau_mom_pct)}
                    </td>
                    <td style={{ ...TD, color: 'rgba(255,255,255,0.7)' }}>
                      {fmtNum(b.total_users)}
                    </td>
                    <td style={{ ...TD, color: '#60a5fa' }}>{fmtNum(b.new_users_30d)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </TableWrap>
    </div>
  );
}

function VolumeSegmentsSection() {
  const { data, isLoading, error } = useVolumeSegments();
  const rows = data?.weekly ?? [];

  const weekMap = new Map<string, Record<string, number>>();
  const segments = new Set<string>();
  rows.forEach((r) => {
    const key = r.week_start_date ?? r.week_start ?? '';
    const seg = r.mm_retail ?? 'unknown';
    segments.add(seg);
    if (!weekMap.has(key)) weekMap.set(key, {});
    weekMap.get(key)![seg] = (weekMap.get(key)![seg] ?? 0) + (r.volume ?? 0);
  });

  const weeks = Array.from(weekMap.keys()).sort();
  const segList = Array.from(segments);

  const chartData: ChartData<'bar'> = {
    labels: weeks.map(weekLabel),
    datasets: segList.map((seg, i) => ({
      label: seg,
      data: weeks.map((w) => weekMap.get(w)?.[seg] ?? 0),
      backgroundColor: CHART_COLORS[i % CHART_COLORS.length] + 'CC',
      borderRadius: 2,
      borderSkipped: false,
      stack: 'vol'
    }))
  };
  const options: ChartOptions<'bar'> = {
    ...baseBarOpts,
    plugins: {
      legend: {
        display: true,
        labels: { color: 'rgba(255,255,255,0.5)', font: { size: 11 }, boxWidth: 12 }
      },
      tooltip: {
        callbacks: { label: (ctx) => ` ${ctx.dataset.label}: ${fmtUsd(ctx.raw as number)}` }
      }
    },
    scales: {
      x: { ...baseBarOpts.scales?.x, stacked: true },
      y: {
        ...baseBarOpts.scales?.y,
        stacked: true,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          callback: (v) => fmtUsd(v as number)
        }
      }
    }
  };

  return (
    <Panel title="Volume Segments" subtitle="weekly by segment (2B / 2C / MM)" height={260}>
      {isLoading ? (
        <Skeleton height={236} />
      ) : error || weeks.length === 0 ? (
        <Empty msg={error ? 'Failed to load' : 'No data'} />
      ) : (
        <Bar data={chartData} options={options} />
      )}
    </Panel>
  );
}

function OmnivaultTvlSection() {
  const { data, isLoading, error } = useOmnivaultTvl();
  const rows = data?.weekly ?? [];

  const weekMap = new Map<string, Record<string, number>>();
  const vaults = new Set<string>();
  rows.forEach((r) => {
    const key = r.week_start ?? r.wk ?? '';
    const vault = r.vault_name ?? 'unknown';
    vaults.add(vault);
    if (!weekMap.has(key)) weekMap.set(key, {});
    weekMap.get(key)![vault] = r.avg_vault_tvl_m ?? 0;
  });

  const weeks = Array.from(weekMap.keys()).sort();
  const vaultList = Array.from(vaults);

  const chartData: ChartData<'line'> = {
    labels: weeks.map(weekLabel),
    datasets: vaultList.map((vault, i) => ({
      label: vault,
      data: weeks.map((w) => weekMap.get(w)?.[vault] ?? 0),
      fill: false,
      borderColor: CHART_COLORS[i % CHART_COLORS.length],
      backgroundColor: CHART_COLORS[i % CHART_COLORS.length] + '22',
      borderWidth: 2,
      pointRadius: 3,
      tension: 0.3
    }))
  };
  const options: ChartOptions<'line'> = {
    ...baseLineOpts,
    plugins: {
      legend: {
        display: true,
        labels: { color: 'rgba(255,255,255,0.5)', font: { size: 11 }, boxWidth: 12 }
      },
      tooltip: {
        callbacks: { label: (ctx) => ` ${ctx.dataset.label}: $${(ctx.raw as number).toFixed(2)}M` }
      }
    },
    scales: {
      ...baseLineOpts.scales,
      y: {
        ...baseLineOpts.scales?.y,
        ticks: { color: 'rgba(255,255,255,0.3)', font: { size: 10 }, callback: (v) => `$${v}M` }
      }
    }
  };

  return (
    <Panel title="Omnivault TVL" subtitle="avg weekly TVL per vault (USD millions)" height={260}>
      {isLoading ? (
        <Skeleton height={236} />
      ) : error || weeks.length === 0 ? (
        <Empty msg={error ? 'Failed to load' : 'No data'} />
      ) : (
        <Line data={chartData} options={options} />
      )}
    </Panel>
  );
}

function StakeUsersSection() {
  const { data, isLoading, error } = useStakeUsers();
  const rows = data?.weekly ?? [];

  const chartData: ChartData<'line'> = {
    labels: rows.map((r) => weekLabel(r.week_start ?? r.wk)),
    datasets: [
      {
        label: 'Avg Active Stakers',
        data: rows.map((r) => r.avg_active_stakers ?? 0),
        fill: true,
        backgroundColor: 'rgba(52,211,153,0.1)',
        borderColor: '#34d399',
        borderWidth: 2,
        pointRadius: 3,
        tension: 0.3
      }
    ]
  };
  const options: ChartOptions<'line'> = {
    ...baseLineOpts,
    plugins: {
      legend: { display: false },
      tooltip: { callbacks: { label: (ctx) => ` ${fmtNum(ctx.raw as number)} stakers` } }
    },
    scales: {
      ...baseLineOpts.scales,
      y: {
        ...baseLineOpts.scales?.y,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          callback: (v) => fmtNum(v as number)
        }
      }
    }
  };

  return (
    <Panel title="Active Stakers" subtitle="weekly avg active ORDER stakers" height={220}>
      {isLoading ? (
        <Skeleton height={196} />
      ) : error || rows.length === 0 ? (
        <Empty msg={error ? 'Failed to load' : 'No data'} />
      ) : (
        <Line data={chartData} options={options} />
      )}
    </Panel>
  );
}

function StakeVsSupplySection() {
  const { data, isLoading, error } = useStakeVsSupply();
  const rows = data?.weekly ?? [];

  const chartData: ChartData<'line'> = {
    labels: rows.map((r) => weekLabel(r.week_start ?? r.wk)),
    datasets: [
      {
        label: 'Staked ORDER',
        data: rows.map((r) => r.stake_order ?? 0),
        fill: true,
        backgroundColor: 'rgba(156,117,255,0.1)',
        borderColor: '#9C75FF',
        borderWidth: 2,
        pointRadius: 0,
        tension: 0.3
      },
      {
        label: 'Circulating Supply',
        data: rows.map((r) => r.circulating_supply ?? 0),
        fill: false,
        borderColor: 'rgba(251,191,36,0.8)',
        borderWidth: 1.5,
        borderDash: [5, 3],
        pointRadius: 0,
        tension: 0.3
      }
    ]
  };
  const options: ChartOptions<'line'> = {
    ...baseLineOpts,
    plugins: {
      legend: {
        display: true,
        labels: { color: 'rgba(255,255,255,0.5)', font: { size: 11 }, boxWidth: 12 }
      },
      tooltip: {
        callbacks: { label: (ctx) => ` ${ctx.dataset.label}: ${fmtNum(ctx.raw as number)} ORDER` }
      }
    },
    scales: {
      ...baseLineOpts.scales,
      y: {
        ...baseLineOpts.scales?.y,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          callback: (v) => fmtNum(v as number)
        }
      }
    }
  };

  return (
    <Panel
      title="Stake vs Circulating Supply"
      subtitle="weekly ORDER token staking vs circulating supply"
      height={220}
    >
      {isLoading ? (
        <Skeleton height={196} />
      ) : error || rows.length === 0 ? (
        <Empty msg={error ? 'Failed to load' : 'No data'} />
      ) : (
        <Line data={chartData} options={options} />
      )}
    </Panel>
  );
}

function DistributorsSection() {
  const { data: stats, isLoading: sLoad, error: sErr } = useDistributorStats();
  const { data: invitees, isLoading: iLoad, error: iErr } = useDistributorInvitees();

  const distributors = Array.isArray(stats)
    ? [...stats].sort((a, b) => (b['30D Revenue Share'] ?? 0) - (a['30D Revenue Share'] ?? 0))
    : [];
  const invList = Array.isArray(invitees)
    ? [...invitees].sort((a, b) => (b['30D Invitee Volume'] ?? 0) - (a['30D Invitee Volume'] ?? 0))
    : [];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
      <TableWrap>
        <div
          style={{
            padding: '14px 20px',
            borderBottom: '1px solid rgba(156,117,255,0.08)',
            display: 'flex',
            alignItems: 'baseline',
            gap: 10
          }}
        >
          <div style={{ fontSize: 14, fontWeight: 600, color: '#fff' }}>Distributors</div>
          {distributors.length > 0 && (
            <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)' }}>
              {distributors.length} records · sorted by 30D revenue
            </div>
          )}
        </div>
        <div style={{ overflowX: 'auto', maxHeight: 300, overflowY: 'auto' }}>
          {sLoad ? (
            <div style={{ padding: 20 }}>
              <Skeleton height={120} />
            </div>
          ) : sErr || distributors.length === 0 ? (
            <div style={{ padding: 24 }}>
              <Empty msg={sErr ? 'Failed to load' : 'No data'} />
            </div>
          ) : (
            <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 12 }}>
              <thead>
                <tr>
                  {[
                    'Name',
                    'Type',
                    'Fee Tier',
                    'Invitees',
                    'Graduated',
                    '30D Volume',
                    '30D Revenue',
                    'Total Revenue'
                  ].map((h) => (
                    <th key={h} style={TH}>
                      {h}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {distributors.map((d, i) => (
                  <tr key={i}>
                    <td style={{ ...TD, color: '#9C75FF', fontWeight: 600 }}>
                      {d['Distributor Name'] ?? '—'}
                    </td>
                    <td style={{ ...TD, color: 'rgba(255,255,255,0.5)' }}>
                      {d['Distributor Type'] ?? '—'}
                    </td>
                    <td style={TD}>
                      <span
                        style={{
                          background:
                            d['Fee Tier'] === 'PLATINUM'
                              ? 'rgba(156,117,255,0.2)'
                              : 'rgba(255,255,255,0.05)',
                          color: d['Fee Tier'] === 'PLATINUM' ? '#c4a8ff' : 'rgba(255,255,255,0.5)',
                          padding: '2px 8px',
                          borderRadius: 6,
                          fontSize: 11
                        }}
                      >
                        {d['Fee Tier'] ?? '—'}
                      </span>
                    </td>
                    <td style={{ ...TD, color: '#fff' }}>{fmtNum(d['Number of Invitees'])}</td>
                    <td style={{ ...TD, color: '#34d399' }}>
                      {fmtNum(d['Number of Graduated Invitees'])}
                    </td>
                    <td style={{ ...TD, color: '#60a5fa' }}>{fmtUsd(d['30D Invitee Volume'])}</td>
                    <td style={{ ...TD, color: '#f59e0b', fontWeight: 600 }}>
                      {fmtUsd(d['30D Revenue Share'])}
                    </td>
                    <td style={{ ...TD, color: 'rgba(255,255,255,0.6)' }}>
                      {fmtUsd(d['Total Revenue Share'])}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </TableWrap>

      <TableWrap>
        <div
          style={{
            padding: '14px 20px',
            borderBottom: '1px solid rgba(156,117,255,0.08)',
            display: 'flex',
            alignItems: 'baseline',
            gap: 10
          }}
        >
          <div style={{ fontSize: 14, fontWeight: 600, color: '#fff' }}>Distributor Invitees</div>
          {invList.length > 0 && (
            <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)' }}>
              {invList.length} records · sorted by 30D volume
            </div>
          )}
        </div>
        <div style={{ overflowX: 'auto', maxHeight: 300, overflowY: 'auto' }}>
          {iLoad ? (
            <div style={{ padding: 20 }}>
              <Skeleton height={120} />
            </div>
          ) : iErr || invList.length === 0 ? (
            <div style={{ padding: 24 }}>
              <Empty msg={iErr ? 'Failed to load' : 'No data'} />
            </div>
          ) : (
            <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 12 }}>
              <thead>
                <tr>
                  {[
                    'Invitee DEX',
                    'Status',
                    'Orderly One',
                    'DEX Link',
                    '30D Volume',
                    '30D Revenue',
                    'Date Invited'
                  ].map((h) => (
                    <th key={h} style={TH}>
                      {h}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {invList.map((inv, i) => (
                  <tr key={i}>
                    <td style={{ ...TD, color: '#9C75FF', fontWeight: 600 }}>
                      {inv['Invitee DEX'] ?? '—'}
                    </td>
                    <td style={TD}>
                      <span
                        style={{
                          background:
                            inv['DEX status'] === 'Graduated'
                              ? 'rgba(52,211,153,0.15)'
                              : 'rgba(255,255,255,0.05)',
                          color:
                            inv['DEX status'] === 'Graduated' ? '#34d399' : 'rgba(255,255,255,0.5)',
                          padding: '2px 8px',
                          borderRadius: 6,
                          fontSize: 11
                        }}
                      >
                        {inv['DEX status'] ?? '—'}
                      </span>
                    </td>
                    <td
                      style={{
                        ...TD,
                        color: inv['is_orderly_one'] === 'yes' ? '#34d399' : 'rgba(255,255,255,0.4)'
                      }}
                    >
                      {inv['is_orderly_one'] === 'yes' ? '✓' : '—'}
                    </td>
                    <td style={{ ...TD, color: '#60a5fa', fontSize: 11 }}>
                      {inv['DEX link'] ?? '—'}
                    </td>
                    <td style={{ ...TD, color: '#60a5fa', fontWeight: 600 }}>
                      {fmtUsd(inv['30D Invitee Volume'])}
                    </td>
                    <td style={{ ...TD, color: '#f59e0b' }}>{fmtUsd(inv['30D Revenue Share'])}</td>
                    <td style={{ ...TD, color: 'rgba(255,255,255,0.4)', fontSize: 11 }}>
                      {inv['Date Invited']
                        ? new Date(inv['Date Invited']).toLocaleDateString()
                        : '—'}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </TableWrap>
    </div>
  );
}

// ── Main component ─────────────────────────────────────────────────────────────

type Props = { role: Role; data: DuneData };

export const DashboardsView: FC<Props> = ({ role, data }) => {
  const [period, setPeriod] = useState<Period>('30D');

  const { volumeRows, tvlChains, feeRows, accountRows, marketRows, builderFees, activeBuilders } =
    data;

  const todayVol = volumeRows[0]?.volume ?? 0;
  const yestVol = volumeRows[1]?.volume ?? 0;
  const cumVol = volumeRows[0]?.cumulative_volume ?? 0;
  const vol30d = volumeRows[0]?.rolling_total_volume ?? 0;
  const vol30dPrev = volumeRows[7]?.rolling_total_volume ?? 0;
  const totalAccounts = accountRows[0]?.cumulative_accounts ?? 0;
  const prevWeekAccounts = accountRows[7]?.cumulative_accounts ?? 0;
  const latestMarkets = marketRows[0]?.markets ?? 0;
  const prevMarkets = marketRows[1]?.markets ?? 0;
  const cumFees = feeRows[0]?.cum_rev ?? 0;
  const dailyFees = feeRows[0]?.daily_rev ?? 0;
  const fees30d = feeRows[0]?.rolling_total_rev ?? 0;
  const rollingAvgFee = feeRows[0]?.rolling_rev ?? 0;
  const tvlTotal = tvlChains.reduce((s, c) => s + c.tvl, 0);

  const ROLE_TITLES: Record<Role, string> = {
    trader: 'Markets · Volume · Users',
    builder: 'Revenue · Ecosystem · Liquidity',
    analyst: 'Full Protocol Overview'
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 24 }}>
      {/* Role badge */}
      <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
        <span
          style={{
            background: 'rgba(156,117,255,0.12)',
            border: '1px solid rgba(156,117,255,0.25)',
            borderRadius: 20,
            padding: '3px 12px',
            fontSize: 12,
            fontWeight: 600,
            color: '#9C75FF'
          }}
        >
          {role.charAt(0).toUpperCase() + role.slice(1)} View
        </span>
        <span style={{ fontSize: 13, color: 'rgba(255,255,255,0.4)' }}>{ROLE_TITLES[role]}</span>
      </div>

      {/* ── TRADER ─────────────────────────────────────────────────────── */}
      {role === 'trader' && (
        <>
          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))',
              gap: 16
            }}
          >
            <KPICard
              label="Day Volume"
              value={fmtCompact(todayVol)}
              delta={fmtDeltaPct(todayVol, yestVol)}
              subValue="vs yesterday"
            />
            <KPICard
              label="30D Volume"
              value={fmtCompact(vol30d)}
              delta={fmtDeltaPct(vol30d, vol30dPrev)}
              subValue="rolling 30-day total"
            />
            <KPICard label="Cumulative Volume" value={fmtCompact(cumVol)} subValue="all-time" />
            <KPICard
              label="Open Markets"
              value={`${latestMarkets}`}
              delta={fmtDeltaPct(latestMarkets, prevMarkets)}
            />
            <KPICard
              label="Total Accounts"
              value={
                totalAccounts >= 1e6
                  ? `${(totalAccounts / 1e6).toFixed(2)}M`
                  : totalAccounts.toLocaleString()
              }
              delta={fmtDeltaPct(totalAccounts, prevWeekAccounts)}
              subValue="vs last week"
            />
          </div>

          <Panel
            title="Trading Volume — Daily"
            controls={<PeriodSelector period={period} onChange={setPeriod} />}
            height={260}
          >
            <VolumeBarChart rows={volumeRows} period={period} />
          </Panel>

          <DexUsersSection />

          <div>
            <SectionHeading>Volume Breakdown</SectionHeading>
            <VolumeSegmentsSection />
          </div>

          <Panel title="TVL by Chain" subtitle={`Total: ${fmtCompact(tvlTotal)}`} height={260}>
            <TvlBarChart chains={tvlChains} />
          </Panel>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
              gap: 16
            }}
          >
            <StatCard
              label="Net Fees (24h)"
              value={fmtCompact(dailyFees)}
              sub={`30D total: ${fmtCompact(fees30d)}`}
              color="#34d399"
            />
            <StatCard
              label="Builder Fees (total)"
              value={fmtCompact(builderFees)}
              sub={`${activeBuilders} active builders`}
              color="#f59e0b"
            />
            <StatCard
              label="Cum. Net Fees"
              value={fmtCompact(cumFees)}
              sub="all-time protocol revenue"
              color="#9C75FF"
            />
          </div>
        </>
      )}

      {/* ── BUILDER ────────────────────────────────────────────────────── */}
      {role === 'builder' && (
        <>
          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))',
              gap: 16
            }}
          >
            <KPICard
              label="Active Builders"
              value={`${activeBuilders}`}
              subValue="unique broker IDs"
            />
            <KPICard
              label="Builder Fees (total)"
              value={fmtCompact(builderFees)}
              subValue="cumulative broker fees"
            />
            <KPICard label="Net Fees (30D)" value={fmtCompact(fees30d)} subValue="rolling 30-day" />
            <KPICard
              label="Open Markets"
              value={`${latestMarkets}`}
              delta={fmtDeltaPct(latestMarkets, prevMarkets)}
            />
            <KPICard
              label="TVL (total)"
              value={fmtCompact(tvlTotal)}
              subValue={`across ${tvlChains.length} chains`}
            />
          </div>

          <div
            style={{
              background: 'rgba(20,15,35,.9)',
              border: '1px solid rgba(245,158,11,0.2)',
              borderRadius: 14,
              padding: '16px 24px',
              display: 'flex',
              alignItems: 'center',
              gap: 24
            }}
          >
            <div>
              <div
                style={{
                  fontSize: 11,
                  fontWeight: 600,
                  color: 'rgba(255,255,255,0.38)',
                  textTransform: 'uppercase',
                  letterSpacing: '0.08em',
                  marginBottom: 4
                }}
              >
                Rolling Avg Daily Fee
              </div>
              <div style={{ fontSize: 22, fontWeight: 700, color: '#f59e0b' }}>
                {fmtCompact(rollingAvgFee)}
                <span
                  style={{
                    fontSize: 12,
                    color: 'rgba(255,255,255,0.35)',
                    fontWeight: 400,
                    marginLeft: 6
                  }}
                >
                  /day
                </span>
              </div>
            </div>
            <div style={{ flex: 1, height: 1, background: 'rgba(245,158,11,0.15)' }} />
            <div style={{ textAlign: 'right' }}>
              <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)', marginBottom: 2 }}>
                Net Fees (24h)
              </div>
              <div style={{ fontSize: 18, fontWeight: 700, color: '#34d399' }}>
                {fmtCompact(dailyFees)}
              </div>
            </div>
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
            <Panel title="TVL by Chain" subtitle={`Total: ${fmtCompact(tvlTotal)}`} height={260}>
              <TvlBarChart chains={tvlChains} />
            </Panel>
            <Panel title="Net Fees (cumulative)" subtitle="90-day running total" height={260}>
              <NetFeesLineChart rows={feeRows} />
            </Panel>
          </div>

          <div>
            <SectionHeading>Liquidity</SectionHeading>
            <OmnivaultTvlSection />
          </div>

          <div>
            <SectionHeading>Distributors</SectionHeading>
            <DistributorsSection />
          </div>
        </>
      )}

      {/* ── ANALYST ────────────────────────────────────────────────────── */}
      {role === 'analyst' && (
        <>
          <div>
            <SectionHeading>Protocol Overview (Weekly)</SectionHeading>
            <OverviewSection />
          </div>

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))',
              gap: 16
            }}
          >
            <KPICard
              label="Day Volume"
              value={fmtCompact(todayVol)}
              delta={fmtDeltaPct(todayVol, yestVol)}
              subValue="vs yesterday"
            />
            <KPICard
              label="30D Volume"
              value={fmtCompact(vol30d)}
              delta={fmtDeltaPct(vol30d, vol30dPrev)}
              subValue="rolling"
            />
            <KPICard label="Cumulative Volume" value={fmtCompact(cumVol)} />
            <KPICard
              label="Total Accounts"
              value={
                totalAccounts >= 1e6
                  ? `${(totalAccounts / 1e6).toFixed(2)}M`
                  : totalAccounts.toLocaleString()
              }
              delta={fmtDeltaPct(totalAccounts, prevWeekAccounts)}
            />
            <KPICard
              label="Open Markets"
              value={`${latestMarkets}`}
              delta={fmtDeltaPct(latestMarkets, prevMarkets)}
            />
            <KPICard
              label="Active Builders"
              value={`${activeBuilders}`}
              subValue={`Fees: ${fmtCompact(builderFees)}`}
            />
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: 12 }}>
            {[
              { label: 'Net Fees (24h)', value: fmtCompact(dailyFees) },
              { label: 'Net Fees (30D)', value: fmtCompact(fees30d) },
              { label: 'Cumulative Net Fees', value: fmtCompact(cumFees) }
            ].map((s) => (
              <div
                key={s.label}
                style={{
                  background: 'rgba(20,15,35,.9)',
                  border: '1px solid rgba(52,211,153,0.12)',
                  borderRadius: 14,
                  padding: '14px 20px',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'space-between'
                }}
              >
                <div
                  style={{
                    fontSize: 11,
                    fontWeight: 600,
                    color: 'rgba(255,255,255,0.38)',
                    textTransform: 'uppercase',
                    letterSpacing: '0.08em'
                  }}
                >
                  {s.label}
                </div>
                <div style={{ fontSize: 18, fontWeight: 700, color: '#34d399' }}>{s.value}</div>
              </div>
            ))}
          </div>

          <Panel
            title="Trading Volume — Daily"
            controls={<PeriodSelector period={period} onChange={setPeriod} />}
            height={260}
          >
            <VolumeBarChart rows={volumeRows} period={period} />
          </Panel>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
            <Panel title="TVL by Chain" subtitle={`Total: ${fmtCompact(tvlTotal)}`} height={260}>
              <TvlBarChart chains={tvlChains} />
            </Panel>
            <Panel title="Net Fees (cumulative)" subtitle="90-day running total" height={260}>
              <NetFeesLineChart rows={feeRows} />
            </Panel>
          </div>

          <DexUsersSection />

          <div>
            <SectionHeading>Volume &amp; Liquidity</SectionHeading>
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
              <VolumeSegmentsSection />
              <OmnivaultTvlSection />
            </div>
          </div>

          <div>
            <SectionHeading>Staking</SectionHeading>
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
              <StakeUsersSection />
              <StakeVsSupplySection />
            </div>
          </div>

          <div>
            <SectionHeading>Distributors</SectionHeading>
            <DistributorsSection />
          </div>

          <div
            style={{
              background: 'rgba(20,15,35,.9)',
              border: '1px solid rgba(156,117,255,0.12)',
              borderRadius: 14,
              padding: '16px 24px',
              display: 'flex',
              alignItems: 'center',
              gap: 24
            }}
          >
            <div>
              <div
                style={{
                  fontSize: 11,
                  fontWeight: 600,
                  color: 'rgba(255,255,255,0.38)',
                  textTransform: 'uppercase',
                  letterSpacing: '0.08em',
                  marginBottom: 4
                }}
              >
                Rolling Avg Daily Fee
              </div>
              <div style={{ fontSize: 13, color: 'rgba(255,255,255,0.5)' }}>
                {fmtCompact(rollingAvgFee)}/day
              </div>
            </div>
            <div style={{ flex: 1 }} />
            <div style={{ textAlign: 'right' }}>
              <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)', marginBottom: 2 }}>
                Builder fees (total)
              </div>
              <div style={{ fontSize: 18, fontWeight: 700, color: '#f59e0b' }}>
                {fmtCompact(builderFees)}
              </div>
            </div>
          </div>
        </>
      )}
    </div>
  );
};
