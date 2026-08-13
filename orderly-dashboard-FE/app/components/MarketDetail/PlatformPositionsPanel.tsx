import { useVirtualizer } from '@tanstack/react-virtual';
import { FC, useMemo, useRef } from 'react';

import { AddressLink } from '~/components/analytics/shared/AddressLink';
import { fmtUsd } from '~/components/analytics/shared/formatters';
import { TableSkeleton, Empty } from '~/components/analytics/shared/primitives';
import { WidgetShareButton } from '~/components/analytics/widgets/WidgetShareButton';
import { useIsEmbed } from '~/hooks/useIsEmbed';
import { usePlatformPositions } from '~/hooks/usePublicInfo';
import { getBaseToken } from '~/hooks/useSymbols';
import { formatPriceByTick } from '~/utils/format';
import { computeUnrealizedPnl } from '~/utils/pnl';

export type PlatformPositionsPanelProps = {
  symbol: string;
  quoteTick?: number | null;
};

const ROW_HEIGHT = 36;

export const PlatformPositionsPanel: FC<PlatformPositionsPanelProps> = ({ symbol, quoteTick }) => {
  const isEmbed = useIsEmbed();
  const { data, isLoading } = usePlatformPositions(symbol, 0);
  const scrollRef = useRef<HTMLDivElement>(null);

  const { longTotal, shortTotal, longPct, rows, visibleCount } = useMemo(() => {
    // Exclude Orderly market-maker rows, then sort by notional descending and
    // take the top 100. Both the L/S ratio and the table are derived from this
    // filtered set so they agree, and `notional` is already in USD.
    const filtered = (data?.rows || []).filter((row) => row.broker_id !== 'orderly');
    const rows = [...filtered]
      .sort((a, b) => parseFloat(b.notional || '0') - parseFloat(a.notional || '0'))
      .slice(0, 100);

    const longTotal = filtered
      .filter((r) => r.side === 'LONG')
      .reduce((sum, r) => sum + Math.abs(parseFloat(r.notional || '0')), 0);
    const shortTotal = filtered
      .filter((r) => r.side === 'SHORT')
      .reduce((sum, r) => sum + Math.abs(parseFloat(r.notional || '0')), 0);
    const total = longTotal + shortTotal;
    const longPct = total > 0 ? (longTotal / total) * 100 : 50;

    return { longTotal, shortTotal, longPct, rows, visibleCount: filtered.length };
  }, [data]);

  const rowVirtualizer = useVirtualizer({
    count: rows.length,
    getScrollElement: () => scrollRef.current,
    estimateSize: () => ROW_HEIGHT,
    overscan: 10
  });

  const virtualItems = rowVirtualizer.getVirtualItems();
  const totalSize = rowVirtualizer.getTotalSize();
  const paddingTop = virtualItems.length > 0 ? virtualItems[0].start : 0;
  const paddingBottom =
    virtualItems.length > 0 ? totalSize - virtualItems[virtualItems.length - 1].end : 0;

  const title = `Open Positions${isEmbed ? ` — ${getBaseToken(symbol)}-PERP` : ''}`;

  const thClass =
    'py-2 px-4 text-xs font-medium text-gray-500 uppercase tracking-wider sticky top-0 z-10';
  const thStyle = { background: 'rgba(20,15,35,.95)' };

  return (
    <div
      data-widget-id="market-platform-positions"
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
          <span className="text-xs text-gray-500">{visibleCount} positions</span>
          <WidgetShareButton widgetId="market-platform-positions" title={title} symbol={symbol} />
        </div>
      </div>

      {/* Long/Short ratio bar — derived from MM-filtered platformPositions rows */}
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

      {/* Virtualized positions table */}
      <div ref={scrollRef} className="overflow-auto flex-1" style={{ minHeight: 0 }}>
        {isLoading && rows.length === 0 ? (
          <div className="px-4 py-4">
            <TableSkeleton rows={8} />
          </div>
        ) : rows.length > 0 ? (
          <table className="w-full">
            <thead>
              <tr>
                <th className={`text-left ${thClass}`} style={thStyle}>
                  Address
                </th>
                <th className={`text-center ${thClass}`} style={thStyle}>
                  Side
                </th>
                <th className={`text-right ${thClass}`} style={thStyle}>
                  Notional
                </th>
                <th className={`text-right ${thClass}`} style={thStyle}>
                  Entry Price
                </th>
                <th className={`text-right ${thClass}`} style={thStyle}>
                  Unrealized PnL
                </th>
              </tr>
            </thead>
            <tbody>
              {paddingTop > 0 && (
                <tr style={{ height: paddingTop }}>
                  <td colSpan={5} style={{ padding: 0 }} />
                </tr>
              )}
              {virtualItems.map((virtualRow) => {
                const pos = rows[virtualRow.index];
                const notional = parseFloat(pos.notional || '0');
                const upnl = computeUnrealizedPnl(pos) ?? 0;
                const entryPrice = parseFloat(pos.average_open_price || '0');

                return (
                  <tr
                    key={`${pos.address}-${pos.account_id}-${virtualRow.index}`}
                    className="border-b border-border-primary hover:bg-bg-tertiary transition-colors"
                  >
                    <td className="py-2 px-4">
                      {pos.address ? (
                        <AddressLink
                          address={pos.address}
                          brokerId={pos.broker_id}
                          className="text-xs font-mono text-[#D4B2FF] hover:text-white transition-colors no-underline"
                        >
                          {pos.address.slice(0, 6)}...{pos.address.slice(-4)}
                        </AddressLink>
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
              {paddingBottom > 0 && (
                <tr style={{ height: paddingBottom }}>
                  <td colSpan={5} style={{ padding: 0 }} />
                </tr>
              )}
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
