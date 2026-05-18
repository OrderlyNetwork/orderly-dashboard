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
import { FC, useMemo, useRef, useState } from 'react';
import { Bar } from 'react-chartjs-2';

import {
  baseBarOpts,
  baseTooltipOpts,
  chartColor,
  rankKeys,
  tooltipTopN,
  useChartReady
} from '../shared/chartConfig';
import { fmtUsd, labelFromDate } from '../shared/formatters';
import { DatasetChips, Empty, Skeleton } from '../shared/primitives';

import { useTokenTvl } from '~/hooks/useOrderlyMetrics';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

export const TvlByTokenWidget: FC = () => {
  const { data, isLoading, error } = useTokenTvl();
  const chartRef = useRef<ChartJS<'bar'>>(null);
  useChartReady(chartRef);
  const rows = data?.rows ?? [];

  const dateMap = new Map<string, Record<string, number>>();
  const allKeys = new Set<string>();
  rows.forEach((r) => {
    if (!r.tvl_usd || r.tvl_usd <= 0) return;
    const key = r.date;
    const token = r.symbol;
    allKeys.add(token);
    if (!dateMap.has(key)) dateMap.set(key, {});
    dateMap.get(key)![token] = r.tvl_usd;
  });

  const { ranked } = rankKeys(dateMap, Array.from(allKeys));
  const dates = Array.from(dateMap.keys()).sort();

  const [hidden, setHidden] = useState<Set<string>>(() => new Set());

  const chartKeys = useMemo(
    () => ranked.filter((k) => !hidden.has(k)).slice(0, 12),
    [ranked, hidden]
  );

  const chips = ranked.map((key, i) => ({
    label: key,
    color: chartColor(i),
    visible: !hidden.has(key)
  }));

  const chartData: ChartData<'bar'> = {
    labels: dates.map(labelFromDate),
    datasets: chartKeys.map((key) => {
      const i = ranked.indexOf(key);
      return {
        label: key,
        data: dates.map((d) => dateMap.get(d)?.[key] ?? 0),
        backgroundColor: chartColor(i) + 'CC',
        borderRadius: 2,
        borderSkipped: false,
        stack: 'tvl'
      };
    })
  };

  const options: ChartOptions<'bar'> = {
    ...baseBarOpts,
    plugins: {
      legend: { display: false },
      tooltip: {
        ...baseTooltipOpts,
        filter: tooltipTopN(12),
        callbacks: {
          label: (ctx) => ` ${ctx.dataset.label}: ${fmtUsd(ctx.raw as number)}`
        }
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
      ) : error || dates.length === 0 ? (
        <Empty msg={error ? 'Failed to load' : 'No data'} />
      ) : (
        <div style={{ position: 'relative', width: '100%', height: '100%' }}>
          <DatasetChips
            items={chips}
            onToggle={(i) =>
              setHidden((prev) => {
                const next = new Set(prev);
                const key = ranked[i];
                if (next.has(key)) next.delete(key);
                else next.add(key);
                return next;
              })
            }
            maxHeight={85}
            onSelectAll={() => setHidden(new Set())}
            onDeselectAll={() => setHidden(new Set(ranked))}
          />
          <div style={{ height: 260 }}>
            <Bar ref={chartRef} data={chartData} options={options} />
          </div>
        </div>
      )}
    </>
  );
};
