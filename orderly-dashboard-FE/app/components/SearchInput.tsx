import { useLocation, useNavigate, useNavigation, useSearchParams } from '@remix-run/react';
import { FC, useEffect, useState } from 'react';
import { match } from 'ts-pattern';
import { useDebounce } from 'use-debounce';

import { ChainNamespace } from '~/hooks';
import { base64UrlSafeDecode, base64UrlSafeEncode } from '~/util';

export const SearchInput: FC = () => {
  const [searchParams] = useSearchParams();
  const rawAddress = searchParams.get('q');
  const chainNamespace: ChainNamespace = match(searchParams.get('chain_namespace'))
    .with('evm', () => 'evm' as const)
    .with('sol', () => 'sol' as const)
    .otherwise(() => 'evm' as const);
  const address = match(chainNamespace)
    .with('evm', () => rawAddress)
    .with('sol', () => base64UrlSafeDecode(rawAddress ?? ''))
    .exhaustive();
  const [search, setSearch] = useState(address ?? '');
  const [focused, setFocused] = useState(false);
  const location = useLocation();
  const [[searchValue, pathname]] = useDebounce([search, location.pathname], 250);
  const navigate = useNavigate();
  const { state } = useNavigation();

  useEffect(() => {
    if (!searchValue) return;
    const isEvm = searchValue.match(/^0x[0-9a-fA-F]{40}$/);
    const isSol = searchValue.match(/^[0-9a-zA-Z]{43,44}$/);
    const isAccountId = searchValue.match(/^0x[0-9a-fA-F]{64}$/);
    if (!isEvm && !isSol && !isAccountId) return;

    const query = new URLSearchParams();
    if (isSol) {
      query.set('q', base64UrlSafeEncode(searchValue));
      query.set('chain_namespace', 'sol');
    } else {
      query.set('q', searchValue);
      query.set('chain_namespace', 'evm');
    }
    navigate({
      pathname: '/search',
      search: `?${query.toString()}`
    });
  }, [searchValue, navigate, pathname]);

  useEffect(() => {
    if (state === 'idle' && location.pathname !== '/search') {
      setSearch('');
    }
  }, [state, location.pathname, setSearch]);

  return (
    <div
      className="relative w-full transition-all duration-200"
      style={{
        borderRadius: '0.75rem',
        border: focused ? '1px solid #6700CE' : '1px solid transparent'
      }}
    >
      <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none z-10">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="rgba(255,255,255,0.5)">
          <path d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z" />
        </svg>
      </div>
      <input
        className="w-full pl-12 pr-4 py-3 rounded-xl border-0 outline-none ring-0 text-white placeholder-gray-400 transition-all duration-200"
        style={{ background: '#221E30', boxShadow: 'none' }}
        type="text"
        placeholder="Search wallet or account ID..."
        onFocus={() => setFocused(true)}
        onBlur={() => setFocused(false)}
        onChange={(event) => {
          setSearch(event.target.value);
        }}
        value={search}
      />
    </div>
  );
};
