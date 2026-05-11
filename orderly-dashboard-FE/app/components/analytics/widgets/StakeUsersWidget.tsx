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
import { FC, useRef } from 'react';
import { Line } from 'react-chartjs-2';

import { baseLineOpts, baseTooltipOpts, useChartReady } from '../shared/chartConfig';
import { fmtWhole, weekLabel } from '../shared/formatters';
import { Empty, Skeleton } from '../shared/primitives';

import { useStakeUsers } from '~/hooks/useOrderlyMetrics';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

export const StakeUsersWidget: FC = () => {
  const { data, isLoading, error } = useStakeUsers();
  const chartRef = useRef<ChartJS<'line'>>(null);
  useChartReady(chartRef);
  const rows = data?.weekly ?? [];

  const chartData: ChartData<'line'> = {
    labels: rows.map((r) => weekLabel(r.wk ?? r.week_start)),
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
      tooltip: {
        ...baseTooltipOpts,
        callbacks: { label: (ctx) => ` ${fmtWhole(ctx.raw as number)} stakers` }
      }
    },
    scales: {
      ...baseLineOpts.scales,
      y: {
        ...baseLineOpts.scales?.y,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          callback: (v) => fmtWhole(v as number)
        }
      }
    }
  };

  return (
    <>
      {isLoading ? (
        <Skeleton height={196} />
      ) : error || rows.length === 0 ? (
        <Empty msg={error ? 'Failed to load' : 'No data'} />
      ) : (
        <div style={{ position: 'relative', width: '100%', height: '100%', minHeight: 200 }}>
          <Line ref={chartRef} data={chartData} options={options} />
        </div>
      )}
    </>
  );
};
