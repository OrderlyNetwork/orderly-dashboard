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
            <h1 className="m-0 text-[22px] font-bold text-white">Explorer</h1>
            <p className="m-0 text-[13px] text-[rgba(255,255,255,0.45)]">
              Search wallets and accounts to view detailed trading history
            </p>
          </div>
        </div>
      </div>

      <div className="rounded-2xl p-5 py-8 mb-8 md:px-9" style={{ background: '#6700CE' }}>
        <h3 className="m-0 mb-2 text-lg font-bold text-white">Explore Trading Data</h3>
        <p className="m-0 mb-5 text-sm text-[rgba(255,255,255,0.8)] leading-relaxed">
          Search for wallet addresses or account IDs to view detailed trading information including
          executed trades, deposits &amp; withdrawals, liquidations, and performance metrics.
        </p>

        <div className="flex gap-5 mb-5 flex-wrap">
          <div className="flex items-center gap-1.5">
            <div className="w-2 h-2 rounded-full bg-[#34D399]" />
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

        <SearchInput />
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {[
          {
            icon: (
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <polyline points="22 7 13.5 15.5 8.5 10.5 2 17" />
                <polyline points="16 7 22 7 22 13" />
              </svg>
            ),
            color: '#34D399',
            bg: 'rgba(52,211,153,0.1)',
            border: 'rgba(52,211,153,0.2)',
            title: 'Trade History',
            desc: 'View all executed perp trades with timestamps, prices, and fees'
          },
          {
            icon: (
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <line x1="12" y1="1" x2="12" y2="23" />
                <path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
              </svg>
            ),
            color: '#FBBF24',
            bg: 'rgba(251,191,36,0.1)',
            border: 'rgba(251,191,36,0.2)',
            title: 'Deposits & Withdrawals',
            desc: 'Track on-chain deposit and withdrawal events across all supported chains'
          },
          {
            icon: (
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
                <line x1="12" y1="9" x2="12" y2="13" />
                <line x1="12" y1="17" x2="12.01" y2="17" />
              </svg>
            ),
            color: '#F87171',
            bg: 'rgba(248,113,113,0.1)',
            border: 'rgba(248,113,113,0.2)',
            title: 'Liquidations',
            desc: 'See liquidation events and the positions that were closed forcefully'
          },
          {
            icon: (
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <circle cx="12" cy="12" r="10" />
                <polyline points="12 6 12 12 16 14" />
              </svg>
            ),
            color: '#9C75FF',
            bg: 'rgba(156,117,255,0.1)',
            border: 'rgba(156,117,255,0.2)',
            title: 'Performance Metrics',
            desc: 'Realized PnL, volume totals, and fee breakdown over any date range'
          }
        ].map((card) => (
          <div
            key={card.title}
            className="rounded-xl py-4 px-[18px]"
            style={{ border: `1px solid ${card.border}`, background: card.bg }}
          >
            <div className="flex items-center gap-2 mb-2">
              <span style={{ color: card.color }}>{card.icon}</span>
              <span className="text-[13px] font-semibold text-white">{card.title}</span>
            </div>
            <p className="m-0 text-xs text-[rgba(255,255,255,0.45)] leading-relaxed">{card.desc}</p>
          </div>
        ))}
      </div>
    </div>
  );
};
