import { useState } from 'react';

import { Dialog } from '@radix-ui/themes';

import { CopyBlock } from './CopyBlock';

type WidgetShareDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  widgetId: string;
  title?: string;
};

export const WidgetShareDialog = ({
  open,
  onOpenChange,
  widgetId,
  title
}: WidgetShareDialogProps) => {
  const [closeHovered, setCloseHovered] = useState(false);

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content className="card card-dialog max-w-2xl w-[calc(100vw-2rem)] sm:w-[95vw] max-h-[90vh] mx-auto flex flex-col overflow-hidden" style={{ paddingTop: '3rem' }}>
        <Dialog.Title
          className="text-lg sm:text-xl font-semibold mb-4 flex-shrink-0"
          style={{ color: '#ffffff' }}
        >
          Share Widget{title ? ` — ${title}` : ''}
        </Dialog.Title>

        <p
          style={{
            fontSize: 13,
            color: 'rgba(255,255,255,0.5)',
            marginBottom: 16,
            lineHeight: 1.5
          }}
        >
          Embed this widget on your site using the iframe code below. You can adjust the{' '}
          <code style={{ color: '#9C75FF', fontSize: 12 }}>width</code> and{' '}
          <code style={{ color: '#9C75FF', fontSize: 12 }}>height</code> attributes to fit your
          layout.
        </p>

        <CopyBlock widgetId={widgetId} />

        <div className="flex justify-end gap-2 mt-6 flex-shrink-0">
          <button
            onClick={() => onOpenChange(false)}
            onMouseEnter={() => setCloseHovered(true)}
            onMouseLeave={() => setCloseHovered(false)}
            style={{
              background: closeHovered ? '#7a00ef' : '#6700CE',
              border: 'none',
              borderRadius: 999,
              color: '#fff',
              fontSize: 14,
              fontWeight: 700,
              padding: '10px 28px',
              cursor: 'pointer',
              transition: 'background 0.15s',
              transform: closeHovered ? 'translateY(-1px)' : 'none'
            }}
          >
            Close
          </button>
        </div>
      </Dialog.Content>
    </Dialog.Root>
  );
};
