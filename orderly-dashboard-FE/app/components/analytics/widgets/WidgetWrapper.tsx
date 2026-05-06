import { FC, ReactNode, useState } from 'react';

import { WidgetShareDialog } from './WidgetShareDialog';

type WidgetWrapperProps = {
  widgetId: string;
  title?: string;
  subtitle?: string;
  controls?: ReactNode;
  height?: number;
  hideLink?: boolean;
  children: ReactNode;
};

const ShareIcon = () => (
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
    <circle cx="18" cy="5" r="3" />
    <circle cx="6" cy="12" r="3" />
    <circle cx="18" cy="19" r="3" />
    <line x1="8.59" y1="13.51" x2="15.42" y2="17.49" />
    <line x1="15.41" y1="6.51" x2="8.59" y2="10.49" />
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
  const [shareOpen, setShareOpen] = useState(false);
  const showHeader = title || controls || !hideLink;
  const linkEl = !hideLink ? (
    <>
      <button
        onClick={() => setShareOpen(true)}
        title="Share widget"
        className="flex items-center justify-center w-7 h-7 rounded-lg shrink-0 transition-all duration-150 text-[rgba(255,255,255,0.3)] cursor-pointer"
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
        <ShareIcon />
      </button>
      <WidgetShareDialog
        open={shareOpen}
        onOpenChange={setShareOpen}
        widgetId={widgetId}
        title={title}
      />
    </>
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
