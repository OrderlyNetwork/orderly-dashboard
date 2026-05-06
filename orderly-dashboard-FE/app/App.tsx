import { Outlet, useLocation, useSearchParams } from '@remix-run/react';
import { FC, createContext, useContext } from 'react';

import { DashboardLayout } from '~/components/DashboardLayout';

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
    return <Outlet />;
  }

  return <DashboardLayout />;
};
