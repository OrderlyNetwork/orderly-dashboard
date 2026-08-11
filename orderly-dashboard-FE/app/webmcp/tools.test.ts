import { beforeEach, describe, expect, it, vi } from 'vitest';

import { asString, clampInt, dateRangeQuery, normalizeSymbol, ro, safeCall } from './tools';

describe('asString', () => {
  it('returns a non-empty string', () => {
    expect(asString('abc')).toBe('abc');
  });
  it('returns undefined for empty string', () => {
    expect(asString('')).toBeUndefined();
  });
  it('returns undefined for non-strings', () => {
    expect(asString(undefined)).toBeUndefined();
    expect(asString(123)).toBeUndefined();
    expect(asString(null)).toBeUndefined();
    expect(asString(true)).toBeUndefined();
  });
});

describe('clampInt', () => {
  it('passes through in-range numbers', () => {
    expect(clampInt(5, 1, 10, 3)).toBe(5);
  });
  it('clamps below the minimum', () => {
    expect(clampInt(-3, 1, 10, 3)).toBe(1);
  });
  it('clamps above the maximum', () => {
    expect(clampInt(999, 1, 10, 3)).toBe(10);
  });
  it('uses fallback for non-numeric input', () => {
    expect(clampInt(NaN, 1, 10, 3)).toBe(3);
    expect(clampInt(undefined, 1, 10, 3)).toBe(3);
    expect(clampInt('abc', 1, 10, 3)).toBe(3);
  });
  it('parses numeric strings and truncates decimals', () => {
    expect(clampInt('7', 1, 10, 3)).toBe(7);
    expect(clampInt(7.9, 1, 10, 3)).toBe(7);
    expect(clampInt('7.9', 1, 10, 3)).toBe(7);
  });
});

describe('dateRangeQuery', () => {
  beforeEach(() => {
    vi.setSystemTime(new Date('2026-01-15T12:00:00Z'));
  });

  it('produces start_date/end_date in YYYY-MM-DD form', () => {
    expect(dateRangeQuery(7)).toBe('start_date=2026-01-08&end_date=2026-01-15');
  });
  it('start equals end when days is 0', () => {
    expect(dateRangeQuery(0)).toBe('start_date=2026-01-15&end_date=2026-01-15');
  });
});

describe('normalizeSymbol', () => {
  it('wraps a bare base tick into the canonical USDC perp', () => {
    expect(normalizeSymbol('BTC')).toBe('PERP_BTC_USDC');
  });
  it('upper-cases a lowercase base tick', () => {
    expect(normalizeSymbol('eth')).toBe('PERP_ETH_USDC');
  });
  it('returns an already-full symbol unchanged (canonical)', () => {
    expect(normalizeSymbol('PERP_BTC_USDC')).toBe('PERP_BTC_USDC');
  });
  it('preserves broker-suffixed symbols and their casing', () => {
    expect(normalizeSymbol('PERP_AAPL_USDC_mythos')).toBe('PERP_AAPL_USDC_mythos');
  });
  it('returns undefined for empty/undefined input', () => {
    expect(normalizeSymbol(undefined)).toBeUndefined();
    expect(normalizeSymbol('')).toBeUndefined();
  });
});

describe('safeCall', () => {
  it('returns the resolved value on success', async () => {
    await expect(safeCall(() => Promise.resolve(42))).resolves.toBe(42);
  });
  it('returns { error } on failure', async () => {
    await expect(safeCall(() => Promise.reject(new Error('boom')))).resolves.toEqual({
      error: 'boom'
    });
  });
  it('stringifies non-Error throws', async () => {
    await expect(safeCall(() => Promise.reject('nope'))).resolves.toEqual({ error: 'nope' });
  });
});

describe('ro (read-only tool factory)', () => {
  const inputSchema = { type: 'object', properties: {}, additionalProperties: false };

  it('derives a Title Case title from the snake_case name', () => {
    const tool = ro('get_funding_rates', 'desc', inputSchema, () => Promise.resolve());
    expect(tool.name).toBe('get_funding_rates');
    expect(tool.title).toBe('Get Funding Rates');
  });
  it('honours an explicit title override', () => {
    const tool = ro('x', 'desc', inputSchema, () => Promise.resolve(), 'Custom Title');
    expect(tool.title).toBe('Custom Title');
  });
  it('marks the tool read-only', () => {
    const tool = ro('x', 'desc', inputSchema, () => Promise.resolve());
    expect(tool.annotations?.readOnlyHint).toBe(true);
  });
  it('exposes the description and schema', () => {
    const tool = ro('x', 'the description', inputSchema, () => Promise.resolve());
    expect(tool.description).toBe('the description');
    expect(tool.inputSchema).toBe(inputSchema);
  });
  it('execute returns the run result', async () => {
    const tool = ro('x', 'd', inputSchema, () => Promise.resolve({ ok: true }));
    await expect(tool.execute({})).resolves.toEqual({ ok: true });
  });
  it('execute rejects when run rejects', async () => {
    const tool = ro('x', 'd', inputSchema, () => Promise.reject(new Error('fail')));
    await expect(tool.execute({})).rejects.toThrow('fail');
  });
});
