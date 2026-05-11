import useSWR from 'swr';

async function fetchJson<T>(url: string): Promise<T> {
  const res = await fetch(url);
  if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
  return res.json() as Promise<T>;
}

function useFetch<T>(path: string) {
  return useSWR<T>(`${import.meta.env.DATA_API_URL}${path}`, fetchJson<T>, {
    revalidateOnFocus: false,
    shouldRetryOnError: false
  });
}

// ── Types ─────────────────────────────────────────────────────────────────────

// /data/summary — returns auth session info only
export type DataSummary = {
  user_id?: number;
  username?: string;
  email?: string;
  mysql_source?: string;
  pg_source?: string;
  logs?: string[];
  [key: string]: unknown;
};

// /data/broker/liquidation_fees — requires broker_id query param
export type BrokerLiquidationFees = {
  success?: boolean;
  code?: number;
  message?: string;
  data?: null;
  [key: string]: unknown;
};

// /distributors/stats — direct array (no wrapper object)
export type DistributorRow = {
  'Distributor ID'?: string;
  'Distributor Type'?: string;
  'Distributor Name'?: string;
  'Number of Invitees'?: number;
  'Number of Graduated Invitees'?: number;
  'Total Revenue Share'?: number;
  '30D Revenue Share'?: number;
  '30D Invitee Volume'?: number;
  'Fee Tier'?: string;
  [key: string]: unknown;
};
export type DistributorStats = DistributorRow[];

// /distributors/invitees — direct array (no wrapper object)
export type DistributorInviteeRow = {
  'Date Invited'?: string;
  'Invitee DEX'?: string;
  'Distributor ID'?: string;
  is_orderly_one?: string;
  'DEX link'?: string;
  'DEX status'?: string;
  '30D Invitee Volume'?: number;
  '30D Revenue Share'?: number;
  [key: string]: unknown;
};
export type DistributorInvitees = DistributorInviteeRow[];

// /metrics/dex-users — per-broker current snapshot
export type DexUsersBroker = {
  broker_id?: string;
  broker_name?: string;
  dau?: number;
  dau_date?: string;
  dau_dod_pct?: number;
  wau?: number;
  wau_wow_pct?: number;
  mau?: number;
  mau_mom_pct?: number;
  total_users?: number;
  new_users_today?: number;
  new_users_7d?: number;
  new_users_30d?: number;
  [key: string]: unknown;
};
export type DexUsers = { data?: DexUsersBroker[] };

// /metrics/overview — weekly & monthly aggregates
export type OverviewPeriod = {
  week_start?: string;
  week_start_date?: string;
  week_end_date?: string;
  month_start?: string;
  month_start_date?: string;
  month_end_date?: string;
  avg_new_user?: number;
  avg_active_user?: number;
  avg_trading_volume?: number;
  avg_orderly_revenue?: number;
  [key: string]: unknown;
};
export type MetricsOverview = { weekly?: OverviewPeriod[]; monthly?: OverviewPeriod[] };

// /metrics/volume-segments — weekly by mm_retail type
export type VolumeSegmentRow = {
  week_start?: string;
  week_start_date?: string;
  week_end_date?: string;
  mm_retail?: string;
  volume?: number;
  pct?: number;
  [key: string]: unknown;
};
export type VolumeSegments = { weekly?: VolumeSegmentRow[] };

// /metrics/stake-users — weekly avg active stakers
export type StakeUsersWeek = {
  wk?: string;
  avg_active_stakers?: number;
  week_start?: string;
  week_end?: string;
  [key: string]: unknown;
};
export type StakeUsers = { weekly?: StakeUsersWeek[] };

// /metrics/stake-vs-supply — weekly ORDER staking vs circulating supply
export type StakeVsSupplyWeek = {
  wk?: string;
  stake_order?: number;
  circulating_supply?: number;
  stake_order_perc_avg?: number;
  week_start?: string;
  week_end?: string;
  wow_growth_rate_percent?: number;
  [key: string]: unknown;
};
export type StakeVsSupply = { weekly?: StakeVsSupplyWeek[] };

// /metrics/omnivault-tvl — weekly TVL per vault (in millions USD)
export type OmnivaultTvlRow = {
  wk?: string;
  vault_name?: string;
  avg_vault_tvl_m?: number;
  week_start?: string;
  week_end?: string;
  [key: string]: unknown;
};
export type OmnivaultTvl = { weekly?: OmnivaultTvlRow[] };

// ── Hooks ─────────────────────────────────────────────────────────────────────

export const useDataSummary = () => useFetch<DataSummary>('/orderly/api/v1/data/summary');

export const useBrokerLiquidationFees = () =>
  useFetch<BrokerLiquidationFees>('/orderly/api/v1/data/broker/liquidation_fees');

export const useDistributorStats = () =>
  useFetch<DistributorStats>('/orderly/api/v1/distributors/stats');

export const useDistributorInvitees = () =>
  useFetch<DistributorInvitees>('/orderly/api/v1/distributors/invitees');

export const useDexUsers = () => useFetch<DexUsers>('/orderly/api/v1/metrics/dex-users');

export const useMetricsOverview = () =>
  useFetch<MetricsOverview>('/orderly/api/v1/metrics/overview');

export const useVolumeSegments = () =>
  useFetch<VolumeSegments>('/orderly/api/v1/metrics/volume-segments');

export const useStakeUsers = () => useFetch<StakeUsers>('/orderly/api/v1/metrics/stake-users');

export const useStakeVsSupply = () =>
  useFetch<StakeVsSupply>('/orderly/api/v1/metrics/stake-vs-supply');

export const useOmnivaultTvl = () =>
  useFetch<OmnivaultTvl>('/orderly/api/v1/metrics/omnivault-tvl');
