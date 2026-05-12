const compactUsd = new Intl.NumberFormat('en', {
  style: 'currency',
  currency: 'USD',
  notation: 'compact',
  compactDisplay: 'short',
  maximumFractionDigits: 2
});

const numFmt = new Intl.NumberFormat('en', {
  notation: 'compact',
  compactDisplay: 'short',
  maximumFractionDigits: 1
});

const numPreciseFmt = new Intl.NumberFormat('en', {
  notation: 'compact',
  compactDisplay: 'short',
  maximumFractionDigits: 2
});

const pctRawFmt = new Intl.NumberFormat('en', {
  minimumFractionDigits: 2,
  maximumFractionDigits: 2
});

const wholeFmt = new Intl.NumberFormat('en', { maximumFractionDigits: 0 });

export function fmtWhole(n: number | undefined | null): string {
  if (n == null || isNaN(n)) return '—';
  return wholeFmt.format(n);
}

export function fmtCompact(n: number): string {
  return compactUsd.format(n);
}

export function fmtUsd(n: number | undefined | null): string {
  if (n == null || isNaN(n)) return '—';
  return compactUsd.format(n);
}

export function fmtNum(n: number | undefined | null): string {
  if (n == null || isNaN(n)) return '—';
  return numFmt.format(n);
}

export function fmtNumPrecise(n: number | undefined | null): string {
  if (n == null || isNaN(n)) return '—';
  return numPreciseFmt.format(n);
}

export function fmtPct(n: number | undefined | null): string {
  if (n == null || isNaN(n)) return '—';
  return `${n > 0 ? '+' : ''}${n.toFixed(1)}%`;
}

export function fmtPctOfSupply(n: number | undefined | null): string {
  if (n == null || isNaN(n)) return '—';
  return `${pctRawFmt.format(n * 100)}% staked`;
}

export function fmtDeltaPct(current: number, previous: number): number | undefined {
  if (!previous) return undefined;
  return ((current - previous) / Math.abs(previous)) * 100;
}

export function labelFromDate(dateStr: string): string {
  return new Intl.DateTimeFormat('en', { month: 'short', day: 'numeric' }).format(
    new Date(dateStr)
  );
}

export function weekLabel(dateStr: string | undefined): string {
  if (!dateStr) return '';
  const d = new Date(dateStr);
  if (isNaN(d.getTime())) return dateStr;
  return new Intl.DateTimeFormat('en', { month: 'short', day: 'numeric' }).format(d);
}

export function monthLabel(dateStr: string | undefined): string {
  if (!dateStr) return '';
  const d = new Date(dateStr);
  if (isNaN(d.getTime())) return dateStr;
  return new Intl.DateTimeFormat('en', { month: 'short', year: 'numeric' }).format(d);
}

export function capitalize(s: string): string {
  return s.charAt(0).toUpperCase() + s.slice(1);
}
