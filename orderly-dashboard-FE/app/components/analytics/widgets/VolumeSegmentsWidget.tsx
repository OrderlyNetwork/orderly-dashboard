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
import { FC, useRef, useState } from 'react';
import { Bar } from 'react-chartjs-2';

import { CHART_COLORS, baseBarOpts, baseTooltipOpts, useChartReady } from '../shared/chartConfig';
import { fmtUsd, weekLabel } from '../shared/formatters';
import { DatasetChips, Empty, Skeleton } from '../shared/primitives';

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
  const chartRef = useRef<ChartJS<'bar'>>(null);
  useChartReady(chartRef);
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

  const [hidden, setHidden] = useState<Set<number>>(() => new Set());

  const chips = segList.map((seg, i) => ({
    label: seg,
    color: CHART_COLORS[i % CHART_COLORS.length],
    visible: !hidden.has(i)
  }));

  const visibleDatasets = segList.map((seg, i) => ({ seg, i })).filter(({ i }) => !hidden.has(i));

  const chartData: ChartData<'bar'> = {
    labels: weeks.map(weekLabel),
    datasets: visibleDatasets.map(({ seg, i }) => ({
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
      legend: { display: false },
      tooltip: {
        ...baseTooltipOpts,
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
          <DatasetChips
            items={chips}
            onToggle={(i) =>
              setHidden((prev) => {
                const next = new Set(prev);
                if (next.has(i)) next.delete(i);
                else next.add(i);
                return next;
              })
            }
          />
          <Bar ref={chartRef} data={chartData} options={options} />
        </div>
      )}
    </>
  );
};
