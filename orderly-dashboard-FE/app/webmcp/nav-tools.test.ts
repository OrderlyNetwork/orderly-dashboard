import { afterEach, describe, expect, it } from 'vitest';

import { createNavTools, isValidAddressShape, toMarketSlug } from './nav-tools';
import { enrichToolError } from './tools';

const navTools = createNavTools();
const byName = (name: string) => navTools.find((t) => t.name === name)!;

describe('toMarketSlug', () => {
  it('passes a bare base tick through uppercased', () => {
    expect(toMarketSlug('BTC')).toBe('BTC');
    expect(toMarketSlug('eth')).toBe('ETH');
  });

  it('strips the canonical PERP_BASE_USDC form to the base tick', () => {
    expect(toMarketSlug('PERP_BTC_USDC')).toBe('BTC');
  });

  it('keeps a broker suffix as BASE_broker', () => {
    expect(toMarketSlug('PERP_AAPL_USDC_mythos')).toBe('AAPL_mythos');
  });

  it('trims surrounding whitespace', () => {
    expect(toMarketSlug('  BTC  ')).toBe('BTC');
  });

  it('throws on empty input', () => {
    expect(() => toMarketSlug('')).toThrow('symbol is required.');
    expect(() => toMarketSlug('   ')).toThrow('symbol is required.');
  });
});

describe('isValidAddressShape', () => {
  it('accepts an EVM address', () => {
    expect(isValidAddressShape('0x' + 'a'.repeat(40))).toBe(true);
  });

  it('accepts an Orderly account_id (0x + 64 hex)', () => {
    expect(isValidAddressShape('0x' + '1'.repeat(64))).toBe(true);
  });

  it('accepts a Solana base58 address (43-44 chars)', () => {
    expect(isValidAddressShape('1'.repeat(43))).toBe(true);
    expect(isValidAddressShape('1'.repeat(44))).toBe(true);
  });

  it('rejects garbage', () => {
    expect(isValidAddressShape('not an address')).toBe(false);
    expect(isValidAddressShape('0xshort')).toBe(false);
    expect(isValidAddressShape('')).toBe(false);
    expect(isValidAddressShape('0x' + 'g'.repeat(40))).toBe(false); // non-hex
  });

  it('rejects a Solana-length string that is too short', () => {
    expect(isValidAddressShape('1'.repeat(42))).toBe(false);
  });
});

describe('focus_widget', () => {
  const focus = byName('focus_widget');

  afterEach(() => {
    document.body.innerHTML = '';
  });

  it('reports found:false when the widget is absent', async () => {
    const res = (await focus.execute({ widget_id: 'tvl-chain' })) as {
      ok: boolean;
      found: boolean;
    };
    expect(res.ok).toBe(false);
    expect(res.found).toBe(false);
  });

  it('reports found:true and focuses when the widget node exists', async () => {
    const el = document.createElement('div');
    el.setAttribute('data-widget-id', 'tvl-chain');
    document.body.appendChild(el);
    const res = (await focus.execute({ widget_id: 'tvl-chain' })) as {
      ok: boolean;
      found: boolean;
    };
    expect(res.ok).toBe(true);
    expect(res.found).toBe(true);
  });
});

describe('enrichToolError', () => {
  it('rewrites a TypeError("Failed to fetch") into actionable guidance', () => {
    expect(() => enrichToolError(new TypeError('Failed to fetch'))).toThrow(/not a bad parameter/);
  });

  it('rewrites a network-shaped Error message', () => {
    expect(() =>
      enrichToolError(new Error('NetworkError when attempting to fetch resource.'))
    ).toThrow(/not a bad parameter/);
  });

  it('passes through a specific API error unchanged', () => {
    expect(() => enrichToolError(new Error('Symbol not found: X'))).toThrow('Symbol not found: X');
  });

  it('does not name a specific data source or recommend a hardcoded tool list', () => {
    let msg = '';
    try {
      enrichToolError(new TypeError('Failed to fetch'));
    } catch (e) {
      msg = (e as Error).message;
    }
    expect(msg).toContain('not a bad parameter');
    expect(msg).not.toMatch(/Data API/i);
    expect(msg).not.toMatch(/get_markets|get_market_summary|get_site_overview/);
    expect(msg).not.toMatch(/do not depend/);
  });
});
describe('open_market', () => {
  const openMarket = byName('open_market');
  let onNav: ((e: Event) => void) | undefined;

  afterEach(() => {
    if (onNav) {
      window.removeEventListener('orderly:nav', onNav as EventListener);
      onNav = undefined;
    }
    window.location.href = 'https://example.com/';
  });

  it('resolves navigated:true when the route reaches the target path', async () => {
    // Simulate <NavBridge>: honor `orderly:nav` by updating location synchronously.
    onNav = (e: Event) => {
      const path = (e as CustomEvent<{ path: string }>).detail.path;
      window.location.href = 'https://example.com' + path;
    };
    window.addEventListener('orderly:nav', onNav as EventListener);
    const res = (await openMarket.execute({ symbol: 'BTC' })) as Record<string, unknown>;
    expect(res.navigated).toBe(true);
    expect(res.ok).toBe(true);
    expect(res.path).toBe('/markets/BTC');
    expect(res.symbol).toBe('BTC');
    expect(window.location.pathname).toBe('/markets/BTC');
  });

  it('resolves navigated:false with a reason when the route never changes', async () => {
    window.location.href = 'https://example.com/';
    // No NavBridge listener registered → pathname never reaches the target.
    const res = (await openMarket.execute({ symbol: 'ETH' })) as Record<string, unknown>;
    expect(res.navigated).toBe(false);
    expect(res.ok).toBe(false);
    expect(res.reason).toMatch(/did not take effect/i);
    expect(window.location.pathname).toBe('/');
  });
});

