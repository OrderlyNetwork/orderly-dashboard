// WebMCP tool registry. Composes per-domain tool factories into a single array.
// Shared helpers live here so each domain module can reuse them.

export type WebMcpCtx = {
  evmApiUrl: string;
  queryServiceUrl: string;
};

// ── helpers ───────────────────────────────────────────────────────────────────

export function asString(v: unknown): string | undefined {
  return typeof v === 'string' && v !== '' ? v : undefined;
}

export function clampInt(v: unknown, min: number, max: number, fallback: number): number {
  const n = typeof v === 'number' ? v : typeof v === 'string' ? parseInt(v, 10) : NaN;
  if (!Number.isFinite(n)) return fallback;
  return Math.min(max, Math.max(min, Math.trunc(n)));
}

// Normalise a perp symbol argument. The site/URLs use bare base ticks ("BTC") but
// the APIs expect the full form ("PERP_BTC_USDC"). Bare ticks are wrapped into the
// canonical USDC perp; already-full symbols — including broker-suffixed ones such as
// "PERP_AAPL_USDC_mythos" — are returned unchanged so their casing is preserved.
export function normalizeSymbol(input: string | undefined): string | undefined {
  if (!input) return undefined;
  const s = input.trim();
  if (/^PERP_.+_.+$/i.test(s)) return s;
  return `PERP_${s.toUpperCase()}_USDC`;
}

export async function safeCall<T>(fn: () => Promise<T>): Promise<T | { error: string }> {
  try {
    return await fn();
  } catch (e) {
    return { error: e instanceof Error ? e.message : String(e) };
  }
}

// Build `start_date=YYYY-MM-DD&end_date=YYYY-MM-DD` for the trailing `days` days.
export function dateRangeQuery(days: number): string {
  const end = new Date();
  const start = new Date();
  start.setDate(start.getDate() - days);
  const fmt = (d: Date) => d.toISOString().slice(0, 10);
  return `start_date=${fmt(start)}&end_date=${fmt(end)}`;
}

// Derive a human-readable title from a snake_case tool name (spec §4.2.1
// recommends `title` for display in the browser-agent's native UI). Callers may
// pass an explicit title to override.
function titleFromName(name: string): string {
  return name
    .split('_')
    .map((w) => (w ? w[0].toUpperCase() + w.slice(1) : w))
    .join(' ');
}

// Construct a read-only tool. `run` may throw — the error propagates to the agent
// as a tool-call failure (correct MCP semantics) rather than a misleading success
// payload.
export function ro(
  name: string,
  description: string,
  inputSchema: object,
  run: (args: Record<string, unknown>) => Promise<unknown>,
  title?: string
): ModelContextTool {
  return {
    name,
    title: title ?? titleFromName(name),
    description,
    inputSchema,
    annotations: { readOnlyHint: true },
    execute: (args) => run(args)
  };
}

// ── tool factory ──────────────────────────────────────────────────────────────

import { createAccountTools } from './account-tools';
import { createDataApiTools } from './data-tools';
import { createMarketTools } from './market-tools';
import { createMetaTools } from './meta-tools';
import { createSiteTools } from './site-tools';

export function createWebMcpTools(ctx: WebMcpCtx): ModelContextTool[] {
  return [
    ...createSiteTools(),
    ...createDataApiTools(),
    ...createMarketTools(ctx),
    ...createAccountTools(ctx),
    ...createMetaTools(ctx)
  ];
}
