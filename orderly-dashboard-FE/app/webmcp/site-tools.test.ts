import { describe, expect, it, vi } from 'vitest';

vi.mock('~/services/orderly', () => ({
  fetchEvmGet: vi.fn(),
  fetchEvmQuery: vi.fn(),
  fetchDataApi: vi.fn(),
  fetchJson: vi.fn(),
  fetchQueryGet: vi.fn()
}));

import { createSiteTools, siteOverview } from './site-tools';
import { createWebMcpTools } from './tools';

const ctx = { evmApiUrl: 'https://evm.test', queryServiceUrl: 'https://q.test' };

describe('createSiteTools', () => {
  it('registers 1 read-only tool with the expected metadata', () => {
    const [tool] = createSiteTools();
    expect(tool.name).toBe('get_site_overview');
    expect(tool.title).toBe('Site Overview');
    expect(tool.description).toContain('START HERE');
    expect(tool.annotations?.readOnlyHint).toBe(true);
    expect(tool.inputSchema).toEqual({
      type: 'object',
      properties: {},
      required: [],
      additionalProperties: false
    });
  });

  it('returns the site map with all four areas', async () => {
    const [tool] = createSiteTools();
    const ov = (await tool.execute({})) as ReturnType<typeof siteOverview>;
    expect(ov.site).toBe('Orderly Dashboard');
    expect(ov.areas.map((a) => a.name)).toEqual([
      'Dashboard',
      'Markets',
      'Leaderboard',
      'Explorer'
    ]);
    expect(ov.reference_tools.length).toBeGreaterThan(0);
    expect(ov.common_flows.length).toBeGreaterThan(0);
    expect(ov.conventions.length).toBeGreaterThan(0);
    expect(ov.suggested_first_queries.length).toBeGreaterThan(0);
  });

  it('flags this as a non-trading analytics site', async () => {
    const [tool] = createSiteTools();
    const ov = (await tool.execute({})) as ReturnType<typeof siteOverview>;
    expect(ov.what_is_this.toLowerCase()).toContain('does not place trades');
    expect(ov.what_is_this.toLowerCase()).toContain('read-only');
  });

  it('every tool referenced in the overview exists in the full registry (drift guard)', async () => {
    const [tool] = createSiteTools();
    const real = new Set(createWebMcpTools(ctx).map((t) => t.name));
    const mentioned = new Set<string>();
    const text = JSON.stringify(await tool.execute({}));
    for (const m of text.matchAll(/\b(?:get_[a-z_]+|resolve_address)\b/g)) {
      mentioned.add(m[0]);
    }
    const missing = [...mentioned].filter((n) => !real.has(n));
    expect(missing).toEqual([]);
  });

  it('each area lists at least one tool', async () => {
    const [tool] = createSiteTools();
    const ov = (await tool.execute({})) as ReturnType<typeof siteOverview>;
    expect(ov.areas.every((a) => a.tools.length >= 1)).toBe(true);
  });
});
