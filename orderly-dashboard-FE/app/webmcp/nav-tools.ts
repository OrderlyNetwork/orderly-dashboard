// Page-steering "action" tools. Unlike the read-only data tools, these navigate
// the SPA and drive widget UI so an agent and the user share one view — the
// WebMCP spec's headline collaborative use case.
//
// Navigation is dispatched as a `orderly:nav` CustomEvent consumed by
// <NavBridge /> (which calls the Remix router), preserving shared SPA context
// (no full reload). Focus/share do direct DOM ops.

import { action, asString, ro } from './tools';
import { WIDGET_TOOL_MAP } from './widget-tools';

import { encodeAddress, SOL_REGEX } from '~/util';

// ── pure helpers (unit-tested) ───────────────────────────────────────────────

/**
 * Convert a symbol argument into the URL slug used by /markets/:symbol.
 *
 * Accepts a base tick ("BTC"), the full form ("PERP_BTC_USDC"), or a
 * broker-suffixed variant ("PERP_AAPL_USDC_mythos"). Returns `BASE` or
 * `BASE_broker`, matching the parser in routes/markets_.$symbol.tsx.
 */
export function toMarketSlug(input: string): string {
  const s = input.trim();
  if (!s) throw new Error('symbol is required.');
  const m = s.match(/^PERP_(.+)_USDC(?:_(.+))?$/i);
  if (m) {
    const base = m[1];
    const broker = m[2];
    return broker ? `${base}_${broker}` : base;
  }
  return s.toUpperCase();
}

/**
 * Validate that `a` looks like an EVM address, an Orderly account_id, or a
 * Solana base58 address. Used by `open_address` before encoding.
 */
export function isValidAddressShape(a: string): boolean {
  const s = a.trim();
  if (!s) return false;
  // EVM address: 0x + 40 hex
  if (/^0x[0-9a-fA-F]{40}$/.test(s)) return true;
  // Orderly account_id: 0x + 64 hex
  if (/^0x[0-9a-fA-F]{64}$/.test(s)) return true;
  // Solana base58
  if (SOL_REGEX.test(s)) return true;
  return false;
}

// ── DOM helpers ──────────────────────────────────────────────────────────────

function findWidgetEl(widgetId: string): HTMLElement | null {
  if (typeof document === 'undefined') return null;
  return document.querySelector(`[data-widget-id="${cssEscape(widgetId)}"]`);
}

// Escape a value for safe interpolation into a CSS attribute selector.
function cssEscape(value: string): string {
  if (typeof CSS !== 'undefined' && typeof CSS.escape === 'function') {
    return CSS.escape(value);
  }
  // Fallback: escape any non-alphanumeric as a hex escape sequence.
  return value.replace(/[^a-zA-Z0-9_-]/g, (ch) => `\\${ch}`);
}

function dispatchNav(path: string): void {
  if (typeof window === 'undefined') return;
  window.dispatchEvent(new CustomEvent('orderly:nav', { detail: { path } }));
}

/**
 * Resolve `true` once the SPA route reaches `targetPath`, or `false` on timeout.
 * Polls `window.location.pathname` (~50ms) after a `dispatchNav`. This makes the
 * action tools truthful: by the time they resolve, the route has actually
 * changed (or the tool reports it did not). In a non-browser environment the
 * promise resolves `true` immediately so the tools stay callable server-side.
 */
export function waitForNav(targetPath: string, timeoutMs = 1500): Promise<boolean> {
  if (typeof window === 'undefined') return Promise.resolve(true);
  // Executor form: the project's TS lib/target is ES2022, which predates
  // Promise.withResolvers (ES2024). tsc does not polyfill runtime methods, so the
  // constructor is the baseline-safe choice here.
  return new Promise<boolean>((resolve) => {
    const start = Date.now();
    const tick = () => {
      if (window.location.pathname === targetPath) return resolve(true);
      if (Date.now() - start >= timeoutMs) return resolve(false);
      setTimeout(tick, 50);
    };
    tick();
  });
}

export function waitForPageStatus(timeoutMs = 3000): Promise<string> {
  if (typeof document === 'undefined') return Promise.resolve('ready');
  return new Promise<string>((resolve) => {
    const start = Date.now();
    const tick = () => {
      const el = document.querySelector('[data-page-status]');
      const status = el?.getAttribute('data-page-status') ?? 'ready';
      if (status !== 'loading') return resolve(status);
      if (Date.now() - start >= timeoutMs) return resolve(status);
      setTimeout(tick, 50);
    };
    tick();
  });
}

// ── tools ────────────────────────────────────────────────────────────────────

