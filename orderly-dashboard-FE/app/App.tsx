import { Outlet } from '@remix-run/react';
import { FC, createContext, useContext } from 'react';

import { NavBar } from '~/components';
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
      <NavBar />
      <div className="m-2 sm:m-3 mt-6 sm:mt-8 max-w-full p-2 sm:p-3 lg:p-6">
        <Outlet />
      </div>
    </>
  );
};
