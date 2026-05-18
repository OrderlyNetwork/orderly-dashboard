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
import { Empty, Skeleton, StatCard, type Granularity } from '../shared/primitives';

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
    color: '#34d399',
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

const periodSub = (g: Granularity, row: OverviewPeriod) =>
  g === 'weekly'
    ? `week of ${weekLabel(row.week_start_date)}`
    : row.month_start_date
      ? monthLabel(row.month_start_date)
      : undefined;

export const OverviewWidget: FC<{ granularity?: Granularity }> = ({ granularity = 'weekly' }) => {
  const { data, isLoading, error } = useMetricsOverview();
  const chartRef = useRef<ChartJS<'bar'>>(null);
  useChartReady(chartRef);

  const [selectedMetric, setSelectedMetric] = useState<MetricKey>('avg_trading_volume');

  const rows = useMemo(
    () => (granularity === 'weekly' ? (data?.weekly ?? []) : (data?.monthly ?? [])),
    [data, granularity]
  );
  const latest = rows[rows.length - 1];
  const active = METRICS.find((m) => m.key === selectedMetric)!;

  const chartData: ChartData<'bar'> = {
    labels: rows.map((r) => periodLabel(granularity, r)),
    datasets: [
      {
        label: active.label(granularity),
        data: rows.map((r) => r[active.key] ?? 0),
        backgroundColor: `${active.color}8C`,
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
        {METRICS.map((m) => (
          <StatCard
            key={m.key}
            label={m.label(granularity)}
            value={m.fmt(latest?.[m.key])}
            color={m.color}
            sub={periodSub(granularity, latest)}
            selected={selectedMetric === m.key}
            onClick={() => setSelectedMetric(m.key)}
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
