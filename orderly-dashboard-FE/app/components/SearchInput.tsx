import { MagnifyingGlassIcon } from '@radix-ui/react-icons';
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
  const location = useLocation();
  const [[searchValue, pathname]] = useDebounce([search, location.pathname], 250);
  const navigate = useNavigate();
  const { state } = useNavigation();

  useEffect(() => {
    if (!searchValue) return;
    const isEvm = searchValue.match(/^0x[0-9a-fA-F]{40}$/);
    const isSol = searchValue.match(/^[0-9a-zA-Z]{44}$/);
    const isAccountId = searchValue.match(/^0x[0-9a-fA-F]{66}$/);
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
    <div className="relative">
      <MagnifyingGlassIcon
        className="absolute left-0 top-0 max-w-full max-h-full p-[0.4rem]"
        height="40"
        width="40"
      />
      <input
        className="p-[0.3rem_0_0.3rem_40px] h-full w-[20rem]"
        type="text"
        placeholder="Search via wallet or account ID"
        onChange={(event) => {
          setSearch(event.target.value);
        }}
        value={search}
      />
    </div>
  );
};
