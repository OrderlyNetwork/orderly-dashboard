import { FC, useMemo } from 'react';

import { TableSkeleton } from '~/components/analytics/shared/primitives';
import { WidgetShareButton } from '~/components/analytics/widgets/WidgetShareButton';
import { useIsEmbed } from '~/hooks/useIsEmbed';
import type {
  MarketSummaryMarket,
  OrderbookLevel,
  SymbolInfoResponse
} from '~/hooks/usePublicInfo';
import { getBaseToken } from '~/hooks/useSymbols';
import { formatPriceByTick, tickToDecimals } from '~/utils/format';

export type OrderbookPanelProps = {
  symbol?: string;
  orderbook?: {
    asks: OrderbookLevel[];
    bids: OrderbookLevel[];
    ts?: number;
  };
  marketInfo?: MarketSummaryMarket;
  symbolInfo?: SymbolInfoResponse;
  isLoading?: boolean;
};

const LEVEL_COUNT = 12;

/** Format quantity — compact for large numbers, tick precision for small */
function fmtQty(n: number, maxDecimals: number): string {
  if (n >= 1000) return n.toLocaleString('en', { notation: 'compact', maximumFractionDigits: 2 });
  if (n >= 1) return n.toLocaleString('en', { maximumFractionDigits: maxDecimals });
  return n.toLocaleString('en', { maximumFractionDigits: Math.max(maxDecimals, 6) });
}

/** Format cumulative total — always compact */
function fmtTotal(n: number): string {
  return n.toLocaleString('en', { notation: 'compact', maximumFractionDigits: 2 });
}

