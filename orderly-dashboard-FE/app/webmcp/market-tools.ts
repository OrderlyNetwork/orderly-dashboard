// WebMCP tools backed by the Orderly EVM API (public market + protocol data).
// Each tool maps to a market/protocol widget's data source. Read-only.

import { asString, clampInt, CANDLE_INTERVALS, normalizeSymbol, ro, type WebMcpCtx } from './tools';

import { fetchEvmGet, fetchEvmQuery } from '~/services/orderly';

export function createMarketTools(ctx: WebMcpCtx): ModelContextTool[] {
  const { evmApiUrl } = ctx;

  // ── symbol resolution (B1 + B3) ────────────────────────────────────────────
  // The public APIs answer a non-existent symbol with `{ success: true }` and
  // null/missing data instead of an error, which would reach an agent as a
  // misleading success. `resolveSymbol` probes /v1/public/info/{symbol}; when that
  // is empty it falls back to the active market list to resolve a broker-suffixed
  // variant (e.g. PERP_AAPL_USDC_mythos) or an alternate quote — and throws a
  // clear, agent-actionable error when the symbol truly does not exist.

  async function safeFetchInfo(s: string): Promise<Record<string, unknown> | null> {
    try {
      const info = await fetchEvmGet<Record<string, unknown> | null>(
        evmApiUrl,
        `/v1/public/info/${s}`
      );
      return info && typeof info === 'object' && Object.keys(info).length > 0 ? info : null;
    } catch {
      return null;
    }
  }

  // Find a single broker-suffixed / alternate-quote variant of `norm` in the active
  // market list. Throws an actionable error on ambiguity or no match.
  async function resolveViaMarketList(input: string | undefined, norm: string): Promise<string> {
    const base = norm.split('_')[1] ?? '';
    const prefix = `PERP_${base.toUpperCase()}_`;
    let candidates: string[] = [];
    try {
      const data = await fetchEvmGet<{ rows?: { symbol?: string }[] }>(
        evmApiUrl,
        '/v1/public/futures_market'
      );
      const rows = data?.rows ?? [];
      candidates = rows
        .map((r) => r?.symbol)
        .filter(
          (s): s is string =>
            !!s && s.toUpperCase() !== norm.toUpperCase() && s.toUpperCase().startsWith(prefix)
        );
    } catch {
      // market list unreachable → fall through to the not-found error
    }
    if (candidates.length === 1) return candidates[0];
    const hint =
      candidates.length > 1
        ? ` Multiple active symbols match: ${candidates.slice(0, 5).join(', ')}` +
          `${candidates.length > 5 ? ` (+${candidates.length - 5} more)` : ''} — pass the full symbol.`
        : ' Call get_markets() to list active symbols.';
    throw new Error(`Symbol not found: ${input ?? ''}.${hint}`);
  }

  // Validate + normalise a required perp symbol, returning the resolved full symbol
  // plus the info payload (reused by get_symbol_info, avoiding a second fetch).
  async function resolveSymbol(
    input: string | undefined
  ): Promise<{ symbol: string; info: Record<string, unknown> | null }> {
    const norm = normalizeSymbol(input);
    if (!norm) throw new Error('symbol is required.');
    const info = await safeFetchInfo(norm);
    if (info) return { symbol: norm, info };
    const resolved = await resolveViaMarketList(input, norm);
    return { symbol: resolved, info: await safeFetchInfo(resolved) };
  }

  return [
    ro(
      'get_markets',
      'Snapshot of all active perpetual markets: the futures market list, 24h price ' +
        'changes, and per-symbol open interest. Drives the Markets page. Returns an ' +
        'object { markets, priceChanges, openInterest }. Each market row reports 24h ' +
        'volume as volume_24h_usd (USD notional) and volume_24h_base (base-token ' +
        'quantity), and open interest as open_interest_usd (notional, oi * mark_price) ' +
        'and open_interest_base (base-token quantity); the raw upstream names ' +
        '24h_amount / 24h_volume / open_interest are not returned. priceChanges rows ' +
        'report percent changes per window (change_5m_pct … change_30d_pct) computed ' +
        'from last_price, not the raw lookback prices; openInterest rows report ' +
        'long_oi_base / short_oi_base plus long_oi_usd / short_oi_usd. Optional search ' +
        '(a case-insensitive substring on the symbol), sort_by (volume_24h_usd | ' +
        'volume_24h_base | change_24h_pct | open_interest_usd | open_interest_base | ' +
        'symbol), limit (max 200), and desc (default true) filter and rank the markets ' +
        'and trim the priceChanges / openInterest arrays to the same symbol set. ' +
        'Defaults to the top 50 markets by USD volume; pass limit: 200 for the full set.',
      {
        type: 'object',
        properties: {
          search: {
            type: 'string',
            description: 'Case-insensitive substring on the symbol (e.g. "btc", "sol", "perp_btc").'
          },
          sort_by: {
            type: 'string',
            enum: [
              'volume_24h_usd',
              'volume_24h_base',
              'change_24h_pct',
              'open_interest_usd',
              'open_interest_base',
              'symbol'
            ],
            description:
              'Rank markets by this metric (default: volume_24h_usd). volume_24h_usd is ' +
              'USD notional; volume_24h_base is base-token quantity; change_24h_pct is the ' +
              'percent price change over 24h; open_interest_usd is OI notional, ' +
              'open_interest_base is OI in base-token quantity.'
          },
          limit: {
            type: 'number',
            description: 'Max markets to return after filter/sort (default 50; max 200).'
          },
          desc: { type: 'boolean', description: 'Sort descending (default true).' }
        },
        required: [],
        additionalProperties: false
      },
      async (args) => {
        const [marketsRaw, priceChangesRaw, openInterestRaw] = await Promise.all([
          fetchEvmGet(evmApiUrl, '/v1/public/futures_market'),
          fetchEvmGet(evmApiUrl, '/v1/public/market_info/price_changes'),
          fetchEvmGet(evmApiUrl, '/v1/public/market_info/traders_open_interests')
        ]);
        // Each endpoint returns either a bare array or the `{ rows: [...] }` EVM
        // envelope; normalise to a plain row array. The API carries numeric-string
        // fields like '24h_volume' / '24h' / 'long_oi' alongside the symbol key.
        type Row = Record<string, unknown> & { symbol: string };
        const asArr = (x: unknown): Row[] => {
          if (x && typeof x === 'object' && 'rows' in x) {
            const rows = (x as { rows?: unknown }).rows;
            if (Array.isArray(rows)) return rows as Row[];
          }
          return Array.isArray(x) ? (x as Row[]) : [];
        };
        let markets = asArr(marketsRaw);
        const priceChanges = asArr(priceChangesRaw);
        const openInterest = asArr(openInterestRaw);

        // Coerce a numeric-string API field; null when missing/not finite.
        const numOrNull = (v: unknown): number | null => {
          if (v === null || v === undefined || v === '') return null;
          const n = Number(v);
          return Number.isFinite(n) ? n : null;
        };
        // Percent change from last_price to the price `window` ago (null when unavailable).
        // The upstream lookback keys ('5m', '1h', '24h', …) hold prices, not changes.
        const pctChange = (r: Row | undefined, window: string): number | null => {
          const last = numOrNull(r?.last_price);
          const prev = numOrNull(r?.[window]);
          if (last === null || prev === null || prev === 0) return null;
          return ((last - prev) / prev) * 100;
        };
        // Open interest notional: base-token quantity * mark price (same as the UI column).
        const oiUsd = (m: Row): number | null => {
          const base = numOrNull(m.open_interest);
          const mark = numOrNull(m.mark_price);
          return base !== null && mark !== null ? base * mark : null;
        };
        const markBySym = new Map<string, number | null>(
          markets.map((m) => [m.symbol, numOrNull(m.mark_price)])
        );

        const search = asString(args.search)?.toLowerCase();
        const sortBy = asString(args.sort_by) ?? 'volume_24h_usd';
        const limit = clampInt(args.limit, 1, 200, 50);
        const desc = args.desc !== false;

        const changeBySym = new Map<string, Row>(
          priceChanges.map((r): [string, Row] => [r.symbol, r])
        );

        if (search) {
          markets = markets.filter((m) => m.symbol.toLowerCase().includes(search));
        }

        // Upstream semantics: '24h_volume' / 'open_interest' are base-token quantities,
        // '24h_amount' is USD notional, and the price_changes lookback keys hold prices.
        // Rank on converted, unit-explicit metrics; an unknown sort_by leaves the list
        // in API order rather than silently ranking by an unintended field.
        const metric: Record<string, (m: Row) => number> = {
          volume_24h_usd: (m) => numOrNull(m['24h_amount']) ?? 0,
          volume_24h_base: (m) => numOrNull(m['24h_volume']) ?? 0,
          change_24h_pct: (m) => pctChange(changeBySym.get(m.symbol), '24h') ?? 0,
          open_interest_usd: (m) => oiUsd(m) ?? 0,
          open_interest_base: (m) => numOrNull(m.open_interest) ?? 0
        };
        if (sortBy === 'symbol') {
          markets.sort((a, b) => a.symbol.localeCompare(b.symbol) * (desc ? -1 : 1));
        } else if (Object.prototype.hasOwnProperty.call(metric, sortBy)) {
          const key = metric[sortBy];
          markets.sort((a, b) => (key(b) - key(a)) * (desc ? 1 : -1));
        }

        if (limit > 0) markets = markets.slice(0, limit);

        const keep = new Set(markets.map((m) => m.symbol));

        // Emit unit-explicit field names; the raw upstream keys are dropped so agents
        // cannot mistake base-token quantities for USD, or lookback prices for changes.
        const normalizedMarkets = markets.map((m) => {
          const { '24h_volume': base, '24h_amount': usd, open_interest: oi, ...rest } = m;
          return {
            ...rest,
            volume_24h_base: numOrNull(base),
            volume_24h_usd: numOrNull(usd),
            open_interest_base: numOrNull(oi),
            open_interest_usd: oiUsd(m)
          };
        });
        const CHANGE_WINDOWS: [string, string][] = [
          ['5m', 'change_5m_pct'],
          ['30m', 'change_30m_pct'],
          ['1h', 'change_1h_pct'],
          ['4h', 'change_4h_pct'],
          ['24h', 'change_24h_pct'],
          ['3d', 'change_3d_pct'],
          ['7d', 'change_7d_pct'],
          ['30d', 'change_30d_pct']
        ];
        const normalizedChanges = priceChanges
          .filter((r) => keep.has(r.symbol))
          .map((r) => {
            const row: Record<string, unknown> = {
              symbol: r.symbol,
              last_price: numOrNull(r.last_price)
            };
            for (const [window, field] of CHANGE_WINDOWS) row[field] = pctChange(r, window);
            return row;
          });
        const normalizedOi = openInterest
          .filter((r) => keep.has(r.symbol))
          .map((r) => {
            const mark = markBySym.get(r.symbol) ?? null;
            const long = numOrNull(r.long_oi);
            const short = numOrNull(r.short_oi);
            return {
              symbol: r.symbol,
              long_oi_base: long,
              short_oi_base: short,
              long_oi_usd: long !== null && mark !== null ? long * mark : null,
              short_oi_usd: short !== null && mark !== null ? short * mark : null
            };
          });

        return {
          markets: normalizedMarkets,
          priceChanges: normalizedChanges,
          openInterest: normalizedOi
        };
      }
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
            enum: [...CANDLE_INTERVALS],
            description: `Candle interval (default '1h'). One of: ${CANDLE_INTERVALS.join(', ')}.`
          }
        },
        required: ['symbol'],
        additionalProperties: false
      },
      async (args) => {
        const include =
          Array.isArray(args.include) && args.include.length
            ? (args.include as string[])
            : ['market_info', 'orderbook', 'recent_trades', 'funding_history', 'candles'];
        const { symbol } = await resolveSymbol(asString(args.symbol));
        return fetchEvmQuery(evmApiUrl, 'marketDetail', {
          symbol,
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
      (args) => resolveSymbol(asString(args.symbol)).then((r) => r.info)
    ),
    ro(
      'get_market_summary',
      'Live platform-wide market snapshot: 24h volume, open interest, trader counts, ' +
        'and fee totals across all symbols. For historical daily time series use ' +
        'get_dashboard_main; for weekly/monthly user aggregates use get_metrics_overview.',
      { type: 'object', properties: {}, required: [], additionalProperties: false },
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
        required: [],
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
      async (args) => {
        const { symbol } = await resolveSymbol(asString(args.symbol));
        return fetchEvmQuery(evmApiUrl, 'liquidations', {
          symbol,
          limit: clampInt(args.limit, 1, 500, 50)
        });
      }
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
          cursor: {
            type: 'string',
            description:
              'Opaque pagination cursor from a previous response — pass it back unchanged.'
          }
        },
        required: [],
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
      async (args) => {
        const { symbol } = await resolveSymbol(asString(args.symbol));
        return fetchEvmQuery(evmApiUrl, 'platformPositions', {
          symbol,
          min_notional: String(clampInt(args.min_notional, 0, Number.MAX_SAFE_INTEGER, 0)),
          limit: 1000
        });
      }
    ),
    ro(
      'get_insurance_fund',
      'Orderly insurance fund balance, holdings, and status. Drives the Insurance Fund ' +
        'widget.',
      { type: 'object', properties: {}, required: [], additionalProperties: false },
      () => fetchEvmGet(evmApiUrl, '/v1/public/insurancefund')
    )
  ];
}
