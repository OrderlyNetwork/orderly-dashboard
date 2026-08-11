// WebMCP tools backed by the Orderly EVM API (public market + protocol data).
// Each tool maps to a market/protocol widget's data source. Read-only.

import { asString, clampInt, normalizeSymbol, ro, type WebMcpCtx } from './tools';

import { fetchEvmGet, fetchEvmQuery } from '~/services/orderly';

export function createMarketTools(ctx: WebMcpCtx): ModelContextTool[] {
  const { evmApiUrl } = ctx;

  return [
    ro(
      'get_markets',
      'Snapshot of all active perpetual markets: the futures market list, 24h price ' +
        'changes, and per-symbol open interest. Drives the Markets page. Returns an object ' +
        '{ markets, priceChanges, openInterest }.',
      { type: 'object', properties: {}, additionalProperties: false },
      () =>
        Promise.all([
          fetchEvmGet(evmApiUrl, '/v1/public/futures_market'),
          fetchEvmGet(evmApiUrl, '/v1/public/market_info/price_changes'),
          fetchEvmGet(evmApiUrl, '/v1/public/market_info/traders_open_interests')
        ]).then(([markets, priceChanges, openInterest]) => ({
          markets,
          priceChanges,
          openInterest
        }))
    ),
    ro(
      'get_market_detail',
      'Composite single-symbol bundle for the market detail page. Returns only the ' +
        'requested sections (default: all of market info, top-50 orderbook levels, ' +
        'recent trades, funding history, and OHLCV candles). Requesting fewer sections ' +
        'keeps the payload small. Drives the Price Chart, Orderbook, Recent Trades, and ' +
        'Funding Chart.',
      {
        type: 'object',
        properties: {
          symbol: {
            type: 'string',
            description: 'Perp symbol. Accepts a base tick (BTC) or full form (PERP_BTC_USDC).'
          },
          include: {
            type: 'array',
            items: {
              type: 'string',
              enum: ['market_info', 'orderbook', 'recent_trades', 'funding_history', 'candles']
            },
            description: 'Sections to return; omit for all.'
          },
          candles_interval: {
            type: 'string',
            description: "Candle interval (default '1h'). e.g. 5m, 15m, 1h, 4h, 1d."
          }
        },
        required: ['symbol'],
        additionalProperties: false
      },
      (args) => {
        const include =
          Array.isArray(args.include) && args.include.length
            ? (args.include as string[])
            : ['market_info', 'orderbook', 'recent_trades', 'funding_history', 'candles'];
        return fetchEvmQuery(evmApiUrl, 'marketDetail', {
          symbol: normalizeSymbol(asString(args.symbol)),
          include,
          orderbook_levels: 50,
          recent_trades_limit: 50,
          funding_history_limit: 100,
          candles_interval: asString(args.candles_interval) ?? '1h',
          candles_limit: 168
        });
      }
    ),
    ro(
      'get_symbol_info',
      'Full configuration for a single perpetual symbol: leverage limits (1/base_imr), ' +
        'tick/lot sizes, price precision, and status flags. Returns the symbol info object.',
      {
        type: 'object',
        properties: {
          symbol: { type: 'string', description: 'Perp symbol: base tick (ETH) or full form.' }
        },
        required: ['symbol'],
        additionalProperties: false
      },
      (args) => fetchEvmGet(evmApiUrl, `/v1/public/info/${normalizeSymbol(asString(args.symbol))}`)
    ),
    ro(
      'get_market_summary',
      'Live platform-wide market snapshot: 24h volume, open interest, trader counts, ' +
        'and fee totals across all symbols. For historical daily time series use ' +
        'get_dashboard_main; for weekly/monthly user aggregates use get_metrics_overview.',
      { type: 'object', properties: {}, additionalProperties: false },
      () => fetchEvmQuery(evmApiUrl, 'marketSummary', {})
    ),
    ro(
      'get_funding_comparison',
      'Cross-exchange funding rate comparison: last value plus 1d/7d/30d averages per ' +
        'symbol across venues (Orderly vs. Binance, Bybit, etc.). Omit symbol for all ' +
        'symbols. For per-symbol 8h funding history use get_funding_rates.',
      {
        type: 'object',
        properties: {
          symbol: {
            type: 'string',
            description: 'Perp symbol: base tick (BTC) or full form; omit for all.'
          }
        },
        additionalProperties: false
      },
      (args) => {
        const symbol = normalizeSymbol(asString(args.symbol));
        return fetchEvmQuery(evmApiUrl, 'fundingComparison', symbol ? { symbol } : {}).then(
          (r) => (r as { rows?: unknown[] }).rows ?? r
        );
      }
    ),
    ro(
      'get_recent_liquidations',
      'Recent individual liquidation events for one symbol: side, size, notional, ' +
        'mark/liquidation price, and the liquidated account. For daily liquidation ' +
        'totals aggregated across all symbols use get_daily_liquidations_by_symbol.',
      {
        type: 'object',
        properties: {
          symbol: {
            type: 'string',
            description: 'Perp symbol: base tick (BTC) or full form (PERP_BTC_USDC).'
          },
          limit: { type: 'number', description: 'Max events (default 50, max 500).' }
        },
        required: ['symbol'],
        additionalProperties: false
      },
      (args) =>
        fetchEvmQuery(evmApiUrl, 'liquidations', {
          symbol: normalizeSymbol(asString(args.symbol)),
          limit: clampInt(args.limit, 1, 500, 50)
        })
    ),
    ro(
      'get_top_traders',
      'Top addresses ranked by a chosen metric (notional, volume, PnL, or trade count) ' +
        'for a symbol or platform-wide. Paginated via cursor. Drives the Top Traders panel ' +
        'and Whale Leaderboard.',
      {
        type: 'object',
        properties: {
          symbol: {
            type: 'string',
            description: 'Perp symbol: base tick (BTC) or full form; omit for platform-wide.'
          },
          sort_by: {
            type: 'string',
            enum: [
              'notional',
              'volume_24h',
              'volume_7d',
              'volume_30d',
              'pnl_24h',
              'pnl_7d',
              'pnl_30d',
              'trade_count_24h'
            ],
            description: 'Ranking metric (default notional).'
          },
          min_notional: { type: 'number', description: 'Minimum notional filter (default 0).' },
          limit: { type: 'number', description: 'Page size (default 50, max 200).' },
          cursor: { type: 'string', description: 'Pagination cursor from a previous response.' }
        },
        additionalProperties: false
      },
      (args) =>
        fetchEvmQuery(evmApiUrl, 'topAddresses', {
          symbol: normalizeSymbol(asString(args.symbol)) || undefined,
          sort_by: asString(args.sort_by) ?? 'notional',
          min_notional: clampInt(args.min_notional, 0, Number.MAX_SAFE_INTEGER, 0),
          limit: clampInt(args.limit, 1, 200, 50),
          cursor: asString(args.cursor) || undefined
        })
    ),
    ro(
      'get_whale_context',
      'Whale research bundle for a single address: account state (collateral, PnL), open ' +
        'positions, and recent trades. Requires an address; optionally scope to a broker or ' +
        'account_id. Drives the Whale Detail modal.',
      {
        type: 'object',
        properties: {
          address: { type: 'string', description: 'Wallet address (EVM 0x…).' },
          broker_id: { type: 'string', description: 'Broker to scope to (optional).' },
          account_id: { type: 'string', description: 'Account ID to scope to (optional).' },
          recent_trades_limit: { type: 'number', description: 'Max trades (default 20, max 100).' }
        },
        required: ['address'],
        additionalProperties: false
      },
      (args) =>
        fetchEvmQuery(evmApiUrl, 'whaleContext', {
          address: asString(args.address),
          broker_id: asString(args.broker_id),
          account_id: asString(args.account_id),
          recent_trades_limit: clampInt(args.recent_trades_limit, 1, 100, 20)
        })
    ),
    ro(
      'get_platform_positions',
      'Platform-wide open positions for a symbol (large-position liquidation-heatmap data). ' +
        'Returns up to 1000 positions filtered by minimum notional. Drives the Liquidation ' +
        'Heatmap.',
      {
        type: 'object',
        properties: {
          symbol: { type: 'string', description: 'Perp symbol: base tick (BTC) or full form.' },
          min_notional: { type: 'number', description: 'Minimum position notional (default 0).' }
        },
        required: ['symbol'],
        additionalProperties: false
      },
      (args) =>
        fetchEvmQuery(evmApiUrl, 'platformPositions', {
          symbol: normalizeSymbol(asString(args.symbol)),
          min_notional: String(clampInt(args.min_notional, 0, Number.MAX_SAFE_INTEGER, 0)),
          limit: 1000
        })
    ),
    ro(
      'get_insurance_fund',
      'Orderly insurance fund balance, holdings, and status. Drives the Insurance Fund ' +
        'widget.',
      { type: 'object', properties: {}, additionalProperties: false },
      () => fetchEvmGet(evmApiUrl, '/v1/public/insurancefund')
    )
  ];
}
