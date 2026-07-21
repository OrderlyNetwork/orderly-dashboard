import dayjs from 'dayjs';
import { FC } from 'react';

import { TableSkeleton } from '~/components/analytics/shared/primitives';
import { WidgetShareButton } from '~/components/analytics/widgets/WidgetShareButton';
import { useIsEmbed } from '~/hooks/useIsEmbed';
import type { MarketTrade } from '~/hooks/usePublicInfo';
import { getBaseToken } from '~/hooks/useSymbols';
import { base64UrlSafeEncode, DASHBOARD_ORIGIN } from '~/util';
import { formatPriceByTick } from '~/utils/format';

export type RecentTradesProps = {
  symbol?: string;
  trades?: MarketTrade[];
  isLoading?: boolean;
  quoteTick?: number | null;
  standalone?: boolean;
};

const thBase = 'py-2 px-4 text-xs font-medium text-gray-500 uppercase tracking-wider sticky top-0';
const thStyle = { background: 'rgba(20,15,35,.95)' };

export const RecentTrades: FC<RecentTradesProps> = ({
  symbol,
  trades,
  isLoading,
  quoteTick,
  standalone
}) => {
  const isEmbed = useIsEmbed();
  const suffix = symbol && isEmbed ? ` — ${getBaseToken(symbol)}-PERP` : '';
  const title = `Recent Trades${suffix}`;

  return (
    /* Outer wrapper: grid item that stretches to row height (matches orderbook).
       On mobile it's natural height; on desktop it stretches + becomes relative.
       Skipped in standalone mode (no grid context -> absolute would collapse). */
    <div className={standalone ? '' : 'lg:self-stretch lg:relative lg:min-h-0'}>
      <div
        className={`flex flex-col rounded-2xl overflow-hidden ${
          standalone ? '' : 'lg:absolute lg:inset-0'
        }`}
        style={{
          background: 'rgba(20,15,35,.9)',
          border: '1px solid rgba(156,117,255,0.15)'
        }}
      >
        {/* Header */}
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
              Latest taker-side trades
            </div>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-xs text-gray-500">{trades?.length ?? 0} trades</span>
            {symbol && (
              <WidgetShareButton widgetId="market-recent-trades" title={title} symbol={symbol} />
            )}
          </div>
        </div>

        {/* Scrollable table area */}
        <div className="overflow-auto flex-1 min-h-0">
          {isLoading && !trades ? (
            <div className="px-4 py-4">
              <TableSkeleton rows={10} />
            </div>
          ) : trades && trades.length > 0 ? (
            <table className="w-full">
              <thead>
                <tr>
                  <th className={`text-left ${thBase}`} style={thStyle}>
                    Time
                  </th>
                  <th className={`text-right ${thBase}`} style={thStyle}>
                    Price
                  </th>
                  <th className={`text-right ${thBase}`} style={thStyle}>
                    Qty
                  </th>
                  <th className={`text-center ${thBase}`} style={thStyle}>
                    Side
                  </th>
                  <th className={`text-right ${thBase}`} style={thStyle}>
                    Trader
                  </th>
                </tr>
              </thead>
              <tbody>
                {trades.map((trade, i) => (
                  <tr
                    key={`${trade.executed_timestamp}-${i}`}
                    className="border-b border-border-primary hover:bg-bg-tertiary transition-colors"
                  >
                    <td className="py-2 px-4 text-xs text-gray-400">
                      {dayjs(trade.executed_timestamp).format('HH:mm:ss')}
                    </td>
                    <td className="py-2 px-4 text-right text-xs font-mono text-white">
                      {formatPriceByTick(trade.executed_price, quoteTick)}
                    </td>
                    <td className="py-2 px-4 text-right text-xs font-mono text-gray-300">
                      {parseFloat(trade.executed_quantity)}
                    </td>
                    <td className="py-2 px-4 text-center">
                      <span
                        className="inline-block px-2 py-0.5 rounded text-xs font-medium"
                        style={{
                          background:
                            trade.side === 'BUY' ? 'rgba(0,222,163,0.15)' : 'rgba(255,99,144,0.15)',
                          color: trade.side === 'BUY' ? '#00dea3' : '#FF6390'
                        }}
                      >
                        {trade.side}
                      </span>
                    </td>
                    <td className="py-2 px-4 text-right">
                      <a
                        href={`${DASHBOARD_ORIGIN}/address/${
                          trade.address.match(/^[0-9a-zA-Z]{43,44}$/)
                            ? base64UrlSafeEncode(trade.address)
                            : trade.address
                        }?broker_id=${trade.broker_id}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-xs font-mono text-[#D4B2FF] hover:text-white transition-colors no-underline"
                      >
                        {trade.address.slice(0, 6)}...{trade.address.slice(-4)}
                      </a>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          ) : (
            <div className="flex items-center justify-center h-full min-h-[200px] text-[rgba(255,255,255,0.25)] text-[13px]">
              No recent trades
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
