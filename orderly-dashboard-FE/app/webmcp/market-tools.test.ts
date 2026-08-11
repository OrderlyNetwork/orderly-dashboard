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

  it('get_markets fans out 3 GETs and combines them', async () => {
    vi.mocked(fetchEvmGet)
      .mockResolvedValueOnce({ markets: 1 })
      .mockResolvedValueOnce({ priceChanges: 2 })
      .mockResolvedValueOnce({ openInterest: 3 });
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
    expect(res).toEqual({
      markets: { markets: 1 },
      priceChanges: { priceChanges: 2 },
      openInterest: { openInterest: 3 }
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
});
