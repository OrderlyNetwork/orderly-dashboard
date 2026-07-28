export type UnrealizedPnlInput = {
  side: 'LONG' | 'SHORT';
  position_qty: string | number | null;
  average_open_price: string | number | null;
  mark_price: string | number | null;
};

const toNum = (v: string | number | null | undefined): number => {
  if (v === null || v === undefined) return NaN;
  const n = typeof v === 'string' ? parseFloat(v) : v;
  return Number.isFinite(n) ? n : NaN;
};

export function computeUnrealizedPnl(pos: UnrealizedPnlInput): number | null {
  const qty = Math.abs(toNum(pos.position_qty));
  const entry = toNum(pos.average_open_price);
  const mark = toNum(pos.mark_price);
  if (![qty, entry, mark].every(Number.isFinite)) return null;
  const diff = mark - entry;
  return pos.side === 'SHORT' ? -diff * qty : diff * qty;
}
