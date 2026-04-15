import { Outlet } from '@remix-run/react';
import { FC, createContext, useContext, useState, useEffect } from 'react';
import { AnimatePresence } from 'motion/react';

import { MorphingHeader, MobileFixedNav, MobileNavDrawer, TabletNav } from '~/components/SiteHeader';
import { SiteFooter, TabletFooter, MobileFooter } from '~/components/SiteFooter';

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
  // Default to 'desktop' for SSR; corrected on client after hydration.
  const [layout, setLayout] = useState<Layout>('desktop');
  const [navOpen, setNavOpen] = useState(false);

  useEffect(() => {
    const update = () => setLayout(getLayout(window.innerWidth));
    update();
    window.addEventListener('resize', update);
    return () => window.removeEventListener('resize', update);
  }, []);

  // Close drawer when switching to desktop breakpoint.
  useEffect(() => {
    if (layout === 'desktop') setNavOpen(false);
  }, [layout]);

  return (
    <>
      {/* ── Mobile fixed top bar (< 600 px) ── */}
      {layout === 'mobile' && (
        <MobileFixedNav onMenuClick={() => setNavOpen(true)} />
      )}

      {/* ── Tablet sticky top bar (600 – 1023 px) ── */}
      {layout === 'tablet' && (
        <TabletNav onMenuClick={() => setNavOpen(true)} />
      )}

      {/* ── Desktop floating pill (≥ 1024 px) ── */}
      {layout === 'desktop' && (
        <div style={{ position: 'fixed', top: 0, left: 0, right: 0, zIndex: 50, display: 'flex', justifyContent: 'center', pointerEvents: 'none' }}>
          <div style={{ pointerEvents: 'auto', width: '100%', display: 'flex', justifyContent: 'center', paddingLeft: 16, paddingRight: 16 }}>
            <MorphingHeader />
          </div>
        </div>
      )}

      {/* ── Nav drawer — shared by mobile and tablet ── */}
      <AnimatePresence>
        {navOpen && (
          <MobileNavDrawer
            key="nav-drawer"
            onClose={() => setNavOpen(false)}
            deviceLayout={layout === 'tablet' ? 'tablet' : 'mobile'}
          />
        )}
      </AnimatePresence>

      {/* ── Page content ── */}
      <div
        className="w-full max-w-screen-2xl mx-auto px-2 sm:px-4 lg:px-8 xl:px-32"
        style={{ paddingTop: layout === 'desktop' ? 120 : layout === 'tablet' ? 0 : 72 }}
      >
        <Outlet />
      </div>

      {layout === 'mobile'  && <MobileFooter />}
      {layout === 'tablet'  && <TabletFooter />}
      {layout === 'desktop' && (
        <div className="w-full max-w-screen-2xl mx-auto px-2 sm:px-4 lg:px-8 xl:px-32">
          <div className="mx-auto w-full" style={{ maxWidth: 1200, paddingLeft: 24, paddingRight: 24 }}>
            <SiteFooter />
          </div>
        </div>
      )}
    </>
  );
};
