import { FC, ReactNode } from 'react';

export type KPICardProps = {
  label: string;
  value: string;
  subValue?: string;
  delta?: number;
  icon?: ReactNode;
  flat?: boolean;
};

export const KPICard: FC<KPICardProps> = ({ label, value, subValue, delta, icon, flat }) => {
  const isPositive = delta !== undefined && delta >= 0;
  const deltaColor = isPositive ? '#34d399' : '#f87171';

  return (
    <div
      style={{
        background: flat ? 'rgba(255,255,255,0.03)' : 'rgba(20,15,35,.9)',
        border: flat ? '1px solid rgba(156,117,255,0.08)' : '1px solid rgba(156,117,255,0.18)',
        borderRadius: flat ? 10 : 16,
        padding: flat ? '14px 16px' : '20px 24px',
        display: 'flex',
        flexDirection: 'column',
        gap: 6,
        backdropFilter: flat ? 'none' : 'blur(12px)',
        transition: 'border-color 0.2s',
        minWidth: 0
      }}
    >
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <span
          style={{
            fontSize: 12,
            fontWeight: 600,
            color: 'rgba(255,255,255,0.5)',
            textTransform: 'uppercase',
            letterSpacing: '0.08em'
          }}
        >
          {label}
        </span>
        {icon && <span style={{ color: '#9C75FF', opacity: 0.8 }}>{icon}</span>}
      </div>

      <div style={{ display: 'flex', alignItems: 'baseline', gap: 10 }}>
        <span
          style={{
            fontSize: 26,
            fontWeight: 700,
            color: '#fff',
            lineHeight: 1.1,
            letterSpacing: '-0.02em'
          }}
        >
          {value}
        </span>
        {delta !== undefined && (
          <span
            style={{
              fontSize: 12,
              fontWeight: 600,
              color: deltaColor,
              background: isPositive ? 'rgba(52,211,153,0.1)' : 'rgba(248,113,113,0.1)',
              borderRadius: 6,
              padding: '2px 7px'
            }}
          >
            {isPositive ? '+' : ''}
            {delta.toFixed(2)}%
          </span>
        )}
      </div>

      {subValue && <span style={{ fontSize: 12, color: 'rgba(255,255,255,0.4)' }}>{subValue}</span>}
    </div>
  );
};
