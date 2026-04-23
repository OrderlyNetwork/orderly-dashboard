import { FC } from 'react';

import { SearchInput } from '~/components/SearchInput';

export const ExplorerView: FC = () => {
  return (
    <div style={{ maxWidth: 860 }}>
      {/* Header */}
      <div style={{ marginBottom: 28 }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginBottom: 6 }}>
          <div
            style={{
              width: 36,
              height: 36,
              borderRadius: 10,
              background: 'rgba(52,211,153,0.15)',
              border: '1px solid rgba(52,211,153,0.3)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#34D399'
            }}
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <circle cx="11" cy="11" r="8" />
              <line x1="21" y1="21" x2="16.65" y2="16.65" />
            </svg>
          </div>
          <div>
            <h1 style={{ margin: 0, fontSize: 22, fontWeight: 700, color: '#fff' }}>Explorer</h1>
            <p style={{ margin: 0, fontSize: 13, color: 'rgba(255,255,255,0.45)' }}>
              Search wallets and accounts to view detailed trading history
            </p>
          </div>
        </div>
      </div>

      {/* Search card */}
      <div
        style={{
          background: '#6700CE',
          borderRadius: 16,
          padding: '32px 36px',
          marginBottom: 32
        }}
      >
        <h3 style={{ margin: '0 0 8px', fontSize: 18, fontWeight: 700, color: '#fff' }}>
          Explore Trading Data
        </h3>
        <p style={{ margin: '0 0 20px', fontSize: 14, color: 'rgba(255,255,255,0.8)', lineHeight: 1.6 }}>
          Search for wallet addresses or account IDs to view detailed trading information
          including executed trades, deposits &amp; withdrawals, liquidations, and performance metrics.
        </p>

        {/* Address type indicators */}
        <div style={{ display: 'flex', gap: 20, marginBottom: 20, flexWrap: 'wrap' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
            <div style={{ width: 8, height: 8, borderRadius: '50%', background: '#34D399' }} />
            <span style={{ fontSize: 13, fontWeight: 600, color: '#fff' }}>EVM Addresses</span>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
            <div style={{ width: 8, height: 8, borderRadius: '50%', background: '#A78BFA' }} />
            <span style={{ fontSize: 13, fontWeight: 600, color: '#fff' }}>Solana Addresses</span>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
            <div style={{ width: 8, height: 8, borderRadius: '50%', background: '#FBBF24' }} />
            <span style={{ fontSize: 13, fontWeight: 600, color: '#fff' }}>Account IDs</span>
          </div>
        </div>

        <SearchInput />
      </div>

      {/* Info cards */}
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(240px, 1fr))', gap: 16 }}>
        {[
          {
            icon: (
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
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
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
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
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
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
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
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
            style={{
              padding: '16px 18px',
              borderRadius: 12,
              border: `1px solid ${card.border}`,
              background: card.bg
            }}
          >
            <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 8 }}>
              <span style={{ color: card.color }}>{card.icon}</span>
              <span style={{ fontSize: 13, fontWeight: 600, color: '#fff' }}>{card.title}</span>
            </div>
            <p style={{ margin: 0, fontSize: 12, color: 'rgba(255,255,255,0.45)', lineHeight: 1.6 }}>
              {card.desc}
            </p>
          </div>
        ))}
      </div>
    </div>
  );
};
