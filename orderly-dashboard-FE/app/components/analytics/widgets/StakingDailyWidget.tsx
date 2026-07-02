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
import { FC, useRef } from 'react';
import { Line } from 'react-chartjs-2';

import { baseLineOpts, baseTooltipOpts, useChartReady } from '../shared/chartConfig';
import { fmtNum, labelFromDate } from '../shared/formatters';
import { Empty, LineChartSkeleton } from '../shared/primitives';

import { useStakingDaily } from '~/hooks/useOrderlyMetrics';

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Filler, Tooltip);

export const StakingDailyWidget: FC = () => {
  const { data, isLoading, error } = useStakingDaily();
  const chartRef = useRef<ChartJS<'line'>>(null);
  useChartReady(chartRef);
  const rows = [...(data?.rows ?? [])].sort((a, b) => a.date.localeCompare(b.date));

  const chartData: ChartData<'line'> = {
    labels: rows.map((r) => labelFromDate(r.date)),
    datasets: [
      {
        label: 'Net Staked ORDER',
        data: rows.map((r) => r.net_staked_order ?? 0),
        borderColor: '#9C75FF',
        backgroundColor: 'rgba(156,117,255,0.1)',
        fill: true,
        tension: 0.3,
        pointRadius: 0,
        borderWidth: 2,
        yAxisID: 'y'
      },
      {
        label: 'Daily Burned ORDER',
        data: rows.map((r) => r.daily_burned_order ?? 0),
        borderColor: '#FF6390',
        backgroundColor: 'rgba(248,113,113,0.1)',
        fill: true,
        tension: 0.3,
        pointRadius: 0,
        borderWidth: 2,
        yAxisID: 'y1'
      }
    ]
  };

  const netStaked = rows.map((r) => r.net_staked_order ?? 0);
  const dailyBurned = rows.map((r) => r.daily_burned_order ?? 0);
  const netMin = Math.min(...netStaked);
  const netMax = Math.max(...netStaked);
  const burnedMax = Math.max(...dailyBurned);

  const yPad = Math.max(Math.abs(netMax - netMin) * 0.1, 1);
  const yMin = netMin < 0 ? netMin - yPad : 0;
  const yMax = Math.max(netMax + yPad, 0);
  const y1Pad = Math.max(burnedMax * 0.1, 1);
  const y1Max = Math.max(burnedMax + y1Pad, 0);
  let y1Min = 0;

  const zeroPos = yMin < 0 ? (0 - yMin) / (yMax - yMin) : 0;
  if (zeroPos > 0) {
    y1Min = (zeroPos * y1Max) / (zeroPos - 1);
  }

  const options: ChartOptions<'line'> = {
    ...baseLineOpts,
    plugins: {
      legend: { display: false },
      tooltip: {
        ...baseTooltipOpts,
        callbacks: {
          label: (ctx) => ` ${ctx.dataset.label}: ${fmtNum(ctx.raw as number)}`
        }
      }
    },
    scales: {
      x: baseLineOpts.scales?.x,
      y: {
        ...baseLineOpts.scales?.y,
        position: 'left',
        min: yMin,
        max: yMax,
        ticks: {
          color: 'rgba(156,117,255,0.5)',
          font: { size: 10 },
          callback: (v) => fmtNum(v as number)
        }
      },
      y1: {
        ...baseLineOpts.scales?.y,
        position: 'right',
        min: y1Min,
        max: y1Max,
        grid: { drawOnChartArea: false },
        ticks: {
          color: 'rgba(248,113,113,0.5)',
          font: { size: 10 },
          callback: (v) => fmtNum(v as number)
        }
      }
    }
  };

  if (isLoading) return <LineChartSkeleton height={236} />;
  if (error || rows.length === 0) return <Empty msg={error ? 'Failed to load' : 'No data'} />;

  return (
    <div style={{ position: 'relative', width: '100%', height: 260 }}>
      <Line ref={chartRef} data={chartData} options={options} />
    </div>
  );
};
