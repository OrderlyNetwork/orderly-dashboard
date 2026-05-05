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
    style={{
      background: 'rgba(20,15,35,.9)',
      border: '1px solid rgba(156,117,255,0.15)',
      borderRadius: 16,
      overflow: 'hidden'
    }}
  >
    <div
      style={{
        padding: '16px 20px',
        borderBottom: '1px solid rgba(156,117,255,0.08)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between'
      }}
    >
      <div>
        <div style={{ fontSize: 14, fontWeight: 600, color: '#fff' }}>{title}</div>
        {subtitle && (
          <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.35)', marginTop: 2 }}>
            {subtitle}
          </div>
        )}
      </div>
      <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>{controls}</div>
    </div>
    <div style={{ padding: '12px 16px 16px', height }}>{children}</div>
  </div>
);

export const PeriodSelector: FC<{ period: Period; onChange: (p: Period) => void }> = ({
  period,
  onChange
}) => (
  <div
    style={{
      display: 'flex',
      background: 'rgba(156,117,255,0.06)',
      border: '1px solid rgba(156,117,255,0.15)',
      borderRadius: 8,
      padding: 3,
      gap: 2
    }}
  >
    {(['30D', '90D'] as Period[]).map((p) => (
      <button
        key={p}
        onClick={() => onChange(p)}
        style={{
          padding: '4px 12px',
          borderRadius: 6,
          border: 'none',
          background: period === p ? '#9C75FF' : 'transparent',
          color: period === p ? '#fff' : 'rgba(255,255,255,0.45)',
          fontSize: 12,
          fontWeight: period === p ? 600 : 400,
          cursor: 'pointer',
          transition: 'all 0.15s'
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
    style={{
      background: 'rgba(20,15,35,.9)',
      border: `1px solid ${color}22`,
      borderLeft: `3px solid ${color}`,
      borderRadius: 12,
      padding: '14px 18px'
    }}
  >
    <div
      style={{
        fontSize: 10,
        fontWeight: 600,
        color: 'rgba(255,255,255,0.38)',
        textTransform: 'uppercase',
        letterSpacing: '0.08em',
        marginBottom: 6
      }}
    >
      {label}
    </div>
    <div style={{ fontSize: 20, fontWeight: 700, color: '#fff' }}>{value}</div>
    {sub && <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)', marginTop: 4 }}>{sub}</div>}
  </div>
);

export const Skeleton: FC<{ height?: number }> = ({ height = 40 }) => (
  <div style={{ height, borderRadius: 8, background: 'rgba(156,117,255,0.07)' }} />
);

export const Empty: FC<{ msg: string }> = ({ msg }) => (
  <div
    style={{
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      height: '100%',
      color: 'rgba(255,255,255,0.25)',
      fontSize: 13,
      gap: 8
    }}
  >
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
  <div
    style={{
      fontSize: 11,
      fontWeight: 600,
      color: 'rgba(255,255,255,0.3)',
      textTransform: 'uppercase',
      letterSpacing: '0.1em',
      marginBottom: 12
    }}
  >
    {children}
  </div>
);

export const TableWrap: FC<{ children: ReactNode }> = ({ children }) => (
  <div
    style={{
      background: 'rgba(20,15,35,.9)',
      border: '1px solid rgba(156,117,255,0.15)',
      borderRadius: 16,
      overflow: 'hidden'
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
