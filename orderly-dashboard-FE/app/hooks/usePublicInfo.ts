import useSWR from 'swr';

import { useAppState } from '~/App';

type Envelope<T> = {
  success: boolean;
  data: T;
  code?: string;
  message?: string;
  ts?: number;
};

async function postPublicInfo<T>(
  baseUrl: string,
  type: string,
  params: Record<string, unknown>
): Promise<T> {
  const res = await fetch(`${baseUrl}/v1/public/query`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ type, ...params })
  });
  const json = (await res.json()) as Envelope<T>;
  if (!json.success) {
    throw new Error(json.code || json.message || `Public Info API error (${res.status})`);
  }
  return json.data;
}

// ── platformPositions ─────────────────────────────────────────────────────────

export type PlatformPosition = {
  address: string | null;
  account_id: string | null;
  broker_id: string | null;
  symbol: string;
  side: 'LONG' | 'SHORT';
  position_qty: string;
  notional: string;
  average_open_price: string;
  mark_price: string;
  est_liq_price: string | null;
  unrealized_pnl: string | null;
  unsettled_pnl: string | null;
  leverage: number | null;
  margin_mode: string | null;
  opened_at: number | null;
};

export type PlatformPositionsResponse = {
  total_long_notional: string;
  total_short_notional: string;
  total_positions: number;
  rows: PlatformPosition[];
  next_cursor: string | null;
};

/**
 * Platform-wide open positions for a symbol. Weight 20 per call — polled at 60s
 * with dedup to respect the anonymous per-IP quota pool.
 */
export function usePlatformPositions(symbol: string, minNotional: number) {
  const { evmApiUrl } = useAppState();
  return useSWR<PlatformPositionsResponse>(
    symbol ? ['platformPositions', evmApiUrl, symbol, minNotional] : null,
    () =>
      postPublicInfo<PlatformPositionsResponse>(evmApiUrl, 'platformPositions', {
        symbol,
        min_notional: String(minNotional),
        limit: 1000
      }),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000,
      refreshInterval: 60000
    }
  );
}

// ── marketSummary ─────────────────────────────────────────────────────────────

export type MarketSummaryMarket = {
  symbol: string;
  mark_price: string;
  '24h_open': string;
  '24h_close': string;
  '24h_high': string;
  '24h_low': string;
  '24h_volume': string;
  '24h_amount': string;
  open_interest: string;
};

export type MarketSummaryResponse = {
  total_24h_volume: string | null;
  total_open_interest: string | null;
  markets: MarketSummaryMarket[];
};

/**
 * All active markets with 24h volume. Weight 1.
 */
export function useMarketSummary() {
  const { evmApiUrl } = useAppState();
  return useSWR<MarketSummaryResponse>(
    ['marketSummary', evmApiUrl],
    () => postPublicInfo<MarketSummaryResponse>(evmApiUrl, 'marketSummary', {}),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000,
      refreshInterval: 60000
    }
  );
}

// ── topAddresses (Whale Leaderboard) ─────────────────────────────────────────

export type TopAddressEntry = {
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

export type TopAddressesResponse = {
  rows: TopAddressEntry[];
  next_cursor: string | null;
  last_updated_time: number;
};

export type TopAddressesSortOption =
  | 'notional'
  | 'volume_24h'
  | 'volume_7d'
  | 'volume_30d'
  | 'pnl_24h'
  | 'pnl_7d'
  | 'pnl_30d'
  | 'trade_count_24h';

export type TopAddressesParams = {
  symbol?: string;
  sort_by?: TopAddressesSortOption;
  min_notional?: number;
  limit?: number;
  cursor?: string;
};

/**
 * Top addresses ranked by various metrics. Weight 10.
 * Used for whale tracking / leaderboard.
 */
export function useTopAddresses(params: TopAddressesParams = {}) {
  const { evmApiUrl } = useAppState();
  return useSWR<TopAddressesResponse>(
    ['topAddresses', evmApiUrl, params],
    () =>
      postPublicInfo<TopAddressesResponse>(evmApiUrl, 'topAddresses', {
        symbol: params.symbol || undefined,
        sort_by: params.sort_by || 'notional',
        min_notional: params.min_notional || 0,
        limit: params.limit || 50,
        cursor: params.cursor || undefined
      }),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000,
      refreshInterval: 60000
    }
  );
}

// ── whaleContext (Whale Detail) ──────────────────────────────────────────────

export type WhaleAccount = {
  account_id: string;
  broker_id: string;
  holding: string;
  frozen: string;
  total_unrealized_pnl: string;
  total_realized_pnl: string;
  margin_ratio: string | null;
  total_collateral: string;
  free_collateral: string;
};

export type WhalePosition = {
  symbol: string;
  side: 'LONG' | 'SHORT';
  position_qty: string;
  notional: string;
  average_open_price: string;
  mark_price: string;
  est_liq_price: string | null;
  unrealized_pnl: string;
  leverage: number | null;
  margin_mode: string | null;
};

export type WhaleTrade = {
  id: string;
  symbol: string;
  side: 'BUY' | 'SELL';
  executed_price: string;
  executed_quantity: string;
  executed_timestamp: number;
  fee: string;
  fee_asset: string;
};

export type WhaleContextResponse = {
  account: WhaleAccount | null;
  positions: WhalePosition[];
  recent_trades: WhaleTrade[];
};

export type WhaleContextParams = {
  address: string;
  broker_id?: string;
  account_id?: string;
  recent_trades_limit?: number;
};

/**
 * Whale research bundle: account state, open positions, and recent trades.
 * Weight 3. Useful for drilling into a specific whale.
 */
export function useWhaleContext(params: WhaleContextParams) {
  const { evmApiUrl } = useAppState();
  return useSWR<WhaleContextResponse>(
    params.address ? ['whaleContext', evmApiUrl, params] : null,
    () =>
      postPublicInfo<WhaleContextResponse>(evmApiUrl, 'whaleContext', {
        address: params.address,
        broker_id: params.broker_id || undefined,
        account_id: params.account_id || undefined,
        recent_trades_limit: params.recent_trades_limit || 20
      }),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 5000,
      refreshInterval: 0
    }
  );
}
