import { CSSProperties, FC, ReactNode } from 'react';

export type CardSize = 'lg' | 'md' | 'sm';

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
};

type TextScheme = { label: string; value: string; sub: string };

function scheme(bg: string): TextScheme {
  const upper = bg.toUpperCase();
  if (upper === '#E9DEFF') return { label: '#000000', value: '#000000', sub: '#000000' };
  if (upper === '#9C75FF') return { label: '#000000', value: '#000000', sub: '#000000' };
  return { label: '#9C75FF', value: '#ffffff', sub: '#9C75FF' };
}

const VALUE_SIZE: Record<CardSize, number> = { lg: 54, md: 40, sm: 30 };
const LABEL_SIZE: Record<CardSize, number> = { lg: 13, md: 11, sm: 10 };
const SUB_SIZE: Record<CardSize, number> = { lg: 14, md: 12, sm: 11 };
const BADGE_SIZE: Record<CardSize, number> = { lg: 13, md: 12, sm: 11 };

export const KPICard: FC<KPICardProps> = ({
  label,
  value,
  subValue,
  delta,
  icon,
  bgColor = '#6700CE',
  size = 'md',
  cardStyle
}) => {
  const isPositive = delta !== undefined && delta >= 0;
  const { label: labelColor, value: valueColor, sub: subColor } = scheme(bgColor);

  return (
    <div
      className="flex flex-col rounded-xl px-5 py-5 min-w-0 overflow-hidden"
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

      {/* spacer pushes value to bottom */}
      <div className="flex-1" />

      {/* value row at bottom-left */}
      <div className="flex items-end gap-3 min-w-0 flex-wrap">
        <span
          className="font-bold leading-none tracking-tight min-w-0 shrink"
          style={{ color: valueColor, fontSize: VALUE_SIZE[size] }}
        >
          {value}
        </span>
        {delta !== undefined && (
          <span
            className="font-semibold rounded-md py-1 px-2 shrink-0 whitespace-nowrap"
            style={{
              fontSize: BADGE_SIZE[size],
              color: isPositive ? '#000000' : '#ffffff',
              background: isPositive ? '#1DF6B5' : '#FF6390'
            }}
          >
            {isPositive ? '+' : ''}
            {delta.toFixed(2)}%
          </span>
        )}
      </div>

      {subValue && (
        <span
          className="font-medium mt-1.5 shrink-0"
          style={{ color: subColor, fontSize: SUB_SIZE[size] }}
        >
          {subValue}
        </span>
      )}
    </div>
  );
};
