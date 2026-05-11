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
import { Bar } from 'react-chartjs-2';

import { CHAIN_COLORS, useChartReady } from '../shared/chartConfig';
import { capitalize, fmtCompact } from '../shared/formatters';

import type { TvlChainRow } from '~/types/dashboard';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

export const TvlByChainWidget: FC<{
  chains: TvlChainRow[];
}> = ({ chains }) => {
  const chartRef = useRef<ChartJS<'bar'>>(null);
  useChartReady(chartRef);
  const sorted = [...chains].sort((a, b) => b.tvl_usd - a.tvl_usd).slice(0, 12);
  const data: ChartData<'bar'> = {
    labels: sorted.map((c) => capitalize(c.chain)),
    datasets: [
      {
        data: sorted.map((c) => c.tvl_usd),
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
    interaction: { mode: 'nearest', axis: 'y', intersect: false },
    plugins: {
      legend: { display: false },
      tooltip: {
        mode: 'nearest',
        axis: 'y',
        intersect: false,
        callbacks: { label: (ctx) => ` ${fmtCompact(ctx.raw as number)}` }
      }
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

  return (
    <div style={{ position: 'relative', width: '100%', height: '100%', minHeight: 200 }}>
      <Bar ref={chartRef} data={data} options={options} />
    </div>
  );
};
