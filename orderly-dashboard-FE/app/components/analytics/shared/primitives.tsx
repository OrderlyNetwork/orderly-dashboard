import type { FC, ReactNode } from 'react';

export type Period = '30D' | '90D';

export const Panel: FC<{
  title: string;
  subtitle?: string;
  height?: number;
  controls?: ReactNode;
  children: ReactNode;
}> = ({ title, subtitle, height = 240, controls, children }) => (
  <div
    className="rounded-2xl overflow-hidden"
    style={{ background: 'rgba(20,15,35,.9)', border: '1px solid rgba(156,117,255,0.15)' }}
  >
    <div
      className="flex items-center justify-between border-b px-5 py-4"
      style={{ borderBottomColor: 'rgba(156,117,255,0.08)' }}
    >
      <div>
        <div className="text-sm font-semibold text-white">{title}</div>
        {subtitle && (
          <div className="text-[11px] mt-0.5 text-[rgba(255,255,255,0.35)]">{subtitle}</div>
        )}
      </div>
      <div className="flex items-center gap-2">{controls}</div>
    </div>
    <div className="px-4 pt-3 pb-4" style={{ height }}>
      {children}
    </div>
  </div>
);

export const PeriodSelector: FC<{ period: Period; onChange: (p: Period) => void }> = ({
  period,
  onChange
}) => (
  <div
    className="flex p-[3px] gap-[2px] rounded-lg"
    style={{ background: 'rgba(156,117,255,0.06)', border: '1px solid rgba(156,117,255,0.15)' }}
  >
    {(['30D', '90D'] as Period[]).map((p) => (
      <button
        key={p}
        onClick={() => onChange(p)}
        className="px-3 py-1 rounded-md border-none cursor-pointer text-xs transition-all duration-150"
        style={{
          background: period === p ? '#9C75FF' : 'transparent',
          color: period === p ? '#fff' : 'rgba(255,255,255,0.45)',
          fontWeight: period === p ? 600 : 400
        }}
      >
        {p}
      </button>
    ))}
  </div>
);

export const StatCard: FC<{
  label: string;
  value: string;
  sub?: string;
  color?: string;
}> = ({ label, value, sub, color = '#9C75FF' }) => (
  <div
    className="rounded-xl py-[14px] px-[18px]"
    style={{
      background: 'rgba(20,15,35,.9)',
      border: `1px solid ${color}22`,
      borderLeft: `3px solid ${color}`
    }}
  >
    <div className="text-[10px] font-semibold text-[rgba(255,255,255,0.38)] uppercase tracking-[0.08em] mb-1.5">
      {label}
    </div>
    <div className="text-xl font-bold text-white">{value}</div>
    {sub && <div className="text-[11px] mt-1 text-[rgba(255,255,255,0.3)]">{sub}</div>}
  </div>
);

export const Skeleton: FC<{ height?: number }> = ({ height = 40 }) => (
  <div className="rounded-lg" style={{ height, background: 'rgba(156,117,255,0.07)' }} />
);

export const Empty: FC<{ msg: string }> = ({ msg }) => (
  <div className="flex items-center justify-center h-full text-[rgba(255,255,255,0.25)] text-[13px] gap-2">
    <svg
      width="15"
      height="15"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
    >
      <circle cx="12" cy="12" r="10" />
      <line x1="12" y1="8" x2="12" y2="12" />
      <line x1="12" y1="16" x2="12.01" y2="16" />
    </svg>
    {msg}
  </div>
);

export const SectionHeading: FC<{ children: ReactNode }> = ({ children }) => (
  <div className="text-[11px] font-semibold text-[rgba(255,255,255,0.3)] uppercase tracking-widest mb-3">
    {children}
  </div>
);

export const TableWrap: FC<{ children: ReactNode }> = ({ children }) => (
  <div
    className="rounded-2xl overflow-x-auto"
    style={{
      background: 'rgba(20,15,35,.9)',
      border: '1px solid rgba(156,117,255,0.15)',
      WebkitOverflowScrolling: 'touch'
    }}
  >
    {children}
  </div>
);

export const TH: React.CSSProperties = {
  padding: '10px 16px',
  textAlign: 'left',
  color: 'rgba(255,255,255,0.3)',
  fontWeight: 600,
  fontSize: 10,
  textTransform: 'uppercase',
  letterSpacing: '0.06em',
  borderBottom: '1px solid rgba(255,255,255,0.07)',
  whiteSpace: 'nowrap'
};

export const TD: React.CSSProperties = {
  padding: '9px 16px',
  borderBottom: '1px solid rgba(255,255,255,0.04)',
  fontSize: 12
};

export const TH_STICKY: React.CSSProperties = {
  ...TH,
  position: 'sticky',
  left: 0,
  zIndex: 2,
  background: 'rgba(20,15,35,.95)'
};

export const tdSticky = (rowIdx: number): React.CSSProperties => ({
  ...TD,
  position: 'sticky',
  left: 0,
  zIndex: 1,
  background: rowIdx % 2 === 0 ? 'rgba(20,15,35,.95)' : 'rgba(22,17,40,.95)'
});
