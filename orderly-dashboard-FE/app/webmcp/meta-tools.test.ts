import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('~/services/orderly', () => ({
  fetchEvmGet: vi.fn(),
  fetchEvmQuery: vi.fn(),
  fetchDataApi: vi.fn(),
  fetchJson: vi.fn(),
  fetchQueryGet: vi.fn()
}));

import { createMetaTools } from './meta-tools';

import { fetchEvmGet, fetchQueryGet } from '~/services/orderly';

const ctx = { evmApiUrl: 'https://evm.test', queryServiceUrl: 'https://q.test' };

function getTool(name: string) {
  return createMetaTools(ctx).find((t) => t.name === name)!;
}

describe('createMetaTools', () => {
  beforeEach(() => {
    vi.mocked(fetchEvmGet).mockResolvedValue({ rows: [{ broker_id: 'b', broker_name: 'B' }] });
    vi.mocked(fetchQueryGet).mockResolvedValue({
      rows: [{ symbol: 'PERP_BTC_USDC', symbol_hash: '0x' }]
    });
  });

  it('registers 3 tools, all read-only', () => {
    const tools = createMetaTools(ctx);
    expect(tools).toHaveLength(3);
    expect(tools.every((t) => t.annotations?.readOnlyHint === true)).toBe(true);
  });

  it('get_brokers unwraps and sorts by name', async () => {
    vi.mocked(fetchEvmGet).mockResolvedValue({
      rows: [
        { broker_id: 'b2', broker_name: 'Zeta' },
        { broker_id: 'b1', broker_name: 'Alpha' }
      ]
    });
    await expect(getTool('get_brokers').execute({})).resolves.toEqual([
      { broker_id: 'b1', broker_name: 'Alpha' },
      { broker_id: 'b2', broker_name: 'Zeta' }
    ]);
    expect(fetchEvmGet).toHaveBeenCalledWith('https://evm.test', '/v1/public/broker/name');
  });

  it('get_all_symbols reads query-service /symbols', async () => {
    vi.mocked(fetchQueryGet).mockResolvedValue({
      rows: [{ symbol: 'PERP_BTC_USDC', symbol_hash: '0xh' }]
    });
    await expect(getTool('get_all_symbols').execute({})).resolves.toEqual([
      { symbol: 'PERP_BTC_USDC', symbol_hash: '0xh' }
    ]);
    expect(fetchQueryGet).toHaveBeenCalledWith('https://q.test', '/symbols');
  });

  it('get_all_tokens reads query-service /tokens', async () => {
    vi.mocked(fetchQueryGet).mockResolvedValue({
      rows: [{ token: 'USDC', token_hash: '0xt' }]
    });
    await expect(getTool('get_all_tokens').execute({})).resolves.toEqual([
      { token: 'USDC', token_hash: '0xt' }
    ]);
    expect(fetchQueryGet).toHaveBeenCalledWith('https://q.test', '/tokens');
  });
});
