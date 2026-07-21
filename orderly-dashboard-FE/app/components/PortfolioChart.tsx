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
import dayjs from 'dayjs';
import { FC, useMemo, useRef, useState } from 'react';
import { Line } from 'react-chartjs-2';

import { Spinner } from './Spinner';
import { baseLineOpts, baseTooltipOpts, useChartReady } from './analytics/shared/chartConfig';
import { fmtCompact, labelFromDate } from './analytics/shared/formatters';
import { PeriodSelector, PERIOD_DAYS, type Period } from './analytics/shared/primitives';

import { usePortfolio } from '~/hooks/usePublicInfo';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

interface PortfolioChartProps {
  address: string;
  brokerId?: string | null;
  accountId?: string | null;
}

export const PortfolioChart: FC<PortfolioChartProps> = ({ address, brokerId, accountId }) => {
  const [range, setRange] = useState<Period>('30D');

  const endTime = useMemo(() => Date.now(), []);
  const startTime = useMemo(() => dayjs().subtract(PERIOD_DAYS[range], 'days').valueOf(), [range]);

  const { data, isLoading, error } = usePortfolio({
    address,
    broker_id: brokerId || undefined,
    account_id: accountId || undefined,
    start_time: startTime,
    end_time: endTime,
    limit: PERIOD_DAYS[range]
  });

  const chartRef = useRef<ChartJS<'line'> | null>(null);
  useChartReady(chartRef);

  const sortedRows = useMemo(() => {
    if (!data?.rows) return [];
    return [...data.rows].sort((a, b) => a.timestamp - b.timestamp);
  }, [data]);

  if (!brokerId) {
    return (
      <div className="card p-8 flex flex-col items-center gap-4 max-w-2xl w-full">
        <p className="text-gray-400 text-sm">Select a broker to view portfolio data.</p>
      </div>
    );
  }

  if (!accountId) {
    return (
      <div className="card p-8 flex justify-center max-w-2xl w-full">
        <Spinner size="2rem" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="card p-8 flex flex-col items-center gap-4 max-w-2xl w-full">
        <p className="text-gray-400 text-sm">{error.message || 'Failed to fetch portfolio data'}</p>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="card p-6 max-w-2xl w-full">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h2 className="text-xl font-bold text-white">Portfolio</h2>
            <p className="text-gray-400 text-sm mt-1">Daily account value and cumulative PnL</p>
          </div>
          <PeriodSelector period={range} onChange={setRange} options={['7D', '30D', '90D']} />
        </div>
        <div
          className="animate-pulse rounded-md"
          style={{ height: 280, background: 'rgba(255,255,255,0.05)' }}
        />
      </div>
    );
  }

  if (sortedRows.length === 0) {
    return (
      <div className="card p-8 flex flex-col items-center gap-4 max-w-2xl w-full">
        <p className="text-gray-400 text-sm">No portfolio data available for this account.</p>
      </div>
    );
  }

  const labels = sortedRows.map((r) => labelFromDate(new Date(r.timestamp).toISOString()));
  const accountValues = sortedRows.map((r) => parseFloat(r.account_value));
  const cumulativePnls = sortedRows.map((r) => parseFloat(r.cumulative_pnl));

  const chartData = {
    labels,
    datasets: [
      {
        type: 'bar' as const,
        label: 'Account Value',
        data: accountValues,
        backgroundColor: 'rgba(156,117,255,0.65)',
        hoverBackgroundColor: 'rgba(156,117,255,0.9)',
        borderWidth: 0,
        borderRadius: 3,
        yAxisID: 'yValue',
        order: 2
      },
      {
        type: 'line' as const,
        label: 'Cumulative PnL',
        data: cumulativePnls,
        fill: false,
        backgroundColor: 'rgba(52,211,153,0.12)',
        borderColor: '#00dea3',
        borderWidth: 2,
        pointRadius: 0,
        tension: 0.35,
        yAxisID: 'yPnl',
        order: 1
      }
    ]
  };

  const options: ChartOptions<'line'> = {
    ...baseLineOpts,
    plugins: {
      legend: {
        display: true,
        position: 'top',
        align: 'end',
        labels: {
          color: 'rgba(255,255,255,0.5)',
          font: { size: 10 },
          boxWidth: 10,
          boxHeight: 10,
          borderRadius: 2,
          useBorderRadius: true,
          padding: 12
        }
      },
      tooltip: {
        ...baseTooltipOpts,
        callbacks: {
          label: (ctx) => {
            const label = ctx.dataset.label ?? '';
            return ` ${label}: ${fmtCompact(ctx.raw as number)}`;
          }
        }
      }
    },
    scales: {
      x: {
        ...baseLineOpts.scales?.x,
        ticks: {
          color: 'rgba(255,255,255,0.3)',
          font: { size: 10 },
          maxTicksLimit: range === '7D' ? 7 : 8,
          maxRotation: 0
        }
      },
      yValue: {
        position: 'left',
        grid: { color: 'rgba(255,255,255,0.04)' },
        ticks: {
          color: 'rgba(156,117,255,0.6)',
          font: { size: 10 },
          callback: (v) => fmtCompact(v as number)
        }
      },
      yPnl: {
        position: 'right',
        grid: { drawOnChartArea: false },
        ticks: {
          color: 'rgba(52,211,153,0.6)',
          font: { size: 10 },
          callback: (v) => fmtCompact(v as number)
        }
      }
    }
  };

  return (
    <div className="card p-6 max-w-2xl w-full">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h2 className="text-xl font-bold text-white">Portfolio</h2>
          <p className="text-gray-400 text-sm mt-1">Daily account value and cumulative PnL</p>
        </div>
        <PeriodSelector period={range} onChange={setRange} options={['7D', '30D', '90D']} />
      </div>
      <div style={{ position: 'relative', width: '100%', height: 280, overflow: 'hidden' }}>
        <Line ref={chartRef} data={chartData as ChartData<'line'>} options={options} />
      </div>
    </div>
  );
};
