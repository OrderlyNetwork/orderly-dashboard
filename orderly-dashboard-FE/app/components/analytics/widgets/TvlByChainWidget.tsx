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

import { CHAIN_COLORS, baseTooltipOpts } from '../shared/chartConfig';
import { capitalize, fmtCompact } from '../shared/formatters';

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

export const TvlByChainWidget: FC<{
  chains: DuneData['tvlChains'];
}> = ({ chains }) => {
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
      tooltip: { ...baseTooltipOpts, callbacks: { label: (ctx) => ` ${fmtCompact(ctx.raw as number)}` } }
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
      <Bar data={data} options={options} />
    </div>
  );
};