export const OrderbookPanel: FC<OrderbookPanelProps> = ({
  symbol,
  orderbook,
  marketInfo,
  symbolInfo,
  isLoading
}) => {
  const isEmbed = useIsEmbed();
  // Derive price & quantity precision from tick sizes
  // Prefer marketInfo, fall back to symbolInfo (which always has tick data)
  const quoteTick = marketInfo?.quote_tick
    ? parseFloat(marketInfo.quote_tick)
    : (symbolInfo?.quote_tick ?? null);
  const baseTick = marketInfo?.base_tick
    ? parseFloat(marketInfo.base_tick)
    : (symbolInfo?.base_tick ?? null);
  const qtyDecimals = baseTick != null ? tickToDecimals(baseTick) : 6;

  const { asks, bids, spread, midPrice, maxTotal } = useMemo(() => {
    if (!orderbook) return { asks: [], bids: [], spread: null, midPrice: null, maxTotal: 1 };

    const asksReversed = orderbook.asks.slice(0, LEVEL_COUNT).reverse();
    const bidsOrdered = orderbook.bids.slice(0, LEVEL_COUNT);

    // Cumulative totals: accumulate from the inside (best price) outward
    let askAccum = 0;
    const askTotals: number[] = [];
    for (let i = asksReversed.length - 1; i >= 0; i--) {
      askAccum += parseFloat(asksReversed[i].quantity);
      askTotals[i] = askAccum;
    }
    const asksWithDepth = asksReversed.map((a, i) => ({ ...a, total: askTotals[i] }));

    let bidAccum = 0;
    const bidsWithDepth = bidsOrdered.map((b) => {
      bidAccum += parseFloat(b.quantity);
      return { ...b, total: bidAccum };
    });

    const askMax = asksWithDepth.length > 0 ? asksWithDepth[0].total : 0;
    const bidMax = bidsWithDepth.length > 0 ? bidsWithDepth[bidsWithDepth.length - 1].total : 0;
    const maxTotal = Math.max(askMax, bidMax, 1);

    const bestAsk = orderbook.asks.length > 0 ? parseFloat(orderbook.asks[0].price) : null;
    const bestBid = orderbook.bids.length > 0 ? parseFloat(orderbook.bids[0].price) : null;
    const spread = bestAsk != null && bestBid != null ? bestAsk - bestBid : null;
    const midPrice = bestAsk != null && bestBid != null ? (bestAsk + bestBid) / 2 : null;

    return { asks: asksWithDepth, bids: bidsWithDepth, spread, midPrice, maxTotal };
  }, [orderbook]);

  const markPrice = marketInfo?.mark_price ? parseFloat(marketInfo.mark_price) : null;

  const headerClass = 'py-1.5 px-2 text-[10px] font-medium text-gray-600 uppercase tracking-wider';

  const suffix = symbol && isEmbed ? ` — ${getBaseToken(symbol)}-PERP` : '';
  const title = `Orderbook${suffix}`;

  return (
    <div
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
            {spread != null && midPrice != null ? (
              <>
                Spread:{' '}
                <span className="text-white font-medium">
                  ${formatPriceByTick(spread, quoteTick)}
                </span>
                {' · '}
                Mid:{' '}
                <span className="text-white font-medium">
                  ${formatPriceByTick(midPrice, quoteTick)}
                </span>
              </>
            ) : (
              'Orderbook depth'
            )}
          </div>
        </div>
        {symbol && <WidgetShareButton widgetId="market-orderbook" title={title} symbol={symbol} />}
      </div>

      <div className="px-4 pt-3 pb-4">
        {isLoading && !orderbook ? (
          <TableSkeleton rows={10} />
        ) : orderbook ? (
          <div className="flex flex-col">
            {/* Column headers */}
            <div
              className="flex items-center px-2 pb-1 border-b"
              style={{ borderColor: 'rgba(255,255,255,0.05)' }}
            >
              <span className={`${headerClass} w-[90px] text-left`}>Price</span>
              <span className={`${headerClass} flex-1 text-right`}>Quantity</span>
              <span className={`${headerClass} w-[80px] text-right`}>Total</span>
            </div>

            {/* Asks */}
            <div className="flex-1 flex flex-col justify-end pt-1">
              {asks.map((level, i) => (
                <OrderbookRow
                  key={`ask-${i}`}
                  price={level.price}
                  quantity={level.quantity}
                  total={level.total}
                  maxTotal={maxTotal}
                  side="ask"
                  quoteTick={quoteTick}
                  qtyDecimals={qtyDecimals}
                />
              ))}
            </div>

            {/* Mark price divider */}
            <div
              className="flex items-center justify-between py-2 my-1 border-t border-b rounded px-2"
              style={{
                borderColor: 'rgba(156,117,255,0.15)',
                background: 'rgba(156,117,255,0.05)'
              }}
            >
              <span className="text-xs text-gray-500">Mark Price</span>
              <span className="text-sm font-semibold text-white">
                {markPrice != null ? `$${formatPriceByTick(markPrice, quoteTick)}` : '-'}
              </span>
            </div>

            {/* Bids */}
            <div className="flex-1 flex flex-col pt-1">
              {bids.map((level, i) => (
                <OrderbookRow
                  key={`bid-${i}`}
                  price={level.price}
                  quantity={level.quantity}
                  total={level.total}
                  maxTotal={maxTotal}
                  side="bid"
                  quoteTick={quoteTick}
                  qtyDecimals={qtyDecimals}
                />
              ))}
            </div>
          </div>
        ) : (
          <div className="flex items-center justify-center h-[400px] text-[rgba(255,255,255,0.25)] text-[13px]">
            No orderbook data available
          </div>
        )}
      </div>
    </div>
  );
};

const OrderbookRow: FC<{
  price: string;
  quantity: string;
  total: number;
  maxTotal: number;
  side: 'bid' | 'ask';
  quoteTick: number | null;
  qtyDecimals: number;
}> = ({ price, quantity, total, maxTotal, side, quoteTick, qtyDecimals }) => {
  const pct = Math.min((total / maxTotal) * 100, 100);
  const isAsk = side === 'ask';

  return (
    <div className="relative flex items-center py-[3px] px-2 hover:bg-white/[0.02]">
      <div
        className="absolute top-0 bottom-0 right-0"
        style={{
          width: `${pct}%`,
          background: isAsk ? 'rgba(255,99,144,0.1)' : 'rgba(0,222,163,0.1)',
          transition: 'width 0.3s ease-out'
        }}
      />
      <span
        className="relative z-10 w-[90px] text-left text-xs font-mono"
        style={{ color: isAsk ? '#FF6390' : '#00dea3' }}
      >
        {formatPriceByTick(parseFloat(price), quoteTick)}
      </span>
      <span className="relative z-10 flex-1 text-right text-xs font-mono text-gray-400">
        {fmtQty(parseFloat(quantity), qtyDecimals)}
      </span>
      <span className="relative z-10 w-[80px] text-right text-xs font-mono text-gray-500">
        {fmtTotal(total)}
      </span>
    </div>
  );
};
