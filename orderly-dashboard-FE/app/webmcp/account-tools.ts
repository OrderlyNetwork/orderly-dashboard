// WebMCP tools backed by the EVM API + Dashboard Query Service (leaderboards,
// rankings, address resolution, account events/state). Read-only.

import dayjs from 'dayjs';

import { asString, clampInt, dateRangeQuery, normalizeSymbol, ro, type WebMcpCtx } from './tools';

import { fetchEvents, type EventsParams, type EventType } from '~/hooks/useEvents';
import { fetchEvmGet, fetchEvmQuery, fetchQueryGet } from '~/services/orderly';

type AccountRow = {
  account_id: string;
  broker_id: string;
  user_id?: number;
  chain_type?: string;
  user_type?: string;
};

type AddressLbStats = {
  account_id: string;
  perp_volume?: number;
  realized_pnl?: number;
};

export function createAccountTools(ctx: WebMcpCtx): ModelContextTool[] {
  const { evmApiUrl, queryServiceUrl } = ctx;

  return [
    ro(
      'get_leaderboard',
      'Daily trading leaderboard: realized PnL and perp volume per trader over a date range ' +
        '(YYYY-MM-DD). Supports pagination, broker/address filters, sort, and aggregation by ' +
        'address, account, or date. Drives the Leaderboard (trading) tab.',
      {
        type: 'object',
        properties: {
          start_date: { type: 'string', description: 'Inclusive start, YYYY-MM-DD.' },
          end_date: { type: 'string', description: 'Inclusive end, YYYY-MM-DD.' },
          page: { type: 'number', description: '1-based page (default 1).' },
          size: { type: 'number', description: 'Page size (default 20, max 100).' },
          sort: {
            type: 'string',
            enum: [
              'ascending_realized_pnl',
              'descending_realized_pnl',
              'ascending_perp_volume',
              'descending_perp_volume'
            ],
            description: 'Default descending_realized_pnl.'
          },
          broker_id: { type: 'string' },
          address: { type: 'string', description: 'Filter to a wallet address.' },
          aggregateBy: {
            type: 'string',
            enum: ['address', 'address_per_builder', 'date', 'account']
          }
        },
        required: ['start_date', 'end_date'],
        additionalProperties: false
      },
      (args) => {
        const startDate = asString(args.start_date);
        const endDate = asString(args.end_date);
        if (!startDate || !endDate) {
          return Promise.reject(new Error('start_date and end_date are required (YYYY-MM-DD).'));
        }
        const sp = new URLSearchParams();
        sp.set('start_date', startDate);
        sp.set('end_date', endDate);
        const page = clampInt(args.page, 1, 10000, 1);
        if (args.page !== undefined) sp.set('page', String(page));
        const size = clampInt(args.size, 1, 100, 20);
        if (args.size !== undefined) sp.set('size', String(size));
        const sort = asString(args.sort);
        if (sort) sp.set('sort', sort);
        const brokerId = asString(args.broker_id);
        if (brokerId) sp.set('broker_id', brokerId);
        const address = asString(args.address);
        if (address) sp.set('address', address);
        const aggregateBy = asString(args.aggregateBy);
        if (aggregateBy) sp.set('aggregateBy', aggregateBy);
        return fetchEvmGet(evmApiUrl, `/v1/broker/leaderboard/daily?${sp.toString()}`);
      }
    ),
    ro(
      'get_positions',
      'Open-positions leaderboard: largest open positions by notional, optionally filtered by ' +
        'account, broker, or symbol. Drives the Leaderboard (positions) tab.',
      {
        type: 'object',
        properties: {
          account_id: { type: 'string' },
          broker_id: { type: 'string' },
          symbol: {
            type: 'string',
            description: 'Perp symbol: base tick (BTC) or full form (PERP_BTC_USDC).'
          },
          offset: { type: 'number', description: 'Pagination offset (default 0).' },
          limit: { type: 'number', description: 'Page size (default 50, max 500).' },
          order_by: { type: 'string', enum: ['ASC', 'DESC'], description: 'Default DESC.' }
        },
        additionalProperties: false
      },
      (args) => {
        const sp = new URLSearchParams();
        const accountId = asString(args.account_id);
        if (accountId) sp.set('account_id', accountId);
        const brokerId = asString(args.broker_id);
        if (brokerId) sp.set('broker_id', brokerId);
        const symbol = normalizeSymbol(asString(args.symbol));
        if (symbol) sp.set('symbol', symbol);
        if (args.offset !== undefined) sp.set('offset', String(clampInt(args.offset, 0, 1e9, 0)));
        if (args.limit !== undefined) sp.set('limit', String(clampInt(args.limit, 1, 500, 50)));
        const orderBy = asString(args.order_by);
        if (orderBy) sp.set('order_by', orderBy);
        return fetchQueryGet(queryServiceUrl, `/ranking/positions?${sp.toString()}`);
      }
    ),
    ro(
      'resolve_address',
      'Resolve a wallet address or account ID into its Orderly broker/account pairs. Accepts ' +
        'an EVM address (0x + 40 hex), a Solana address (43-44 base58 chars), or an account ID ' +
        '(0x + 64 hex). Returns all (broker_id, account_id) pairs for the identity. For address ' +
        'queries (not account IDs) each account is enriched with 90-day perp_volume and ' +
        'realized_pnl, sorted by volume descending — matching the Search results page. First ' +
        'step of the Explorer/address flow.',
      {
        type: 'object',
        properties: {
          query: {
            type: 'string',
            description: 'EVM address, Solana address, or Orderly account ID.'
          }
        },
        required: ['query'],
        additionalProperties: false
      },
      (args) => {
        const q = asString(args.query) ?? '';
        const isEvm = /^0x[0-9a-fA-F]{40}$/.test(q);
        const isSol = /^[0-9a-zA-Z]{43,44}$/.test(q);
        const isAccountId = /^0x[0-9a-fA-F]{64}$/.test(q);
        if (isAccountId) {
          return fetchEvmGet(evmApiUrl, `/v1/public/account?account_id=${q}`);
        }
        if (isEvm || isSol) {
          return fetchEvmGet<{ rows?: AccountRow[] }>(
            evmApiUrl,
            `/v1/get_all_accounts?address=${q}${isSol ? '&chain_type=SOL' : ''}`
          ).then(async (res) => {
            const accounts = res.rows ?? [];
            try {
              const lb = await fetchEvmGet<{ rows?: AddressLbStats[] }>(
                evmApiUrl,
                `/v1/broker/leaderboard/daily?${dateRangeQuery(89)}&page=1&sort=descending_perp_volume&address=${encodeURIComponent(q)}&aggregateBy=account`
              );
              const byId = new Map((lb.rows ?? []).map((r) => [r.account_id, r] as const));
              const enriched = accounts.map((a) => {
                const s = byId.get(a.account_id);
                return { ...a, perp_volume: s?.perp_volume, realized_pnl: s?.realized_pnl };
              });
              enriched.sort((a, b) => (b.perp_volume ?? 0) - (a.perp_volume ?? 0));
              return enriched;
            } catch {
              return accounts;
            }
          });
        }
        return Promise.reject(
          new Error(
            'Unrecognized identifier: provide an EVM address (0x+40 hex), Solana address (43-44 base58), or account ID (0x+64 hex).'
          )
        );
      }
    ),
    ro(
      'get_events',
      'Historical trading events for an account: trades, settlements, liquidations, ADLs, and ' +
        'margin transfers (versioned v1/v2/v3 already flattened to a uniform shape). Time range ' +
        'is capped at 31 days; omit both times for the most recent page. Drives the address ' +
        'Events tab.',
      {
        type: 'object',
        properties: {
          account_id: { type: 'string', description: 'Orderly account ID (0x + 64 hex).' },
          event_type: {
            type: 'string',
            enum: ['TRANSACTION', 'PERPTRADE', 'SETTLEMENT', 'LIQUIDATION', 'ADL', 'MARGINTRANSFER']
          },
          from_time: { type: 'number', description: 'Inclusive start, Unix seconds.' },
          to_time: { type: 'number', description: 'Inclusive end, Unix seconds.' },
          cursor: {
            type: 'object',
            description: 'Pagination cursor (trading_event_next_cursor) from a prior response.'
          }
        },
        required: ['account_id'],
        additionalProperties: false
      },
      (args) => {
        const accountId = asString(args.account_id);
        if (!accountId) {
          return Promise.reject(new Error('account_id is required (0x + 64 hex).'));
        }
        const query: EventsParams = {
          account_id: accountId,
          event_type: asString(args.event_type) as EventType | undefined,
          from_time: typeof args.from_time === 'number' ? dayjs.unix(args.from_time) : undefined,
          to_time: typeof args.to_time === 'number' ? dayjs.unix(args.to_time) : undefined,
          trading_event_next_cursor:
            args.cursor && typeof args.cursor === 'object'
              ? (args.cursor as EventsParams['trading_event_next_cursor'])
              : undefined
        };
        return fetchEvents(query, queryServiceUrl);
      }
    ),
    ro(
      'get_account_state',
      'Account snapshot for an address: collateral, margin ratios, 24h PnL, and all OPEN ' +
        'positions (qty, notional, entry/mark price, leverage, est. liquidation price). ' +
        'Drives the address Positions tab.',
      {
        type: 'object',
        properties: {
          address: { type: 'string', description: 'Wallet address (EVM 0x…).' },
          broker_id: { type: 'string' },
          account_id: { type: 'string' }
        },
        required: ['address'],
        additionalProperties: false
      },
      (args) =>
        fetchEvmQuery(evmApiUrl, 'accountState', {
          address: asString(args.address),
          broker_id: asString(args.broker_id),
          account_id: asString(args.account_id)
        })
    ),
    ro(
      'get_portfolio',
      'Daily account-value time series with cumulative PnL (equity curve) for an address, up ' +
        'to 365 days. Drives the Portfolio Chart on the address page.',
      {
        type: 'object',
        properties: {
          address: { type: 'string', description: 'Wallet address (EVM 0x…).' },
          broker_id: { type: 'string' },
          account_id: { type: 'string' },
          start_time: { type: 'number', description: 'Inclusive start, Unix seconds.' },
          end_time: { type: 'number', description: 'Inclusive end, Unix seconds.' },
          limit: { type: 'number', description: 'Max snapshots (default 90, max 365).' }
        },
        required: ['address'],
        additionalProperties: false
      },
      (args) =>
        fetchEvmQuery(evmApiUrl, 'portfolio', {
          address: asString(args.address),
          broker_id: asString(args.broker_id),
          account_id: asString(args.account_id),
          interval: '1d',
          start_time: typeof args.start_time === 'number' ? args.start_time : undefined,
          end_time: typeof args.end_time === 'number' ? args.end_time : undefined,
          limit: clampInt(args.limit, 1, 365, 90)
        })
    )
  ];
}
