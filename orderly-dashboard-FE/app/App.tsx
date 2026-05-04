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
  return <DashboardLayout />;
};
