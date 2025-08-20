import { useMemo } from 'react';
import useSWR from 'swr';

import { useAppState } from '~/App';
import { PositionsResponse } from '~/types/leaderboard';

export type PositionsParams = {
  account_id?: string;
  broker_id?: string;
  symbol?: string;
  offset?: number;
  limit?: number;
  order_by?: 'ASC' | 'DESC';
};

export function usePositions(params: PositionsParams) {
  const { queryServiceUrl } = useAppState();

  const queryKey = useMemo(() => {
    const searchParams = new URLSearchParams();

    if (params.account_id) {
      searchParams.set('account_id', params.account_id);
    }
    if (params.broker_id) {
      searchParams.set('broker_id', params.broker_id);
    }
    if (params.symbol) {
      searchParams.set('symbol', params.symbol);
    }
    if (params.offset !== undefined) {
      searchParams.set('offset', params.offset.toString());
    }
    if (params.limit !== undefined) {
      searchParams.set('limit', params.limit.toString());
    }
    if (params.order_by) {
      searchParams.set('order_by', params.order_by);
    }

    return `${queryServiceUrl}/ranking/positions?${searchParams.toString()}`;
  }, [queryServiceUrl, params]);

  const { data, error, isLoading, mutate } = useSWR<{
    success: boolean;
    err_code: number;
    err_msg: string | null;
    data: PositionsResponse;
  }>(
    queryKey,
    async (url: string) => {
      const response = await fetch(url);
      const result = await response.json();

      if (!result.success) {
        throw new Error(result.err_msg || 'Failed to fetch positions data');
      }

      return result;
    },
    {
      revalidateOnFocus: false
    }
  );

  return {
    data: data?.data,
    error,
    isLoading,
    mutate
  };
}
