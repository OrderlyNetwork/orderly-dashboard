// Site-orientation tool: the "front door" for a cold agent. Returns a static map
// of what this site is, its areas, which tools back each, and where to start.

import { ro } from './tools';

export function siteOverview() {
  return {
    site: 'Orderly Dashboard',
    tagline: 'Read-only analytics for the Orderly Network, a multi-chain perpetual-futures DEX.',
    what_is_this:
      'This site is an analytics dashboard. It does NOT place trades. Use these tools to query ' +
      'on-chain trading data: volume, TVL, funding rates, liquidations, per-market detail, ' +
      'leaderboards, and per-address activity. Every tool is read-only and returns JSON.',
    how_to_start:
      'You have already called the orientation tool. Next, pick a tool by area below and call it. ' +
      'If unsure, run one of the suggested first queries at the bottom.',
    areas: [
      {
        name: 'Dashboard',
        url: '/',
        purpose: 'Platform-wide analytics and KPIs aggregated across all markets.',
        tools: [
          'get_dashboard_main — daily volume, fees(revenue), TVL, deposits/withdrawals, new accounts (time series)',
          'get_market_summary — live 24h snapshot: volume, open interest, trader counts, fees',
          'get_metrics_overview — weekly/monthly user & revenue aggregates',
          'get_tvl_by_chain, get_tvl_by_token — TVL broken down by chain / by settlement token',
          'get_weekly_symbol_volume — weekly per-symbol volume + listed-markets count',
          'get_funding_rates — 8-hour funding rates per symbol (from a trailing start date)',
          'get_funding_comparison — cross-exchange funding comparison for a symbol',
          'get_daily_liquidations_by_symbol — daily liquidation notional aggregated per symbol',
          'get_builder_daily — per-builder daily volume / fees / active traders',
          'get_fund_flows_by_broker, get_fund_flows_by_chain — deposit/withdrawal flows',
          'get_staking_daily, get_stake_users, get_stake_vs_supply — ORDER staking analytics',
          'get_volume_segments — volume split by maker/taker & chain',
          'get_omnivault_tvl — OmniVault TVL series',
          'get_dex_users — DEX user counts',
          'get_market_share — Orderly market share vs other venues',
          'get_insurance_fund — insurance fund balance',
          'get_distributor_stats, get_distributor_invitees — referral distributor analytics'
        ]
      },
      {
        name: 'Markets',
        url: '/markets',
        purpose: 'Live market list and per-symbol detail pages.',
        tools: [
          'get_markets — all active perp markets: prices, 24h change, open interest, funding',
          'get_symbol_info — one symbol config: max leverage, tick/lot size, precision',
          'get_market_detail — per-symbol bundle (candles, orderbook, trades, funding). Pass `include` to select sections; default returns all',
          'get_top_traders — top addresses by notional/volume/PnL for a symbol',
          'get_platform_positions — largest open positions (liquidation-heatmap data)',
          'get_recent_liquidations — recent individual liquidation events for one symbol'
        ]
      },
      {
        name: 'Leaderboard',
        url: '/leaderboard',
        purpose: 'Trader and open-position rankings.',
        tools: [
          'get_leaderboard — daily realized-PnL / perp-volume ranking (requires a date range)',
          'get_positions — largest open positions by notional value',
          'get_top_traders — whale leaderboard by volume/PnL (also usable per-symbol)',
          'get_whale_context — deep-dive on one address: state, positions, recent trades'
        ]
      },
      {
        name: 'Explorer',
        url: '/explorer',
        purpose: 'Look up any EVM or Solana address / account and analyze its activity.',
        tools: [
          'resolve_address — pass an address or account_id; returns broker/account pairs. START HERE for any wallet query',
          'get_events — historical trades/settlements/liquidations/ADL for an account (31-day cap per call)',
          'get_account_state — collateral, margin usage, PnL, open positions',
          'get_portfolio — daily equity-curve (portfolio value over time)',
          'get_whale_context — same shape as the leaderboard tool, scoped to the address'
        ]
      }
    ],
    reference_tools: [
      'get_brokers — broker_id -> broker name map',
      'get_all_symbols — symbol_hash -> human-readable symbol name (event/position data uses hashes)',
      'get_all_tokens — token_hash -> token name'
    ],
    common_flows: [
      {
        goal: 'Analyze a wallet',
        steps: [
          'resolve_address(address) -> get account_id(s) + broker_id(s)',
          'get_account_state({account_id}) for current balances / positions',
          'get_events({account_id, ...}) for trade history (page with the cursor if needed)',
          'get_portfolio({address}) for the equity curve'
        ]
      },
      {
        goal: 'Understand a single market',
        steps: [
          'get_markets() to find the symbol, or get_symbol_info({symbol})',
          'get_market_detail({symbol, include:[...]}) for candles/orderbook/trades',
          'get_top_traders({symbol}) and/or get_recent_liquidations({symbol})'
        ]
      },
      {
        goal: 'Get the platform pulse',
        steps: [
          'get_market_summary() for live 24h numbers',
          'get_dashboard_main({days:30}) for the trailing month of totals'
        ]
      }
    ],
    conventions: [
      'Symbols accept a base tick (BTC) or the full form (PERP_BTC_USDC) — both work.',
      'Date params: get_leaderboard uses start_date/end_date as YYYY-MM-DD; ' +
        'get_events/get_portfolio use from_time/to_time as Unix seconds.',
      'get_events time range is capped at 31 days per call — page via trading_event_next_cursor for longer ranges.',
      'Pagination: get_leaderboard/get_positions use page/offset; get_top_traders/get_events use a cursor.',
      'All tools are read-only. On failure a tool throws — you will see the error as a tool-call failure.'
    ],
    suggested_first_queries: [
      'get_market_summary() — current 24h platform snapshot',
      'get_markets() — list every active market',
      'get_dashboard_main({days:30}) — trailing month of platform totals'
    ]
  };
}

export function createSiteTools(): ModelContextTool[] {
  return [
    ro(
      'get_site_overview',
      'START HERE. Returns a map of this site: what it is, its areas, which tools back each ' +
        'area, common multi-step flows, parameter conventions, and suggested first queries. ' +
        'Call this once to get oriented before using the other tools. This is a read-only ' +
        'analytics dashboard for the Orderly Network perpetual-futures DEX; it does not place trades.',
      { type: 'object', properties: {}, additionalProperties: false },
      () => Promise.resolve(siteOverview()),
      'Site Overview'
    )
  ];
}
