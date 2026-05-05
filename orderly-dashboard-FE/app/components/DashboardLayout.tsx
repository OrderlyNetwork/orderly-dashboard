import { Outlet, useLocation } from '@remix-run/react';
import { createContext, useContext, useState, type FC } from 'react';

import { Sidebar, type NavId, type Role } from '~/components/analytics/Sidebar';
import { Topbar } from '~/components/analytics/Topbar';

const PATH_TO_NAV: Record<string, NavId> = {
  '/': 'dashboards',
  '/leaderboard': 'leaderboard',
  '/explorer': 'explorer'
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
  const activeNav = getActiveNav(location.pathname);

  const [role, setRole] = useState<Role>('trader');

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
          <Topbar activeNav={activeNav} />

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
    </DashboardLayoutContext.Provider>
  );
};
