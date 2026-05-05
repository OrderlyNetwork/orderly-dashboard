import { FC } from 'react';

import type { NavId } from '~/components/analytics/Sidebar';

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
};

export const Topbar: FC<TopbarProps> = ({ activeNav }) => {
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
          <span style={{ fontSize: 12, color: 'rgba(255,255,255,0.4)', fontWeight: 400 }}>
            {NAV_SUBTITLES[activeNav] ?? ''}
          </span>
        </div>
      </div>
    </div>
  );
};
