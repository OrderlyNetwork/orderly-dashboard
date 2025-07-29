import { Link } from '@remix-run/react';
import { FC } from 'react';

import { BrokerSelection, SearchInput } from '.';

export const NavBar: FC = () => {
  return (
    <nav className="w-full flex flex-col sm:flex-row flex-self-stretch flex-items-center gap-3 sm:gap-sm p-4">
      <h2 className="flex-auto m0 text-center sm:text-left">
        <Link to="/" className="color-unset">
          Orderly Dashboard
        </Link>
      </h2>
      <div className="flex flex-col sm:flex-row items-center gap-3 sm:gap-sm w-full sm:w-auto">
        <BrokerSelection />
        <SearchInput />
      </div>
    </nav>
  );
};
