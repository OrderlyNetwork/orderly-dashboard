import { useEffect, useMemo, useState } from 'react';
import useSWR from 'swr';
import { match } from 'ts-pattern';

import { useAppState } from '~/App';

type SearchType = 'address' | 'accountId' | undefined;
export type AddressData = { address: string; account_id: string; broker_id: string };

export const useSearchAddress = (
  query: string | null
): { addressData?: AddressData[]; loading: boolean } => {
  const { evmApiUrl } = useAppState();
  const [index, setIndex] = useState(0);
  const [loadingCount, setLoadingCount] = useState(1_000);
  const [urls, setUrls] = useState<string[] | undefined>();
  const [addressData, setAddressData] = useState<AddressData[]>();

  const isEvm = useMemo(() => query != null && !!query.match(/^0x[0-9a-fA-F]{40}$/), [query]);
  const isSol = useMemo(() => query != null && !!query.match(/^[0-9a-zA-Z]{44}$/), [query]);
  const isAccountId = useMemo(() => query != null && !!query.match(/^0x[0-9a-fA-F]{66}$/), [query]);

  let searchType: SearchType;
  if (isEvm || isSol) {
    searchType = 'address';
  } else if (isAccountId) {
    searchType = 'accountId';
  } else {
    searchType = undefined;
  }

  useEffect(() => {
    if (searchType != null) return;
    setAddressData(undefined);
  }, [searchType]);

  useEffect(() => {
    if (loadingCount === 0 && addressData == null) {
      setAddressData([]);
    }
  }, [loadingCount, addressData, setAddressData]);

  useEffect(() => {
    match(searchType)
      .with('accountId', () => {
        setUrls([`${evmApiUrl}/v1/public/account?account_id=${query}`]);
      })
      .with('address', () =>
        fetch(`${evmApiUrl}/v1/get_broker?address=${query}${isSol ? '&chain_type=SOL' : ''}`)
          .then((res) => res.json())
          .then((res) => {
            const brokerIds: string[] = res.data.broker_id;
            setUrls(
              brokerIds.map(
                (brokerId) =>
                  `${evmApiUrl}/v1/get_account?address=${query}&broker_id=${brokerId}${isSol ? '&chain_type=SOL' : ''}`
              )
            );
          })
      )
      .with(undefined, () => [])
      .exhaustive();
  }, [searchType, evmApiUrl, query, isSol]);

  useEffect(() => {
    if (!urls) return;
    setIndex(0);
    setLoadingCount(urls.length);
  }, [urls]);

  useSWR(
    (urls ?? [])[index],
    (url: string) =>
      fetch(url)
        .then((r) => r.json())
        .then((val) => {
          if (!val.success) {
            if (val.code === -1_003) {
              throw new Error(val.code);
            }
            // setLoadingCount(loadingCount - 1);
            setLoadingCount((count) => count - 1);
            setIndex(index + 1);
            throw new Error(val.message);
          }
          const data: AddressData | undefined = match(searchType)
            // eslint-disable-next-line @typescript-eslint/no-unused-vars
            .with('address', () => {
              const uri = new URL(url);
              const broker_id = uri.searchParams.get('broker_id');
              if (!broker_id) throw new Error();
              return {
                address: query!,
                broker_id,
                account_id: val.data.account_id
              };
            })
            .with('accountId', () => ({ ...val.data, account_id: query! }))
            .otherwise(() => undefined);
          if (!data) throw new Error();
          setAddressData((cur) => [...(cur ?? []), data]);
          setLoadingCount((count) => count - 1);
          setIndex(index + 1);
        }),
    {
      onErrorRetry: (error, _key, _config, revalidate, { retryCount }) => {
        let code = -1;
        try {
          code = Number(error.message);
        } catch (err) {
          return;
        }
        if (code !== -1_003 || retryCount >= 10) {
          return;
        }
        setTimeout(() => revalidate({ retryCount }), 1_000);
      }
    }
  );
  return { addressData, loading: loadingCount > 0 };
};
