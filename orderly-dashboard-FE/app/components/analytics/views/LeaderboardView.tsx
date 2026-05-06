import { FC, useState } from 'react';

import { Leaderboard } from '~/components/Leaderboard';
import { Positions } from '~/components/Positions';

type Tab = 'trading' | 'positions';

const TAB_CONFIG: {
  id: Tab;
  label: string;
  activeColor: string;
  activeBg: string;
  activeBorder: string;
  icon: JSX.Element;
}[] = [
  {
    id: 'trading',
    label: 'Trading Leaderboard',
    activeColor: '#FB923C',
    activeBg: 'rgba(251,146,60,0.15)',
    activeBorder: 'rgba(251,146,60,0.4)',
    icon: (
      <svg
        width="13"
        height="13"
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
    )
  },
  {
    id: 'positions',
    label: 'Positions Leaderboard',
    activeColor: '#60A5FA',
    activeBg: 'rgba(96,165,250,0.15)',
    activeBorder: 'rgba(96,165,250,0.4)',
    icon: (
      <svg
        width="13"
        height="13"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <rect x="2" y="7" width="20" height="14" rx="2" />
        <path d="M16 7V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2" />
      </svg>
    )
  }
];

export const LeaderboardView: FC = () => {
  const [activeTab, setActiveTab] = useState<Tab>('trading');

  return (
    <div className="w-full">
      <div className="mb-6">
        <div className="flex items-center gap-3 mb-1.5">
          <div
            className="w-9 h-9 rounded-[10px] flex items-center justify-center text-[#FB923C]"
            style={{
              background: 'rgba(251,146,60,0.15)',
              border: '1px solid rgba(251,146,60,0.3)'
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
              <line x1="18" y1="20" x2="18" y2="10" />
              <line x1="12" y1="20" x2="12" y2="4" />
              <line x1="6" y1="20" x2="6" y2="14" />
            </svg>
          </div>
          <div>
            <h1 className="m-0 text-[22px] font-bold text-white">Leaderboard</h1>
            <p className="m-0 text-[13px] text-[rgba(255,255,255,0.45)]">
              Trading performance and open positions ranked across the Orderly network
            </p>
          </div>
        </div>
      </div>

      <div className="flex gap-2 mb-5 flex-wrap">
        {TAB_CONFIG.map((tab) => {
          const isActive = activeTab === tab.id;
          return (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className="flex items-center gap-[7px] py-2 px-5 rounded-[10px] text-[13px] font-semibold cursor-pointer transition-all duration-150"
              style={{
                border: `1px solid ${isActive ? tab.activeBorder : 'rgba(255,255,255,0.1)'}`,
                background: isActive ? tab.activeBg : 'transparent',
                color: isActive ? tab.activeColor : 'rgba(255,255,255,0.45)'
              }}
            >
              {tab.icon}
              {tab.label}
            </button>
          );
        })}
      </div>

      {activeTab === 'trading' ? <Leaderboard /> : <Positions />}
    </div>
  );
};
