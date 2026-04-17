import { Outlet } from '@remix-run/react';
import { AnimatePresence } from 'motion/react';
import { FC, createContext, useContext, useState, useEffect } from 'react';

import { SiteFooter, TabletFooter, MobileFooter } from '~/components/SiteFooter';
import {
  MorphingHeader,
  MobileFixedNav,
  MobileNavDrawer,
  TabletNav
} from '~/components/SiteHeader';

export type AppContextType = {
  queryServiceUrl: string;
  evmApiUrl: string;
};

export const AppContext = createContext<AppContextType>({
  queryServiceUrl: '',
  evmApiUrl: ''
});

export const useAppState = () => {
  return useContext<AppContextType>(AppContext);
};

type Layout = 'mobile' | 'tablet' | 'desktop';

function getLayout(w: number): Layout {
  if (w < 600) return 'mobile';
  if (w < 1024) return 'tablet';
  return 'desktop';
}

export const App: FC = () => {
  const [layout, setLayout] = useState<Layout | null>(null);
  const [navOpen, setNavOpen] = useState(false);

  useEffect(() => {
    const update = () => setLayout(getLayout(window.innerWidth));
    update();
    window.addEventListener('resize', update);
    return () => window.removeEventListener('resize', update);
  }, []);

  useEffect(() => {
    if (layout === 'desktop') setNavOpen(false);
  }, [layout]);

  const resolved = layout ?? 'desktop';

  return (
    <>
      {resolved === 'mobile' && <MobileFixedNav onMenuClick={() => setNavOpen(true)} />}
      {resolved === 'tablet' && <TabletNav onMenuClick={() => setNavOpen(true)} />}
      {resolved === 'desktop' && (
        <div
          style={{
            position: 'fixed',
            top: 0,
            left: 0,
            right: 0,
            zIndex: 50,
            display: 'flex',
            justifyContent: 'center',
            pointerEvents: 'none'
          }}
        >
          <div
            style={{
              pointerEvents: 'auto',
              width: '100%',
              display: 'flex',
              justifyContent: 'center',
              paddingLeft: 16,
              paddingRight: 16
            }}
          >
            <MorphingHeader />
          </div>
        </div>
      )}

      <AnimatePresence>
        {navOpen && (
          <MobileNavDrawer
            key="nav-drawer"
            onClose={() => setNavOpen(false)}
            deviceLayout={resolved === 'tablet' ? 'tablet' : 'mobile'}
          />
        )}
      </AnimatePresence>

      <div
        className="w-full max-w-screen-2xl mx-auto px-2 sm:px-4 lg:px-8 xl:px-32"
        style={{ paddingTop: resolved === 'desktop' ? 120 : resolved === 'tablet' ? 0 : 72 }}
      >
        <Outlet />
      </div>

      {resolved === 'mobile' && <MobileFooter />}
      {resolved === 'tablet' && <TabletFooter />}
      {resolved === 'desktop' && (
        <div className="w-full max-w-screen-2xl mx-auto px-2 sm:px-4 lg:px-8 xl:px-32">
          <div
            className="mx-auto w-full"
            style={{ maxWidth: 1200, paddingLeft: 24, paddingRight: 24 }}
          >
            <SiteFooter />
          </div>
        </div>
      )}
    </>
  );
};
