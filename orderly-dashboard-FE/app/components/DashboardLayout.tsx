import { Outlet, useLocation } from '@remix-run/react';
import { createContext, useCallback, useContext, useState, type FC } from 'react';

import {
  MobileSidebarDrawer,
  Sidebar,
  type NavId,
  type Role
} from '~/components/analytics/Sidebar';
import { Topbar } from '~/components/analytics/Topbar';
import { useIsMobile } from '~/hooks/useMediaQuery';

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
  const isMobile = useIsMobile(768);

  const [role, setRole] = useState<Role>('analyst');
  const [drawerOpen, setDrawerOpen] = useState(false);

  const toggleDrawer = useCallback(() => setDrawerOpen((v) => !v), []);
  const closeDrawer = useCallback(() => setDrawerOpen(false), []);

  return (
    <DashboardLayoutContext.Provider value={{ role, setRole }}>
      <div
        className="fixed inset-0 flex min-w-[375px]"
        style={{
          background: '#0A0010',
          fontFamily:
            '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif'
        }}
      >
        {!isMobile && <Sidebar activeNav={activeNav} role={role} onRoleChange={setRole} />}

        {isMobile && (
          <MobileSidebarDrawer
            open={drawerOpen}
            onClose={closeDrawer}
            activeNav={activeNav}
            role={role}
            onRoleChange={setRole}
          />
        )}

        <div
          className={`flex flex-col flex-1 min-w-0 overflow-hidden ${isMobile ? 'ml-0' : 'ml-[224px]'}`}
        >
          <Topbar
            activeNav={activeNav}
            isMobile={isMobile}
            onMenuToggle={isMobile ? toggleDrawer : undefined}
          />

          <div
            className={`flex-1 overflow-y-auto min-h-0 ${isMobile ? 'mt-14 p-4' : 'mt-16 py-7 px-8'}`}
          >
            <Outlet />
          </div>
        </div>
      </div>
    </DashboardLayoutContext.Provider>
  );
};
