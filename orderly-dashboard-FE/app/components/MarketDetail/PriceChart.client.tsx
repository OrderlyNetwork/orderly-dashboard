import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  TimeScale,
  Tooltip,
  type ChartOptions
} from 'chart.js';
import { CandlestickController, CandlestickElement } from 'chartjs-chart-financial';
import 'chartjs-adapter-date-fns';
import { FC, useEffect, useMemo, useRef } from 'react';
import { Chart } from 'react-chartjs-2';

import { LineChartSkeleton } from '~/components/analytics/shared/primitives';
import { WidgetShareButton } from '~/components/analytics/widgets/WidgetShareButton';
import { useIsEmbed } from '~/hooks/useIsEmbed';
import type { Candle } from '~/hooks/usePublicInfo';
import { formatPriceByTick } from '~/utils/format';

ChartJS.register(
  CategoryScale,
  LinearScale,
  TimeScale,
  Tooltip,
  CandlestickController,
  CandlestickElement
);

const INTERVAL_LABELS: Record<string, string> = {
  '15m': '15m',
  '1h': '1H',
  '4h': '4H',
  '1d': '1D'
};

const baseToken = (symbol: string) => symbol.split('_')[1] ?? symbol;

export type PriceChartClientProps = {
  symbol: string;
  candles?: Candle[];
  isLoading?: boolean;
  interval: string;
  onIntervalChange: (interval: string) => void;
  quoteTick?: number | null;
};

export const PriceChartClient: FC<PriceChartClientProps> = ({
  symbol,
  candles,
  isLoading,
  interval,
  onIntervalChange,
  quoteTick
}) => {
  const isEmbed = useIsEmbed();
  const chartRef = useRef<ChartJS<'candlestick'> | null>(null);

  useEffect(() => {
    requestAnimationFrame(() => {
      const chart = chartRef.current;
      if (chart) {
        chart.resize();
        chart.update('none');
      }
    });
  }, []);

  const chartData = useMemo(() => {
    if (!candles || candles.length === 0) return null;

    const sorted = [...candles].sort((a, b) => a.timestamp - b.timestamp);

    return {
      datasets: [
        {
          label: symbol,
          data: sorted.map((c) => ({
            x: c.timestamp,
            o: parseFloat(c.open),
            h: parseFloat(c.high),
            l: parseFloat(c.low),
            c: parseFloat(c.close)
          })),
          backgroundColors: {
            up: '#00dea3',
            down: '#FF6390',
            unchanged: 'rgba(255,255,255,0.5)'
          },
          borderColors: {
            up: '#00dea3',
            down: '#FF6390',
            unchanged: 'rgba(255,255,255,0.5)'
          },
          borderWidth: 1
        }
      ]
    };
  }, [candles, symbol]);

  const options = useMemo<ChartOptions<'candlestick'>>(
    () => ({
      responsive: true,
      maintainAspectRatio: false,
      interaction: { mode: 'index', intersect: false },
      plugins: {
        legend: { display: false },
        tooltip: {
          mode: 'index',
          intersect: false,
          callbacks: {
            label: (ctx) => {
              const raw = ctx.raw as { o: number; h: number; l: number; c: number } | undefined;
              if (!raw) return '';
              return [
                `O: ${formatPriceByTick(raw.o, quoteTick)}`,
                `H: ${formatPriceByTick(raw.h, quoteTick)}`,
                `L: ${formatPriceByTick(raw.l, quoteTick)}`,
                `C: ${formatPriceByTick(raw.c, quoteTick)}`
              ];
            }
          }
        }
      },
      scales: {
        x: {
          type: 'time',
          time: {
            unit: interval === '15m' || interval === '1h' ? 'hour' : 'day',
            displayFormats: {
              hour: 'MMM d HH:mm',
              day: 'MMM d'
            }
          },
          grid: { color: 'rgba(255,255,255,0.04)' },
          ticks: {
            color: 'rgba(255,255,255,0.3)',
            font: { size: 10 },
            maxTicksLimit: 8,
            maxRotation: 0
          }
        },
        y: {
          position: 'right',
          grid: { color: 'rgba(255,255,255,0.04)' },
          ticks: {
            color: 'rgba(255,255,255,0.3)',
            font: { size: 10 },
            callback: (val) => formatPriceByTick(Number(val), quoteTick)
          }
        }
      }
    }),
    [interval, quoteTick]
  );

  return (
    <div
      className="rounded-2xl overflow-hidden"
      style={{ background: 'rgba(20,15,35,.9)', border: '1px solid rgba(156,117,255,0.15)' }}
    >
      <div
        className="flex items-center justify-between border-b px-5 py-4"
        style={{ borderBottomColor: 'rgba(156,117,255,0.08)' }}
      >
        <div>
          <div
            className="text-lg font-semibold text-white"
            style={{
              fontFamily: "'Atyp BL Text', sans-serif",
              fontFeatureSettings: "'ss02' 1, 'ss03' 1, 'ss05' 1"
            }}
          >
            Price Chart{isEmbed ? ` — ${baseToken(symbol)}-PERP` : ''}
          </div>
          <div className="text-[13px] mt-0.5 text-[rgba(255,255,255,0.35)]">
            OHLCV candles for {symbol}
          </div>
        </div>
        <div className="flex items-center gap-2">
          <div className="flex gap-1 rounded-lg p-1" style={{ background: '#130E1D' }}>
            {Object.entries(INTERVAL_LABELS).map(([iv, label]) => (
              <button
                key={iv}
                onClick={() => onIntervalChange(iv)}
                className="px-3 py-1 rounded-md border-none cursor-pointer text-[13px] transition-all duration-150"
                style={{
                  background: interval === iv ? '#6700CE' : 'transparent',
                  color: interval === iv ? '#E9DEFF' : 'rgba(255,255,255,0.45)',
                  fontWeight: interval === iv ? 600 : 400
                }}
              >
                {label}
              </button>
            ))}
          </div>
          <WidgetShareButton
            widgetId="market-price-chart"
            title={`Price Chart${isEmbed ? ` — ${baseToken(symbol)}-PERP` : ''}`}
            symbol={symbol}
          />
        </div>
      </div>
      <div className="px-4 pt-3 pb-4 relative" style={{ height: 420 }}>
        {isLoading && !candles ? (
          <LineChartSkeleton height={400} />
        ) : chartData ? (
          <>
            <Chart ref={chartRef} type="candlestick" data={chartData} options={options} />
            {isLoading && candles && (
              <div
                className="absolute inset-0 flex items-center justify-center"
                style={{
                  background: 'rgba(10,0,16,0.5)',
                  backdropFilter: 'blur(4px)',
                  zIndex: 10
                }}
              >
                <div
                  style={{
                    width: 36,
                    height: 36,
                    borderRadius: '50%',
                    border: '3px solid rgba(156,117,255,0.2)',
                    borderTopColor: '#9C75FF',
                    animation: 'spin 0.7s linear infinite'
                  }}
                />
                <style>{`@keyframes spin{to{transform:rotate(360deg)}}`}</style>
              </div>
            )}
          </>
        ) : (
          <div className="flex items-center justify-center h-full text-[rgba(255,255,255,0.25)] text-[13px]">
            No candle data available
          </div>
        )}
      </div>
    </div>
  );
};
