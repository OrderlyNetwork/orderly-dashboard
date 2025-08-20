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

export type PositionEntry = {
  account_id: string;
  address: string;
  broker_id: string;
  symbol: string;
  symbol_hash: string;
  holding: string;
  total_realized_pnl: string;
  index_price: string;
  mark_price: string;
  holding_value: string;
  opening_cost: string;
  average_entry_price: string;
  un_realized_pnl: string;
};

export type PositionsResponse = {
  rows: PositionEntry[];
};
