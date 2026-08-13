import { afterEach, beforeEach, describe, expect, it } from 'vitest';

import { exposeAgentSurface } from './expose';

const ctx = { evmApiUrl: 'https://evm.test', queryServiceUrl: 'https://q.test' };

describe('exposeAgentSurface', () => {
  beforeEach(() => {
    window.orderlyWebMcp = undefined;
  });
  afterEach(() => {
    window.orderlyWebMcp = undefined;
  });

  it('publishes the catalogue on window.orderlyWebMcp without execute callbacks', () => {
    exposeAgentSurface(ctx);
    const surface = window.orderlyWebMcp!;
    expect(surface.toolCount).toBe(surface.tools.length);
    expect(surface.toolCount).toBeGreaterThan(10);
    expect(surface.tools.every((t) => t.name && t.description && t.inputSchema)).toBe(true);
    expect(surface.tools.every((t) => !('execute' in t))).toBe(true);
  });

  it('reports webmcpSupported=false when no modelContext is present', () => {
    exposeAgentSurface(ctx);
    expect(window.orderlyWebMcp!.webmcpSupported).toBe(false);
  });

  it('call() invokes the named tool and rejects unknown names', async () => {
    exposeAgentSurface(ctx);
    await expect(window.orderlyWebMcp!.call('get_site_overview', {})).resolves.toMatchObject({
      site: 'Orderly Dashboard'
    });
    await expect(window.orderlyWebMcp!.call('does_not_exist')).rejects.toThrow(/Unknown tool/);
  });

  it('scopes the catalogue to one widget when widgetId is given', () => {
    exposeAgentSurface(ctx, 'volume');
    const surface = window.orderlyWebMcp!;
    expect(surface.tools.map((t) => t.name)).toEqual(['get_dashboard_main']);
    expect(surface.tools.find((t) => t.name === 'get_site_overview')).toBeUndefined();
  });

  it('scopes to multiple tools for a multi-tool widget', () => {
    exposeAgentSurface(ctx, 'distributors');
    expect(window.orderlyWebMcp!.tools.map((t) => t.name)).toEqual([
      'get_distributor_stats',
      'get_distributor_invitees'
    ]);
  });
});
