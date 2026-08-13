import { useMemo } from 'react';
import useSWR from 'swr';

import { useAppState } from '~/App';
import { fetchQueryGet } from '~/services/orderly';
import { FlowsRankResponse } from '~/types/leaderboard';

export type FlowsRankParams = {
  days: number;
  size: number;
  token: string;
};

export type FlowsDirection = 'deposit' | 'withdraw';

function useFlowsRank(direction: FlowsDirection, params: FlowsRankParams) {
  const { queryServiceUrl } = useAppState();

  const qs = useMemo(() => {
    const searchParams = new URLSearchParams();
    searchParams.set('days', params.days.toString());
    searchParams.set('size', params.size.toString());
    searchParams.set('token', params.token);
    return searchParams.toString();
  }, [params]);

  const { data, error, isLoading, mutate } = useSWR<FlowsRankResponse>(
    queryServiceUrl ? ['flowsRank', direction, queryServiceUrl, qs] : null,
    () => fetchQueryGet<FlowsRankResponse>(queryServiceUrl, `/ranking/${direction}?${qs}`),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000
    }
  );

  return {
    data,
    error,
    isLoading,
    mutate
  };
}

export const useDepositRank = (params: FlowsRankParams) => useFlowsRank('deposit', params);

export const useWithdrawRank = (params: FlowsRankParams) => useFlowsRank('withdraw', params);
