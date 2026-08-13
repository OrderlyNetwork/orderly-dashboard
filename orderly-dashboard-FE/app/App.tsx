import { Outlet, useLocation, useSearchParams } from '@remix-run/react';
import { FC, createContext, useContext } from 'react';

import { DashboardLayout } from '~/components/DashboardLayout';
import { NavBridge } from '~/components/NavBridge';
import { WebMcpRegistrar } from '~/components/WebMcpRegistrar';

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
  const location = useLocation();
  const [searchParams] = useSearchParams();
  const isEmbed = searchParams.get('embed') === 'true';

  if (isEmbed && location.pathname.startsWith('/widget')) {
    // No WebMCP on chromeless embeds: the "tools" policy defaults to ['self']
    // (cross-origin iframes reject registerTool), and a single embedded widget
    // should not expose the full tool set.
    return <Outlet />;
  }

  return (
    <>
      <WebMcpRegistrar />
      <NavBridge />
      <DashboardLayout />
    </>
  );
};
