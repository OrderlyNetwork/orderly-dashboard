import { Outlet, useLocation, useNavigate } from '@remix-run/react';
import { createContext, useContext, useEffect, useState, type FC } from 'react';

import { SearchModal } from '~/components/analytics/SearchModal';
import { Sidebar, type NavId, type Role } from '~/components/analytics/Sidebar';
import { Topbar } from '~/components/analytics/Topbar';

const PATH_TO_NAV: Record<string, NavId> = {
  '/': 'dashboards',
  '/leaderboard': 'leaderboard',
  '/explorer': 'explorer'
};

const NAV_TO_PATH: Record<NavId, string> = {
  dashboards: '/',
  leaderboard: '/leaderboard',
  explorer: '/explorer'
};

type DashboardLayoutContextType = {
  role: Role;
  setRole: (role: Role) => void;
};

const DashboardLayoutContext = createContext<DashboardLayoutContextType>({
  role: 'trader',
  setRole: () => {}
});

export const useDashboardLayout = () => useContext(DashboardLayoutContext);

function getActiveNav(pathname: string): NavId {
  if (PATH_TO_NAV[pathname]) return PATH_TO_NAV[pathname];
  if (pathname.startsWith('/search')) return 'explorer';
  if (pathname.startsWith('/address')) return 'explorer';
  return 'dashboards';
}

export const DashboardLayout: FC = () => {
  const location = useLocation();
  const navigate = useNavigate();

  const activeNav = getActiveNav(location.pathname);

  const [role, setRole] = useState<Role>('trader');
  const [searchOpen, setSearchOpen] = useState(false);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setSearchOpen((v) => !v);
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, []);

  const handleSearchNavigate = (id: string) => {
    const path = NAV_TO_PATH[id as NavId];
    if (path) {
      navigate(path);
    } else {
      navigate('/');
    }
  };

  return (
    <DashboardLayoutContext.Provider value={{ role, setRole }}>
      <div
        style={{
          position: 'fixed',
          inset: 0,
          background: '#0A0010',
          display: 'flex',
          fontFamily:
            '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif'
        }}
      >
        <Sidebar activeNav={activeNav} role={role} onRoleChange={setRole} />

        <div
          style={{
            marginLeft: 224,
            flex: 1,
            display: 'flex',
            flexDirection: 'column',
            minWidth: 0,
            overflow: 'hidden'
          }}
        >
          <Topbar activeNav={activeNav} onSearchOpen={() => setSearchOpen(true)} />

          <div
            style={{
              marginTop: 64,
              flex: 1,
              overflowY: 'auto',
              padding: '28px 32px',
              minHeight: 0
            }}
          >
            <Outlet />
          </div>
        </div>
      </div>

      <SearchModal
        open={searchOpen}
        onClose={() => setSearchOpen(false)}
        onNavigate={handleSearchNavigate}
      />
    </DashboardLayoutContext.Provider>
  );
};
