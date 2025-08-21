import { Link } from '@remix-run/react';
import { FC } from 'react';

import { SearchInput } from '.';

export const NavBar: FC = () => {
  return (
    <nav className="glass-no-radius w-full flex flex-col sm:flex-row items-center justify-between gap-4 p-6 mb-8">
      <div className="flex items-center gap-4">
        <Link to="/" className="group">
          <h2 className="text-2xl font-bold bg-gradient-to-r from-white to-gray-300 bg-clip-text text-transparent group-hover:from-primary-light group-hover:to-primary transition-all duration-300">
            Orderly Dashboard
          </h2>
        </Link>
      </div>

      <div className="flex flex-col sm:flex-row items-center gap-4 w-full sm:w-auto">
        <SearchInput />
      </div>
    </nav>
  );
};
