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
import { fmtCompact, labelFromDate } from '../shared/formatters';

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

export const NetFeesWidget: FC<{ rows: DuneData['feeRows'] }> = ({ rows }) => {
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

  return (
    <div style={{ position: 'relative', width: '100%', height: '100%', minHeight: 200 }}>
      <Line data={data} options={options} />
    </div>
  );
};
