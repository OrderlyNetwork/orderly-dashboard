import { FC } from 'react';

import type { NavId } from './Sidebar';

const NAV_LABELS: Record<NavId, string> = {
  dashboards: 'Dashboards',
  leaderboard: 'Leaderboard',
  explorer: 'Explorer'
};

const NAV_SUBTITLES: Record<NavId, string> = {
  dashboards: 'Overview of key metrics and performance indicators',
  leaderboard: 'Trading performance and open positions across the network',
  explorer: 'Search wallets and accounts for detailed trading history'
};

type TopbarProps = {
  activeNav: NavId;
  secondsAgo: number;
  onRefresh: () => void;
  onSearchOpen: () => void;
};

export const Topbar: FC<TopbarProps> = ({ activeNav, secondsAgo, onRefresh, onSearchOpen }) => {
  return (
    <div
      style={{
        position: 'fixed',
        top: 0,
        left: 224,
        right: 0,
        height: 64,
        background: 'rgba(10,0,16,0.97)',
        borderBottom: '1px solid rgba(156,117,255,0.12)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '0 28px',
        zIndex: 105,
        backdropFilter: 'blur(20px)',
        gap: 16
      }}
    >
      {/* Title + subtitle */}
      <div style={{ minWidth: 0 }}>
        <div style={{ display: 'flex', alignItems: 'baseline', gap: 10, flexWrap: 'wrap' }}>
          <h1
            style={{
              fontSize: 18,
              fontWeight: 700,
              color: '#fff',
              margin: 0,
              lineHeight: 1.2,
              letterSpacing: '-0.01em'
            }}
          >
            {NAV_LABELS[activeNav] ?? 'Dashboard'}
          </h1>
          <span
            style={{
              fontSize: 12,
              color: 'rgba(255,255,255,0.4)',
              fontWeight: 400
            }}
          >
            {NAV_SUBTITLES[activeNav] ?? ''}
          </span>
        </div>
      </div>

      {/* Right controls */}
      <div style={{ display: 'flex', alignItems: 'center', gap: 10, flexShrink: 0 }}>
        {/* Search trigger */}
        <button
          onClick={onSearchOpen}
          title="Search (⌘K)"
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: 8,
            background: 'rgba(156,117,255,0.08)',
            border: '1px solid rgba(156,117,255,0.2)',
            borderRadius: 8,
            color: 'rgba(255,255,255,0.5)',
            fontSize: 12,
            padding: '6px 12px',
            cursor: 'pointer',
            transition: 'all 0.15s'
          }}
          onMouseEnter={(e) => {
            (e.currentTarget as HTMLButtonElement).style.borderColor = 'rgba(156,117,255,0.5)';
            (e.currentTarget as HTMLButtonElement).style.color = 'rgba(255,255,255,0.8)';
          }}
          onMouseLeave={(e) => {
            (e.currentTarget as HTMLButtonElement).style.borderColor = 'rgba(156,117,255,0.2)';
            (e.currentTarget as HTMLButtonElement).style.color = 'rgba(255,255,255,0.5)';
          }}
        >
          <svg
            width="14"
            height="14"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
          >
            <circle cx="11" cy="11" r="8" />
            <line x1="21" y1="21" x2="16.65" y2="16.65" />
          </svg>
          <span>Search</span>
          <kbd
            style={{
              background: 'rgba(255,255,255,0.08)',
              border: '1px solid rgba(255,255,255,0.12)',
              borderRadius: 4,
              padding: '1px 5px',
              fontSize: 10,
              fontFamily: 'monospace'
            }}
          >
            ⌘K
          </kbd>
        </button>

        {/* Updated N seconds ago */}
        <div
          style={{
            fontSize: 12,
            color: 'rgba(255,255,255,0.35)',
            whiteSpace: 'nowrap'
          }}
        >
          Updated {secondsAgo}s ago
        </div>

        {/* Refresh button */}
        <button
          onClick={onRefresh}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: 6,
            background: 'rgba(156,117,255,0.1)',
            border: '1px solid rgba(156,117,255,0.25)',
            borderRadius: 8,
            color: '#9C75FF',
            fontSize: 12,
            fontWeight: 600,
            padding: '6px 14px',
            cursor: 'pointer',
            transition: 'all 0.15s'
          }}
          onMouseEnter={(e) => {
            (e.currentTarget as HTMLButtonElement).style.background = 'rgba(156,117,255,0.2)';
          }}
          onMouseLeave={(e) => {
            (e.currentTarget as HTMLButtonElement).style.background = 'rgba(156,117,255,0.1)';
          }}
        >
          <svg
            width="13"
            height="13"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2.5"
            strokeLinecap="round"
          >
            <polyline points="23 4 23 10 17 10" />
            <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
          </svg>
          Refresh
        </button>
      </div>
    </div>
  );
};
