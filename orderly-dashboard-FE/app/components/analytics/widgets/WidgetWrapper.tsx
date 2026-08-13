import { FC, ReactNode } from 'react';

import { WidgetShareButton } from './WidgetShareButton';

type WidgetWrapperProps = {
  widgetId: string;
  title?: string;
  subtitle?: ReactNode;
  controls?: ReactNode;
  height?: number;
  autoHeight?: boolean;
  hideLink?: boolean;
  symbol?: string;
  children: ReactNode;
};

export const WidgetWrapper: FC<WidgetWrapperProps> = ({
  widgetId,
  title,
  subtitle,
  controls,
  height,
  autoHeight,
  hideLink,
  symbol,
  children
}) => {
  const showHeader = title || controls || !hideLink;
  const linkEl = !hideLink ? (
    <WidgetShareButton widgetId={widgetId} title={title} symbol={symbol} />
  ) : null;

  const contentStyle = height
    ? { height, overflow: 'hidden' as const }
    : autoHeight
      ? {}
      : { flex: '1 1 0%', minHeight: 0 };

  return (
    <div
      data-widget-id={widgetId}
      className="rounded-2xl"
      style={{
        display: 'flex',
        flexDirection: 'column',
        minWidth: 0,
        overflow: 'hidden',
        background: 'rgba(20,15,35,.9)',
        border: '1px solid rgba(156,117,255,0.15)'
      }}
    >
      {showHeader && (
        <div
          className="flex items-center justify-between gap-3 py-4 px-5"
          style={{ borderBottom: '1px solid rgba(255,255,255,0.06)' }}
        >
          <div className="min-w-0">
            {title && (
              <div
                className="text-lg font-semibold text-white"
                style={{
                  fontFamily: "'Atyp BL Text', sans-serif",
                  fontFeatureSettings: "'ss02' 1, 'ss03' 1, 'ss05' 1"
                }}
              >
                {title}
              </div>
            )}
            {subtitle && (
              <div className="text-[13px] mt-0.5 text-[rgba(255,255,255,0.35)]">{subtitle}</div>
            )}
          </div>
          <div className="flex items-center gap-2 shrink-0">
            {controls}
            {linkEl}
          </div>
        </div>
      )}

      <div className="pt-4 pb-5 px-5" style={contentStyle}>
        {children}
      </div>
    </div>
  );
};
