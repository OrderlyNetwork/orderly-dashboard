import { Link } from '@remix-run/react';
import { FC } from 'react';

import { BrokerSelection, SearchInput } from '.';

export const NavBar: FC = () => {
  return (
    <nav className="w-full flex flex-self-stretch flex-items-center gap-sm p-4">
      <h2 className="flex-auto m0">
        <Link to="/" className="color-unset">
          Orderly Dashboard
        </Link>
      </h2>
      <BrokerSelection />
      <SearchInput />
    </nav>
  );
};
