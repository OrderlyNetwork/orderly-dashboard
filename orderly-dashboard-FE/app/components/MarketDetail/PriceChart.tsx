import { lazy, Suspense, type FC } from 'react';

import { ClientOnly } from '~/components/ClientOnly';
import { LineChartSkeleton } from '~/components/analytics/shared/primitives';
import type { Candle } from '~/hooks/usePublicInfo';

const PriceChartClient = lazy(() =>
  import('./PriceChart.client').then((m) => ({ default: m.PriceChartClient }))
);

export type PriceChartProps = {
  symbol: string;
  candles?: Candle[];
  isLoading?: boolean;
  interval: string;
  onIntervalChange: (interval: string) => void;
};

export const PriceChart: FC<PriceChartProps> = (props) => {
  return (
    <ClientOnly fallback={<PriceChartShell />}>
      <Suspense fallback={<PriceChartShell />}>
        <PriceChartClient {...props} />
      </Suspense>
    </ClientOnly>
  );
};

const PriceChartShell: FC = () => (
  <div
    className="rounded-2xl overflow-hidden"
    style={{ background: 'rgba(20,15,35,.9)', border: '1px solid rgba(156,117,255,0.15)' }}
  >
    <div
      className="flex items-center justify-between border-b px-5 py-4"
      style={{ borderBottomColor: 'rgba(156,117,255,0.08)' }}
    >
      <div>
        <div className="text-lg font-semibold text-white">Price Chart</div>
        <div className="text-[13px] mt-0.5 text-[rgba(255,255,255,0.35)]">OHLCV candles</div>
      </div>
    </div>
    <div className="px-4 pt-3 pb-4" style={{ height: 420 }}>
      <LineChartSkeleton height={400} />
    </div>
  </div>
);
