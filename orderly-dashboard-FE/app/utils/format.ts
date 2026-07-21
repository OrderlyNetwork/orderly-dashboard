/**
 * Derive decimal places from a tick size (e.g. 0.1 → 1, 0.001 → 3, 1e-8 → 8).
 * Mirrors the helper used by the market subpage's orderbook component so that
 * price formatting stays consistent across views.
 */
export function tickToDecimals(tick: number): number {
  if (tick >= 1) return 0;
  const str = String(tick);
  if (str.includes('e-')) {
    return parseInt(str.split('e-')[1], 10);
  }
  const dot = str.indexOf('.');
  return dot === -1 ? 0 : str.length - dot - 1;
}

/**
 * Format a price using fixed tick-based precision. Matches the orderbook's
 * `fmtPrice(n, decimals)` behavior: `minimumFractionDigits === maximumFractionDigits`.
 *
 * Pass `tick = null` (or undefined) to fall back to `fallbackDecimals` (default 8,
 * matching OrderbookPanel).
 *
 * Returns `'-'` for null/undefined values.
 */
export function formatPriceByTick(
  value: string | number | null | undefined,
  tick: number | null | undefined,
  fallbackDecimals: number = 8
): string {
  if (value === null || value === undefined) return '-';
  const n = typeof value === 'string' ? parseFloat(value) : value;
  if (!Number.isFinite(n)) return '-';
  const decimals = tick != null ? tickToDecimals(tick) : fallbackDecimals;
  return n.toLocaleString('en', {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals
  });
}
