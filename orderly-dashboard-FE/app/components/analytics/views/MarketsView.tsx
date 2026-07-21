import { FC } from 'react';

import { Markets } from '~/components/Markets';

export const MarketsView: FC = () => {
  return (
    <div className="w-full">
      <div className="text-center space-y-2 mb-6">
        <h2 className="text-2xl font-bold text-white">Markets</h2>
        <p className="text-gray-400 max-w-2xl mx-auto text-sm">
          Real-time overview of all perpetual markets. Track prices, volume, open interest, funding
          rates, and long/short ratios.
        </p>
      </div>
      <Markets />
    </div>
  );
};
