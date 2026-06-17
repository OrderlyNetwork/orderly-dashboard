import { FC } from 'react';

import { SearchInput } from '~/components/SearchInput';

export const ExplorerView: FC = () => {
  return (
    <div className="max-w-[860px]">
      <div className="rounded-2xl p-5 py-8 mb-8 md:px-9" style={{ background: '#6700CE' }}>
        <h3 className="m-0 mb-2 text-lg font-bold text-white">Explore Trading Data</h3>
        <p className="m-0 mb-5 text-sm text-[rgba(255,255,255,0.8)] leading-relaxed max-w-2xl">
          Search for wallet addresses or account IDs to view detailed trading information including
          executed trades, deposits &amp; withdrawals, liquidations, and performance metrics.
        </p>

        <div className="flex gap-5 mb-5 flex-wrap">
          <div className="flex items-center gap-1.5">
            <div className="w-2 h-2 rounded-full bg-[#00dea3]" />
            <span className="text-[13px] font-semibold text-white">EVM Addresses</span>
          </div>
          <div className="flex items-center gap-1.5">
            <div className="w-2 h-2 rounded-full bg-[#A78BFA]" />
            <span className="text-[13px] font-semibold text-white">Solana Addresses</span>
          </div>
          <div className="flex items-center gap-1.5">
            <div className="w-2 h-2 rounded-full bg-[#FBBF24]" />
            <span className="text-[13px] font-semibold text-white">Account IDs</span>
          </div>
        </div>

        <div className="w-full max-w-md">
          <SearchInput />
        </div>
      </div>
    </div>
  );
};
