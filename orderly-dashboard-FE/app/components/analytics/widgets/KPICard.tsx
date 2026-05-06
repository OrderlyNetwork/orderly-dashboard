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
      className={`flex flex-col gap-1.5 min-w-0 overflow-hidden transition-colors duration-200 ${flat ? 'rounded-[10px] py-[14px] px-4' : 'rounded-2xl py-5 px-6 backdrop-blur-md'}`}
      style={{
        background: flat ? 'rgba(255,255,255,0.03)' : 'rgba(20,15,35,.9)',
        border: flat ? '1px solid rgba(156,117,255,0.08)' : '1px solid rgba(156,117,255,0.18)',
        backdropFilter: flat ? 'none' : undefined
      }}
    >
      <div className="flex items-center justify-between">
        <span className="text-xs font-semibold text-[rgba(255,255,255,0.5)] uppercase tracking-[0.08em]">
          {label}
        </span>
        {icon && <span className="text-[#9C75FF] opacity-80">{icon}</span>}
      </div>

      <div className="flex items-baseline gap-[10px] min-w-0">
        <span className="text-[26px] font-bold text-white leading-tight tracking-tight min-w-0 shrink">
          {value}
        </span>
        {delta !== undefined && (
          <span
            className="text-xs font-semibold rounded-md py-[2px] px-[7px] shrink-0 whitespace-nowrap"
            style={{
              color: deltaColor,
              background: isPositive ? 'rgba(52,211,153,0.1)' : 'rgba(248,113,113,0.1)'
            }}
          >
            {isPositive ? '+' : ''}
            {delta.toFixed(2)}%
          </span>
        )}
      </div>

      {subValue && <span className="text-xs text-[rgba(255,255,255,0.4)]">{subValue}</span>}
    </div>
  );
};
