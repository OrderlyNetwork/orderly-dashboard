import { Chart as ChartJS, type ChartOptions } from 'chart.js';
import { useEffect, type RefObject } from 'react';

export function useChartReady<T extends 'bar' | 'line' | 'doughnut'>(chartRef: RefObject<ChartJS<T> | null>) {
  useEffect(() => {
    requestAnimationFrame(() => {
      const chart = chartRef.current;
      if (chart) {
        chart.resize();
        chart.update('none');
      }
    });
  }, []);
}

export const CHAIN_COLORS: Record<string, string> = {
  solana: '#9945FF',
  arbitrum: '#28A0F0',
  ethereum: '#627EEA',
  bnb: '#F3BA2F',
  optimism: '#FF0420',
  base: '#0052FF',
  polygon: '#8247E5',
  avalanche: '#E84142',
  abstract: '#A0F0A0',
  berachain: '#F5A623',
  ceffu: '#34d399',
  story: '#FF6B6B',
  sonic: '#60a5fa',
  mode: '#9C75FF',
  sei: '#F87171',
  mantle: '#C084FC',
  plume: '#FCD34D',
  monad: '#6EE7B7',
  morph: '#A78BFA'
};

export const baseTooltipOpts = {
  mode: 'index' as const,
  intersect: false,
  itemSort: (a: { raw: unknown }, b: { raw: unknown }) => (b.raw as number) - (a.raw as number)
};

export const CHART_COLORS = [
  '#9C75FF',
  '#34d399',
  '#60a5fa',
  '#f59e0b',
  '#f87171',
  '#a78bfa',
  '#fb923c'
];

export const baseLineOpts: ChartOptions<'line'> = {
  responsive: true,
  maintainAspectRatio: false,
  interaction: { mode: 'index', intersect: false },
  plugins: { legend: { display: false }, tooltip: { ...baseTooltipOpts } },
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
      ticks: { color: 'rgba(255,255,255,0.3)', font: { size: 10 } }
    }
  }
};

export const baseBarOpts: ChartOptions<'bar'> = {
  responsive: true,
  maintainAspectRatio: false,
  interaction: { mode: 'index', intersect: false },
  plugins: { legend: { display: false }, tooltip: { ...baseTooltipOpts } },
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
      ticks: { color: 'rgba(255,255,255,0.3)', font: { size: 10 } }
    }
  }
};
