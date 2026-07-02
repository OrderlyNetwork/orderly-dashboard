import { type FC, type ReactNode, useMemo, useState } from 'react';

export type Period = '7D' | '30D' | '90D';

export type Granularity = 'weekly' | 'monthly';

export const PERIOD_DAYS: Record<Period, number> = { '7D': 7, '30D': 30, '90D': 90 };

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
        <div
          className="text-lg font-semibold text-white"
          style={{
            fontFamily: "'Atyp BL Text', sans-serif",
            fontFeatureSettings: "'ss02' 1, 'ss03' 1, 'ss05' 1"
          }}
        >
          {title}
        </div>
        {subtitle && (
          <div className="text-[13px] mt-0.5 text-[rgba(255,255,255,0.35)]">{subtitle}</div>
        )}
      </div>
      <div className="flex items-center gap-2">{controls}</div>
    </div>
    <div className="px-4 pt-3 pb-4" style={{ height }}>
      {children}
    </div>
  </div>
);

export const PeriodSelector: FC<{
  period: Period;
  onChange: (p: Period) => void;
  options?: Period[];
}> = ({ period, onChange, options = ['30D', '90D'] }) => (
  <div className="flex gap-1 rounded-lg p-1" style={{ background: '#130E1D' }}>
    {options.map((p) => (
      <button
        key={p}
        onClick={() => onChange(p)}
        className="px-3 py-1 rounded-md border-none cursor-pointer text-[13px] transition-all duration-150"
        style={{
          background: period === p ? '#6700CE' : 'transparent',
          color: period === p ? '#E9DEFF' : 'rgba(255,255,255,0.45)',
          fontWeight: period === p ? 600 : 400
        }}
      >
        {p}
      </button>
    ))}
  </div>
);

export const GranularitySelector: FC<{
  granularity: Granularity;
  onChange: (g: Granularity) => void;
}> = ({ granularity, onChange }) => (
  <div className="flex gap-1 rounded-lg p-1" style={{ background: '#130E1D' }}>
    {(
      [
        ['weekly', '7D'],
        ['monthly', '30D']
      ] as [Granularity, string][]
    ).map(([g, label]) => (
      <button
        key={g}
        onClick={() => onChange(g)}
        className="px-3 py-1 rounded-md border-none cursor-pointer text-[13px] transition-all duration-150"
        style={{
          background: granularity === g ? '#6700CE' : 'transparent',
          color: granularity === g ? '#E9DEFF' : 'rgba(255,255,255,0.45)',
          fontWeight: granularity === g ? 600 : 400
        }}
      >
        {label}
      </button>
    ))}
  </div>
);

function statTextScheme(): { label: string; value: string } {
  return { label: 'rgba(0,0,0,0.55)', value: '#000000' };
}

export const StatCard: FC<{
  label: string;
  value: string;
  sub?: string;
  color?: string;
  selected?: boolean;
  onClick?: () => void;
  inStrip?: boolean;
  pill?: boolean;
}> = ({ label, value, color = '#9C75FF', selected, onClick, inStrip, pill }) => {
  const { label: labelColor, value: valueColor } = statTextScheme();

  if (inStrip) {
    return (
      <div
        role={onClick ? 'button' : undefined}
        tabIndex={onClick ? 0 : undefined}
        onClick={onClick}
        onKeyDown={onClick ? (e) => e.key === 'Enter' && onClick() : undefined}
        className="flex-1 flex flex-row items-center justify-between min-w-0 px-8"
        style={{
          background: selected ? color : '#221E30',
          borderRadius: 9999,
          paddingTop: 20,
          paddingBottom: 20,
          cursor: onClick ? 'pointer' : undefined,
          transition: 'opacity .15s'
        }}
        onMouseEnter={
          onClick && !selected
            ? (e) => {
                (e.currentTarget as HTMLElement).style.opacity = '0.9';
              }
            : undefined
        }
        onMouseLeave={
          onClick && !selected
            ? (e) => {
                (e.currentTarget as HTMLElement).style.opacity = '1';
              }
            : undefined
        }
      >
        {/* left: label */}
        <div
          className="text-xs font-semibold uppercase tracking-[0.08em] min-w-0"
          style={{ color: selected ? labelColor : 'rgba(255,255,255,0.45)' }}
        >
          {label}
        </div>
        {/* right: value + sub */}
        {selected && (
          <div className="flex flex-col items-end shrink-0 ml-4">
            <div className="text-[28px] font-bold leading-none" style={{ color: valueColor }}>
              {value}
            </div>
          </div>
        )}
      </div>
    );
  }

  return (
    <div
      role={onClick ? 'button' : undefined}
      tabIndex={onClick ? 0 : undefined}
      onClick={onClick}
      onKeyDown={onClick ? (e) => e.key === 'Enter' && onClick() : undefined}
      className={`${pill ? 'rounded-full' : 'rounded-xl'} py-[10px] flex flex-col min-w-0 flex-1 overflow-hidden`}
      style={{
        paddingLeft: 'clamp(16px, 5cqw, 40px)',
        paddingRight: 'clamp(16px, 5cqw, 40px)',
        containerType: 'inline-size',
        background: selected ? color : '#221E30',
        transform: selected ? 'scale(1)' : 'scale(0.97)',
        cursor: onClick ? 'pointer' : undefined,
        transition: 'background .15s, transform .15s'
      }}
      onMouseEnter={
        onClick && !selected
          ? (e) => {
              (e.currentTarget as HTMLElement).style.transform = 'scale(0.99)';
            }
          : undefined
      }
      onMouseLeave={
        onClick && !selected
          ? (e) => {
              (e.currentTarget as HTMLElement).style.transform = 'scale(0.97)';
            }
          : undefined
      }
    >
      <div
        className="font-semibold uppercase tracking-[0.08em] mb-0.5 whitespace-nowrap"
        style={{
          color: selected ? labelColor : 'rgba(255,255,255,0.45)',
          fontSize: 'clamp(8px, 2.8cqw, 11px)'
        }}
      >
        {label}
      </div>
      <div
        className="font-bold"
        style={{
          color: selected ? valueColor : 'rgba(255,255,255,0.85)',
          fontSize: 'clamp(22px, 13cqw, 48px)'
        }}
      >
        {value}
      </div>
    </div>
  );
};

