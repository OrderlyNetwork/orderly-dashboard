import dayjs from 'dayjs';
import { useEffect, useMemo, useState } from 'react';
import useSWR from 'swr';
import { match } from 'ts-pattern';

import { useAppState } from '~/App';
import { LeaderboardEntry } from '~/types/leaderboard';

type SearchType = 'address' | 'accountId' | undefined;
type AccountRow = {
  user_id: number;
  account_id: string;
  broker_id: string;
  chain_type: string;
  user_type: string;
};

export type AddressData = {
  address: string;
  account_id: string;
  broker_id: string;
  user_id?: number;
  chain_type?: string;
  user_type?: string;
  perp_volume?: number;
  realized_pnl?: number;
};
export type ChainNamespace = 'evm' | 'sol';

export const useSearchAddress = (
  query: string | null
): { addressData?: AddressData[]; loading: boolean } => {
  const { evmApiUrl } = useAppState();
  const [index, setIndex] = useState(0);
  const [loadingCount, setLoadingCount] = useState(1_000);
  const [urls, setUrls] = useState<string[] | undefined>();
  const [addressData, setAddressData] = useState<AddressData[]>();

  const endDate = dayjs().format('YYYY-MM-DD');
  const startDate = dayjs().subtract(89, 'days').format('YYYY-MM-DD');

  const isEvm = useMemo(() => query != null && !!query.match(/^0x[0-9a-fA-F]{40}$/), [query]);
  const isSol = useMemo(() => query != null && !!query.match(/^[0-9a-zA-Z]{43,44}$/), [query]);
  const isAccountId = useMemo(() => query != null && !!query.match(/^0x[0-9a-fA-F]{64}$/), [query]);

  let searchType: SearchType;
  if (isEvm || isSol) {
    searchType = 'address';
  } else if (isAccountId) {
    searchType = 'accountId';
  } else {
    searchType = undefined;
  }

  useEffect(() => {
    setAddressData(undefined);
    setIndex(0);
    setLoadingCount(1_000);
  }, [query]);

  useEffect(() => {
    if (searchType != null) return;
    setAddressData(undefined);
    setIndex(0);
    setLoadingCount(0);
  }, [searchType]);

  useEffect(() => {
    if (loadingCount === 0 && addressData == null) {
      setAddressData([]);
    }
  }, [loadingCount, addressData]);

  useEffect(() => {
    match(searchType)
      .with('accountId', () => {
        setUrls([`${evmApiUrl}/v1/public/account?account_id=${query}`]);
      })
      .with('address', () => {
        setUrls([
          `${evmApiUrl}/v1/get_all_accounts?address=${query}${isSol ? '&chain_type=SOL' : ''}`
        ]);
      })
      .with(undefined, () => {
        setUrls(undefined);
        setLoadingCount(0);
      })
      .exhaustive();
  }, [searchType, evmApiUrl, query, isSol]);

  useEffect(() => {
    if (!urls) return;
    setIndex(0);
    setLoadingCount(urls.length);
    setAddressData(undefined);
  }, [urls]);

  useSWR(
    urls && index < urls.length ? urls[index] : null,
    async (url: string) => {
      const response = await fetch(url);
      const val = await response.json();

      if (!val.success) {
        if (val.code === -1_003) {
          throw new Error(val.code);
        }
        setLoadingCount((count) => count - 1);
        setIndex((currentIndex) => currentIndex + 1);
        throw new Error(val.message);
      }

      const data: AddressData[] | undefined = match(searchType)
        .with('address', () => {
          const rows = val.data.rows || [];
          return rows.map((row: AccountRow) => ({
            address: query!,
            broker_id: row.broker_id,
            account_id: row.account_id,
            user_id: row.user_id,
            chain_type: row.chain_type,
            user_type: row.user_type
          }));
        })
        .with('accountId', () => [{ ...val.data, account_id: query! }])
        .otherwise(() => undefined);

      if (!data) throw new Error();

      if (searchType === 'address') {
        try {
          const leaderboardUrl = `${evmApiUrl}/v1/broker/leaderboard/daily?start_date=${startDate}&end_date=${endDate}&page=1&sort=descending_perp_volume&address=${query}&aggregateBy=account`;
          const leaderboardResponse = await fetch(leaderboardUrl);
          const leaderboardData = await leaderboardResponse.json();

          if (leaderboardData.success && leaderboardData.data.rows) {
            const enrichedData = data.map((account) => {
              const accountStats = leaderboardData.data.rows.find(
                (row: LeaderboardEntry) => row.account_id === account.account_id
              );

              if (accountStats) {
                return {
                  ...account,
                  perp_volume: accountStats.perp_volume,
                  realized_pnl: accountStats.realized_pnl
                };
              }

              return account;
            });

            const sortedData = enrichedData.sort((a, b) => {
              const volumeA = a.perp_volume ?? 0;
              const volumeB = b.perp_volume ?? 0;
              return volumeB - volumeA;
            });
            setAddressData((cur) => [...(cur ?? []), ...sortedData]);
          } else {
            setAddressData((cur) => [...(cur ?? []), ...data]);
          }
        } catch (error) {
          console.warn(`Failed to fetch leaderboard stats for address ${query}:`, error);
          setAddressData((cur) => [...(cur ?? []), ...data]);
        }
      } else {
        setAddressData((cur) => [...(cur ?? []), ...data]);
      }

      setLoadingCount((count) => count - 1);
      setIndex((currentIndex) => currentIndex + 1);
    },
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
