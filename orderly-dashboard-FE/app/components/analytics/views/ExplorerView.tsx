import { FC } from 'react';

import { SearchInput } from '~/components/SearchInput';

export const ExplorerView: FC = () => {
  return (
    <div className="max-w-[860px]">
      <div className="mb-7">
        <div className="flex items-center gap-3 mb-1.5">
          <div
            className="w-9 h-9 rounded-[10px] flex items-center justify-center text-[#34D399]"
            style={{
              background: 'rgba(52,211,153,0.15)',
              border: '1px solid rgba(52,211,153,0.3)'
            }}
          >
            <svg
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <circle cx="11" cy="11" r="8" />
              <line x1="21" y1="21" x2="16.65" y2="16.65" />
            </svg>
          </div>
          <div>
            <h1 className="m-0 text-[22px] font-bold text-white">Search</h1>
            <p className="m-0 text-[13px] text-[rgba(255,255,255,0.45)]">
              Search wallets and accounts to view detailed trading history
            </p>
          </div>
        </div>
      </div>

      <div className="rounded-2xl p-5 py-8 mb-8 md:px-9" style={{ background: '#6700CE' }}>
        <h3 className="m-0 mb-2 text-lg font-bold text-white">Explore Trading Data</h3>
        <p className="m-0 mb-5 text-sm text-[rgba(255,255,255,0.8)] leading-relaxed max-w-2xl">
          Search for wallet addresses or account IDs to view detailed trading information including
          executed trades, deposits &amp; withdrawals, liquidations, and performance metrics.
        </p>

        <div className="flex gap-5 mb-5 flex-wrap justify-center">
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
