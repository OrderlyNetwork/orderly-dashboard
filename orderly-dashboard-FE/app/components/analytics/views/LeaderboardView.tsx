import { FC, useState } from 'react';

import { Leaderboard } from '~/components/Leaderboard';
import { Positions } from '~/components/Positions';

type Tab = 'trading' | 'positions';

export const LeaderboardView: FC = () => {
  const [activeTab, setActiveTab] = useState<Tab>('trading');

  return (
    <div style={{ width: '100%' }}>
      {/* Header */}
      <div style={{ marginBottom: 24 }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginBottom: 6 }}>
          <div
            style={{
              width: 36,
              height: 36,
              borderRadius: 10,
              background: 'rgba(251,146,60,0.15)',
              border: '1px solid rgba(251,146,60,0.3)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#FB923C'
            }}
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <line x1="18" y1="20" x2="18" y2="10" />
              <line x1="12" y1="20" x2="12" y2="4" />
              <line x1="6" y1="20" x2="6" y2="14" />
            </svg>
          </div>
          <div>
            <h1 style={{ margin: 0, fontSize: 22, fontWeight: 700, color: '#fff' }}>Leaderboard</h1>
            <p style={{ margin: 0, fontSize: 13, color: 'rgba(255,255,255,0.45)' }}>
              Trading performance and open positions ranked across the Orderly network
            </p>
          </div>
        </div>
      </div>

      {/* Tab switcher */}
      <div style={{ display: 'flex', gap: 8, marginBottom: 20 }}>
        <button
          onClick={() => setActiveTab('trading')}
          style={{
            padding: '8px 20px',
            borderRadius: 10,
            border: '1px solid',
            borderColor: activeTab === 'trading' ? 'rgba(251,146,60,0.4)' : 'rgba(255,255,255,0.1)',
            background: activeTab === 'trading' ? 'rgba(251,146,60,0.15)' : 'transparent',
            color: activeTab === 'trading' ? '#FB923C' : 'rgba(255,255,255,0.45)',
            fontSize: 13,
            fontWeight: 600,
            cursor: 'pointer',
            transition: 'all 0.15s',
            display: 'flex',
            alignItems: 'center',
            gap: 7
          }}
        >
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <polyline points="22 7 13.5 15.5 8.5 10.5 2 17" />
            <polyline points="16 7 22 7 22 13" />
          </svg>
          Trading Leaderboard
        </button>
        <button
          onClick={() => setActiveTab('positions')}
          style={{
            padding: '8px 20px',
            borderRadius: 10,
            border: '1px solid',
            borderColor: activeTab === 'positions' ? 'rgba(96,165,250,0.4)' : 'rgba(255,255,255,0.1)',
            background: activeTab === 'positions' ? 'rgba(96,165,250,0.15)' : 'transparent',
            color: activeTab === 'positions' ? '#60A5FA' : 'rgba(255,255,255,0.45)',
            fontSize: 13,
            fontWeight: 600,
            cursor: 'pointer',
            transition: 'all 0.15s',
            display: 'flex',
            alignItems: 'center',
            gap: 7
          }}
        >
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <rect x="2" y="7" width="20" height="14" rx="2" />
            <path d="M16 7V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2" />
          </svg>
          Positions Leaderboard
        </button>
      </div>

      {/* Content */}
      {activeTab === 'trading' ? <Leaderboard /> : <Positions />}
    </div>
  );
};
