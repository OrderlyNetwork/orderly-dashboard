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

import { CHART_COLORS, baseLineOpts, baseTooltipOpts } from '../shared/chartConfig';
import { weekLabel } from '../shared/formatters';
import { Empty, Skeleton } from '../shared/primitives';

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

  const chartData: ChartData<'line'> = {
    labels: weeks.map(weekLabel),
    datasets: vaultList.map((vault, i) => ({
      label: vault,
      data: weeks.map((w) => weekMap.get(w)?.[vault] ?? 0),
      fill: false,
      borderColor: CHART_COLORS[i % CHART_COLORS.length],
      backgroundColor: CHART_COLORS[i % CHART_COLORS.length] + '22',
      borderWidth: 2,
      pointRadius: 3,
      tension: 0.3
    }))
  };
  const options: ChartOptions<'line'> = {
    ...baseLineOpts,
    plugins: {
      legend: {
        display: true,
        labels: { color: 'rgba(255,255,255,0.5)', font: { size: 11 }, boxWidth: 12 }
      },
      tooltip: {
        ...baseTooltipOpts,
        callbacks: { label: (ctx) => ` ${ctx.dataset.label}: $${(ctx.raw as number).toFixed(2)}M` }
      }
    },
    scales: {
      ...baseLineOpts.scales,
      y: {
        ...baseLineOpts.scales?.y,
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
          <Line data={chartData} options={options} />
        </div>
      )}
    </>
  );
};
