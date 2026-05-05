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

import { CHART_COLORS, baseBarOpts } from '../shared/chartConfig';
import { fmtUsd, weekLabel } from '../shared/formatters';
import { Empty, Skeleton } from '../shared/primitives';

import { useVolumeSegments } from '~/hooks/useOrderlyMetrics';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

export const VolumeSegmentsWidget: FC = () => {
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
    <>
      {isLoading ? (
        <Skeleton height={236} />
      ) : error || weeks.length === 0 ? (
        <Empty msg={error ? 'Failed to load' : 'No data'} />
      ) : (
        <div style={{ position: 'relative', width: '100%', height: '100%', minHeight: 200 }}>
          <Bar data={chartData} options={options} />
        </div>
      )}
    </>
  );
};
