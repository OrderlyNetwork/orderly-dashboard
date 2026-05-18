import { ArcElement, Chart as ChartJS, Tooltip, type ChartData, type ChartOptions } from 'chart.js';
import { FC, useRef, useState } from 'react';
import { Doughnut } from 'react-chartjs-2';

import { CHAIN_COLORS, useChartReady } from '../shared/chartConfig';
import { capitalize, fmtCompact } from '../shared/formatters';
import { DatasetChips } from '../shared/primitives';

import type { TvlChainRow } from '~/types/dashboard';

ChartJS.register(ArcElement, Tooltip);

export const TvlByChainWidget: FC<{
  chains: TvlChainRow[];
}> = ({ chains }) => {
  const chartRef = useRef<ChartJS<'doughnut'>>(null);
  useChartReady(chartRef);
  const sorted = [...chains].sort((a, b) => b.tvl_usd - a.tvl_usd).slice(0, 12);

  const [hidden, setHidden] = useState<Set<number>>(() => new Set());

  const chips = sorted.map((c, i) => ({
    label: capitalize(c.chain),
    color: CHAIN_COLORS[c.chain] ?? '#9C75FF',
    visible: !hidden.has(i)
  }));

  const visible = sorted.map((c, i) => ({ c, i })).filter(({ i }) => !hidden.has(i));

  const data: ChartData<'doughnut'> = {
    labels: visible.map(({ c }) => capitalize(c.chain)),
    datasets: [
      {
        data: visible.map(({ c }) => c.tvl_usd),
        backgroundColor: visible.map(({ c }) => (CHAIN_COLORS[c.chain] ?? '#9C75FF') + 'CC'),
        hoverBackgroundColor: visible.map(({ c }) => CHAIN_COLORS[c.chain] ?? '#9C75FF'),
        borderWidth: 0,
        spacing: 2
      }
    ]
  };

  const options: ChartOptions<'doughnut'> = {
    responsive: true,
    maintainAspectRatio: false,
    cutout: '55%',
    plugins: {
      legend: { display: false },
      tooltip: {
        callbacks: {
          label: (ctx) => {
            const total = ctx.dataset.data.reduce((s, v) => s + (v as number), 0);
            const pct = (((ctx.raw as number) / total) * 100).toFixed(1);
            return ` ${fmtCompact(ctx.raw as number)} (${pct}%)`;
          }
        }
      }
    }
  };

  return (
    <div style={{ width: '100%' }}>
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
      <div style={{ position: 'relative', height: 240 }}>
        <Doughnut ref={chartRef} data={data} options={options} />
      </div>
    </div>
  );
};
