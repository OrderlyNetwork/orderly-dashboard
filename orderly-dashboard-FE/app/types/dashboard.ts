export type MainDailyRow = {
  date: string;
  total_deposits_usd: number;
  total_withdrawals_usd: number;
  tvl_usd: number;
  taker_volume_usd: number;
  cumulative_volume_usd: number;
  rolling_30d_volume_usd: number;
  daily_revenue_usd: number;
  cumulative_revenue_usd: number;
  rolling_30d_revenue_usd: number;
  new_accounts: number;
  new_addresses: number;
  cumulative_accounts: number;
  cumulative_addresses: number;
  cumulative_broker_fees_usd: number;
  active_builders_count: number;
};

export type TvlChainRow = {
  as_of_date: string;
  chain: string;
  is_total: boolean;
  tvl_usd: number;
};

export type MarketRow = {
  trade_week: string;
  markets: number;
};

export type DashboardData = {
  mainRows: MainDailyRow[];
  tvlChains: TvlChainRow[];
  tvlTotal: number;
  marketRows: MarketRow[];
};
