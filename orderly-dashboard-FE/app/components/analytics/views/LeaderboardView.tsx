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
    activeColor: '#9C75FF',
    activeBg: 'rgba(156,117,255,0.15)',
    activeBorder: 'rgba(156,117,255,0.4)',
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
      <div className="flex gap-2 mb-5 justify-center">
        {TAB_CONFIG.map((tab) => {
          const isActive = activeTab === tab.id;
          return (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className="flex items-center gap-[7px] py-2 px-5 rounded-full text-[13px] font-semibold cursor-pointer transition-all duration-150"
              style={{
                border: 'none',
                background: isActive ? '#6700CE' : '#221E30',
                color: isActive ? '#fff' : 'rgba(255,255,255,0.45)'
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
