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
import { weekLabel } from '../shared/formatters';
import { DatasetChips, Empty, Skeleton } from '../shared/primitives';

import { useOmnivaultTvl } from '~/hooks/useOrderlyMetrics';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

export const OmnivaultTvlWidget: FC = () => {
  const { data, isLoading, error } = useOmnivaultTvl();
  const chartRef = useRef<ChartJS<'bar'>>(null);
  useChartReady(chartRef);
  const rows = data?.weekly ?? [];

  const weekMap = new Map<string, Record<string, number>>();
  const vaults = new Set<string>();
  rows.forEach((r) => {
    const key = r.wk ?? r.week_start ?? '';
    const vault = r.vault_name ?? 'unknown';
    vaults.add(vault);
    if (!weekMap.has(key)) weekMap.set(key, {});
    weekMap.get(key)![vault] = r.avg_vault_tvl_m ?? 0;
  });

  const weeks = Array.from(weekMap.keys()).sort();
  const vaultList = Array.from(vaults);

  const [hidden, setHidden] = useState<Set<number>>(() => new Set());

  const chips = vaultList.map((vault, i) => ({
    label: vault,
    color: CHART_COLORS[i % CHART_COLORS.length],
    visible: !hidden.has(i)
  }));

  const visibleDatasets = vaultList
    .map((vault, i) => ({ vault, i }))
    .filter(({ i }) => !hidden.has(i));

  const chartData: ChartData<'bar'> = {
    labels: weeks.map(weekLabel),
    datasets: visibleDatasets.map(({ vault, i }) => ({
      label: vault,
      data: weeks.map((w) => weekMap.get(w)?.[vault] ?? 0),
      backgroundColor: CHART_COLORS[i % CHART_COLORS.length] + 'CC',
      borderRadius: 2,
      borderSkipped: false,
      stack: 'tvl'
    }))
  };
  const options: ChartOptions<'bar'> = {
    ...baseBarOpts,
    plugins: {
      legend: { display: false },
      tooltip: {
        ...baseTooltipOpts,
        callbacks: { label: (ctx) => ` ${ctx.dataset.label}: $${(ctx.raw as number).toFixed(2)}M` }
      }
    },
    scales: {
      x: { ...baseBarOpts.scales?.x, stacked: true },
      y: {
        ...baseBarOpts.scales?.y,
        stacked: true,
        ticks: { color: 'rgba(255,255,255,0.3)', font: { size: 10 }, callback: (v) => `$${v}M` }
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
