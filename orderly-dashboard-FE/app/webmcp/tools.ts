// WebMCP tool registry. Composes per-domain tool factories into a single array.
// Shared helpers live here so each domain module can reuse them.

export type WebMcpCtx = {
  evmApiUrl: string;
  queryServiceUrl: string;
};

// ── helpers ───────────────────────────────────────────────────────────────────

export function asString(v: unknown): string | undefined {
  return typeof v === 'string' && v !== '' ? v : undefined;
}

export function clampInt(v: unknown, min: number, max: number, fallback: number): number {
  const n = typeof v === 'number' ? v : typeof v === 'string' ? parseInt(v, 10) : NaN;
  if (!Number.isFinite(n)) return fallback;
  return Math.min(max, Math.max(min, Math.trunc(n)));
}

// Supported candle intervals for get_market_detail. Exposed as a JSON-Schema enum
// so agents can't pass an interval the API would reject. The default is '1h'.
export const CANDLE_INTERVALS = ['5m', '15m', '1h', '4h', '1d'] as const;

// Normalise a perp symbol argument. The site/URLs use bare base ticks ("BTC") but
// the APIs expect the full form ("PERP_BTC_USDC"). Bare ticks are wrapped into the
// canonical USDC perp; already-full symbols — including broker-suffixed ones such as
// "PERP_AAPL_USDC_mythos" — are returned unchanged so their casing is preserved.
export function normalizeSymbol(input: string | undefined): string | undefined {
  if (!input) return undefined;
  const s = input.trim();
  if (/^PERP_.+_.+$/i.test(s)) return s;
  return `PERP_${s.toUpperCase()}_USDC`;
}

// Build `start_date=YYYY-MM-DD&end_date=YYYY-MM-DD` for the trailing `days` days.
export function dateRangeQuery(days: number): string {
  const end = new Date();
  const start = new Date();
  start.setDate(start.getDate() - days);
  const fmt = (d: Date) => d.toISOString().slice(0, 10);
  return `start_date=${fmt(start)}&end_date=${fmt(end)}`;
}

// Derive a human-readable title from a snake_case tool name (spec §4.2.1
// recommends `title` for display in the browser-agent's native UI). Callers may
// pass an explicit title to override. An explicit override map keeps common names
// clean (e.g. "TVL by Chain" rather than "Get Tvl By Chain") for the 38 tools
// that omit the `title` arg; unknown names fall back to title-casing.
const TITLE_OVERRIDES: Record<string, string> = {
  get_dashboard_main: 'Platform Daily Totals',
  get_tvl_by_chain: 'TVL by Chain',
  get_tvl_by_token: 'TVL by Token',
  get_weekly_symbol_volume: 'Weekly Symbol Volume',
  get_funding_rates: 'Funding Rates',
  get_daily_liquidations_by_symbol: 'Daily Liquidations by Symbol',
  get_staking_daily: 'Daily Staking',
  get_builder_daily: 'Builder Daily Metrics',
  get_fund_flows_by_broker: 'Fund Flows by Builder',
  get_fund_flows_by_chain: 'Fund Flows by Chain',
  get_dex_users: 'DEX Users',
  get_metrics_overview: 'Metrics Overview',
  get_volume_segments: 'Volume Segments',
  get_stake_users: 'Stake Users',
  get_stake_vs_supply: 'Stake vs Supply',
  get_omnivault_tvl: 'OmniVault TVL',
  get_distributor_stats: 'Distributor Stats',
  get_distributor_invitees: 'Distributor Invitees',
  get_market_share: 'DEX Market Share',
  get_markets: 'Markets',
  get_market_detail: 'Market Detail',
  get_symbol_info: 'Symbol Info',
  get_market_summary: 'Market Summary',
  get_funding_comparison: 'Funding Rate Comparison',
  get_recent_liquidations: 'Recent Liquidations',
  get_top_traders: 'Top Traders',
  get_whale_context: 'Whale Context',
  get_platform_positions: 'Platform Positions',
  get_insurance_fund: 'Insurance Fund',
  get_leaderboard: 'Trading Leaderboard',
  get_positions: 'Open Positions',
  resolve_address: 'Resolve Address',
  get_events: 'Account Events',
  get_account_state: 'Account State',
  get_portfolio: 'Portfolio',
  get_brokers: 'Brokers',
  get_all_symbols: 'Symbols',
  get_all_tokens: 'Tokens'
};

function titleFromName(name: string): string {
  return (
    TITLE_OVERRIDES[name] ??
    name
      .split('_')
      .map((w) => (w ? w[0].toUpperCase() + w.slice(1) : w))
      .join(' ')
  );
}

// Rewrite bare network failures into an actionable, agent-readable message.
// API-envelope errors ("Data API error: …", "Symbol not found: …") and anything
// thrown with a specific message are already actionable and pass through.
export function enrichToolError(err: unknown): never {
  const msg = err instanceof Error ? err.message : String(err);
  if (
    err instanceof TypeError ||
    /failed to fetch|networkerror|load failed|network request failed/i.test(msg)
  ) {
    throw new Error(
      'Tool could not reach its data source (network/CORS/timeout) — not a bad parameter. ' +
        'The upstream may be down or this origin may be disallowed; retry, or try a ' +
        'different tool that answers the same question. (original: ' +
        msg +
        ')'
    );
  }
  throw err;
}

// Construct a read-only tool. `run` may throw — the error propagates to the agent
// as a tool-call failure (correct MCP semantics) rather than a misleading success
// payload. Network/CORS/timeout failures are enriched so the agent gets guidance
// instead of a bare "Failed to fetch".
export function ro(
  name: string,
  description: string,
  inputSchema: object,
  run: (args: Record<string, unknown>) => Promise<unknown>,
  title?: string
): ModelContextTool {
  return {
    name,
    title: title ?? titleFromName(name),
    description,
    inputSchema,
    annotations: { readOnlyHint: true },
    execute: (args) => run(args).catch(enrichToolError)
  };
}

// Construct an action tool — one that changes the user's view (navigates the SPA,
// drives widget UI). Mirrors `ro()` but omits the read-only hint so the catalogue
// correctly marks these as non-read-only.
export function action(
  name: string,
  description: string,
  inputSchema: object,
  run: (args: Record<string, unknown>) => Promise<unknown>,
  title?: string
): ModelContextTool {
  return {
    name,
    title: title ?? titleFromName(name),
    description,
    inputSchema,
    execute: (args) => run(args)
  };
}

// ── tool factory ──────────────────────────────────────────────────────────────

import { createAccountTools } from './account-tools';
import { createDataApiTools } from './data-tools';
import { createMarketTools } from './market-tools';
import { createMetaTools } from './meta-tools';
import { createNavTools } from './nav-tools';
import { createSiteTools } from './site-tools';

export function createWebMcpTools(ctx: WebMcpCtx): ModelContextTool[] {
  return [
    ...createSiteTools(),
    ...createDataApiTools(),
    ...createMarketTools(ctx),
    ...createAccountTools(ctx),
    ...createMetaTools(ctx),
    ...createNavTools()
  ];
}
