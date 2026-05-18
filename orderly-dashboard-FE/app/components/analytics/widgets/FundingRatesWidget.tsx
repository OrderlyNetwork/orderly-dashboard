import {
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
import { Line } from 'react-chartjs-2';

import {
  baseLineOpts,
  baseTooltipOpts,
  chartColor,
  tooltipTopN,
  useChartReady
} from '../shared/chartConfig';
import { labelFromDate } from '../shared/formatters';
import { DatasetChips, Empty, Skeleton } from '../shared/primitives';

import { useFundingRates, useSymbolWeekly } from '~/hooks/useOrderlyMetrics';

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Filler, Tooltip);

export const FundingRatesWidget: FC = () => {
  const { data: frData, isLoading: frLoading, error: frError } = useFundingRates();
  const { data: volData } = useSymbolWeekly();
  const chartRef = useRef<ChartJS<'line'>>(null);
  useChartReady(chartRef);

  const volumeBySymbol = useMemo(() => {
    const totals = new Map<string, number>();
    for (const r of volData?.rows ?? []) {
      if (r.volume_usd && r.symbol !== 'ALL') {
        totals.set(r.symbol, (totals.get(r.symbol) ?? 0) + r.volume_usd);
      }
    }
    return totals;
  }, [volData]);

  const rows = frData?.rows;
  const { timeMap, allSymbols } = useMemo(() => {
    const tm = new Map<string, Record<string, number>>();
    const syms = new Set<string>();
    for (const r of rows ?? []) {
      const key = r.funding_time;
      const sym = r.symbol;
      syms.add(sym);
      if (!tm.has(key)) tm.set(key, {});
      tm.get(key)![sym] = r.funding_rate;
    }
    return { timeMap: tm, allSymbols: syms };
  }, [rows]);

  const ranked = useMemo(() => {
    return Array.from(allSymbols).sort(
      (a, b) => (volumeBySymbol.get(b) ?? 0) - (volumeBySymbol.get(a) ?? 0)
    );
  }, [allSymbols, volumeBySymbol]);

  const times = Array.from(timeMap.keys()).sort();

  const [hidden, setHidden] = useState<Set<string>>(() => new Set());

  const chartKeys = useMemo(
    () => ranked.filter((k) => !hidden.has(k)).slice(0, 12),
    [ranked, hidden]
  );

  const chips = ranked.map((key, i) => ({
    label: key.replace('PERP_', '').replace('_USDC', ''),
    color: chartColor(i),
    visible: !hidden.has(key)
  }));

  const chartData: ChartData<'line'> = {
    labels: times.map((t) => labelFromDate(t)),
    datasets: chartKeys.map((key) => {
      const i = ranked.indexOf(key);
      return {
        label: key.replace('PERP_', '').replace('_USDC', ''),
        data: times.map((t) => (timeMap.get(t)?.[key] ?? 0) * 100),
        borderColor: chartColor(i),
        backgroundColor: chartColor(i) + '20',
        tension: 0.2,
        pointRadius: 0,
        borderWidth: 1.5,
        fill: false
      };
    })
  };

  const options: ChartOptions<'line'> = {
    ...baseLineOpts,
    plugins: {
      legend: { display: false },
      tooltip: {
        ...baseTooltipOpts,
        filter: tooltipTopN(12),
        callbacks: {
          label: (ctx) => {
            const val = ctx.raw as number;
            const sign = val >= 0 ? '+' : '';
            return ` ${ctx.dataset.label}: ${sign}${val.toFixed(4)}%`;
          }
        }
      }
    },
    scales: {
      x: baseLineOpts.scales?.x,
      y: {
        ...baseLineOpts.scales?.y,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          callback: (v) => `${(v as number) >= 0 ? '+' : ''}${(v as number).toFixed(3)}%`
        }
      }
    }
  };

  if (frLoading) return <Skeleton height={236} />;
  if (frError || times.length === 0) return <Empty msg={frError ? 'Failed to load' : 'No data'} />;

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
        <Line ref={chartRef} data={chartData} options={options} />
      </div>
    </div>
  );
};
