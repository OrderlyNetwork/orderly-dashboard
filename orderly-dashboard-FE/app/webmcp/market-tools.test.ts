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

  it('get_markets fans out 3 GETs and unwraps the row envelopes with no args', async () => {
    const mk = (symbol: string, vol: number) => ({ symbol, '24h_volume': vol });
    vi.mocked(fetchEvmGet)
      .mockResolvedValueOnce({ rows: [mk('PERP_BTC_USDC', 100), mk('PERP_ETH_USDC', 50)] })
      .mockResolvedValueOnce({ rows: [{ symbol: 'PERP_BTC_USDC', '24h': 1.2 }] })
      .mockResolvedValueOnce({ rows: [{ symbol: 'PERP_BTC_USDC', long_oi: 7, short_oi: 3 }] });
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
    // No args → full unfiltered set, envelopes unwrapped to row arrays.
    expect(res).toEqual({
      markets: [mk('PERP_BTC_USDC', 100), mk('PERP_ETH_USDC', 50)],
      priceChanges: [{ symbol: 'PERP_BTC_USDC', '24h': 1.2 }],
      openInterest: [{ symbol: 'PERP_BTC_USDC', long_oi: 7, short_oi: 3 }]
    });
  });

  it('get_markets filters, sorts, limits, and trims the side arrays', async () => {
    vi.mocked(fetchEvmGet)
      .mockResolvedValueOnce({
        rows: [
          { symbol: 'PERP_ETH_USDC', '24h_volume': 50 },
          { symbol: 'PERP_BTC_USDC', '24h_volume': 100 },
          { symbol: 'PERP_SOL_USDC', '24h_volume': 30 },
          { symbol: 'PERP_BTC_USDT', '24h_volume': 10 }
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
      sort_by: '24h_volume',
      limit: 5
    })) as {
      markets: { symbol: string }[];
      priceChanges: { symbol: string }[];
      openInterest: { symbol: string }[];
    };
    // Only symbols containing "btc", sorted by 24h_volume descending.
    expect(res.markets.map((m) => m.symbol)).toEqual(['PERP_BTC_USDC', 'PERP_BTC_USDT']);
    expect(res.markets.length).toBeLessThanOrEqual(5);
    expect(res.markets.every((m) => m.symbol.toLowerCase().includes('btc'))).toBe(true);
    // Side arrays trimmed to the same symbol set.
    expect(res.priceChanges.map((r) => r.symbol)).toEqual(['PERP_BTC_USDC', 'PERP_BTC_USDT']);
    expect(res.openInterest.map((r) => r.symbol)).toEqual(['PERP_BTC_USDC', 'PERP_BTC_USDT']);
  });

  it('get_markets asc sort and limit slicing', async () => {
    vi.mocked(fetchEvmGet)
      .mockResolvedValueOnce({
        rows: [
          { symbol: 'PERP_BTC_USDC', '24h_volume': 100 },
          { symbol: 'PERP_ETH_USDC', '24h_volume': 50 },
          { symbol: 'PERP_SOL_USDC', '24h_volume': 30 }
        ]
      })
      .mockResolvedValueOnce({ rows: [] })
      .mockResolvedValueOnce({ rows: [] });
    const res = (await getTool('get_markets').execute({
      sort_by: '24h_volume',
      desc: false,
      limit: 2
    })) as { markets: { symbol: string }[]; priceChanges: unknown[]; openInterest: unknown[] };
    // Ascending by 24h_volume, then sliced to 2 → SOL(30), ETH(50).
    expect(res.markets.map((m) => m.symbol)).toEqual(['PERP_SOL_USDC', 'PERP_ETH_USDC']);
    expect(res.priceChanges).toEqual([]);
    expect(res.openInterest).toEqual([]);
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
