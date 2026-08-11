// WebMCP tools backed by the Orderly Data API (analytics). Each tool maps to a
// dashboard widget's data source. All are read-only and return JSON payloads.

import { clampInt, dateRangeQuery, ro } from './tools';

import { fetchMarketShare } from '~/hooks/useMarketShare';
import { fetchDataApi } from '~/services/orderly';

function startDateDaysAgo(days: number): string {
  const d = new Date();
  d.setDate(d.getDate() - days);
  return d.toISOString().slice(0, 10);
}

export function createDataApiTools(): ModelContextTool[] {
  const numDays = (args: Record<string, unknown>, fallback: number) =>
    clampInt(args.days, 1, 730, fallback);

  return [
    ro(
      'get_dashboard_main',
      'Daily platform totals for Orderly: taker/cumulative volume, revenue (fees), TVL, ' +
        'deposits/withdrawals, new accounts/addresses, and active builders, as a historical ' +
        'time series. For a live snapshot use get_market_summary; for weekly/monthly user ' +
        'aggregates use get_metrics_overview. Returns rows newest-last over the trailing ' +
        '`days` window.',
      {
        type: 'object',
        properties: {
          days: { type: 'number', description: 'Trailing days (default 90, max 730).' }
        },
        additionalProperties: false
      },
      (args) =>
        fetchDataApi<{ rows: unknown[] }>(
          `/orderly/api/v1/dashboard/orderly/main?${dateRangeQuery(numDays(args, 90))}`
        ).then((r) => r.rows)
    ),
    ro(
      'get_tvl_by_chain',
      'Total Value Locked broken down by chain, plus a total row. Drives the TVL By Chain ' +
        'widget. Returns newest snapshot rows.',
      { type: 'object', properties: {}, additionalProperties: false },
      () =>
        fetchDataApi<{ rows: unknown[] }>('/orderly/api/v1/dashboard/orderly/tvl-by-chain').then(
          (r) => r.rows
        )
    ),
    ro(
      'get_tvl_by_token',
      'TVL and token balance held per collateral token over a trailing window. Drives the ' +
        'TVL By Token widget.',
      {
        type: 'object',
        properties: {
          days: { type: 'number', description: 'Trailing days (default 30, max 730).' }
        },
        additionalProperties: false
      },
      (args) =>
        fetchDataApi<{ rows: unknown[] }>(
          `/orderly/api/v1/dashboard/orderly/by-symbol/daily?symbol_type=token&${dateRangeQuery(
            numDays(args, 30)
          )}`
        ).then((r) => r.rows)
    ),
    ro(
      'get_weekly_symbol_volume',
      'Weekly per-symbol volume and listed-markets count from the analytics API. For ' +
        'the live market list (prices, 24h change, open interest) use get_markets.',
      { type: 'object', properties: {}, additionalProperties: false },
      () =>
        fetchDataApi<{ rows: unknown[] }>(
          '/orderly/api/v1/dashboard/orderly/by-symbol/weekly'
        ).then((r) => r.rows)
    ),
    ro(
      'get_funding_rates',
      '8-hour funding rates for every perpetual symbol, from a trailing start date. For ' +
        'cross-exchange funding comparisons use get_funding_comparison.',
      {
        type: 'object',
        properties: {
          days: { type: 'number', description: 'Trailing days for the start date (default 30).' }
        },
        additionalProperties: false
      },
      (args) =>
        fetchDataApi<{ rows: unknown[] }>(
          `/orderly/api/v1/dashboard/orderly/funding-rates?start_date=${startDateDaysAgo(
            numDays(args, 30)
          )}`
        ).then((r) => r.rows)
    ),
    ro(
      'get_daily_liquidations_by_symbol',
      'Daily liquidation notional aggregated per perpetual symbol across all markets. For ' +
        'individual recent liquidation events of one symbol use get_recent_liquidations.',
      { type: 'object', properties: {}, additionalProperties: false },
      () =>
        fetchDataApi<{ rows: unknown[] }>(
          '/orderly/api/v1/dashboard/orderly/by-symbol/daily?symbol_type=perp'
        ).then((r) => r.rows)
    ),
    ro(
      'get_staking_daily',
      'Daily $ORDER staking metrics: staked/unstaked/net amounts, cumulative staker ' +
        'addresses, buybacks, and burned ORDER. Drives the Staking Daily widget.',
      { type: 'object', properties: {}, additionalProperties: false },
      () =>
        fetchDataApi<{ rows: unknown[] }>('/orderly/api/v1/dashboard/staking/daily').then(
          (r) => r.rows
        )
    ),
    ro(
      'get_builder_daily',
      'Per-builder daily metrics: taker/maker/total volume, active users, broker fees, and ' +
        'net flow. Drives the Builder Volume, Builder Active Traders, Builder Revenue, and ' +
        'Net Flow By Builder widgets.',
      {
        type: 'object',
        properties: {
          days: { type: 'number', description: 'Trailing days (default 30, max 730).' }
        },
        additionalProperties: false
      },
      (args) =>
        fetchDataApi<{ rows: unknown[] }>(
          `/orderly/api/v1/dashboard/orderly/by-broker?exclude_zero_volume=true&${dateRangeQuery(
            numDays(args, 30)
          )}`
        ).then((r) => r.rows)
    ),
    ro(
      'get_fund_flows_by_broker',
      'Daily deposits, withdrawals, and net fund flow per broker. Drives the Net Flow By ' +
        'Builder widget (fund-flow variant).',
      {
        type: 'object',
        properties: {
          days: { type: 'number', description: 'Trailing days (default 30, max 730).' }
        },
        additionalProperties: false
      },
      (args) =>
        fetchDataApi<{ rows: unknown[] }>(
          `/orderly/api/v1/dashboard/fund-flows/by-broker?${dateRangeQuery(numDays(args, 30))}`
        ).then((r) => r.rows)
    ),
    ro(
      'get_fund_flows_by_chain',
      'Daily deposits, withdrawals, and net fund flow per chain. Drives the Fund Flows By ' +
        'Chain widget.',
      { type: 'object', properties: {}, additionalProperties: false },
      () =>
        fetchDataApi<{ rows: unknown[] }>('/orderly/api/v1/dashboard/fund-flows/by-chain').then(
          (r) => r.rows
        )
    ),
    ro(
      'get_dex_users',
      'Per-builder user activity snapshot: DAU/WAU/MAU with day/week/month-over-month deltas ' +
        'and new-user counts. Drives the DEX Users widget.',
      { type: 'object', properties: {}, additionalProperties: false },
      () =>
        fetchDataApi<{ data?: unknown[] }>('/orderly/api/v1/metrics/dex-users').then(
          (r) => r.data ?? []
        )
    ),
    ro(
      'get_metrics_overview',
      'Weekly and monthly aggregates for new users, active users, trading volume, and Orderly ' +
        'revenue. For daily historical totals use get_dashboard_main.',
      { type: 'object', properties: {}, additionalProperties: false },
      () => fetchDataApi<unknown>('/orderly/api/v1/metrics/overview')
    ),
    ro(
      'get_volume_segments',
      'Weekly trading volume split by market-maker vs retail segment. Drives the Volume ' +
        'Segments widget.',
      { type: 'object', properties: {}, additionalProperties: false },
      () => fetchDataApi<unknown>('/orderly/api/v1/metrics/volume-segments')
    ),
    ro(
      'get_stake_users',
      'Weekly average active stakers. Drives the Stake Users widget.',
      { type: 'object', properties: {}, additionalProperties: false },
      () => fetchDataApi<unknown>('/orderly/api/v1/metrics/stake-users')
    ),
    ro(
      'get_stake_vs_supply',
      'Weekly ORDER staked vs circulating supply and staking ratio. Drives the Stake vs ' +
        'Supply widget.',
      { type: 'object', properties: {}, additionalProperties: false },
      () => fetchDataApi<unknown>('/orderly/api/v1/metrics/stake-vs-supply')
    ),
    ro(
      'get_omnivault_tvl',
      'Weekly average TVL per OmniVault (USD millions). Drives the OmniVault TVL widget.',
      { type: 'object', properties: {}, additionalProperties: false },
      () => fetchDataApi<unknown>('/orderly/api/v1/metrics/omnivault-tvl')
    ),
    ro(
      'get_distributor_stats',
      'Referral distributor statistics: invitee counts, graduated invitees, revenue share, ' +
        'and fee tier. Drives the Distributors widget. Returns an array of distributor rows.',
      { type: 'object', properties: {}, additionalProperties: false },
      () => fetchDataApi<unknown[]>('/orderly/api/v1/distributors/stats')
    ),
    ro(
      'get_distributor_invitees',
      'Per-invitee detail for referral distributors: invite date, DEX, status, and 30D ' +
        'volume/revenue share. Returns an array of invitee rows.',
      { type: 'object', properties: {}, additionalProperties: false },
      () => fetchDataApi<unknown[]>('/orderly/api/v1/distributors/invitees')
    ),
    ro(
      'get_market_share',
      "Orderly's DEX perpetuals market share vs. other on-chain perp venues, computed from " +
        'CoinGecko 24h derivatives volume and DefiLlama open interest. Drives the Market ' +
        'Share widget. Returns ranked protocols with volume, open interest, and share %.',
      { type: 'object', properties: {}, additionalProperties: false },
      () => fetchMarketShare()
    )
  ];
}
