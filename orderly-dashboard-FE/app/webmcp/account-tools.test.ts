import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('~/services/orderly', () => ({
  fetchEvmGet: vi.fn(),
  fetchEvmQuery: vi.fn(),
  fetchDataApi: vi.fn(),
  fetchJson: vi.fn(),
  fetchQueryGet: vi.fn()
}));

vi.mock('~/hooks/useEvents', () => ({ fetchEvents: vi.fn() }));

import { createAccountTools } from './account-tools';
import { dateRangeQuery } from './tools';

import { fetchEvents } from '~/hooks/useEvents';
import { fetchEvmGet, fetchEvmQuery, fetchQueryGet } from '~/services/orderly';

const ctx = { evmApiUrl: 'https://evm.test', queryServiceUrl: 'https://q.test' };

function getTool(name: string) {
  return createAccountTools(ctx).find((t) => t.name === name)!;
}

describe('createAccountTools', () => {
  beforeEach(() => {
    vi.setSystemTime(new Date('2026-01-15T12:00:00Z'));
    vi.mocked(fetchEvmGet).mockResolvedValue({});
    vi.mocked(fetchEvmQuery).mockResolvedValue({});
    vi.mocked(fetchQueryGet).mockResolvedValue({});
    vi.mocked(fetchEvents).mockResolvedValue({
      events: [],
      nextCursor: null,
      pageSizeLimit: 500,
      tradesCount: 0
    });
  });

  it('registers 6 tools, all read-only', () => {
    const tools = createAccountTools(ctx);
    expect(tools).toHaveLength(6);
    expect(tools.every((t) => t.annotations?.readOnlyHint === true)).toBe(true);
  });

  describe('get_leaderboard', () => {
    it('requires start_date and end_date', async () => {
      await expect(getTool('get_leaderboard').execute({})).rejects.toThrow(
        /start_date and end_date are required/
      );
      expect(fetchEvmGet).not.toHaveBeenCalled();
    });

    it('builds the leaderboard query string in insertion order', async () => {
      await getTool('get_leaderboard').execute({
        start_date: '2026-01-01',
        end_date: '2026-01-15',
        broker_id: 'b1',
        aggregateBy: 'account',
        sort: 'descending_perp_volume',
        page: 2,
        size: 50
      });
      expect(fetchEvmGet).toHaveBeenCalledWith(
        'https://evm.test',
        '/v1/broker/leaderboard/daily?start_date=2026-01-01&end_date=2026-01-15&page=2&size=50&sort=descending_perp_volume&broker_id=b1&aggregateBy=account'
      );
    });

    it('omits unset optional params', async () => {
      await getTool('get_leaderboard').execute({
        start_date: '2026-01-01',
        end_date: '2026-01-15'
      });
      expect(vi.mocked(fetchEvmGet).mock.calls[0][1]).toBe(
        '/v1/broker/leaderboard/daily?start_date=2026-01-01&end_date=2026-01-15'
      );
    });
  });

  describe('get_positions', () => {
    it('builds the ranking query string', async () => {
      await getTool('get_positions').execute({
        symbol: 'PERP_BTC_USDC',
        limit: 10,
        order_by: 'DESC'
      });
      expect(fetchQueryGet).toHaveBeenCalledWith(
        'https://q.test',
        '/ranking/positions?symbol=PERP_BTC_USDC&limit=10&order_by=DESC'
      );
    });

    it('normalises a bare base tick symbol', async () => {
      await getTool('get_positions').execute({ symbol: 'btc' });
      expect(fetchQueryGet).toHaveBeenCalledWith(
        'https://q.test',
        '/ranking/positions?symbol=PERP_BTC_USDC'
      );
    });

    it('yields a bare path when no params are given', async () => {
      await getTool('get_positions').execute({});
      expect(vi.mocked(fetchQueryGet).mock.calls[0][1]).toBe('/ranking/positions?');
    });
  });

  describe('get_events', () => {
    it('requires account_id', async () => {
      await expect(getTool('get_events').execute({})).rejects.toThrow(/account_id is required/);
      expect(fetchEvents).not.toHaveBeenCalled();
    });

    it('forwards params to fetchEvents', async () => {
      const acct = '0x' + 'a'.repeat(64);
      await getTool('get_events').execute({
        account_id: acct,
        event_type: 'LIQUIDATION',
        from_time: 1736000000,
        to_time: 1736100000
      });
      expect(fetchEvents).toHaveBeenCalledTimes(1);
      const [query, baseUrl] = vi.mocked(fetchEvents).mock.calls[0];
      expect(baseUrl).toBe('https://q.test');
      expect(query.account_id).toBe(acct);
      expect(query.event_type).toBe('LIQUIDATION');
      expect(query.from_time?.unix()).toBe(1736000000);
      expect(query.to_time?.unix()).toBe(1736100000);
    });

    it('passes through a cursor object', async () => {
      const cursor = { key: 'k' };
      await getTool('get_events').execute({
        account_id: '0x' + 'a'.repeat(64),
        cursor
      });
      expect(vi.mocked(fetchEvents).mock.calls[0][0].trading_event_next_cursor).toEqual(cursor);
    });
  });

  describe('get_account_state', () => {
    it('posts accountState', async () => {
      await getTool('get_account_state').execute({ address: '0xABC', broker_id: 'b1' });
      expect(fetchEvmQuery).toHaveBeenCalledWith('https://evm.test', 'accountState', {
        address: '0xABC',
        broker_id: 'b1',
        account_id: undefined
      });
    });
  });

  describe('get_portfolio', () => {
    it('posts portfolio with interval 1d and clamped limit', async () => {
      await getTool('get_portfolio').execute({ address: '0xABC', limit: 9999 });
      expect(fetchEvmQuery).toHaveBeenCalledWith(
        'https://evm.test',
        'portfolio',
        expect.objectContaining({ address: '0xABC', interval: '1d', limit: 365 })
      );
    });
  });

  describe('resolve_address', () => {
    const ACCT_ID = '0x' + 'a'.repeat(64);
    const EVM = '0x' + 'b'.repeat(40);

    it('rejects an unrecognized identifier', async () => {
      await expect(getTool('resolve_address').execute({ address: 'garbage' })).rejects.toThrow(
        /Unrecognized identifier/
      );
      expect(fetchEvmGet).not.toHaveBeenCalled();
    });

    it('resolves an account id via /v1/public/account as a single-element array', async () => {
      vi.mocked(fetchEvmGet).mockResolvedValueOnce({
        account_id: ACCT_ID,
        address: EVM,
        broker_id: 'b1'
      });
      await expect(getTool('resolve_address').execute({ address: ACCT_ID })).resolves.toEqual([
        { account_id: ACCT_ID, address: EVM, broker_id: 'b1' }
      ]);
      expect(fetchEvmGet).toHaveBeenCalledWith(
        'https://evm.test',
        `/v1/public/account?account_id=${ACCT_ID}`
      );
      expect(fetchEvmGet).toHaveBeenCalledTimes(1);
    });

    it('enriches an EVM address with 90d volume/pnl, sorted by volume desc', async () => {
      vi.mocked(fetchEvmGet)
        .mockResolvedValueOnce({
          rows: [
            { account_id: 'a1', broker_id: 'b1' },
            { account_id: 'a2', broker_id: 'b2' }
          ]
        })
        .mockResolvedValueOnce({
          rows: [
            { account_id: 'a1', perp_volume: 100, realized_pnl: 5 },
            { account_id: 'a2', perp_volume: 200, realized_pnl: -3 }
          ]
        });
      const res = await getTool('resolve_address').execute({ address: EVM });
      expect(fetchEvmGet).toHaveBeenNthCalledWith(
        1,
        'https://evm.test',
        `/v1/get_all_accounts?address=${EVM}`
      );
      expect(fetchEvmGet).toHaveBeenNthCalledWith(
        2,
        'https://evm.test',
        `/v1/broker/leaderboard/daily?${dateRangeQuery(89)}&page=1&sort=descending_perp_volume&address=${EVM}&aggregateBy=account`
      );
      expect(res).toEqual([
        { account_id: 'a2', broker_id: 'b2', perp_volume: 200, realized_pnl: -3 },
        { account_id: 'a1', broker_id: 'b1', perp_volume: 100, realized_pnl: 5 }
      ]);
    });

    it('sends chain_type=SOL for a Solana-shaped address', async () => {
      const SOL = 'C' + '1'.repeat(43);
      vi.mocked(fetchEvmGet).mockResolvedValueOnce({ rows: [] }).mockResolvedValueOnce({
        rows: []
      });
      await getTool('resolve_address').execute({ address: SOL });
      expect(fetchEvmGet).toHaveBeenNthCalledWith(
        1,
        'https://evm.test',
        `/v1/get_all_accounts?address=${SOL}&chain_type=SOL`
      );
    });

    it('falls back to raw accounts (and warns) when the leaderboard call fails', async () => {
      const warn = vi.spyOn(console, 'warn').mockImplementation(() => {});
      vi.mocked(fetchEvmGet)
        .mockResolvedValueOnce({ rows: [{ account_id: 'a1', broker_id: 'b1' }] })
        .mockRejectedValueOnce(new Error('lb down'));
      await expect(getTool('resolve_address').execute({ address: EVM })).resolves.toEqual([
        { account_id: 'a1', broker_id: 'b1' }
      ]);
      expect(warn).toHaveBeenCalledTimes(1);
    });
  });
});
