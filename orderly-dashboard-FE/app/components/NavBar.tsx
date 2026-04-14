import { Link } from '@remix-run/react';
import { FC } from 'react';

import { SearchInput } from '.';

export const NavBar: FC = () => {
  return (
    <nav className="glass-no-radius w-full flex flex-col sm:flex-row items-center justify-between gap-4 px-8 py-5 mb-8">
      <div className="flex items-center gap-3">
        <Link to="/" className="group flex items-center gap-3">
          <div
            style={{
              width: 36,
              height: 36,
              borderRadius: '50%',
              background: 'rgba(156, 117, 255, 0.15)',
              border: '1px solid rgba(156,117,255,0.3)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              flexShrink: 0
            }}
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
              <circle cx="12" cy="12" r="10" stroke="#9C75FF" strokeWidth="2" />
              <circle cx="12" cy="12" r="5" fill="#6700CE" />
            </svg>
          </div>
          <span
            style={{
              fontFamily: 'var(--font-display)',
              fontSize: '1.125rem',
              fontWeight: 700,
              color: '#fff',
              letterSpacing: '0.01em'
            }}
          >
            Orderly Dashboard
          </span>
        </Link>
      </div>

      <div className="flex flex-col sm:flex-row items-center gap-4 w-full sm:w-auto">
        <SearchInput />
      </div>
    </nav>
  );
};
