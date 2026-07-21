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

export type FlowsRankEntry = {
  account_id: string;
  token_hash: string;
  amount: string;
};

export type FlowsRankResponse = FlowsRankEntry[];

export type WhaleEntry = {
  address: string;
  broker_id: string;
  total_notional: string;
  pnl_24h: string;
  pnl_7d: string;
  pnl_30d: string;
  volume_24h: string;
  volume_7d: string;
  volume_30d: string;
  trade_count_24h: number;
  win_rate_24h: number | null;
  win_rate_7d: number | null;
  win_rate_30d: number | null;
  avg_trade_size: string | null;
  position_count: number;
};

export type WhaleLeaderboardResponse = {
  rows: WhaleEntry[];
  next_cursor: string | null;
  last_updated_time: number;
};

export type WhaleSortOption =
  | 'notional'
  | 'volume_24h'
  | 'volume_7d'
  | 'volume_30d'
  | 'pnl_24h'
  | 'pnl_7d'
  | 'pnl_30d'
  | 'trade_count_24h';
