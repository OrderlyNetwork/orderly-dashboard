import { describe, expect, it, vi } from 'vitest';

vi.mock('~/services/orderly', () => ({
  fetchEvmGet: vi.fn(),
  fetchEvmQuery: vi.fn(),
  fetchDataApi: vi.fn(),
  fetchJson: vi.fn(),
  fetchQueryGet: vi.fn()
}));
vi.mock('~/hooks/useMarketShare', () => ({ fetchMarketShare: vi.fn() }));
vi.mock('~/hooks/useEvents', () => ({ fetchEvents: vi.fn() }));

import { createWidgetTools, WIDGET_TOOL_MAP } from './widget-tools';

const ctx = { evmApiUrl: 'https://evm.test', queryServiceUrl: 'https://q.test' };

describe('WIDGET_TOOL_MAP', () => {
  it('maps every widgetId to a non-empty tool list', () => {
    for (const [id, names] of Object.entries(WIDGET_TOOL_MAP)) {
      expect(names.length, id).toBeGreaterThan(0);
    }
  });

  it('distributors maps to two tools', () => {
    expect(WIDGET_TOOL_MAP['distributors']).toEqual([
      'get_distributor_stats',
      'get_distributor_invitees'
    ]);
  });
});

describe('createWidgetTools', () => {
  it('returns only the mapped tool for a single-tool widget', () => {
    const tools = createWidgetTools(ctx, 'volume');
    expect(tools.map((t) => t.name)).toEqual(['get_dashboard_main']);
  });

  it('returns multiple tools for a multi-tool widget', () => {
    const tools = createWidgetTools(ctx, 'distributors');
    expect(tools.map((t) => t.name)).toEqual(['get_distributor_stats', 'get_distributor_invitees']);
  });

  it('returns [] for an unknown widget', () => {
    expect(createWidgetTools(ctx, 'nope')).toEqual([]);
  });

  it('every mapped widget resolves to real read-only tools', () => {
    for (const id of Object.keys(WIDGET_TOOL_MAP)) {
      const tools = createWidgetTools(ctx, id);
      expect(tools.length, id).toBe(WIDGET_TOOL_MAP[id].length);
      expect(
        tools.every((t) => t.annotations?.readOnlyHint === true),
        id
      ).toBe(true);
    }
  });
});
