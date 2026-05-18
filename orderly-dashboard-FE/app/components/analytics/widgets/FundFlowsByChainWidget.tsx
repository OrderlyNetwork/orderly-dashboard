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
  CHAIN_COLORS,
  chartColor,
  normalizeChainName,
  rankKeys,
  tooltipTopN,
  useChartReady
} from '../shared/chartConfig';
import { fmtUsd, labelFromDate } from '../shared/formatters';
import { DatasetChips, Empty, Skeleton } from '../shared/primitives';

import { useFundFlowsByChain } from '~/hooks/useOrderlyMetrics';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

export const FundFlowsByChainWidget: FC = () => {
  const { data, isLoading, error } = useFundFlowsByChain();
  const chartRef = useRef<ChartJS<'bar'>>(null);
  useChartReady(chartRef);
  const rows = data?.rows ?? [];

  const dateMap = new Map<string, Record<string, number>>();
  const allKeys = new Set<string>();
  rows.forEach((r) => {
    const key = r.date;
    const chain = normalizeChainName(r.chain);
    allKeys.add(chain);
    if (!dateMap.has(key)) dateMap.set(key, {});
    dateMap.get(key)![chain] = r.net_flow_usd ?? 0;
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
    color: CHAIN_COLORS[key.toLowerCase()] || chartColor(i),
    visible: !hidden.has(key)
  }));

  const chartData: ChartData<'bar'> = {
    labels: dates.map(labelFromDate),
    datasets: chartKeys.map((key) => {
      const i = ranked.indexOf(key);
      const color = CHAIN_COLORS[key.toLowerCase()] || chartColor(i);
      return {
        label: key,
        data: dates.map((d) => dateMap.get(d)?.[key] ?? 0),
        backgroundColor: color + 'CC',
        borderRadius: 2,
        borderSkipped: false,
        stack: 'netflow'
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
          label: (ctx) => {
            const val = ctx.raw as number;
            const sign = val >= 0 ? '+' : '';
            return ` ${ctx.dataset.label}: ${sign}${fmtUsd(val)}`;
          }
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

  if (isLoading) return <Skeleton height={236} />;
  if (error || dates.length === 0) return <Empty msg={error ? 'Failed to load' : 'No data'} />;

  return (
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
  );
};
