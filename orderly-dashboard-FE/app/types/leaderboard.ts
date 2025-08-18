export type LeaderboardEntry = {
  date: string;
  account_id?: string;
  perp_volume: number;
  perp_taker_volume: number;
  perp_maker_volume: number;
  total_fee: number;
  broker_fee: number;
  address?: string;
  broker_id?: string;
  realized_pnl: number;
};
export type LeaderboardResponse = {
  rows: LeaderboardEntry[];
  meta: {
    total: number;
    records_per_page: number;
    current_page: number;
  };
  snapshot_time: number;
};
