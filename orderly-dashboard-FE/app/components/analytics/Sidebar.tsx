import { Link } from '@remix-run/react';
import { FC, useEffect, useState } from 'react';

export type NavId = 'dashboards' | 'leaderboard' | 'explorer';

export type Role = 'trader' | 'builder' | 'analyst';

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

const ROLES: { id: Role; label: string }[] = [
  { id: 'analyst', label: 'Analyst' },
  { id: 'trader', label: 'Trader' },
  { id: 'builder', label: 'Builder' }
];

type SidebarProps = {
  activeNav: NavId;
  role: Role;
  onRoleChange: (role: Role) => void;
};

const ROLE_NAV_IDS: NavId[] = ['dashboards'];

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

function renderRoleSwitcher(role: Role, onRoleChange: (role: Role) => void) {
  return ROLES.map((r) => {
    const isActive = role === r.id;
    return (
      <button
        key={r.id}
        onClick={() => onRoleChange(r.id)}
        className="w-full py-[7px] px-3 rounded-lg border-none cursor-pointer text-left transition-all duration-150 text-xs"
        style={{
          background: isActive ? '#9C75FF' : 'transparent',
          color: isActive ? '#fff' : 'rgba(255,255,255,0.45)',
          fontWeight: isActive ? 600 : 400
        }}
      >
        {r.label}
      </button>
    );
  });
}

function SidebarContent({
  activeNav,
  role,
  onRoleChange,
  onNavigate
}: SidebarProps & { onNavigate?: () => void }) {
  return (
    <>
      <div className="py-5 px-5 pb-4 border-b border-[rgba(156,117,255,0.1)]">
        <div className="flex items-center gap-[10px]">
          <div
            className="w-8 h-8 rounded-[10px] flex items-center justify-center shrink-0"
            style={{ background: 'linear-gradient(135deg, #9C75FF 0%, #6b3fcb 100%)' }}
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="white">
              <path d="M12 2L2 7l10 5 10-5-10-5z" />
              <path d="M2 17l10 5 10-5" opacity=".6" />
              <path d="M2 12l10 5 10-5" opacity=".8" />
            </svg>
          </div>
          <div>
            <div className="text-[13px] font-bold text-white leading-none">Dashboard</div>
            <div className="text-[11px] mt-0.5 text-[rgba(156,117,255,0.7)]">Orderly Network</div>
          </div>
        </div>
      </div>

      <nav className="flex-1 py-3 px-[10px] overflow-y-auto">
        <div className="text-[10px] font-semibold text-[rgba(255,255,255,0.25)] tracking-widest uppercase px-[10px] pb-2">
          Navigation
        </div>
        {renderNavItems(activeNav, onNavigate)}
      </nav>

      {ROLE_NAV_IDS.includes(activeNav) && (
        <div className="py-3 px-[10px] pb-4 border-t border-[rgba(156,117,255,0.1)]">
          <div className="text-[10px] font-semibold text-[rgba(255,255,255,0.25)] tracking-widest uppercase px-[10px] pb-2">
            Role
          </div>
          <div className="bg-[rgba(156,117,255,0.06)] border border-[rgba(156,117,255,0.15)] rounded-[10px] p-1 flex flex-col gap-[2px]">
            {renderRoleSwitcher(role, onRoleChange)}
          </div>
        </div>
      )}
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
