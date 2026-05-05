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
import { Line } from 'react-chartjs-2';

import { baseLineOpts } from '../shared/chartConfig';
import { fmtNum, weekLabel } from '../shared/formatters';
import { Empty, Skeleton } from '../shared/primitives';

import { useStakeVsSupply } from '~/hooks/useOrderlyMetrics';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

export const StakeVsSupplyWidget: FC = () => {
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
    <>
      {isLoading ? (
        <Skeleton height={196} />
      ) : error || rows.length === 0 ? (
        <Empty msg={error ? 'Failed to load' : 'No data'} />
      ) : (
        <div style={{ position: 'relative', width: '100%', height: '100%', minHeight: 200 }}>
          <Line data={chartData} options={options} />
        </div>
      )}
    </>
  );
};
