import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('~/services/orderly', () => ({
  fetchEvmGet: vi.fn(),
  fetchEvmQuery: vi.fn(),
  fetchDataApi: vi.fn(),
  fetchJson: vi.fn(),
  fetchQueryGet: vi.fn()
}));

import { createMarketTools } from './market-tools';

import { fetchEvmGet, fetchEvmQuery } from '~/services/orderly';

const ctx = { evmApiUrl: 'https://evm.test', queryServiceUrl: 'https://q.test' };

function getTool(name: string) {
  return createMarketTools(ctx).find((t) => t.name === name)!;
}

describe('createMarketTools', () => {
  beforeEach(() => {
    vi.mocked(fetchEvmGet).mockResolvedValue({ ok: true });
    vi.mocked(fetchEvmQuery).mockResolvedValue({ ok: true });
  });

  it('registers 10 tools, all read-only', () => {
    const tools = createMarketTools(ctx);
    expect(tools).toHaveLength(10);
    expect(tools.every((t) => t.annotations?.readOnlyHint === true)).toBe(true);
  });

  it('get_markets fans out 3 GETs, normalises units, and defaults to USD-volume order', async () => {
    const mk = (symbol: string, base: number, usd: number, oi: number, mark: number) => ({
      symbol,
      '24h_volume': base,
      '24h_amount': usd,
      open_interest: oi,
      mark_price: mark
    });
    vi.mocked(fetchEvmGet)
      .mockResolvedValueOnce({
        rows: [
          mk('PERP_BTC_USDC', 47, 3_675_098, 31.13, 81_093),
          mk('PERP_ETH_USDC', 3_158, 7_977_912, 11_891, 2_633)
        ]
      })
      .mockResolvedValueOnce({
        rows: [
          { symbol: 'PERP_BTC_USDC', last_price: 81_109.9, '24h': 77_542.6 },
          { symbol: 'PERP_ETH_USDC', last_price: 2_631.44, '24h': 2_489.72 }
        ]
      })
      .mockResolvedValueOnce({
        rows: [
          { symbol: 'PERP_BTC_USDC', long_oi: 7, short_oi: 3 },
          { symbol: 'PERP_ETH_USDC', long_oi: 2, short_oi: 1 }
        ]
      });
    const res = await getTool('get_markets').execute({});
    expect(fetchEvmGet).toHaveBeenCalledTimes(3);
    expect(fetchEvmGet).toHaveBeenNthCalledWith(1, 'https://evm.test', '/v1/public/futures_market');
    expect(fetchEvmGet).toHaveBeenNthCalledWith(
      2,
      'https://evm.test',
      '/v1/public/market_info/price_changes'
    );
    expect(fetchEvmGet).toHaveBeenNthCalledWith(
      3,
      'https://evm.test',
      '/v1/public/market_info/traders_open_interests'
    );
    // No args → full set, envelopes unwrapped, unit-explicit volume/OI fields, lookback
    // prices turned into percent changes, and ranked by USD volume (ETH outranks BTC
    // despite a much smaller base-token quantity).
    expect(res).toEqual({
      markets: [
        {
          symbol: 'PERP_ETH_USDC',
          mark_price: 2_633,
          volume_24h_base: 3_158,
          volume_24h_usd: 7_977_912,
          open_interest_base: 11_891,
          open_interest_usd: 11_891 * 2_633
        },
        {
          symbol: 'PERP_BTC_USDC',
          mark_price: 81_093,
          volume_24h_base: 47,
          volume_24h_usd: 3_675_098,
          open_interest_base: 31.13,
          open_interest_usd: 31.13 * 81_093
        }
      ],
      priceChanges: [
        {
          symbol: 'PERP_BTC_USDC',
          last_price: 81_109.9,
          change_5m_pct: null,
          change_30m_pct: null,
          change_1h_pct: null,
          change_4h_pct: null,
          change_24h_pct: ((81_109.9 - 77_542.6) / 77_542.6) * 100,
          change_3d_pct: null,
          change_7d_pct: null,
          change_30d_pct: null
        },
        {
          symbol: 'PERP_ETH_USDC',
          last_price: 2_631.44,
          change_5m_pct: null,
          change_30m_pct: null,
          change_1h_pct: null,
          change_4h_pct: null,
          change_24h_pct: ((2_631.44 - 2_489.72) / 2_489.72) * 100,
          change_3d_pct: null,
          change_7d_pct: null,
          change_30d_pct: null
        }
      ],
      openInterest: [
        {
          symbol: 'PERP_BTC_USDC',
          long_oi_base: 7,
          short_oi_base: 3,
          long_oi_usd: 7 * 81_093,
          short_oi_usd: 3 * 81_093
        },
        {
          symbol: 'PERP_ETH_USDC',
          long_oi_base: 2,
          short_oi_base: 1,
          long_oi_usd: 2 * 2_633,
          short_oi_usd: 1 * 2_633
        }
      ]
    });
  });

  it('get_markets filters, sorts by USD volume, limits, and trims the side arrays', async () => {
    vi.mocked(fetchEvmGet)
      .mockResolvedValueOnce({
        rows: [
          { symbol: 'PERP_ETH_USDC', '24h_volume': 50, '24h_amount': 5_000 },
          { symbol: 'PERP_BTC_USDC', '24h_volume': 100, '24h_amount': 9_000 },
          { symbol: 'PERP_SOL_USDC', '24h_volume': 30, '24h_amount': 300 },
          { symbol: 'PERP_BTC_USDT', '24h_volume': 10, '24h_amount': 100 }
        ]
      })
      .mockResolvedValueOnce({
        rows: [
          { symbol: 'PERP_BTC_USDC', '24h': 2.5 },
          { symbol: 'PERP_ETH_USDC', '24h': -1.1 },
          { symbol: 'PERP_SOL_USDC', '24h': 0.4 },
          { symbol: 'PERP_BTC_USDT', '24h': 0.1 }
        ]
      })
      .mockResolvedValueOnce({
        rows: [
          { symbol: 'PERP_BTC_USDC', long_oi: 5, short_oi: 5 },
          { symbol: 'PERP_ETH_USDC', long_oi: 1, short_oi: 1 },
          { symbol: 'PERP_SOL_USDC', long_oi: 2, short_oi: 2 },
          { symbol: 'PERP_BTC_USDT', long_oi: 0, short_oi: 0 }
        ]
      });
    const res = (await getTool('get_markets').execute({
      search: 'btc',
      sort_by: 'volume_24h_usd',
      limit: 5
    })) as {
      markets: (Record<string, unknown> & { symbol: string })[];
      priceChanges: { symbol: string }[];
      openInterest: { symbol: string }[];
    };
    // Only symbols containing "btc", ranked by USD volume descending.
    expect(res.markets.map((m) => m.symbol)).toEqual(['PERP_BTC_USDC', 'PERP_BTC_USDT']);
    expect(res.markets.length).toBeLessThanOrEqual(5);
    expect(res.markets.every((m) => m.symbol.toLowerCase().includes('btc'))).toBe(true);
    // Unit-normalised row: USD notional + base-token quantity, raw keys removed.
    expect(res.markets[0]).toMatchObject({ volume_24h_usd: 9_000, volume_24h_base: 100 });
    expect(res.markets[0]).not.toHaveProperty('24h_volume');
    expect(res.markets[0]).not.toHaveProperty('24h_amount');
    // Side arrays trimmed to the same symbol set.
    expect(res.priceChanges.map((r) => r.symbol)).toEqual(['PERP_BTC_USDC', 'PERP_BTC_USDT']);
    expect(res.openInterest.map((r) => r.symbol)).toEqual(['PERP_BTC_USDC', 'PERP_BTC_USDT']);
  });

  it('get_markets asc sort and limit slicing', async () => {
    vi.mocked(fetchEvmGet)
      .mockResolvedValueOnce({
        rows: [
          { symbol: 'PERP_BTC_USDC', '24h_volume': 100, '24h_amount': 9_000 },
          { symbol: 'PERP_ETH_USDC', '24h_volume': 50, '24h_amount': 5_000 },
          { symbol: 'PERP_SOL_USDC', '24h_volume': 30, '24h_amount': 300 }
        ]
      })
      .mockResolvedValueOnce({ rows: [] })
      .mockResolvedValueOnce({ rows: [] });
    const res = (await getTool('get_markets').execute({
      sort_by: 'volume_24h_base',
      desc: false,
      limit: 2
    })) as { markets: { symbol: string }[]; priceChanges: unknown[]; openInterest: unknown[] };
    // Ascending by base-token quantity, then sliced to 2 → SOL(30), ETH(50).
    expect(res.markets.map((m) => m.symbol)).toEqual(['PERP_SOL_USDC', 'PERP_ETH_USDC']);
    expect(res.priceChanges).toEqual([]);
    expect(res.openInterest).toEqual([]);
  });

  it('get_markets ranks by percent change, not the raw lookback price', async () => {
    // SYN is cheap and up 30%; BTC is expensive and up 4.65%. Ranking on the raw
    // '24h' lookback price would put BTC first; the percent metric must not.
    const rows = [
      {
        symbol: 'PERP_BTC_USDC',
        '24h_volume': 47,
        '24h_amount': 3_675_098,
        open_interest: 31.13,
        mark_price: 81_093
      },
      {
        symbol: 'PERP_SYN_USDC',
        '24h_volume': 1_000,
        '24h_amount': 500_000,
        open_interest: 1_000_000,
        mark_price: 0.183
      }
    ];
    const changes = [
      { symbol: 'PERP_BTC_USDC', last_price: 81_109.9, '24h': 77_542.6 },
      { symbol: 'PERP_SYN_USDC', last_price: 0.183, '24h': 0.1408 }
    ];
    const run = async (sortBy: string) => {
      vi.mocked(fetchEvmGet)
        .mockResolvedValueOnce({ rows })
        .mockResolvedValueOnce({ rows: changes })
        .mockResolvedValueOnce({ rows: [] });
      return (await getTool('get_markets').execute({ sort_by: sortBy })) as {
        markets: { symbol: string }[];
        priceChanges: { symbol: string; change_24h_pct: number | null }[];
      };
    };
    const res = await run('change_24h_pct');
    expect(res.markets.map((m) => m.symbol)).toEqual(['PERP_SYN_USDC', 'PERP_BTC_USDC']);
    // Percent changes are computed from last_price vs the lookback price and surfaced
    // on the priceChanges rows (markets carry volume/OI only).
    const syn = res.priceChanges.find((r) => r.symbol === 'PERP_SYN_USDC')!;
    const btc = res.priceChanges.find((r) => r.symbol === 'PERP_BTC_USDC')!;
    expect(syn.change_24h_pct).toBeCloseTo(((0.183 - 0.1408) / 0.1408) * 100, 6);
    expect(btc.change_24h_pct).toBeCloseTo(((81_109.9 - 77_542.6) / 77_542.6) * 100, 6);
  });

  it('get_markets ranks open interest by notional or base quantity', async () => {
    const rows = [
      // ETH: small base OI, huge USD OI. PUMP: huge base OI, tiny USD OI.
      {
        symbol: 'PERP_ETH_USDC',
        '24h_volume': 3_158,
        '24h_amount': 7_977_912,
        open_interest: 11_891,
        mark_price: 2_633
      },
      {
        symbol: 'PERP_PUMP_USDC',
        '24h_volume': 9_587_400,
        '24h_amount': 39_433,
        open_interest: 9_587_400,
        mark_price: 0.0041
      }
    ];
    const run = async (sortBy: string) => {
      vi.mocked(fetchEvmGet)
        .mockResolvedValueOnce({ rows })
        .mockResolvedValueOnce({ rows: [] })
        .mockResolvedValueOnce({ rows: [] });
      return (await getTool('get_markets').execute({ sort_by: sortBy })) as {
        markets: { symbol: string }[];
      };
    };
    expect((await run('open_interest_usd')).markets.map((m) => m.symbol)).toEqual([
      'PERP_ETH_USDC',
      'PERP_PUMP_USDC'
    ]);
    expect((await run('open_interest_base')).markets.map((m) => m.symbol)).toEqual([
      'PERP_PUMP_USDC',
      'PERP_ETH_USDC'
    ]);
  });

  it('get_markets ranks by base volume only when asked; USD is the default metric', async () => {
    const rows = [
      // BTC trades less base-token quantity than PENGU but far more USD notional.
      { symbol: 'PERP_BTC_USDC', '24h_volume': 47, '24h_amount': 3_675_098 },
      { symbol: 'PERP_PENGU_USDC', '24h_volume': 130_389_350, '24h_amount': 981_436 }
    ];
    const run = async (sortBy?: string) => {
      vi.mocked(fetchEvmGet)
        .mockResolvedValueOnce({ rows })
        .mockResolvedValueOnce({ rows: [] })
        .mockResolvedValueOnce({ rows: [] });
      return (await getTool('get_markets').execute(sortBy ? { sort_by: sortBy } : {})) as {
        markets: { symbol: string }[];
      };
    };
    expect((await run()).markets.map((m) => m.symbol)).toEqual([
      'PERP_BTC_USDC',
      'PERP_PENGU_USDC'
    ]);
    expect((await run('volume_24h_usd')).markets.map((m) => m.symbol)).toEqual([
      'PERP_BTC_USDC',
      'PERP_PENGU_USDC'
    ]);
    expect((await run('volume_24h_base')).markets.map((m) => m.symbol)).toEqual([
      'PERP_PENGU_USDC',
      'PERP_BTC_USDC'
    ]);
  });

  it('get_markets exposes unit-explicit sort keys and null-safe normalised fields', async () => {
    const schema = getTool('get_markets').inputSchema as {
      properties: { sort_by?: { enum?: readonly string[] } };
    };
    expect(schema.properties.sort_by?.enum).toEqual([
      'volume_24h_usd',
      'volume_24h_base',
      'change_24h_pct',
      'open_interest_usd',
      'open_interest_base',
      'symbol'
    ]);

    vi.mocked(fetchEvmGet)
      .mockResolvedValueOnce({
        rows: [{ symbol: 'PERP_BTC_USDC', '24h_volume': null, open_interest: 31.13 }]
      })
      .mockResolvedValueOnce({
        rows: [{ symbol: 'PERP_BTC_USDC', last_price: 81_109.9, '24h': null }]
      })
      .mockResolvedValueOnce({
        rows: [{ symbol: 'PERP_BTC_USDC', long_oi: null, short_oi: 3 }]
      });
    const res = (await getTool('get_markets').execute({})) as {
      markets: {
        volume_24h_base: number | null;
        volume_24h_usd: number | null;
        open_interest_base: number | null;
        open_interest_usd: number | null;
      }[];
      priceChanges: { change_24h_pct: number | null }[];
      openInterest: {
        long_oi_base: number | null;
        long_oi_usd: number | null;
        short_oi_base: number | null;
        short_oi_usd: number | null;
      }[];
    };
    expect(res.markets[0]).toEqual({
      symbol: 'PERP_BTC_USDC',
      volume_24h_base: null,
      volume_24h_usd: null,
      open_interest_base: 31.13,
      open_interest_usd: null
    });
    expect(res.priceChanges[0].change_24h_pct).toBeNull();
    expect(res.openInterest[0]).toEqual({
      symbol: 'PERP_BTC_USDC',
      long_oi_base: null,
      short_oi_base: 3,
      long_oi_usd: null,
      short_oi_usd: null
    });
  });

  it('get_market_detail posts marketDetail with default 1h candles', async () => {
    await getTool('get_market_detail').execute({ symbol: 'PERP_BTC_USDC' });
    expect(fetchEvmQuery).toHaveBeenCalledWith('https://evm.test', 'marketDetail', {
      symbol: 'PERP_BTC_USDC',
      include: ['market_info', 'orderbook', 'recent_trades', 'funding_history', 'candles'],
      orderbook_levels: 50,
      recent_trades_limit: 50,
      funding_history_limit: 100,
      candles_interval: '1h',
      candles_limit: 168
    });
  });

  it('get_market_detail honours a custom candle interval', async () => {
    await getTool('get_market_detail').execute({
      symbol: 'PERP_ETH_USDC',
      candles_interval: '5m'
    });
    expect(vi.mocked(fetchEvmQuery).mock.calls[0][2]).toMatchObject({ candles_interval: '5m' });
  });

  it('get_symbol_info GETs /v1/public/info/{symbol}', async () => {
    await getTool('get_symbol_info').execute({ symbol: 'PERP_ETH_USDC' });
    expect(fetchEvmGet).toHaveBeenCalledWith('https://evm.test', '/v1/public/info/PERP_ETH_USDC');
  });

  it('normalises a bare base tick to the canonical USDC perp', async () => {
    await getTool('get_symbol_info').execute({ symbol: 'btc' });
    expect(fetchEvmGet).toHaveBeenCalledWith('https://evm.test', '/v1/public/info/PERP_BTC_USDC');
    await getTool('get_market_detail').execute({ symbol: 'BTC', include: ['candles'] });
    expect(vi.mocked(fetchEvmQuery).mock.calls[0][2]).toMatchObject({
      symbol: 'PERP_BTC_USDC',
      include: ['candles']
    });
  });

  it('get_market_detail honours a custom include selection', async () => {
    await getTool('get_market_detail').execute({
      symbol: 'PERP_BTC_USDC',
      include: ['market_info', 'orderbook']
    });
    expect(fetchEvmQuery).toHaveBeenCalledWith('https://evm.test', 'marketDetail', {
      symbol: 'PERP_BTC_USDC',
      include: ['market_info', 'orderbook'],
      orderbook_levels: 50,
      recent_trades_limit: 50,
      funding_history_limit: 100,
      candles_interval: '1h',
      candles_limit: 168
    });
  });

  it('get_market_summary posts marketSummary {}', async () => {
    await getTool('get_market_summary').execute({});
    expect(fetchEvmQuery).toHaveBeenCalledWith('https://evm.test', 'marketSummary', {});
  });

  it('get_funding_comparison omits symbol when none given', async () => {
    await getTool('get_funding_comparison').execute({});
    expect(fetchEvmQuery).toHaveBeenCalledWith('https://evm.test', 'fundingComparison', {});
  });

  it('get_funding_comparison passes symbol and unwraps .rows', async () => {
    vi.mocked(fetchEvmQuery).mockResolvedValueOnce({ rows: [{ f: 1 }] });
    await expect(
      getTool('get_funding_comparison').execute({ symbol: 'PERP_BTC_USDC' })
    ).resolves.toEqual([{ f: 1 }]);
    expect(fetchEvmQuery).toHaveBeenCalledWith('https://evm.test', 'fundingComparison', {
      symbol: 'PERP_BTC_USDC'
    });
  });

  it('get_funding_comparison returns payload when no .rows', async () => {
    vi.mocked(fetchEvmQuery).mockResolvedValueOnce({ flat: true });
    await expect(getTool('get_funding_comparison').execute({})).resolves.toEqual({ flat: true });
  });

  it('get_recent_liquidations clamps limit to 500', async () => {
    await getTool('get_recent_liquidations').execute({ symbol: 'PERP_BTC_USDC', limit: 9999 });
    expect(fetchEvmQuery).toHaveBeenCalledWith('https://evm.test', 'liquidations', {
      symbol: 'PERP_BTC_USDC',
      limit: 500
    });
  });

  it('get_top_traders applies defaults', async () => {
    await getTool('get_top_traders').execute({});
    expect(fetchEvmQuery).toHaveBeenCalledWith('https://evm.test', 'topAddresses', {
      symbol: undefined,
      sort_by: 'notional',
      min_notional: 0,
      limit: 50,
      cursor: undefined
    });
  });

  it('get_top_traders forwards symbol, cursor and metric', async () => {
    await getTool('get_top_traders').execute({
      symbol: 'PERP_BTC_USDC',
      sort_by: 'pnl_7d',
      cursor: 'abc'
    });
    expect(vi.mocked(fetchEvmQuery).mock.calls[0][2]).toMatchObject({
      symbol: 'PERP_BTC_USDC',
      sort_by: 'pnl_7d',
      cursor: 'abc'
    });
  });

  it('get_whale_context builds whaleContext params', async () => {
    await getTool('get_whale_context').execute({ address: '0xABC', recent_trades_limit: 5 });
    expect(fetchEvmQuery).toHaveBeenCalledWith('https://evm.test', 'whaleContext', {
      address: '0xABC',
      broker_id: undefined,
      account_id: undefined,
      recent_trades_limit: 5
    });
  });

  it('get_platform_positions stringifies min_notional and caps at 1000', async () => {
    await getTool('get_platform_positions').execute({
      symbol: 'PERP_BTC_USDC',
      min_notional: 250
    });
    expect(fetchEvmQuery).toHaveBeenCalledWith('https://evm.test', 'platformPositions', {
      symbol: 'PERP_BTC_USDC',
      min_notional: '250',
      limit: 1000
    });
  });

  it('get_insurance_fund GETs the insurance fund endpoint', async () => {
    await getTool('get_insurance_fund').execute({});
    expect(fetchEvmGet).toHaveBeenCalledWith('https://evm.test', '/v1/public/insurancefund');
  });
  it('constrains candles_interval to a known enum', () => {
    const schema = getTool('get_market_detail').inputSchema as {
      properties: { candles_interval?: { enum?: readonly string[] } };
    };
    expect(schema.properties.candles_interval?.enum).toEqual(['5m', '15m', '1h', '4h', '1d']);
  });

  it('get_symbol_info throws when the symbol does not exist', async () => {
    vi.mocked(fetchEvmGet).mockResolvedValueOnce(undefined).mockResolvedValueOnce({ rows: [] });
    await expect(getTool('get_symbol_info').execute({ symbol: 'PERP_NOPE_USDC' })).rejects.toThrow(
      /Symbol not found/i
    );
  });

  it('get_symbol_info resolves a broker-suffixed variant via the market list', async () => {
    vi.mocked(fetchEvmGet)
      .mockResolvedValueOnce(undefined) // info for PERP_AAPL_USDC → not found
      .mockResolvedValueOnce({ rows: [{ symbol: 'PERP_AAPL_USDC_mythos' }] }) // market list
      .mockResolvedValueOnce({ symbol: 'PERP_AAPL_USDC_mythos', quote_min: '5' }); // resolved info
    await expect(getTool('get_symbol_info').execute({ symbol: 'AAPL' })).resolves.toMatchObject({
      symbol: 'PERP_AAPL_USDC_mythos'
    });
  });

  it('reports ambiguous broker-suffixed matches', async () => {
    vi.mocked(fetchEvmGet)
      .mockResolvedValueOnce(undefined)
      .mockResolvedValueOnce({
        rows: [{ symbol: 'PERP_BTC_USDC_a' }, { symbol: 'PERP_BTC_USDC_b' }]
      });
    await expect(getTool('get_symbol_info').execute({ symbol: 'BTC' })).rejects.toThrow(
      /Multiple active symbols match/i
    );
  });

  it('get_market_detail throws on an unknown symbol without hitting marketDetail', async () => {
    vi.mocked(fetchEvmGet).mockResolvedValueOnce(undefined).mockResolvedValueOnce({ rows: [] });
    await expect(
      getTool('get_market_detail').execute({ symbol: 'PERP_NOPE_USDC' })
    ).rejects.toThrow(/Symbol not found/i);
    expect(fetchEvmQuery).not.toHaveBeenCalled();
  });
});
