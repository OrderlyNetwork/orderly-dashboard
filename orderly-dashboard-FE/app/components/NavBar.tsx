import { TabNav } from '@radix-ui/themes';
import { Link, useLocation } from '@remix-run/react';
import { FC, useState } from 'react';

import { BrokerSelection, SearchInput } from '.';

export const NavBar: FC = () => {
  const [_open, setOpen] = useState(false);
  const location = useLocation();

  const closeMenu = () => {
    setOpen(false);
  };

  const pathname = location.pathname;

  return (
    <nav className="w-full flex flex-self-stretch flex-items-center gap-sm p-4">
      <h2 className="flex-auto m0">
        <Link to="/" className="color-unset">
          Orderly Dashboard
        </Link>
      </h2>
      <BrokerSelection />
      <SearchInput />
      <TabNav.Root>
        <TabNav.Link asChild active={pathname === '/' || pathname === '/search'}>
          <Link to="/" className="w-full h-full px5 line-height-8" onClick={closeMenu}>
            Home
          </Link>
        </TabNav.Link>
        <TabNav.Link asChild active={pathname === '/leaderboard'}>
          <Link to="/leaderboard" className="w-full h-full px5 line-height-8" onClick={closeMenu}>
            Leaderboard
          </Link>
        </TabNav.Link>
      </TabNav.Root>
    </nav>
  );
};
