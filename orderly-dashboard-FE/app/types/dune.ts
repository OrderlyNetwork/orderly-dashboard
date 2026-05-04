export type VolumeRow = {
  volume_date: string;
  volume: number;
  cumulative_volume: number;
  rolling_total_volume: number;
};

export type TvlChainRow = {
  chain: string;
  tvl: number;
};

export type FeeRow = {
  volume_date: string;
  cum_rev: number;
  daily_rev: number;
  rolling_rev: number;
  rolling_total_rev: number;
};

export type AccountRow = {
  date: string;
  cumulative_accounts: number;
  new_accounts: number;
};

export type MarketRow = {
  trade_week: string;
  markets: number;
};

export type DuneData = {
  volumeRows: VolumeRow[];
  tvlChains: TvlChainRow[];
  feeRows: FeeRow[];
  accountRows: AccountRow[];
  marketRows: MarketRow[];
  builderFees: number;
  activeBuilders: number;
};
