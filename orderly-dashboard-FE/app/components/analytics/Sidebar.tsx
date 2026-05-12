import { Link } from '@remix-run/react';
import { FC, useEffect, useState } from 'react';

import orderlyLogo from '~/assets/orderly.svg';

export type NavId = 'dashboards' | 'leaderboard' | 'explorer';

const NAV_ITEMS: {
  id: NavId;
  path: string;
  label: string;
  color: string;
  bgAlpha: string;
  icon: JSX.Element;
}[] = [
  {
    id: 'dashboards',
    path: '/',
    label: 'Dashboards',
    color: '#9C75FF',
    bgAlpha: 'rgba(156,117,255,0.15)',
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
        <rect x="3" y="3" width="7" height="7" rx="1" />
        <rect x="14" y="3" width="7" height="7" rx="1" />
        <rect x="3" y="14" width="7" height="7" rx="1" />
        <rect x="14" y="14" width="7" height="7" rx="1" />
      </svg>
    )
  },
  {
    id: 'leaderboard',
    path: '/leaderboard',
    label: 'Leaderboard',
    color: '#FB923C',
    bgAlpha: 'rgba(251,146,60,0.15)',
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
        <line x1="18" y1="20" x2="18" y2="10" />
        <line x1="12" y1="20" x2="12" y2="4" />
        <line x1="6" y1="20" x2="6" y2="14" />
      </svg>
    )
  },
  {
    id: 'explorer',
    path: '/explorer',
    label: 'Explorer',
    color: '#34D399',
    bgAlpha: 'rgba(52,211,153,0.15)',
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
        <circle cx="11" cy="11" r="8" />
        <line x1="21" y1="21" x2="16.65" y2="16.65" />
      </svg>
    )
  }
];

type SidebarProps = {
  activeNav: NavId;
};

function renderNavItems(activeNav: NavId, onNavigate?: () => void) {
  return NAV_ITEMS.map((item) => {
    const isActive = activeNav === item.id;
    return (
      <Link
        key={item.id}
        to={item.path}
        onClick={onNavigate}
        className="flex items-center gap-[10px] w-full py-2 px-[10px] rounded-[10px] text-[13px] mb-[2px] no-underline transition-all duration-150 cursor-pointer text-left"
        style={{
          background: isActive
            ? `linear-gradient(90deg, ${item.bgAlpha} 0%, rgba(0,0,0,0) 100%)`
            : 'transparent',
          color: isActive ? '#fff' : 'rgba(255,255,255,0.45)',
          fontWeight: isActive ? 600 : 400,
          borderLeft: isActive ? `2px solid ${item.color}` : '2px solid transparent'
        }}
        onMouseEnter={(e) => {
          if (!isActive) {
            (e.currentTarget as HTMLElement).style.background = 'rgba(255,255,255,0.04)';
            (e.currentTarget as HTMLElement).style.color = 'rgba(255,255,255,0.75)';
          }
        }}
        onMouseLeave={(e) => {
          if (!isActive) {
            (e.currentTarget as HTMLElement).style.background = 'transparent';
            (e.currentTarget as HTMLElement).style.color = 'rgba(255,255,255,0.45)';
          }
        }}
      >
        <span
          className="shrink-0 w-7 h-7 rounded-lg flex items-center justify-center transition-all duration-150"
          style={{
            background: isActive ? item.bgAlpha : 'rgba(255,255,255,0.05)',
            color: isActive ? item.color : 'rgba(255,255,255,0.35)',
            border: isActive ? `1px solid ${item.color}40` : '1px solid rgba(255,255,255,0.06)'
          }}
        >
          {item.icon}
        </span>
        <span className="whitespace-nowrap overflow-hidden text-ellipsis">{item.label}</span>
        {isActive && (
          <span
            className="ml-auto w-1.5 h-1.5 rounded-full shrink-0"
            style={{ background: item.color, boxShadow: `0 0 6px ${item.color}` }}
          />
        )}
      </Link>
    );
  });
}

