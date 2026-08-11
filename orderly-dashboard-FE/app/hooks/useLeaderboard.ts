import { useMemo } from 'react';
import useSWR from 'swr';

import { useAppState } from '~/App';
import { fetchEvmGet } from '~/services/orderly';
import { LeaderboardResponse } from '~/types/leaderboard';

export type LeaderboardSortOption =
  | 'ascending_realized_pnl'
  | 'descending_realized_pnl'
  | 'ascending_perp_volume'
  | 'descending_perp_volume';

export type LeaderboardParams = {
  start_date: string;
  end_date: string;
  page?: number;
  size?: number;
  order_tag?: string;
  broker_id?: string;
  sort?: LeaderboardSortOption;
  address?: string;
  aggregateBy?: 'address' | 'address_per_builder' | 'date' | 'account';
};

export function useLeaderboard(params: LeaderboardParams) {
  const { evmApiUrl } = useAppState();

  const qs = useMemo(() => {
    const searchParams = new URLSearchParams();
    searchParams.set('start_date', params.start_date);
    searchParams.set('end_date', params.end_date);

    if (params.page !== undefined) {
      searchParams.set('page', params.page.toString());
    }
    if (params.size !== undefined) {
      searchParams.set('size', params.size.toString());
    }
    if (params.order_tag) {
      searchParams.set('order_tag', params.order_tag);
    }
    if (params.broker_id) {
      searchParams.set('broker_id', params.broker_id);
    }
    if (params.sort) {
      searchParams.set('sort', params.sort);
    }
    if (params.address) {
      searchParams.set('address', params.address);
    }
    if (params.aggregateBy) {
      searchParams.set('aggregateBy', params.aggregateBy);
    }

    return searchParams.toString();
  }, [params]);

  const { data, error, isLoading, mutate } = useSWR<LeaderboardResponse>(
    evmApiUrl ? ['leaderboard', evmApiUrl, qs] : null,
    () => fetchEvmGet<LeaderboardResponse>(evmApiUrl, `/v1/broker/leaderboard/daily?${qs}`),
    {
      revalidateOnFocus: false
    }
  );

  return {
    data,
    error,
    isLoading,
    mutate
  };
}
