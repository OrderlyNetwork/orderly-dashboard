// Maps each embeddable widgetId (WIDGET_META in routes/widget.$widgetId.tsx) to the
// WebMCP tool(s) that expose that widget's data. Lets a chromeless embed register
// just its own tool(s) instead of the full 35-tool set.

import { createWebMcpTools, type WebMcpCtx } from './tools';

export const WIDGET_TOOL_MAP: Record<string, string[]> = {
  volume: ['get_dashboard_main'],
  'tvl-chain': ['get_tvl_by_chain'],
  'net-fees': ['get_dashboard_main'],
  overview: ['get_metrics_overview'],
  'dex-users': ['get_dex_users'],
  'market-share': ['get_market_share'],
  'volume-segments': ['get_volume_segments'],
  'omnivault-tvl': ['get_omnivault_tvl'],
  'stake-users': ['get_stake_users'],
  'stake-vs-supply': ['get_stake_vs_supply'],
  distributors: ['get_distributor_stats', 'get_distributor_invitees'],
  'fees-stats': ['get_dashboard_main'],
  'builder-volume': ['get_builder_daily'],
  'builder-active-traders': ['get_builder_daily'],
  'builder-revenue': ['get_builder_daily'],
  'net-flow-by-builder': ['get_fund_flows_by_broker'],
  'tvl-by-token': ['get_tvl_by_token'],
  leaderboard: ['get_leaderboard'],
  positions: ['get_positions'],
  'kpi-analyst': ['get_dashboard_main'],
  'funding-comparison': ['get_funding_comparison'],
  'funding-rates': ['get_funding_rates'],
  'staking-daily': ['get_staking_daily'],
  'fund-flows-by-chain': ['get_fund_flows_by_chain'],
  'liquidations-by-symbol': ['get_daily_liquidations_by_symbol'],
  'liquidation-heatmap': ['get_platform_positions'],
  'insurance-fund': ['get_insurance_fund'],
  'market-price-chart': ['get_market_detail'],
  'market-orderbook': ['get_market_detail'],
  'market-recent-trades': ['get_market_detail'],
  'market-funding-chart': ['get_market_detail'],
  'market-recent-liquidations': ['get_recent_liquidations'],
  'market-top-traders': ['get_top_traders'],
  'market-platform-positions': ['get_platform_positions']
  // 'top-flows' intentionally omitted: no deposit/withdraw rank tool exists.
};

export function createWidgetTools(ctx: WebMcpCtx, widgetId: string): ModelContextTool[] {
  const names = WIDGET_TOOL_MAP[widgetId];
  if (!names || names.length === 0) return [];
  const byName = new Map(createWebMcpTools(ctx).map((t) => [t.name, t]));
  return names
    .map((n) => byName.get(n))
    .filter((t): t is ModelContextTool => typeof t !== 'undefined');
}
