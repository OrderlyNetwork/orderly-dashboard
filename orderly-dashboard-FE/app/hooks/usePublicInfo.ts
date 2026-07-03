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