export const Skeleton: FC<{ height?: number }> = ({ height = 40 }) => (
  <div
    className="skeleton-shimmer rounded-lg"
    style={{ height, background: 'rgba(156,117,255,0.07)' }}
  />
);

/** Placeholder that mimics a stacked/grouped bar chart with a legend chip row. */
export const BarChartSkeleton: FC<{ height?: number; bars?: number }> = ({
  height = 236,
  bars = 14
}) => (
  <div className="flex flex-col" style={{ height }} aria-hidden>
    <div className="flex gap-1.5 mb-3">
      {Array.from({ length: 4 }).map((_, i) => (
        <div
          key={i}
          className="skeleton-shimmer rounded-full"
          style={{ width: 54, height: 18, background: 'rgba(156,117,255,0.08)' }}
        />
      ))}
    </div>
    <div className="flex items-end gap-1.5 flex-1">
      {Array.from({ length: bars }).map((_, i) => {
        const pct = 30 + Math.abs(Math.sin(i * 1.3)) * 55;
        return (
          <div
            key={i}
            className="skeleton-shimmer flex-1 rounded-t"
            style={{ height: `${pct}%`, background: 'rgba(156,117,255,0.10)' }}
          />
        );
      })}
    </div>
  </div>
);

/** Placeholder that mimics a line/area chart. */
export const LineChartSkeleton: FC<{ height?: number }> = ({ height = 236 }) => (
  <div className="relative" style={{ height }} aria-hidden>
    <svg width="100%" height="100%" preserveAspectRatio="none" viewBox="0 0 100 100">
      <defs>
        <linearGradient id="sk-line-fill" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="rgba(156,117,255,0.14)" />
          <stop offset="100%" stopColor="rgba(156,117,255,0)" />
        </linearGradient>
      </defs>
      <path
        d="M0,72 C12,60 20,48 30,52 C42,57 50,30 62,34 C74,38 82,18 100,24 L100,100 L0,100 Z"
        fill="url(#sk-line-fill)"
      />
      <path
        className="skeleton-shimmer"
        d="M0,72 C12,60 20,48 30,52 C42,57 50,30 62,34 C74,38 82,18 100,24"
        fill="none"
        stroke="rgba(156,117,255,0.30)"
        strokeWidth="1.5"
        vectorEffect="non-scaling-stroke"
      />
    </svg>
  </div>
);

