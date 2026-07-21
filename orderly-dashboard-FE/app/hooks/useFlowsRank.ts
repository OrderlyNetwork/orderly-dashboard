import { useMemo } from 'react';
import useSWR from 'swr';

import { useAppState } from '~/App';
import { FlowsRankResponse } from '~/types/leaderboard';

export type FlowsRankParams = {
  days: number;
  size: number;
  token: string;
};

export type FlowsDirection = 'deposit' | 'withdraw';

function useFlowsRank(direction: FlowsDirection, params: FlowsRankParams) {
  const { queryServiceUrl } = useAppState();

  const queryKey = useMemo(() => {
    const searchParams = new URLSearchParams();
    searchParams.set('days', params.days.toString());
    searchParams.set('size', params.size.toString());
    searchParams.set('token', params.token);
    return `${queryServiceUrl}/ranking/${direction}?${searchParams.toString()}`;
  }, [queryServiceUrl, direction, params]);

  const { data, error, isLoading, mutate } = useSWR<{
    success: boolean;
    err_code: number;
    err_msg: string | null;
    data: FlowsRankResponse;
  }>(
    queryKey,
    async (url: string) => {
      const response = await fetch(url);
      const result = await response.json();

      if (!result.success) {
        throw new Error(result.err_msg || 'Failed to fetch flows ranking');
      }

      return result;
    },
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000
    }
  );

  return {
    data: data?.data,
    error,
    isLoading,
    mutate
  };
}

export const useDepositRank = (params: FlowsRankParams) => useFlowsRank('deposit', params);

export const useWithdrawRank = (params: FlowsRankParams) => useFlowsRank('withdraw', params);
