import dayjs from 'dayjs';
import { FC } from 'react';

import { fmtUsd } from '~/components/analytics/shared/formatters';
import { TableSkeleton } from '~/components/analytics/shared/primitives';
import { WidgetShareButton } from '~/components/analytics/widgets/WidgetShareButton';
import { useIsEmbed } from '~/hooks/useIsEmbed';
import { useLiquidations, useMarketSummary } from '~/hooks/usePublicInfo';
import { base64UrlSafeEncode, DASHBOARD_ORIGIN } from '~/util';
import { formatPriceByTick } from '~/utils/format';

export type RecentLiquidationsProps = {
  symbol: string;
};

const thBase = 'py-2 px-4 text-xs font-medium text-gray-500 uppercase tracking-wider sticky top-0';
const thStyle = { background: 'rgba(20,15,35,.95)' };

const baseToken = (symbol: string) => symbol.split('_')[1] ?? symbol;

export const RecentLiquidations: FC<RecentLiquidationsProps> = ({ symbol }) => {
  const isEmbed = useIsEmbed();
  const { data, isLoading } = useLiquidations(symbol, 100);
  const { data: marketSummary } = useMarketSummary();

  const quoteTick = (() => {
    const tick = marketSummary?.markets.find((m) => m.symbol === symbol)?.quote_tick;
    const parsed = tick != null ? parseFloat(tick) : NaN;
    return Number.isFinite(parsed) ? parsed : null;
  })();

  const rows = (data?.rows ?? []).filter((liq) => parseFloat(liq.position_qty) > 0).slice(0, 50);
  const title = `Recent Liquidations${isEmbed ? ` — ${baseToken(symbol)}-PERP` : ''}`;

  return (
    <div
      className="rounded-2xl overflow-hidden flex flex-col"
      style={{ background: 'rgba(20,15,35,.9)', border: '1px solid rgba(156,117,255,0.15)' }}
    >
      <div
        className="flex items-center justify-between border-b px-5 py-4 shrink-0"
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
            Latest liquidation events for this market
          </div>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-xs text-gray-500">{rows.length} events</span>
          <WidgetShareButton widgetId="market-recent-liquidations" title={title} symbol={symbol} />
        </div>
      </div>

      <div className="overflow-auto flex-1 min-h-0 max-h-[440px]">
        {isLoading && !data ? (
          <div className="px-4 py-4">
            <TableSkeleton rows={10} />
          </div>
        ) : rows.length > 0 ? (
          <table className="w-full">
            <thead>
              <tr>
                <th className={`text-left ${thBase}`} style={thStyle}>
                  Time
                </th>
                <th className={`text-center ${thBase}`} style={thStyle}>
                  Side
                </th>
                <th className={`text-right ${thBase}`} style={thStyle}>
                  Mark Price
                </th>
                <th className={`text-right ${thBase}`} style={thStyle}>
                  Qty
                </th>
                <th className={`text-right ${thBase}`} style={thStyle}>
                  Notional
                </th>
                <th className={`text-right ${thBase}`} style={thStyle}>
                  Trader
                </th>
              </tr>
            </thead>
            <tbody>
              {rows.map((liq, i) => {
                const isLong = liq.side === 'LONG';
                const color = isLong ? '#FF6390' : '#00dea3';
                const bg = isLong ? 'rgba(255,99,144,0.15)' : 'rgba(0,222,163,0.15)';
                const traderHref =
                  liq.address &&
                  `/address/${
                    liq.address.match(/^[0-9a-zA-Z]{43,44}$/)
                      ? base64UrlSafeEncode(liq.address)
                      : liq.address
                  }${liq.broker_id ? `?broker_id=${liq.broker_id}` : ''}`;
                return (
                  <tr
                    key={`${liq.timestamp}-${i}`}
                    className="border-b border-border-primary hover:bg-bg-tertiary transition-colors"
                  >
                    <td className="py-2 px-4 text-xs text-gray-400">
                      {dayjs(liq.timestamp).format('MMM DD HH:mm:ss')}
                    </td>
                    <td className="py-2 px-4 text-center">
                      <span
                        className="inline-block px-2 py-0.5 rounded text-xs font-medium"
                        style={{ background: bg, color }}
                      >
                        {liq.side}
                      </span>
                    </td>
                    <td className="py-2 px-4 text-right text-xs font-mono text-white">
                      {formatPriceByTick(liq.mark_price, quoteTick)}
                    </td>
                    <td className="py-2 px-4 text-right text-xs font-mono text-gray-300">
                      {parseFloat(liq.position_qty)}
                    </td>
                    <td className="py-2 px-4 text-right text-xs font-mono text-gray-300">
                      {fmtUsd(parseFloat(liq.notional))}
                    </td>
                    <td className="py-2 px-4 text-right">
                      {liq.address && traderHref ? (
                        <a
                          href={`${DASHBOARD_ORIGIN}${traderHref}`}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-xs font-mono text-[#D4B2FF] hover:text-white transition-colors no-underline"
                        >
                          {liq.address.slice(0, 6)}...{liq.address.slice(-4)}
                        </a>
                      ) : (
                        <span className="text-xs text-gray-600">—</span>
                      )}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        ) : (
          <div className="flex items-center justify-center h-full min-h-[200px] text-[rgba(255,255,255,0.25)] text-[13px]">
            No recent liquidations for this market
          </div>
        )}
      </div>
    </div>
  );
};
