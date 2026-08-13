import { FC } from 'react';

import { LiquidationHeatmapWidget } from '../analytics/widgets/LiquidationHeatmapWidget';
import { WidgetShareButton } from '../analytics/widgets/WidgetShareButton';

import { useIsEmbed } from '~/hooks/useIsEmbed';
import { getBaseToken } from '~/hooks/useSymbols';

export type LiquidationHeatmapPanelProps = {
  symbol: string;
};

export const LiquidationHeatmapPanel: FC<LiquidationHeatmapPanelProps> = ({ symbol }) => {
  const isEmbed = useIsEmbed();
  const title = `Liquidation Heatmap${isEmbed ? ` — ${getBaseToken(symbol)}-PERP` : ''}`;
  return (
    <div
      data-widget-id="liquidation-heatmap"
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
          <div className="text-[13px] mt-0.5 text-[rgba(255,255,255,0.35)]">
            Open-position notional at each estimated liquidation price level
          </div>
        </div>
        <WidgetShareButton widgetId="liquidation-heatmap" title={title} symbol={symbol} />
      </div>
      <div className="px-4 pt-3 pb-4">
        <LiquidationHeatmapWidget symbol={symbol} />
      </div>
    </div>
  );
};