/** Placeholder that mimics a table. */
export const TableSkeleton: FC<{ rows?: number; height?: number }> = ({ rows = 6, height }) => (
  <div className="flex flex-col" style={{ height }} aria-hidden>
    {Array.from({ length: rows }).map((_, i) => (
      <div
        key={i}
        className="skeleton-shimmer flex items-center gap-4"
        style={{ padding: '12px 20px', borderBottom: '1px solid rgba(255,255,255,0.04)' }}
      >
        <div
          className="skeleton-shimmer rounded-md"
          style={{ width: '24%', height: 12, background: 'rgba(156,117,255,0.08)' }}
        />
        <div
          className="skeleton-shimmer rounded-md"
          style={{ width: '18%', height: 12, background: 'rgba(156,117,255,0.08)' }}
        />
        <div
          className="skeleton-shimmer rounded-md"
          style={{ width: '20%', height: 12, background: 'rgba(156,117,255,0.08)' }}
        />
      </div>
    ))}
  </div>
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
  <div className="text-xs font-semibold text-[rgba(255,255,255,0.3)] uppercase tracking-widest mb-3">
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

export const DatasetChips: FC<{
  items: Array<{ label: string; color: string; visible: boolean }>;
  onToggle: (index: number) => void;
  maxHeight?: number;
  onSelectAll?: () => void;
  onDeselectAll?: () => void;
  hideSearch?: boolean;
}> = ({ items, onToggle, maxHeight, onSelectAll, onDeselectAll, hideSearch }) => {
  const [search, setSearch] = useState('');
  const [sortOrder, setSortOrder] = useState<'default' | 'az'>('default');

  const displayItems = useMemo(() => {
    let result = items.map((item, i) => ({ ...item, originalIndex: i }));
    if (search)
      result = result.filter((item) => item.label.toLowerCase().includes(search.toLowerCase()));
    if (sortOrder === 'az') result = [...result].sort((a, b) => a.label.localeCompare(b.label));
    return result;
  }, [items, search, sortOrder]);

  return (
    <div>
      <div className="flex items-center gap-1.5 mb-1.5 flex-wrap">
        {onSelectAll && (
          <button
            onClick={onSelectAll}
            className="px-2 py-0.5 rounded text-xs cursor-pointer transition-all duration-150"
            style={{ background: '#221E30', border: 'none', color: '#ffffff' }}
          >
            Select all
          </button>
        )}
        {onDeselectAll && (
          <button
            onClick={onDeselectAll}
            className="px-2 py-0.5 rounded text-xs cursor-pointer transition-all duration-150"
            style={{ background: '#221E30', border: 'none', color: '#ffffff' }}
          >
            Deselect all
          </button>
        )}
        <div className="flex gap-0.5 rounded-md p-0.5" style={{ background: '#130E1D' }}>
          {(['default', 'az'] as const).map((s) => (
            <button
              key={s}
              onClick={() => setSortOrder(s)}
              className="px-2 py-0.5 rounded text-xs cursor-pointer transition-all duration-150"
              style={{
                background: sortOrder === s ? '#6700CE' : 'transparent',
                border: 'none',
                color: sortOrder === s ? '#E9DEFF' : 'rgba(255,255,255,0.45)',
                fontWeight: sortOrder === s ? 600 : 400
              }}
            >
              {s === 'default' ? 'Qty' : 'A–Z'}
            </button>
          ))}
        </div>
        {!hideSearch && (
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Search…"
            className="rounded text-xs px-2 py-0.5 outline-none"
            style={{
              width: 110,
              background: '#130E1D',
              border: 'none',
              color: '#ffffff'
            }}
          />
        )}
      </div>
      <div
        className={maxHeight ? 'chips-scrollbar' : undefined}
        style={{
          maxHeight,
          overflowY: maxHeight ? 'auto' : undefined,
          scrollbarWidth: maxHeight ? 'thin' : undefined,
          scrollbarColor: maxHeight ? 'rgba(156,117,255,0.25) #130E1D' : undefined
        }}
      >
        <div className="flex flex-wrap gap-1.5 mb-2">
          {displayItems.map((item) => (
            <button
              key={item.label}
              onClick={() => onToggle(item.originalIndex)}
              className="flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs cursor-pointer transition-all duration-150"
              style={{
                background: item.visible ? `${item.color}18` : '#130E1D',
                border: 'none',
                color: '#ffffff',
                textDecoration: item.visible ? 'none' : 'line-through'
              }}
            >
              <span
                className="inline-block rounded-full shrink-0"
                style={{
                  width: 8,
                  height: 8,
                  background: item.visible ? item.color : 'rgba(255,255,255,0.15)'
                }}
              />
              {item.label}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
};

export const TH: React.CSSProperties = {
  padding: '10px 16px',
  textAlign: 'left',
  color: 'rgba(255,255,255,0.3)',
  fontWeight: 600,
  fontSize: 12,
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
  top: 0,
  left: 0,
  zIndex: 3,
  background: 'rgba(20,15,35,.95)'
};

export const tdSticky = (rowIdx: number): React.CSSProperties => ({
  ...TD,
  position: 'sticky',
  left: 0,
  zIndex: 1,
  background: rowIdx % 2 === 0 ? 'rgba(20,15,35,.95)' : 'rgba(22,17,40,.95)'
});