export function createNavTools() {
  // Deferred to function body (not module load) to avoid a circular-import
  // hazard: widget-tools → tools → nav-tools → widget-tools.
  const WIDGET_IDS = Object.keys(WIDGET_TOOL_MAP);

  return [
    action(
      'open_market',
      'Navigate the user to a market detail page. Pass a base tick (BTC), the full ' +
        'form (PERP_BTC_USDC), or a broker-suffixed variant (PERP_AAPL_USDC_mythos). ' +
        'The page renders a graceful "Market Not Found" for unknown symbols, so no ' +
        'pre-validation is performed — the user lands on the right place regardless.',
      {
        type: 'object',
        properties: {
          symbol: { type: 'string', description: 'Market symbol (e.g. BTC, ETH, PERP_BTC_USDC).' }
        },
        required: ['symbol'],
        additionalProperties: false
      },
      async (args) => {
        const symbol = toMarketSlug(String(args.symbol ?? ''));
        const path = `/markets/${symbol}`;
        dispatchNav(path);
        const navigated = await waitForNav(path);
        const url = typeof location !== 'undefined' ? location.origin + path : path;
        const result: Record<string, unknown> = {
          ok: navigated,
          navigated,
          action: 'open_market',
          path,
          url,
          symbol
        };
        if (!navigated) {
          result.reason = 'navigation did not take effect within timeout';
          return result;
        }
        const status = await waitForPageStatus();
        result.page_status = status;
        if (status === 'not-found') {
          result.note = `Page loaded with status "${status}" — the symbol may not exist.`;
        } else {
          const summary = document
            .querySelector('[data-page-summary]')
            ?.getAttribute('data-page-summary');
          if (summary) result.summary = summary;
        }
        return result;
      },
      'Open Market'
    ),

    action(
      'open_address',
      'Navigate the user to an address / account detail page. Accepts an EVM address ' +
        '(0x + 40 hex), an Orderly account_id (0x + 64 hex), or a Solana base58 address. ' +
        'Solana addresses are url-safe-base64 encoded to match in-app AddressLink links.',
      {
        type: 'object',
        properties: {
          address: {
            type: 'string',
            description: 'EVM address, Solana address, or Orderly account_id.'
          }
        },
        required: ['address'],
        additionalProperties: false
      },
      async (args) => {
        const raw = String(args.address ?? '').trim();
        if (!isValidAddressShape(raw)) {
          throw new Error(
            'Invalid address: expected an EVM address, Solana address, or Orderly account_id.'
          );
        }
        const encoded = encodeAddress(raw);
        const path = `/address/${encoded}`;
        dispatchNav(path);
        const navigated = await waitForNav(path);
        const url = typeof location !== 'undefined' ? location.origin + path : path;
        const result: Record<string, unknown> = {
          ok: navigated,
          navigated,
          action: 'open_address',
          path,
          url
        };
        if (!navigated) {
          result.reason = 'navigation did not take effect within timeout';
          return result;
        }
        const status = await waitForPageStatus();
        result.page_status = status;
        const summary = document
          .querySelector('[data-page-summary]')
          ?.getAttribute('data-page-summary');
        if (summary) result.summary = summary;
        return result;
      },
      'Open Address'
    ),

    action(
      'focus_widget',
      "Scroll a dashboard widget into the user's view and highlight it briefly. " +
        'Only widgets present on the current page can be focused (widgets render on ' +
        'specific pages, not everywhere). Use get_site_overview to learn which widgets ' +
        'live where.',
      {
        type: 'object',
        properties: {
          widget_id: { type: 'string', enum: WIDGET_IDS, description: 'Widget id to focus.' }
        },
        required: ['widget_id'],
        additionalProperties: false
      },
      async (args) => {
        const widgetId = String(args.widget_id ?? '');
        if (typeof document === 'undefined') {
          return { ok: false, found: false, reason: 'Not running in a browser.' };
        }
        const el = findWidgetEl(widgetId);
        if (!el) {
          return {
            ok: false,
            found: false,
            hint:
              'No such widget on the current page — widgets exist only on the page that ' +
              'renders them. Known ids: ' +
              WIDGET_IDS.join(', ')
          };
        }
        el.scrollIntoView({ behavior: 'smooth', block: 'center' });
        const prev = el.style.outline;
        el.style.outline = '3px solid #9C75FF';
        setTimeout(() => {
          el.style.outline = prev;
        }, 1500);
        return { ok: true, found: true, widget_id: widgetId };
      },
      'Focus Widget'
    ),

    action(
      'share_widget',
      'Open the share dialog for a dashboard widget, presenting the user with the ' +
        "widget's embed/share link. Only works when the widget and its share button " +
        'are rendered on the current page (share buttons are hidden on embeds).',
      {
        type: 'object',
        properties: {
          widget_id: { type: 'string', enum: WIDGET_IDS, description: 'Widget id to share.' }
        },
        required: ['widget_id'],
        additionalProperties: false
      },
      async (args) => {
        const widgetId = String(args.widget_id ?? '');
        if (typeof document === 'undefined') {
          return { ok: false, reason: 'Not running in a browser.' };
        }
        const el = findWidgetEl(widgetId);
        if (!el) {
          return {
            ok: false,
            found: false,
            hint: 'No such widget on the current page. Known ids: ' + WIDGET_IDS.join(', ')
          };
        }
        const btn = el.querySelector('[title="Share widget"]') as HTMLButtonElement | null;
        if (!btn) {
          return { ok: false, reason: 'share button not rendered (e.g. on an embed)' };
        }
        btn.click();
        return { ok: true, note: 'Opened the share dialog for the user.', widget_id: widgetId };
      },
      'Share Widget'
    ),

    ro(
      'get_current_view',
      'Read-only snapshot of what the user is currently looking at: the current route/URL, ' +
        'the page area, the active market symbol (base tick) when on a market page, and the ' +
        'list of widget ids currently rendered on screen (valid inputs for focus_widget / ' +
        "share_widget / get_widget_url). Call this to ground yourself in the user's current " +
        'view before navigating or pointing at a widget. After open_market or open_address, ' +
        'call this to confirm the destination rendered — the result includes a status ' +
        '(loading | ready | not-found | error) and, when available, a short summary.',
      { type: 'object', properties: {}, required: [], additionalProperties: false },
      async () => {
        if (typeof window === 'undefined') {
          return { path: '', url: '', page: 'Unknown', widgets: [] };
        }
        const { pathname, search, href } = window.location;
        const path = pathname + search;
        let page = 'Unknown';
        let symbol: string | undefined;
        let address: string | undefined;
        let widgetId: string | undefined;
        if (pathname === '/') {
          page = 'Dashboard';
        } else if (pathname === '/markets') {
          page = 'Markets';
        } else if (pathname.startsWith('/markets/')) {
          page = 'Market Detail';
          const slug = decodeURIComponent(pathname.slice('/markets/'.length));
          symbol = slug.split('_')[0].toUpperCase();
        } else if (pathname === '/leaderboard') {
          page = 'Leaderboard';
        } else if (pathname === '/explorer' || pathname === '/search') {
          page = 'Search';
        } else if (pathname.startsWith('/address/')) {
          page = 'Address';
          address = decodeURIComponent(pathname.slice('/address/'.length));
        } else if (pathname.startsWith('/widget/')) {
          page = 'Widget Embed';
          widgetId = decodeURIComponent(pathname.slice('/widget/'.length));
        }
        const seen = new Set<string>();
        const widgets: string[] = [];
        if (typeof document !== 'undefined') {
          document.querySelectorAll('[data-widget-id]').forEach((el) => {
            const id = el.getAttribute('data-widget-id');
            if (id && !seen.has(id)) {
              seen.add(id);
              widgets.push(id);
            }
          });
        }
        let status = 'ready';
        let summary: string | undefined;
        if (typeof document !== 'undefined') {
          const statusEl = document.querySelector('[data-page-status]');
          if (statusEl) status = statusEl.getAttribute('data-page-status') ?? 'ready';
          const t = document
            .querySelector('[data-page-summary]')
            ?.getAttribute('data-page-summary');
          if (t) summary = t;
        }
        const result: Record<string, unknown> = { path, url: href, page, widgets, status };
        if (symbol) result.symbol = symbol;
        if (address) result.address = address;
        if (widgetId) result.widget_id = widgetId;
        if (summary) result.summary = summary;
        return result;
      },
      'Current View'
    ),

    ro(
      'get_widget_url',
      'Return the embeddable share URL for a dashboard widget — the same link the Share dialog ' +
        'produces — without opening any UI. Use this to give the user a link to a widget. Market ' +
        'widgets (market-*) take an optional symbol.',
      {
        type: 'object',
        properties: {
          widget_id: {
            type: 'string',
            enum: WIDGET_IDS,
            description: 'Widget id to get a link for.'
          },
          symbol: {
            type: 'string',
            description: 'Symbol for market widgets (e.g. BTC). Ignored for non-market widgets.'
          }
        },
        required: ['widget_id'],
        additionalProperties: false
      },
      async (args) => {
        const widgetId = String(args.widget_id ?? '');
        const origin = typeof location !== 'undefined' ? location.origin : '';
        const sym = asString(args.symbol);
        const query = sym ? `?embed=true&symbol=${encodeURIComponent(sym)}` : '?embed=true';
        const url = `${origin}/widget/${widgetId}${query}`;
        return {
          widget_id: widgetId,
          url,
          iframe: `<iframe src="${url}" width="800" height="400" frameborder="0" allow="tools"></iframe>`
        };
      },
      'Widget URL'
    )
  ];
}
