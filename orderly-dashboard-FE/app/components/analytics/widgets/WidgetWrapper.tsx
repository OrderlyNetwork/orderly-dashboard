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

const linkStyle: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  width: 28,
  height: 28,
  borderRadius: 8,
  background: 'rgba(156,117,255,0.08)',
  border: '1px solid rgba(156,117,255,0.15)',
  color: 'rgba(255,255,255,0.3)',
  textDecoration: 'none',
  transition: 'all 0.15s',
  flexShrink: 0
};

const hoverOn = (e: React.MouseEvent) => {
  const el = e.currentTarget as HTMLElement;
  el.style.background = 'rgba(156,117,255,0.2)';
  el.style.color = '#9C75FF';
  el.style.borderColor = 'rgba(156,117,255,0.4)';
};

const hoverOff = (e: React.MouseEvent) => {
  const el = e.currentTarget as HTMLElement;
  el.style.background = 'rgba(156,117,255,0.08)';
  el.style.color = 'rgba(255,255,255,0.3)';
  el.style.borderColor = 'rgba(156,117,255,0.15)';
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
    <Link to={`/widget/${widgetId}`} title="Open widget" style={linkStyle} onMouseEnter={hoverOn} onMouseLeave={hoverOff}>
      <LinkIcon />
    </Link>
  ) : null;

  return (
    <div
      style={{
        background: 'rgba(20,15,35,.9)',
        border: '1px solid rgba(156,117,255,0.15)',
        borderRadius: 16,
        overflow: 'hidden'
      }}
    >
      {showHeader && (
        <div
          style={{
            padding: '16px 20px',
            borderBottom: '1px solid rgba(156,117,255,0.08)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            gap: 12
          }}
        >
          <div style={{ minWidth: 0 }}>
            {title && <div style={{ fontSize: 14, fontWeight: 600, color: '#fff' }}>{title}</div>}
            {subtitle && (
              <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.35)', marginTop: 2 }}>
                {subtitle}
              </div>
            )}
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: 8, flexShrink: 0 }}>
            {controls}
            {linkEl}
          </div>
        </div>
      )}

      <div
        style={{
          padding: '12px 16px 16px',
          ...(height ? { height, overflow: 'hidden' } : {})
        }}
      >
        {children}
      </div>
    </div>
  );
};