function SidebarContent({ activeNav, onNavigate }: SidebarProps & { onNavigate?: () => void }) {
  return (
    <>
      <div className="py-5 px-5 pb-4 border-b border-[rgba(156,117,255,0.1)]">
        <Link to="/" className="no-underline inline-block">
          <img src={orderlyLogo} alt="Orderly" className="h-[30px] w-auto" />
          <div className="text-[24px] mt--2 text-[#9C75FF] font-bold">Dashboard</div>
        </Link>
      </div>

      <nav className="flex-1 py-3 px-[10px] overflow-y-auto">
        <div className="text-[10px] font-semibold text-[rgba(255,255,255,0.25)] tracking-widest uppercase px-[10px] pb-2">
          Navigation
        </div>
        {renderNavItems(activeNav, onNavigate)}
        <div className="my-3 mx-[10px]" style={{ borderTop: '2px solid rgba(255,255,255,0.2)' }} />
        <a
          href="https://orderly.network"
          target="_blank"
          rel="noopener noreferrer"
          className="flex items-center gap-[10px] w-full py-2 px-[10px] rounded-[10px] text-[13px] no-underline transition-all duration-150 cursor-pointer text-left"
          style={{ color: 'rgba(255,255,255,0.45)', fontWeight: 400 }}
          onMouseEnter={(e) => {
            (e.currentTarget as HTMLElement).style.background = 'rgba(255,255,255,0.04)';
            (e.currentTarget as HTMLElement).style.color = 'rgba(255,255,255,0.75)';
          }}
          onMouseLeave={(e) => {
            (e.currentTarget as HTMLElement).style.background = 'transparent';
            (e.currentTarget as HTMLElement).style.color = 'rgba(255,255,255,0.45)';
          }}
        >
          <span className="whitespace-nowrap overflow-hidden text-ellipsis">Orderly Website</span>
          <svg
            width="12"
            height="12"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
            className="shrink-0"
            style={{ opacity: 0.5 }}
          >
            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
            <polyline points="15 3 21 3 21 9" />
            <line x1="10" y1="14" x2="21" y2="3" />
          </svg>
        </a>
      </nav>
    </>
  );
}

export const Sidebar: FC<SidebarProps> = (props) => {
  return (
    <div
      className="sidebar-desktop fixed left-0 top-0 bottom-0 w-[224px] flex flex-col z-[110] backdrop-blur-[20px]"
      style={{ background: 'rgba(10,0,16,0.97)', borderRight: '1px solid rgba(156,117,255,0.15)' }}
    >
      <SidebarContent {...props} />
    </div>
  );
};

type MobileSidebarDrawerProps = SidebarProps & {
  open: boolean;
  onClose: () => void;
};

export const MobileSidebarDrawer: FC<MobileSidebarDrawerProps> = ({
  open,
  onClose,
  ...sidebarProps
}) => {
  const [mounted, setMounted] = useState(false);
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    if (open) {
      setMounted(true);
      requestAnimationFrame(() => {
        requestAnimationFrame(() => setVisible(true));
      });
    } else if (mounted) {
      setVisible(false);
    }
  }, [open, mounted]);

  useEffect(() => {
    if (open) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
    return () => {
      document.body.style.overflow = '';
    };
  }, [open]);

  if (!mounted) return null;

  return (
    <>
      <div
        className="fixed inset-0 z-[200] transition-opacity duration-150"
        style={{ background: 'rgba(0,0,0,0.6)', opacity: visible ? 1 : 0 }}
        onClick={onClose}
        onKeyDown={onClose}
        onTransitionEnd={() => {
          if (!visible) setMounted(false);
        }}
        role="button"
        tabIndex={-1}
        aria-label="Close navigation menu"
      />
      <div
        className="fixed left-0 top-0 bottom-0 w-[280px] flex flex-col z-[210] backdrop-blur-[20px] transition-transform duration-150 ease-out"
        style={{
          background: 'rgba(10,0,16,0.98)',
          borderRight: '1px solid rgba(156,117,255,0.15)',
          transform: visible ? 'translateX(0)' : 'translateX(-100%)'
        }}
      >
        <div className="flex justify-end pt-3 px-4">
          <button
            onClick={onClose}
            className="bg-[rgba(255,255,255,0.08)] border-none rounded-lg w-9 h-9 flex items-center justify-center cursor-pointer text-[rgba(255,255,255,0.6)]"
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
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        </div>
        <SidebarContent {...sidebarProps} onNavigate={onClose} />
      </div>
    </>
  );
};
