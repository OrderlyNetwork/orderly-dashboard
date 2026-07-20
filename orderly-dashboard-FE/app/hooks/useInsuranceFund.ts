import useSWR from 'swr';

import { useAppState } from '~/App';

type Envelope<T> = {
  success: boolean;
  data: T;
  code?: string;
  message?: string;
  ts?: number;
};

async function fetchPublicGet<T>(url: string): Promise<T> {
  const res = await fetch(url);
  if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
  const json = (await res.json()) as Envelope<T>;
  if (!json.success) {
    throw new Error(json.code || json.message || `API error (${res.status})`);
  }
  return json.data;
}

export type InsuranceFundPosition = {
  symbol: string;
  timestamp: number;
  position_qty: number;
  cost_position: number;
  last_sum_unitary_funding: number;
  settle_price: number;
  average_open_price: number;
  pnl_24_h: number;
  fee_24_h: number;
  mark_price: number;
};

export type InsuranceFundData = {
  balance: number;
  free_collateral: number;
  margin_ratio: number;
  min_insurance_fund_initial_margin_ratio: number;
  min_insurance_fund_margin_ratio: number;
  total_collateral_value: number;
  total_account_value: number;
  total_pnl_24_h: number;
  rows: InsuranceFundPosition[];
};

/**
 * Orderly insurance fund balance, collateral, margin ratio, 24h PnL, and any
 * open positions currently held by the fund. Public endpoint, no auth required.
 */
export function useInsuranceFund() {
  const { evmApiUrl } = useAppState();
  return useSWR<InsuranceFundData>(
    evmApiUrl ? ['insuranceFund', evmApiUrl] : null,
    () => fetchPublicGet<InsuranceFundData>(`${evmApiUrl}/v1/public/insurancefund`),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000,
      refreshInterval: 60000
    }
  );
}
