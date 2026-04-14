import { Outlet } from '@remix-run/react';
import { FC, createContext, useContext } from 'react';

import { MorphingHeader } from '~/components/SiteHeader';
import { SiteFooter } from '~/components/SiteFooter';

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

export const App: FC = () => {
  return (
    <>
      <div style={{ position: 'fixed', top: 0, left: 0, right: 0, zIndex: 50, display: 'flex', justifyContent: 'center', pointerEvents: 'none' }}>
        <div style={{ pointerEvents: 'auto', width: '100%', display: 'flex', justifyContent: 'center', paddingLeft: 16, paddingRight: 16 }}>
          <MorphingHeader />
        </div>
      </div>
      <div className="w-full max-w-screen-2xl mx-auto px-2 sm:px-4 lg:px-8 xl:px-32" style={{ paddingTop: 120 }}>
        <Outlet />
      </div>
      <SiteFooter />
    </>
  );
};
