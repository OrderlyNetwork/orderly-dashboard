import { Link } from '@remix-run/react';
import { FC, ReactNode } from 'react';

type WidgetWrapperProps = {
  widgetId: string;
  title?: string;
  subtitle?: string;
  controls?: ReactNode;
  height?: number;
  hideLink?: boolean;
  children: ReactNode;
};

const LinkIcon = () => (
  <svg
    width="13"
    height="13"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
    <polyline points="15 3 21 3 21 9" />
    <line x1="10" y1="14" x2="21" y2="3" />
  </svg>
);

export const WidgetWrapper: FC<WidgetWrapperProps> = ({
  widgetId,
  title,
  subtitle,
  controls,
  height,
  hideLink,
  children
}) => {
  const showHeader = title || controls || !hideLink;
  const linkEl = !hideLink ? (
    <Link
      to={`/widget/${widgetId}`}
      title="Open widget"
      className="flex items-center justify-center w-7 h-7 rounded-lg shrink-0 no-underline transition-all duration-150 text-[rgba(255,255,255,0.3)]"
      style={{
        background: 'rgba(156,117,255,0.08)',
        border: '1px solid rgba(156,117,255,0.15)'
      }}
      onMouseEnter={(e) => {
        const el = e.currentTarget as HTMLElement;
        el.style.background = 'rgba(156,117,255,0.2)';
        el.style.color = '#9C75FF';
        el.style.borderColor = 'rgba(156,117,255,0.4)';
      }}
      onMouseLeave={(e) => {
        const el = e.currentTarget as HTMLElement;
        el.style.background = 'rgba(156,117,255,0.08)';
        el.style.color = 'rgba(255,255,255,0.3)';
        el.style.borderColor = 'rgba(156,117,255,0.15)';
      }}
    >
      <LinkIcon />
    </Link>
  ) : null;

  return (
    <div
      className="rounded-2xl overflow-hidden"
      style={{
        background: 'rgba(20,15,35,.9)',
        border: '1px solid rgba(156,117,255,0.15)'
      }}
    >
      {showHeader && (
        <div
          className="flex items-center justify-between gap-3 py-4 px-5"
          style={{ borderBottom: '1px solid rgba(156,117,255,0.08)' }}
        >
          <div className="min-w-0">
            {title && <div className="text-sm font-semibold text-white">{title}</div>}
            {subtitle && (
              <div className="text-[11px] mt-0.5 text-[rgba(255,255,255,0.35)]">{subtitle}</div>
            )}
          </div>
          <div className="flex items-center gap-2 shrink-0">
            {controls}
            {linkEl}
          </div>
        </div>
      )}

      <div className="pt-3 pb-4 px-4" style={{ ...(height ? { height, overflow: 'hidden' } : {}) }}>
        {children}
      </div>
    </div>
  );
};
