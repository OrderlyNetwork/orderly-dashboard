import { MagnifyingGlassIcon } from '@radix-ui/react-icons';
import { useLocation, useNavigate, useNavigation, useSearchParams } from '@remix-run/react';
import { FC, useEffect, useState } from 'react';
import { useDebounce } from 'use-debounce';

export const SearchInput: FC = () => {
  const [searchParams] = useSearchParams();
  const [search, setSearch] = useState(searchParams.get('q') ?? '');
  const location = useLocation();
  const [[searchValue, pathname]] = useDebounce([search, location.pathname], 250);
  const navigate = useNavigate();
  const { state } = useNavigation();

  useEffect(() => {
    if (!searchValue) return;
    if (!searchValue.startsWith('0x') || searchValue.length != 42) return;

    const query = new URLSearchParams();
    query.set('q', searchValue);
    navigate(`/search?${query.toString()}`);
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
        placeholder="Search for wallet"
        onChange={(event) => {
          setSearch(event.target.value.toLowerCase());
        }}
        value={search}
      />
    </div>
  );
};