describe('get_current_view', () => {
  afterEach(() => {
    document.body.innerHTML = '';
  });

  it('reports the market page, base symbol, and rendered widget ids', async () => {
    window.location.href = 'https://example.com/markets/BTC';
    for (const id of ['market-price-chart', 'market-orderbook']) {
      const el = document.createElement('div');
      el.setAttribute('data-widget-id', id);
      document.body.appendChild(el);
    }
    const res = (await byName('get_current_view').execute({})) as Record<string, unknown>;
    expect(res.page).toBe('Market Detail');
    expect(res.symbol).toBe('BTC');
    expect(res.widgets).toEqual(['market-price-chart', 'market-orderbook']);
    expect(res.path).toBe('/markets/BTC');
    expect(res.status).toBe('ready'); // jsdom: no [data-page-status] element → default
  });

  it('strips a broker suffix down to the base tick', async () => {
    window.location.href = 'https://example.com/markets/AAPL_mythos';
    const res = (await byName('get_current_view').execute({})) as Record<string, unknown>;
    expect(res.page).toBe('Market Detail');
    expect(res.symbol).toBe('AAPL');
  });

  it('derives the dashboard page and no symbol at the root route', async () => {
    window.location.href = 'https://example.com/';
    document.body.innerHTML =
      '<div data-widget-id="tvl-chain"></div><div data-widget-id="tvl-chain"></div>';
    const res = (await byName('get_current_view').execute({})) as Record<string, unknown>;
    expect(res.page).toBe('Dashboard');
    expect(res.symbol).toBeUndefined();
    expect(res.widgets).toEqual(['tvl-chain']); // de-duped
  });

  it('extracts the address segment on /address/:addr', async () => {
    window.location.href = 'https://example.com/address/0x' + 'a'.repeat(40);
    const res = (await byName('get_current_view').execute({})) as Record<string, unknown>;
    expect(res.page).toBe('Address');
    expect(res.address).toBe('0x' + 'a'.repeat(40));
  });

  it('reads back a stamped not-found status with no summary', async () => {
    window.location.href = 'https://example.com/markets/NOPE';
    const el = document.createElement('div');
    el.setAttribute('data-page-status', 'not-found');
    document.body.appendChild(el);
    const res = (await byName('get_current_view').execute({})) as Record<string, unknown>;
    expect(res.status).toBe('not-found');
    expect(res.summary).toBeUndefined();
  });

  it('reads back a stamped ready status and the data-page-summary value', async () => {
    window.location.href = 'https://example.com/markets/ETH';
    const el = document.createElement('div');
    el.setAttribute('data-page-status', 'ready');
    el.setAttribute('data-page-summary', 'ETH · $3,000.5');
    document.body.appendChild(el);
    const res = (await byName('get_current_view').execute({})) as Record<string, unknown>;
    expect(res.status).toBe('ready');
    expect(res.summary).toBe('ETH · $3,000.5');
  });
});

describe('get_widget_url', () => {
  it('builds a plain embed URL without a symbol', async () => {
    const res = (await byName('get_widget_url').execute({ widget_id: 'tvl-chain' })) as {
      url: string;
      iframe: string;
    };
    expect(res.url).toMatch(/\/widget\/tvl-chain\?embed=true$/);
    expect(res.iframe).toContain(res.url);
    expect(res.iframe).toContain('allow="tools"');
  });

  it('appends an encoded symbol for market widgets', async () => {
    const res = (await byName('get_widget_url').execute({
      widget_id: 'market-price-chart',
      symbol: 'PERP_BTC_USDC'
    })) as { url: string };
    expect(res.url).toContain('?embed=true&symbol=PERP_BTC_USDC');
  });
});
