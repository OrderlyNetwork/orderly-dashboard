import { Outlet, useLocation, useNavigation } from '@remix-run/react';
import { useCallback, useState, type FC } from 'react';

import { MobileSidebarDrawer, Sidebar, type NavId } from '~/components/analytics/Sidebar';
import { Topbar } from '~/components/analytics/Topbar';
import { useIsMobile } from '~/hooks/useMediaQuery';

const PATH_TO_NAV: Record<string, NavId> = {
  '/': 'dashboards',
  '/leaderboard': 'leaderboard',
  '/explorer': 'explorer'
};

function getActiveNav(pathname: string): NavId {
  if (PATH_TO_NAV[pathname]) return PATH_TO_NAV[pathname];
  if (pathname.startsWith('/search')) return 'explorer';
  if (pathname.startsWith('/address')) return 'explorer';
  return 'dashboards';
}

export const DashboardLayout: FC = () => {
  const location = useLocation();
  const navigation = useNavigation();
  const isMobile = useIsMobile(768);

  const pendingPathname =
    navigation.state === 'loading' ? navigation.location?.pathname : undefined;
  const activeNav = getActiveNav(pendingPathname ?? location.pathname);
  const isNavigating = navigation.state === 'loading';

  const [drawerOpen, setDrawerOpen] = useState(false);

  const toggleDrawer = useCallback(() => setDrawerOpen((v) => !v), []);
  const closeDrawer = useCallback(() => setDrawerOpen(false), []);

  return (
    <div
      className="fixed inset-0 flex min-w-[375px]"
      style={{
        background: '#0A0010',
        fontFamily:
          '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif'
      }}
    >
      {!isMobile && <Sidebar activeNav={activeNav} />}

      {isMobile && (
        <MobileSidebarDrawer open={drawerOpen} onClose={closeDrawer} activeNav={activeNav} />
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
          className={`flex-1 overflow-y-auto min-h-0 relative ${isMobile ? 'mt-14 py-4 px-1' : 'mt-16 py-7 px-8'}`}
        >
          {isNavigating && (
            <div
              style={{
                position: 'absolute',
                inset: 0,
                zIndex: 50,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                background: 'rgba(10,0,16,0.55)',
                backdropFilter: 'blur(4px)'
              }}
            >
              <div
                style={{
                  width: 36,
                  height: 36,
                  borderRadius: '50%',
                  border: '3px solid rgba(156,117,255,0.2)',
                  borderTopColor: '#9C75FF',
                  animation: 'spin 0.7s linear infinite'
                }}
              />
              <style>{`@keyframes spin{to{transform:rotate(360deg)}}`}</style>
            </div>
          )}
          <Outlet />
        </div>
      </div>
    </div>
  );
};
