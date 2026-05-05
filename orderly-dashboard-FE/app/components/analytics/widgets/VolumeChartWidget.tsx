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
import { fmtCompact, labelFromDate } from '../shared/formatters';
import { type Period } from '../shared/primitives';

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

export const VolumeChartWidget: FC<{ rows: DuneData['volumeRows']; period?: Period }> = ({
  rows,
  period = '30D'
}) => {
  const count = period === '30D' ? 30 : 90;
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
      tooltip: { ...baseTooltipOpts, callbacks: { label: (ctx) => ` ${fmtCompact(ctx.raw as number)}` } }
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

  return (
    <div style={{ position: 'relative', width: '100%', height: '100%', minHeight: 200 }}>
      <Bar data={data} options={options} />
    </div>
  );
};
