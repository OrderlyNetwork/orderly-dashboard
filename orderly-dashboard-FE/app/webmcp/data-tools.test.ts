import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('~/services/orderly', () => ({
  fetchEvmGet: vi.fn(),
  fetchEvmQuery: vi.fn(),
  fetchDataApi: vi.fn(),
  fetchJson: vi.fn(),
  fetchQueryGet: vi.fn()
}));

vi.mock('~/hooks/useMarketShare', () => ({ fetchMarketShare: vi.fn() }));

import { createDataApiTools } from './data-tools';
import { dateRangeQuery } from './tools';

import { fetchMarketShare } from '~/hooks/useMarketShare';
import { fetchDataApi } from '~/services/orderly';

const NOW = new Date('2026-01-15T12:00:00Z');

function getTool(name: string) {
  return createDataApiTools().find((t) => t.name === name)!;
}

describe('createDataApiTools', () => {
  beforeEach(() => {
    vi.setSystemTime(NOW);
    vi.mocked(fetchDataApi).mockResolvedValue({ rows: [{ r: 1 }] });
  });

  it('registers 19 tools, all read-only', () => {
    const tools = createDataApiTools();
    expect(tools).toHaveLength(19);
    expect(tools.every((t) => t.annotations?.readOnlyHint === true)).toBe(true);
    expect(tools.every((t) => t.title && t.inputSchema && typeof t.execute === 'function')).toBe(
      true
    );
  });

  it('calls the expected DATA API path for each tool', async () => {
    const rq = (days: number) => dateRangeQuery(days);
    const startOnly = rq(30).split('&')[0];
    const cases: Array<[string, Record<string, unknown> | undefined, string | RegExp]> = [
      ['get_dashboard_main', undefined, `/orderly/api/v1/dashboard/orderly/main?${rq(90)}`],
      ['get_tvl_by_chain', undefined, '/orderly/api/v1/dashboard/orderly/tvl-by-chain'],
      [
        'get_tvl_by_token',
        undefined,
        `/orderly/api/v1/dashboard/orderly/by-symbol/daily?symbol_type=token&${rq(30)}`
      ],
      ['get_weekly_symbol_volume', undefined, '/orderly/api/v1/dashboard/orderly/by-symbol/weekly'],
      [
        'get_funding_rates',
        undefined,
        /^\/orderly\/api\/v1\/dashboard\/orderly\/funding-rates\?start_date=\d{4}-\d{2}-\d{2}$/
      ],
      [
        'get_daily_liquidations_by_symbol',
        undefined,
        '/orderly/api/v1/dashboard/orderly/by-symbol/daily?symbol_type=perp'
      ],
      ['get_staking_daily', undefined, '/orderly/api/v1/dashboard/staking/daily'],
      [
        'get_builder_daily',
        undefined,
        `/orderly/api/v1/dashboard/orderly/by-broker?exclude_zero_volume=true&${rq(30)}`
      ],
      [
        'get_fund_flows_by_broker',
        undefined,
        `/orderly/api/v1/dashboard/fund-flows/by-broker?${rq(30)}`
      ],
      ['get_fund_flows_by_chain', undefined, '/orderly/api/v1/dashboard/fund-flows/by-chain'],
      ['get_dex_users', undefined, '/orderly/api/v1/metrics/dex-users'],
      ['get_metrics_overview', undefined, '/orderly/api/v1/metrics/overview'],
      ['get_volume_segments', undefined, '/orderly/api/v1/metrics/volume-segments'],
      ['get_stake_users', undefined, '/orderly/api/v1/metrics/stake-users'],
      ['get_stake_vs_supply', undefined, '/orderly/api/v1/metrics/stake-vs-supply'],
      ['get_omnivault_tvl', undefined, '/orderly/api/v1/metrics/omnivault-tvl'],
      ['get_distributor_stats', undefined, '/orderly/api/v1/distributors/stats'],
      ['get_distributor_invitees', undefined, '/orderly/api/v1/distributors/invitees']
    ];

    for (const [name, args, expected] of cases) {
      await getTool(name).execute(args ?? {});
      if (expected instanceof RegExp) {
        expect(fetchDataApi, `${name} path`).toHaveBeenCalledWith(expect.stringMatching(expected));
      } else {
        expect(fetchDataApi, `${name} path`).toHaveBeenCalledWith(expected);
      }
    }
    // sanity: the start_date-only helper matches the computed value
    expect(startOnly).toMatch(/^start_date=\d{4}-\d{2}-\d{2}$/);
  });

  it('honours a custom `days` window', async () => {
    await getTool('get_dashboard_main').execute({ days: 30 });
    expect(fetchDataApi).toHaveBeenCalledWith(
      `/orderly/api/v1/dashboard/orderly/main?${dateRangeQuery(30)}`
    );
  });

  it('clamps `days` to the 1–730 range', async () => {
    await getTool('get_dashboard_main').execute({ days: 99999 });
    expect(fetchDataApi).toHaveBeenCalledWith(
      `/orderly/api/v1/dashboard/orderly/main?${dateRangeQuery(730)}`
    );
  });

  it('unwraps .rows from the envelope for row-based tools', async () => {
    vi.mocked(fetchDataApi).mockResolvedValue({ rows: [{ a: 1 }, { a: 2 }] });
    await expect(getTool('get_tvl_by_chain').execute({})).resolves.toEqual([{ a: 1 }, { a: 2 }]);
  });

  it('get_dex_users returns data ?? []', async () => {
    vi.mocked(fetchDataApi).mockResolvedValue({ data: [{ d: 1 }] });
    await expect(getTool('get_dex_users').execute({})).resolves.toEqual([{ d: 1 }]);
    vi.mocked(fetchDataApi).mockResolvedValue({});
    await expect(getTool('get_dex_users').execute({})).resolves.toEqual([]);
  });

  it('raw metrics tools return the payload unchanged', async () => {
    vi.mocked(fetchDataApi).mockResolvedValue({ weekly: 1, monthly: 2 });
    await expect(getTool('get_metrics_overview').execute({})).resolves.toEqual({
      weekly: 1,
      monthly: 2
    });
  });

  it('distributor tools return the array payload unchanged', async () => {
    vi.mocked(fetchDataApi).mockResolvedValue([{ id: 'x' }]);
    await expect(getTool('get_distributor_stats').execute({})).resolves.toEqual([{ id: 'x' }]);
  });

  it('get_market_share delegates to fetchMarketShare (no DATA API call)', async () => {
    const share = {
      dexProtocols: [
        {
          name: 'orderly',
          slug: 'orderly',
          logo: '',
          volume24h: 0,
          openInterest: 0,
          marketShare: 1,
          isOrderly: true
        }
      ],
      totalDexVolume: 0,
      totalDexOI: 0,
      orderlyRank: 1,
      lastUpdated: 0
    };
    vi.mocked(fetchMarketShare).mockResolvedValue(share);
    await expect(getTool('get_market_share').execute({})).resolves.toEqual(share);
    expect(fetchDataApi).not.toHaveBeenCalled();
  });
});
