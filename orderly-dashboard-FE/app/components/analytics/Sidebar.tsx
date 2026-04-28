import { FC } from 'react';

export type NavId =
  | 'dashboards'
  | 'queries'
  | 'api-catalog'
  | 'starred'
  | 'leaderboard'
  | 'explorer';

export type Role = 'trader' | 'builder' | 'analyst';

const NAV_ITEMS: {
  id: NavId;
  label: string;
  color: string;
  bgAlpha: string;
  icon: JSX.Element;
}[] = [
  {
    id: 'dashboards',
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
    id: 'queries',
    label: 'Queries',
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
        <polyline points="4 17 10 11 4 5" />
        <line x1="12" y1="19" x2="20" y2="19" />
      </svg>
    )
  },
  {
    id: 'api-catalog',
    label: 'API Catalog',
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
        <polyline points="16 18 22 12 16 6" />
        <polyline points="8 6 2 12 8 18" />
      </svg>
    )
  },
  {
    id: 'starred',
    label: 'Starred',
    color: '#FBBF24',
    bgAlpha: 'rgba(251,191,36,0.15)',
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
        <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
      </svg>
    )
  },
  {
    id: 'leaderboard',
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
  { id: 'trader', label: 'Trader' },
  { id: 'builder', label: 'Builder' },
  { id: 'analyst', label: 'Analyst' }
];

type SidebarProps = {
  activeNav: NavId;
  onNavChange: (id: NavId) => void;
  role: Role;
  onRoleChange: (role: Role) => void;
};

// Role switcher is only relevant on the Dashboards tab
const ROLE_NAV_IDS: NavId[] = ['dashboards'];

export const Sidebar: FC<SidebarProps> = ({ activeNav, onNavChange, role, onRoleChange }) => {
  return (
    <div
      style={{
        position: 'fixed',
        left: 0,
        top: 0,
        bottom: 0,
        width: 224,
        background: 'rgba(10,0,16,0.97)',
        borderRight: '1px solid rgba(156,117,255,0.15)',
        display: 'flex',
        flexDirection: 'column',
        zIndex: 110,
        backdropFilter: 'blur(20px)'
      }}
    >
      {/* Logo */}
      <div style={{ padding: '20px 20px 16px', borderBottom: '1px solid rgba(156,117,255,0.1)' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
          <div
            style={{
              width: 32,
              height: 32,
              borderRadius: 10,
              background: 'linear-gradient(135deg, #9C75FF 0%, #6b3fcb 100%)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              flexShrink: 0
            }}
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="white">
              <path d="M12 2L2 7l10 5 10-5-10-5z" />
              <path d="M2 17l10 5 10-5" opacity=".6" />
              <path d="M2 12l10 5 10-5" opacity=".8" />
            </svg>
          </div>
          <div>
            <div style={{ fontSize: 13, fontWeight: 700, color: '#fff', lineHeight: 1 }}>
              Analytics
            </div>
            <div style={{ fontSize: 11, color: 'rgba(156,117,255,0.7)', marginTop: 2 }}>
              Orderly Network
            </div>
          </div>
        </div>
      </div>

      {/* Nav items */}
      <nav style={{ flex: 1, padding: '12px 10px', overflowY: 'auto' }}>
        <div
          style={{
            fontSize: 10,
            fontWeight: 600,
            color: 'rgba(255,255,255,0.25)',
            letterSpacing: '0.1em',
            textTransform: 'uppercase',
            padding: '4px 10px 8px'
          }}
        >
          Navigation
        </div>
        {NAV_ITEMS.map((item) => {
          const isActive = activeNav === item.id;
          return (
            <button
              key={item.id}
              onClick={() => onNavChange(item.id)}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 10,
                width: '100%',
                padding: '8px 10px',
                borderRadius: 10,
                border: 'none',
                background: isActive
                  ? `linear-gradient(90deg, ${item.bgAlpha} 0%, rgba(0,0,0,0) 100%)`
                  : 'transparent',
                color: isActive ? '#fff' : 'rgba(255,255,255,0.45)',
                fontSize: 13,
                fontWeight: isActive ? 600 : 400,
                cursor: 'pointer',
                textAlign: 'left',
                transition: 'all 0.15s',
                borderLeft: isActive ? `2px solid ${item.color}` : '2px solid transparent',
                marginBottom: 2
              }}
              onMouseEnter={(e) => {
                if (!isActive) {
                  (e.currentTarget as HTMLButtonElement).style.background =
                    'rgba(255,255,255,0.04)';
                  (e.currentTarget as HTMLButtonElement).style.color = 'rgba(255,255,255,0.75)';
                }
              }}
              onMouseLeave={(e) => {
                if (!isActive) {
                  (e.currentTarget as HTMLButtonElement).style.background = 'transparent';
                  (e.currentTarget as HTMLButtonElement).style.color = 'rgba(255,255,255,0.45)';
                }
              }}
            >
              {/* Color-coded icon badge */}
              <span
                style={{
                  flexShrink: 0,
                  width: 28,
                  height: 28,
                  borderRadius: 8,
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  background: isActive ? item.bgAlpha : 'rgba(255,255,255,0.05)',
                  color: isActive ? item.color : 'rgba(255,255,255,0.35)',
                  border: isActive
                    ? `1px solid ${item.color}40`
                    : '1px solid rgba(255,255,255,0.06)',
                  transition: 'all 0.15s'
                }}
              >
                {item.icon}
              </span>
              <span style={{ whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                {item.label}
              </span>
              {/* Active glow dot */}
              {isActive && (
                <span
                  style={{
                    marginLeft: 'auto',
                    width: 6,
                    height: 6,
                    borderRadius: '50%',
                    background: item.color,
                    flexShrink: 0,
                    boxShadow: `0 0 6px ${item.color}`
                  }}
                />
              )}
            </button>
          );
        })}
      </nav>

      {/* Role switcher — only shown on Dashboards tab */}
      {ROLE_NAV_IDS.includes(activeNav) && (
        <div style={{ padding: '12px 10px 16px', borderTop: '1px solid rgba(156,117,255,0.1)' }}>
          <div
            style={{
              fontSize: 10,
              fontWeight: 600,
              color: 'rgba(255,255,255,0.25)',
              letterSpacing: '0.1em',
              textTransform: 'uppercase',
              padding: '0 10px 8px'
            }}
          >
            Role
          </div>
          <div
            style={{
              background: 'rgba(156,117,255,0.06)',
              border: '1px solid rgba(156,117,255,0.15)',
              borderRadius: 10,
              padding: 4,
              display: 'flex',
              flexDirection: 'column',
              gap: 2
            }}
          >
            {ROLES.map((r) => {
              const isActive = role === r.id;
              return (
                <button
                  key={r.id}
                  onClick={() => onRoleChange(r.id)}
                  style={{
                    width: '100%',
                    padding: '7px 12px',
                    borderRadius: 8,
                    border: 'none',
                    background: isActive ? '#9C75FF' : 'transparent',
                    color: isActive ? '#fff' : 'rgba(255,255,255,0.45)',
                    fontSize: 12,
                    fontWeight: isActive ? 600 : 400,
                    cursor: 'pointer',
                    textAlign: 'left',
                    transition: 'all 0.15s'
                  }}
                >
                  {r.label}
                </button>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
};
