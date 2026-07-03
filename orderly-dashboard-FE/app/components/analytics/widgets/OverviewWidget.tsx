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
import { FC, useMemo, useRef, useState } from 'react';
import { Bar } from 'react-chartjs-2';

import { baseBarOpts, baseTooltipOpts, useChartReady } from '../shared/chartConfig';
import { fmtNum, fmtUsd, monthLabel, weekLabel } from '../shared/formatters';
import { Empty, StatCard, type Granularity } from '../shared/primitives';

import { useMetricsOverview, type OverviewPeriod } from '~/hooks/useOrderlyMetrics';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

type MetricKey = 'avg_new_user' | 'avg_active_user' | 'avg_trading_volume' | 'avg_orderly_revenue';

const METRICS: {
  key: MetricKey;
  label: (g: Granularity) => string;
  color: string;
  fmt: (n: number | undefined | null) => string;
}[] = [
  {
    key: 'avg_new_user',
    label: () => 'Avg New Users / day',
    color: '#60a5fa',
    fmt: fmtNum
  },
  {
    key: 'avg_active_user',
    label: () => 'Avg Active Users / day',
    color: '#00dea3',
    fmt: fmtNum
  },
  {
    key: 'avg_trading_volume',
    label: () => 'Avg Volume / day',
    color: '#9C75FF',
    fmt: fmtUsd
  },
  {
    key: 'avg_orderly_revenue',
    label: () => 'Avg Revenue / day',
    color: '#f59e0b',
    fmt: fmtUsd
  }
];

const periodLabel = (g: Granularity, row: OverviewPeriod) =>
  g === 'weekly' ? weekLabel(row.week_start_date) : monthLabel(row.month_start_date);

export const OverviewWidget: FC<{ granularity?: Granularity }> = ({ granularity = 'weekly' }) => {
  const { data, isLoading, error } = useMetricsOverview();
  const chartRef = useRef<ChartJS<'bar'>>(null);
  useChartReady(chartRef);

  const [selectedMetric, setSelectedMetric] = useState<MetricKey>('avg_new_user');

  const rows = useMemo(
    () => (granularity === 'weekly' ? (data?.weekly ?? []) : (data?.monthly ?? [])),
    [data, granularity]
  );

  // Use last complete period (skip partial current period detected by day count)
  const latest = useMemo(() => {
    const [startKey, endKey] =
      granularity === 'weekly'
        ? (['week_start_date', 'week_end_date'] as const)
        : (['month_start_date', 'month_end_date'] as const);
    const minDays = granularity === 'weekly' ? 6 : 20;
    const complete = rows.filter((r) => {
      const s = r[startKey];
      const e = r[endKey];
      if (!s || !e) return false;
      const diff = (new Date(e).getTime() - new Date(s).getTime()) / 86400000;
      return diff >= minDays;
    });
    return complete.length > 0 ? complete[complete.length - 1] : rows[rows.length - 1];
  }, [rows, granularity]);
  const active = METRICS.find((m) => m.key === selectedMetric)!;

  const chartData: ChartData<'bar'> = {
    labels: rows.map((r) => periodLabel(granularity, r)),
    datasets: [
      {
        label: active.label(granularity),
        data: rows.map((r) => r[active.key] ?? 0),
        backgroundColor: `${active.color}CC`,
        borderRadius: 3,
        borderSkipped: false
      }
    ]
  };

  const tooltipFmt =
    active.key === 'avg_new_user' || active.key === 'avg_active_user'
      ? (v: number) => fmtNum(v)
      : (v: number) => fmtUsd(v);

  const options: ChartOptions<'bar'> = {
    ...baseBarOpts,
    plugins: {
      legend: { display: false },
      tooltip: {
        ...baseTooltipOpts,
        callbacks: { label: (ctx) => ` ${tooltipFmt(ctx.raw as number)}` }
      }
    },
    scales: {
      ...baseBarOpts.scales,
      y: {
        ...baseBarOpts.scales?.y,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          callback: (v) => tooltipFmt(v as number)
        }
      }
    }
  };

  if (isLoading)
    return (
      <>
        <div className="grid grid-cols-2 gap-2 mb-5 xl:hidden">
          {Array.from({ length: 4 }).map((_, i) => (
            <div
              key={i}
              style={{ height: 70, borderRadius: 12, background: 'rgba(156,117,255,0.10)' }}
            />
          ))}
        </div>
        <div className="hidden xl:flex gap-1.5" style={{ marginBottom: 20 }}>
          {Array.from({ length: 4 }).map((_, i) => (
            <div
              key={i}
              className="flex-1"
              style={{ height: 50, borderRadius: 9999, background: 'rgba(156,117,255,0.10)' }}
            />
          ))}
        </div>
      </>
    );
  if (error)
    return (
      <div style={{ color: 'rgba(248,113,113,0.8)', fontSize: 13 }}>
        Failed to load overview metrics
      </div>
    );

  return (
    <>
      {/* Mobile + Tablet: 2×2 grid */}
      <div className="grid grid-cols-2 gap-2 mb-5 xl:hidden">
        {METRICS.map((m) => (
          <StatCard
            key={m.key}
            label={m.label(granularity)}
            value={m.fmt(latest?.[m.key])}
            color={m.color}
            selected={selectedMetric === m.key}
            onClick={() => setSelectedMetric(m.key)}
            pill
          />
        ))}
      </div>

      {/* Desktop: horizontal strip */}
      <div className="hidden xl:flex gap-1.5" style={{ marginBottom: 20 }}>
        {METRICS.map((m) => (
          <StatCard
            key={m.key}
            label={m.label(granularity)}
            value={m.fmt(latest?.[m.key])}
            color={m.color}
            selected={selectedMetric === m.key}
            onClick={() => setSelectedMetric(m.key)}
            pill
            inStrip
          />
        ))}
      </div>
      <div style={{ height: 220 }}>
        {rows.length === 0 ? (
          <Empty msg="No data" />
        ) : (
          <Bar ref={chartRef} data={chartData} options={options} />
        )}
      </div>
    </>
  );
};
