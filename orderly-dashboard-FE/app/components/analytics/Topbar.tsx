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
  isMobile?: boolean;
  onMenuToggle?: () => void;
};

export const Topbar: FC<TopbarProps> = ({ activeNav, isMobile, onMenuToggle }) => {
  return (
    <div
      className={`fixed top-0 right-0 h-14 flex items-center justify-between z-[105] backdrop-blur-[20px] gap-3 ${isMobile ? 'left-0 px-4' : 'left-[224px] px-7'}`}
      style={{ background: 'rgba(10,0,16,0.97)', borderBottom: '1px solid rgba(156,117,255,0.12)' }}
    >
      <div className="flex items-center gap-3 min-w-0">
        {isMobile && onMenuToggle && (
          <button
            onClick={onMenuToggle}
            className="bg-[rgba(255,255,255,0.06)] border border-[rgba(156,117,255,0.12)] rounded-lg w-9 h-9 flex items-center justify-center cursor-pointer text-[rgba(255,255,255,0.7)] shrink-0 p-0"
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
              <line x1="3" y1="6" x2="21" y2="6" />
              <line x1="3" y1="12" x2="21" y2="12" />
              <line x1="3" y1="18" x2="21" y2="18" />
            </svg>
          </button>
        )}
        <div className="min-w-0">
          <div className="flex items-baseline gap-2 flex-wrap">
            <h1
              className={`font-bold text-white m-0 leading-tight tracking-tight ${isMobile ? 'text-base' : 'text-lg'}`}
            >
              {NAV_LABELS[activeNav] ?? 'Dashboard'}
            </h1>
            {!isMobile && (
              <span className="text-xs text-[rgba(255,255,255,0.4)] font-normal">
                {NAV_SUBTITLES[activeNav] ?? ''}
              </span>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
