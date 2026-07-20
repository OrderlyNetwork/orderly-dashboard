import { FC, useMemo } from 'react';

import { fmtUsd } from '~/components/analytics/shared/formatters';
import { TableSkeleton, Empty } from '~/components/analytics/shared/primitives';
import { WidgetShareButton } from '~/components/analytics/widgets/WidgetShareButton';
import { useIsEmbed } from '~/hooks/useIsEmbed';
import { usePlatformPositions, useTradersOpenInterests } from '~/hooks/usePublicInfo';
import { base64UrlSafeEncode, DASHBOARD_ORIGIN } from '~/util';
import { formatPriceByTick } from '~/utils/format';

export type PlatformPositionsPanelProps = {
  symbol: string;
  quoteTick?: number | null;
};

const baseToken = (symbol: string) => symbol.split('_')[1] ?? symbol;

export const PlatformPositionsPanel: FC<PlatformPositionsPanelProps> = ({ symbol, quoteTick }) => {
  const isEmbed = useIsEmbed();
  const { data, isLoading } = usePlatformPositions(symbol, 0);
  const { data: oiData } = useTradersOpenInterests();

  const { longTotal, shortTotal, longPct, topPositions } = useMemo(() => {
    // Use traders_open_interests (excludes MM) for the ratio — matches markets page
    let longTotal = 0;
    let shortTotal = 0;
    let longPct = 50;

    if (oiData?.rows) {
      const row = oiData.rows.find((r) => r.symbol === symbol);
      if (row) {
        longTotal = Math.abs(row.long_oi || 0);
        shortTotal = Math.abs(row.short_oi || 0);
        const total = longTotal + shortTotal;
        longPct = total > 0 ? (longTotal / total) * 100 : 50;
      }
    }

    // Sort by notional value descending, take top 15
    const topPositions = [...(data?.rows || [])]
      .sort((a, b) => parseFloat(b.notional || '0') - parseFloat(a.notional || '0'))
      .slice(0, 15);

    return { longTotal, shortTotal, longPct, topPositions };
  }, [data, oiData, symbol]);

  const title = `Open Positions${isEmbed ? ` — ${baseToken(symbol)}-PERP` : ''}`;

  return (
    <div
      className="rounded-2xl overflow-hidden flex flex-col"
      style={{
        background: 'rgba(20,15,35,.9)',
        border: '1px solid rgba(156,117,255,0.15)',
        maxHeight: 600
      }}
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
            Platform-wide positions for this symbol
          </div>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-xs text-gray-500">{data?.total_positions ?? 0} positions</span>
          <WidgetShareButton widgetId="market-platform-positions" title={title} symbol={symbol} />
        </div>
      </div>

      {/* Long/Short ratio bar — uses traders_open_interests (excludes MM) */}
      <div className="px-5 pt-4 pb-3">
        <div className="flex items-center justify-between mb-1.5">
          <span className="text-xs font-medium" style={{ color: '#00dea3' }}>
            Long {longPct.toFixed(1)}%
          </span>
          <span className="text-xs font-medium" style={{ color: '#FF6390' }}>
            Short {(100 - longPct).toFixed(1)}%
          </span>
        </div>
        <div className="h-2 rounded-full overflow-hidden flex" style={{ background: '#221E30' }}>
          <div style={{ width: `${longPct}%`, background: '#00dea3', transition: 'width 0.3s' }} />
          <div
            style={{ width: `${100 - longPct}%`, background: '#FF6390', transition: 'width 0.3s' }}
          />
        </div>
        <div className="flex items-center justify-between mt-1.5">
          <span className="text-xs text-gray-500">{fmtUsd(longTotal)}</span>
          <span className="text-xs text-gray-500">{fmtUsd(shortTotal)}</span>
        </div>
      </div>

      {/* Top positions table */}
      <div className="overflow-auto flex-1" style={{ minHeight: 0 }}>
        {isLoading && topPositions.length === 0 ? (
          <div className="px-4 py-4">
            <TableSkeleton rows={8} />
          </div>
        ) : topPositions.length > 0 ? (
          <table className="w-full">
            <thead>
              <tr>
                <th
                  className="text-left py-2 px-4 text-xs font-medium text-gray-500 uppercase tracking-wider sticky top-0"
                  style={{ background: 'rgba(20,15,35,.95)' }}
                >
                  Address
                </th>
                <th
                  className="text-center py-2 px-4 text-xs font-medium text-gray-500 uppercase tracking-wider sticky top-0"
                  style={{ background: 'rgba(20,15,35,.95)' }}
                >
                  Side
                </th>
                <th
                  className="text-right py-2 px-4 text-xs font-medium text-gray-500 uppercase tracking-wider sticky top-0"
                  style={{ background: 'rgba(20,15,35,.95)' }}
                >
                  Notional
                </th>
                <th
                  className="text-right py-2 px-4 text-xs font-medium text-gray-500 uppercase tracking-wider sticky top-0"
                  style={{ background: 'rgba(20,15,35,.95)' }}
                >
                  Entry Price
                </th>
                <th
                  className="text-right py-2 px-4 text-xs font-medium text-gray-500 uppercase tracking-wider sticky top-0"
                  style={{ background: 'rgba(20,15,35,.95)' }}
                >
                  Unrealized PnL
                </th>
              </tr>
            </thead>
            <tbody>
              {topPositions.map((pos, i) => {
                const notional = parseFloat(pos.notional || '0');
                const upnl = parseFloat(pos.unrealized_pnl || '0');
                const entryPrice = parseFloat(pos.average_open_price || '0');

                return (
                  <tr
                    key={`${pos.address}-${pos.account_id}-${i}`}
                    className="border-b border-border-primary hover:bg-bg-tertiary transition-colors"
                  >
                    <td className="py-2 px-4">
                      {pos.address ? (
                        <a
                          href={`${DASHBOARD_ORIGIN}/address/${
                            pos.address.match(/^[0-9a-zA-Z]{43,44}$/)
                              ? base64UrlSafeEncode(pos.address)
                              : pos.address
                          }${pos.broker_id ? `?broker_id=${pos.broker_id}` : ''}`}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-xs font-mono text-[#D4B2FF] hover:text-white transition-colors no-underline"
                        >
                          {pos.address.slice(0, 6)}...{pos.address.slice(-4)}
                        </a>
                      ) : (
                        <span className="text-xs text-gray-500">—</span>
                      )}
                    </td>
                    <td className="py-2 px-4 text-center">
                      <span
                        className="inline-block px-2 py-0.5 rounded text-xs font-medium"
                        style={{
                          background:
                            pos.side === 'LONG' ? 'rgba(0,222,163,0.15)' : 'rgba(255,99,144,0.15)',
                          color: pos.side === 'LONG' ? '#00dea3' : '#FF6390'
                        }}
                      >
                        {pos.side}
                      </span>
                    </td>
                    <td className="py-2 px-4 text-right text-xs font-mono text-white">
                      {fmtUsd(notional)}
                    </td>
                    <td className="py-2 px-4 text-right text-xs font-mono text-gray-300">
                      {formatPriceByTick(entryPrice, quoteTick)}
                    </td>
                    <td
                      className="py-2 px-4 text-right text-xs font-mono"
                      style={{ color: upnl >= 0 ? '#00dea3' : '#FF6390' }}
                    >
                      {fmtUsd(upnl)}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        ) : (
          <div className="flex items-center justify-center flex-1">
            <Empty msg="No open positions" />
          </div>
        )}
      </div>
    </div>
  );
};
