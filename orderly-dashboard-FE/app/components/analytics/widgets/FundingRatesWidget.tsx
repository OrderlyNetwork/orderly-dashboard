import {
  Chart as ChartJS,
  Filler,
  LinearScale,
  LineElement,
  PointElement,
  TimeScale,
  Tooltip,
  type ChartData,
  type ChartOptions
} from 'chart.js';
import 'chartjs-adapter-date-fns';
import { format } from 'date-fns';
import { FC, useMemo, useRef, useState } from 'react';
import { Line } from 'react-chartjs-2';

import { baseTooltipOpts, chartColor, tooltipTopN, useChartReady } from '../shared/chartConfig';
import { DatasetChips, Empty, Skeleton } from '../shared/primitives';

import { useFundingRates, useSymbolWeekly } from '~/hooks/useOrderlyMetrics';

ChartJS.register(TimeScale, LinearScale, PointElement, LineElement, Filler, Tooltip);

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
  const { seriesBySymbol, allSymbols } = useMemo(() => {
    const map = new Map<string, { x: number; y: number }[]>();
    const syms = new Set<string>();
    for (const r of rows ?? []) {
      syms.add(r.symbol);
      const arr = map.get(r.symbol) ?? [];
      arr.push({ x: new Date(r.funding_time).getTime(), y: r.funding_rate * 100 });
      map.set(r.symbol, arr);
    }
    for (const arr of map.values()) {
      arr.sort((a, b) => a.x - b.x);
    }
    return { seriesBySymbol: map, allSymbols: syms };
  }, [rows]);

  const ranked = useMemo(() => {
    return Array.from(allSymbols).sort(
      (a, b) => (volumeBySymbol.get(b) ?? 0) - (volumeBySymbol.get(a) ?? 0)
    );
  }, [allSymbols, volumeBySymbol]);

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
    datasets: chartKeys.map((key) => {
      const i = ranked.indexOf(key);
      return {
        label: key.replace('PERP_', '').replace('_USDC', ''),
        data: seriesBySymbol.get(key) ?? [],
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
    responsive: true,
    maintainAspectRatio: false,
    interaction: { mode: 'nearest', axis: 'x', intersect: false },
    plugins: {
      legend: { display: false },
      tooltip: {
        ...baseTooltipOpts,
        mode: 'nearest',
        axis: 'x',
        filter: tooltipTopN(12),
        callbacks: {
          title: (items) => {
            if (!items.length) return '';
            const x = items[0].parsed.x;
            return format(new Date(x ?? 0), 'MMM d, HH:mm');
          },
          label: (ctx) => {
            const val = ctx.parsed.y;
            if (val == null) return '';
            const sign = val >= 0 ? '+' : '';
            return ` ${ctx.dataset.label}: ${sign}${val.toFixed(4)}%`;
          }
        }
      }
    },
    scales: {
      x: {
        type: 'time',
        grid: { color: 'rgba(255,255,255,0.04)' },
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          maxTicksLimit: 8,
          maxRotation: 0
        },
        time: {
          tooltipFormat: 'MMM d, HH:mm',
          displayFormats: {
            hour: 'MMM d HH:mm',
            day: 'MMM d'
          }
        }
      },
      y: {
        grid: { color: 'rgba(255,255,255,0.04)' },
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          callback: (v) => `${(v as number) >= 0 ? '+' : ''}${(v as number).toFixed(3)}%`
        }
      }
    }
  };

  if (frLoading) return <Skeleton height={236} />;
  if (frError || !rows?.length) return <Empty msg={frError ? 'Failed to load' : 'No data'} />;

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
