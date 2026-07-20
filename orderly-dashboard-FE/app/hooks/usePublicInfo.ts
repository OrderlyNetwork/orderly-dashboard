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
    symbol && evmApiUrl ? ['platformPositions', evmApiUrl, symbol, minNotional] : null,
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

// ── symbolInfo (includes base_imr for max leverage calculation) ──────────────

export type SymbolInfoResponse = {
  symbol: string;
  quote_min: number;
  quote_max: number;
  quote_tick: number;
  base_min: number;
  base_max: number;
  base_tick: number;
  min_notional: number;
  price_range: number;
  price_scope: number;
  std_liquidation_fee: number;
  liquidator_fee: number;
  claim_insurance_fund_discount: number;
  funding_period: number;
  cap_funding: number;
  floor_funding: number;
  cap_ir: number;
  floor_ir: number;
  interest_rate: number;
  imr_factor: number;
  created_time: number;
  updated_time: number;
  base_mmr: number;
  base_imr: number;
  liquidation_tier: number;
  global_max_oi_cap: number;
};

/**
 * Get symbol info including base_imr (initial margin rate).
 * Max leverage = 1 / base_imr
 */
export function useSymbolInfo(symbol: string) {
  const { evmApiUrl } = useAppState();
  return useSWR<SymbolInfoResponse>(
    symbol && evmApiUrl ? ['symbolInfo', evmApiUrl, symbol] : null,
    () => fetchPublicGet<SymbolInfoResponse>(`${evmApiUrl}/v1/public/info/${symbol}`),
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
  index_price?: string;
  '24h_open': string;
  '24h_close': string;
  '24h_high': string;
  '24h_low': string;
  '24h_volume': string;
  '24h_amount': string;
  open_interest: string;
  last_funding_rate?: string;
  est_funding_rate?: string | null;
  next_funding_time?: number;
  bid_price?: string;
  ask_price?: string;
  max_leverage?: string;
  min_notional?: string;
  quote_tick?: string;
  base_tick?: string;
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
    evmApiUrl ? ['marketSummary', evmApiUrl] : null,
    () => postPublicInfo<MarketSummaryResponse>(evmApiUrl, 'marketSummary', {}),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000,
      refreshInterval: 60000
    }
  );
}

// ── futuresMarket ────────────────────────────────────────────────────────────

export type FuturesMarketRow = {
  symbol: string;
  display_symbol_name: string;
  broker_id: string | null;
  status: string;
  index_price: number;
  mark_price: number;
  sum_unitary_funding: number;
  est_funding_rate: number;
  last_funding_rate: number;
  next_funding_time: number;
  open_interest: number;
  is_pretge: boolean;
  '24h_open': number;
  '24h_close': number;
  '24h_high': number;
  '24h_low': number;
  '24h_volume': number;
  '24h_amount': number;
};

export type FuturesMarketResponse = {
  rows: FuturesMarketRow[];
};

export function useFuturesMarket() {
  const { evmApiUrl } = useAppState();
  return useSWR<FuturesMarketResponse>(
    evmApiUrl ? ['futuresMarket', evmApiUrl] : null,
    () => fetchPublicGet<FuturesMarketResponse>(`${evmApiUrl}/v1/public/futures_market`),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000,
      refreshInterval: 60000
    }
  );
}

// ── futuresSymbol (single symbol market info) ────────────────────────────────

export type FuturesSymbolResponse = {
  symbol: string;
  display_symbol_name: string;
  broker_id: string | null;
  status: string;
  index_price: number;
  mark_price: number;
  sum_unitary_funding: number;
  est_funding_rate: number;
  last_funding_rate: number;
  next_funding_time: number;
  open_interest: number;
  is_pretge: boolean;
  '24h_open': number;
  '24h_close': number;
  '24h_high': number;
  '24h_low': number;
  '24h_volume': number;
  '24h_amount': number;
};

export function useFuturesSymbol(symbol: string) {
  const { evmApiUrl } = useAppState();
  return useSWR<FuturesSymbolResponse>(
    symbol && evmApiUrl ? ['futuresSymbol', evmApiUrl, symbol] : null,
    () => fetchPublicGet<FuturesSymbolResponse>(`${evmApiUrl}/v1/public/futures/${symbol}`),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000,
      refreshInterval: 60000
    }
  );
}

// ── priceChanges ─────────────────────────────────────────────────────────────

export type PriceChangeRow = {
  symbol: string;
  last_price: number;
  '5m': number | null;
  '30m': number | null;
  '1h': number | null;
  '4h': number | null;
  '24h': number | null;
  '3d': number | null;
  '7d': number | null;
  '30d': number | null;
};

export type PriceChangesResponse = {
  rows: PriceChangeRow[];
};

export function usePriceChanges() {
  const { evmApiUrl } = useAppState();
  return useSWR<PriceChangesResponse>(
    evmApiUrl ? ['priceChanges', evmApiUrl] : null,
    () => fetchPublicGet<PriceChangesResponse>(`${evmApiUrl}/v1/public/market_info/price_changes`),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000,
      refreshInterval: 60000
    }
  );
}

// ── tradersOpenInterests ─────────────────────────────────────────────────────

export type TradersOIRow = {
  symbol: string;
  long_oi: number;
  short_oi: number;
};

export type TradersOpenInterestsResponse = {
  rows: TradersOIRow[];
};

