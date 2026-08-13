import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Filler,
  Tooltip,
  type ChartOptions
} from 'chart.js';
import dayjs from 'dayjs';
import { FC, useMemo, useRef } from 'react';
import { Line } from 'react-chartjs-2';

import { useChartReady } from '~/components/analytics/shared/chartConfig';
import { LineChartSkeleton } from '~/components/analytics/shared/primitives';
import { WidgetShareButton } from '~/components/analytics/widgets/WidgetShareButton';
import { useIsEmbed } from '~/hooks/useIsEmbed';
import type { FundingRateEntry } from '~/hooks/usePublicInfo';
import { getBaseToken } from '~/hooks/useSymbols';
import { formatPriceByTick } from '~/utils/format';

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Filler, Tooltip);

export type FundingChartProps = {
  symbol?: string;
  fundingHistory?: FundingRateEntry[];
  isLoading?: boolean;
  quoteTick?: number | null;
};

export const FundingChart: FC<FundingChartProps> = ({
  symbol,
  fundingHistory,
  isLoading,
  quoteTick
}) => {
  const isEmbed = useIsEmbed();
  const chartRef = useRef<ChartJS<'line'> | null>(null);
  useChartReady(chartRef);

  const chartData = useMemo(() => {
    if (!fundingHistory || fundingHistory.length === 0) return null;

    const sorted = [...fundingHistory].sort(
      (a, b) => a.funding_rate_timestamp - b.funding_rate_timestamp
    );

    return {
      labels: sorted.map((f) => dayjs(f.funding_rate_timestamp).format('MMM d HH:mm')),
      datasets: [
        {
          label: 'Funding Rate',
          data: sorted.map((f) => parseFloat(f.funding_rate) * 100), // Convert to percentage
          borderColor: '#9C75FF',
          backgroundColor: 'rgba(156,117,255,0.1)',
          fill: true,
          tension: 0.3,
          pointRadius: 0,
          pointHitRadius: 8,
          borderWidth: 1.5
        }
      ]
    };
  }, [fundingHistory]);

  const options = useMemo<ChartOptions<'line'>>(
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
              const val = ctx.parsed.y;
              if (val == null) return '';
              return `Funding: ${val >= 0 ? '+' : ''}${val.toFixed(4)}%`;
            },
            afterLabel: (ctx) => {
              if (!fundingHistory) return '';
              const sorted = [...fundingHistory].sort(
                (a, b) => a.funding_rate_timestamp - b.funding_rate_timestamp
              );
              const entry = sorted[ctx.dataIndex];
              if (!entry) return '';
              return `Mark Price: $${formatPriceByTick(parseFloat(entry.mark_price), quoteTick)}`;
            }
          }
        }
      },
      scales: {
        x: {
          grid: { color: 'rgba(255,255,255,0.04)' },
          ticks: {
            color: 'rgba(255,255,255,0.3)',
            font: { size: 10 },
            maxTicksLimit: 8,
            maxRotation: 0
          }
        },
        y: {
          grid: { color: 'rgba(255,255,255,0.04)' },
          ticks: {
            color: 'rgba(255,255,255,0.3)',
            font: { size: 10 },
            callback: (val) => `${Number(val).toFixed(3)}%`
          }
        }
      }
    }),
    [fundingHistory, quoteTick]
  );

  const suffix = symbol && isEmbed ? ` — ${getBaseToken(symbol)}-PERP` : '';
  const title = `Funding Rate History${suffix}`;

  return (
    <div
      data-widget-id="market-funding-chart"
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
            {title}
          </div>
          <div className="text-[13px] mt-0.5 text-[rgba(255,255,255,0.35)]">
            8-hour funding rate epochs
          </div>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-xs text-gray-500">{fundingHistory?.length ?? 0} epochs</span>
          {symbol && (
            <WidgetShareButton widgetId="market-funding-chart" title={title} symbol={symbol} />
          )}
        </div>
      </div>
      <div className="px-4 pt-3 pb-4" style={{ height: 280 }}>
        {isLoading && !fundingHistory ? (
          <LineChartSkeleton height={260} />
        ) : chartData ? (
          <Line ref={chartRef} data={chartData} options={options} />
        ) : (
          <div className="flex items-center justify-center h-full text-[rgba(255,255,255,0.25)] text-[13px]">
            No funding history available
          </div>
        )}
      </div>
    </div>
  );
};
