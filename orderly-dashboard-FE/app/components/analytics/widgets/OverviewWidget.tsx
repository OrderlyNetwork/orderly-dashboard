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
import { FC } from 'react';
import { Bar } from 'react-chartjs-2';

import { baseBarOpts, baseTooltipOpts } from '../shared/chartConfig';
import { fmtNum, fmtUsd, weekLabel } from '../shared/formatters';
import { Empty, Skeleton, StatCard } from '../shared/primitives';

import { useMetricsOverview } from '~/hooks/useOrderlyMetrics';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

export const OverviewWidget: FC = () => {
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
      tooltip: { ...baseTooltipOpts, callbacks: { label: (ctx) => ` ${fmtUsd(ctx.raw as number)}` } }
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
      <div style={{ height: 220 }}>
        {weeks.length === 0 ? <Empty msg="No data" /> : <Bar data={chartData} options={options} />}
      </div>
    </>
  );
};