export function useTradersOpenInterests() {
  const { evmApiUrl } = useAppState();
  return useSWR<TradersOpenInterestsResponse>(
    evmApiUrl ? ['tradersOpenInterests', evmApiUrl] : null,
    () =>
      fetchPublicGet<TradersOpenInterestsResponse>(
        `${evmApiUrl}/v1/public/market_info/traders_open_interests`
      ),
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
    evmApiUrl ? ['topAddresses', evmApiUrl, params] : null,
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

// ── marketDetail (Composite single-symbol bundle) ────────────────────────────

export type OrderbookLevel = {
  price: string;
  quantity: string;
};

export type MarketTrade = {
  symbol: string;
  side: 'BUY' | 'SELL';
  executed_price: string;
  executed_quantity: string;
  address: string;
  account_id: string;
  broker_id: string;
  executed_timestamp: number;
};

export type FundingRateEntry = {
  funding_rate: string;
  mark_price: string;
  funding_rate_timestamp: number;
};

export type Candle = {
  open: string;
  high: string;
  low: string;
  close: string;
  volume: string;
  timestamp: number;
};

export type MarketDetailResponse = {
  symbol: string;
  market_info?: MarketSummaryMarket;
  orderbook?: {
    asks: OrderbookLevel[];
    bids: OrderbookLevel[];
    ts?: number;
  };
  recent_trades?: MarketTrade[];
  funding_history?: FundingRateEntry[];
  candles?: Candle[];
};

export type MarketDetailParams = {
  symbol: string;
  include?: string[];
  orderbook_levels?: number;
  recent_trades_limit?: number;
  funding_history_limit?: number;
  candles_interval?: string;
  candles_limit?: number;
};

/**
 * Composite single-symbol bundle: market info, orderbook, recent trades,
 * funding history, and candles in one call. Weight 2.
 */
export function useMarketDetail(symbol: string, candlesInterval: string = '1h') {
  const { evmApiUrl } = useAppState();
  return useSWR<MarketDetailResponse>(
    symbol && evmApiUrl ? ['marketDetail', evmApiUrl, symbol, candlesInterval] : null,
    () =>
      postPublicInfo<MarketDetailResponse>(evmApiUrl, 'marketDetail', {
        symbol,
        include: ['market_info', 'orderbook', 'recent_trades', 'funding_history', 'candles'],
        orderbook_levels: 50,
        recent_trades_limit: 50,
        funding_history_limit: 100,
        candles_interval: candlesInterval,
        candles_limit: 168
      }),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 10000,
      refreshInterval: 15000,
      keepPreviousData: true
    }
  );
}

// ── portfolio (Account Equity Curve) ───────────────────────────────────────

export type PortfolioRow = {
  account_value: string;
  cumulative_pnl: string;
  timestamp: number;
};

export type PortfolioResponse = {
  rows: PortfolioRow[];
  next_cursor: string | null;
};

export type PortfolioParams = {
  address: string;
  broker_id?: string;
  account_id?: string;
  start_time?: number;
  end_time?: number;
  limit?: number;
};

/**
 * Daily account-value time series with cumulative PnL. Weight 5.
 * UTC-day-aligned snapshots, up to 365 days.
 */
export function usePortfolio(params: PortfolioParams) {
  const { evmApiUrl } = useAppState();
  return useSWR<PortfolioResponse>(
    params.address ? ['portfolio', evmApiUrl, params] : null,
    () =>
      postPublicInfo<PortfolioResponse>(evmApiUrl, 'portfolio', {
        address: params.address,
        broker_id: params.broker_id || undefined,
        account_id: params.account_id || undefined,
        interval: '1d',
        start_time: params.start_time || undefined,
        end_time: params.end_time || undefined,
        limit: params.limit || 90
      }),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000,
      refreshInterval: 0
    }
  );
}

// ── accountState (Address open positions + account snapshot) ────────────────

export type AccountStatePosition = {
  symbol: string;
  side: 'LONG' | 'SHORT';
  position_qty: string;
  notional: string | null;
  average_open_price: string | null;
  mark_price: string | null;
  unrealized_pnl: string | null;
  unsettled_pnl: string | null;
  pnl_24_h: string | null;
  imr: string | null;
  mmr: string | null;
  est_liq_price: string | null;
  leverage: number | null;
  margin_mode: string | null;
  opened_at: number | null;
  updated_at: number | null;
};

export type AccountStateResponse = {
  address: string;
  broker_id: string;
  account_id: string;
  account_type: string;
  account_value: string | null;
  total_collateral_value: string;
  free_collateral: string | null;
  margin_ratio: string | null;
  initial_margin_ratio: string | null;
  maintenance_margin_ratio: string | null;
  total_unrealized_pnl: string | null;
  total_unsettled_pnl: string | null;
  total_pnl_24_h: string | null;
  positions: AccountStatePosition[];
};

export type AccountStateParams = {
  address: string;
  broker_id?: string;
  account_id?: string;
};

/**
 * Account snapshot: collateral, margin metrics, PnL, and open positions.
 * Weight 5. Only returns OPEN positions (holding ≠ 0).
 */
export function useAccountState(params: AccountStateParams) {
  const { evmApiUrl } = useAppState();
  return useSWR<AccountStateResponse>(
    params.address ? ['accountState', evmApiUrl, params] : null,
    () =>
      postPublicInfo<AccountStateResponse>(evmApiUrl, 'accountState', {
        address: params.address,
        broker_id: params.broker_id || undefined,
        account_id: params.account_id || undefined
      }),
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 5000,
      refreshInterval: 15000
    }
  );
}
