import { CSSProperties, FC, ReactNode } from 'react';

export type CardSize = 'xl' | 'lg' | 'md' | 'sm';

export type KPICardProps = {
  label: string;
  value: string;
  subValue?: string;
  delta?: number;
  icon?: ReactNode;
  flat?: boolean;
  bgColor?: string;
  size?: CardSize;
  cardStyle?: CSSProperties;
  wrapBadge?: boolean;
};

type TextScheme = { label: string; value: string; sub: string };

function scheme(bg: string): TextScheme {
  const upper = bg.toUpperCase();
  if (upper === '#E9DEFF') return { label: '#000000', value: '#000000', sub: '#000000' };
  if (upper === '#9C75FF') return { label: '#000000', value: '#000000', sub: '#000000' };
  return { label: '#9C75FF', value: '#ffffff', sub: '#9C75FF' };
}

// clamp(min, responsive-cqw, max) — scales with card container width, never wraps
const VALUE_SIZE: Record<CardSize, string> = {
  xl: 'clamp(28px, 42cqw, 90px)',
  lg: 'clamp(22px, 23cqw, 64px)',
  md: 'clamp(20px, 22cqw, 50px)',
  sm: 'clamp(18px, 20cqw, 42px)'
};
const LABEL_SIZE: Record<CardSize, number> = { xl: 10, lg: 10, md: 10, sm: 10 };
const SUB_SIZE: Record<CardSize, number> = { xl: 14, lg: 14, md: 12, sm: 11 };
const BADGE_SIZE: Record<CardSize, number> = { xl: 13, lg: 13, md: 13, sm: 13 };

export const KPICard: FC<KPICardProps> = ({
  label,
  value,
  subValue,
  delta,
  icon,
  bgColor = '#6700CE',
  size = 'md',
  cardStyle,
  wrapBadge = false
}) => {
  const isPositive = delta !== undefined && delta >= 0;
  const { label: labelColor, value: valueColor, sub: subColor } = scheme(bgColor);

  return (
    <div
      className="flex flex-col justify-between rounded-xl px-5 py-5 min-w-0 overflow-hidden kpi-card"
      style={{ background: bgColor, border: 'none', ...cardStyle }}
    >
      {/* label anchored top-left */}
      <div className="flex items-center justify-between shrink-0">
        <span
          className="font-semibold uppercase tracking-[0.1em]"
          style={{ color: labelColor, fontSize: LABEL_SIZE[size] }}
        >
          {label}
        </span>
        {icon && <span style={{ color: labelColor }}>{icon}</span>}
      </div>

      {/* value + subValue grouped at bottom */}
      <div className="min-w-0">
        <div className={`flex items-end gap-2 min-w-0 ${wrapBadge ? 'flex-wrap' : ''}`}>
          <span
            className="font-bold leading-none tracking-tight shrink-0"
            style={{ color: valueColor, fontSize: VALUE_SIZE[size], whiteSpace: 'nowrap' }}
          >
            {value}
          </span>
          {delta !== undefined && (
            <span
              className="font-semibold rounded-md py-1 px-2 shrink-0 whitespace-nowrap"
              style={{
                fontSize: `clamp(9px, 6cqw, ${BADGE_SIZE[size]}px)`,
                color: isPositive ? '#000000' : '#ffffff',
                background: isPositive ? '#00dea3' : '#FF6390'
              }}
            >
              {isPositive ? '+' : ''}
              {delta.toFixed(2)}%
            </span>
          )}
          {subValue && (
            <span
              className="font-medium shrink-0 whitespace-nowrap"
              style={{ color: subColor, fontSize: SUB_SIZE[size] }}
            >
              {subValue}
            </span>
          )}
        </div>
      </div>
    </div>
  );
};
