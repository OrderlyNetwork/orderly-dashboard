import { FC, useState } from 'react';

import { WidgetShareDialog } from './WidgetShareDialog';

import { useIsEmbed } from '~/hooks/useIsEmbed';

type WidgetShareButtonProps = {
  widgetId: string;
  title?: string;
  symbol?: string;
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

export const WidgetShareButton: FC<WidgetShareButtonProps> = ({ widgetId, title, symbol }) => {
  const isEmbed = useIsEmbed();
  const [open, setOpen] = useState(false);

  if (isEmbed) return null;

  return (
    <>
      <button
        onClick={() => setOpen(true)}
        title="Share widget"
        className="flex items-center justify-center w-7 h-7 rounded-lg shrink-0 transition-all duration-150 text-[rgba(255,255,255,0.3)] cursor-pointer outline-none focus:outline-none"
        style={{ background: '#221E30', border: 'none' }}
        onMouseEnter={(e) => {
          const el = e.currentTarget as HTMLElement;
          el.style.background = '#2e2840';
          el.style.color = '#9C75FF';
        }}
        onMouseLeave={(e) => {
          const el = e.currentTarget as HTMLElement;
          el.style.background = '#221E30';
          el.style.color = 'rgba(255,255,255,0.3)';
        }}
      >
        <ShareIcon />
      </button>
      <WidgetShareDialog
        open={open}
        onOpenChange={setOpen}
        widgetId={widgetId}
        title={title}
        symbol={symbol}
      />
    </>
  );
};
