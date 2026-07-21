import { FC } from 'react';

import { fmtBps, fmtUsd } from '~/components/analytics/shared/formatters';
import type {
  MarketSummaryMarket,
  FuturesSymbolResponse,
  SymbolInfoResponse
} from '~/hooks/usePublicInfo';
import { formatPriceByTick } from '~/utils/format';

export type MarketHeaderProps = {
  symbol: string;
  baseToken: string;
  marketInfo?: MarketSummaryMarket;
  futuresInfo?: FuturesSymbolResponse;
  symbolInfo?: SymbolInfoResponse;
};

export const MarketHeader: FC<MarketHeaderProps> = ({
  symbol,
  baseToken,
  marketInfo,
  futuresInfo,
  symbolInfo
}) => {
  const markPrice = marketInfo?.mark_price ? parseFloat(marketInfo.mark_price) : null;
  const open24h = marketInfo?.['24h_open'] ? parseFloat(marketInfo['24h_open']) : null;
  const high24h = marketInfo?.['24h_high'] ? parseFloat(marketInfo['24h_high']) : null;
  const low24h = marketInfo?.['24h_low'] ? parseFloat(marketInfo['24h_low']) : null;
  // Use 24h_amount if available, otherwise calculate from 24h_volume * mark_price
  const volume24h = marketInfo?.['24h_amount']
    ? parseFloat(marketInfo['24h_amount'])
    : marketInfo?.['24h_volume'] && markPrice
      ? parseFloat(marketInfo['24h_volume']) * markPrice
      : null;
  const openInterest =
    marketInfo?.open_interest && markPrice
      ? parseFloat(marketInfo.open_interest) * markPrice
      : null;
  const fundingRate = marketInfo?.last_funding_rate
    ? parseFloat(marketInfo.last_funding_rate)
    : null;

  // Calculate max leverage from base_imr (initial margin rate)
  // Max leverage = 1 / base_imr
  const maxLeverageFromImr = symbolInfo?.base_imr ? Math.round(1 / symbolInfo.base_imr) : null;

  const quoteTick = marketInfo?.quote_tick
    ? parseFloat(marketInfo.quote_tick)
    : (symbolInfo?.quote_tick ?? null);

  // Prefer futuresInfo 24h_amount (same source as markets page, reliable),
  // fall back to calculated volume from marketDetail
  const finalVolume24h =
    futuresInfo?.['24h_amount'] != null ? futuresInfo['24h_amount'] : volume24h;
  // Max leverage is calculated from base_imr (1 / base_imr)
  const finalMaxLeverage = maxLeverageFromImr;

  const change24h =
    markPrice != null && open24h != null && open24h !== 0
      ? ((markPrice - open24h) / open24h) * 100
      : null;

  const changeColor = change24h != null && change24h >= 0 ? '#00dea3' : '#FF6390';

  return (
    <div
      className="rounded-2xl overflow-hidden"
      style={{ background: 'rgba(20,15,35,.9)', border: '1px solid rgba(156,117,255,0.15)' }}
    >
      <div className="px-5 py-5 sm:px-6 sm:py-6">
        {/* Title row */}
        <div className="flex flex-col sm:flex-row sm:items-center gap-3 sm:gap-4 mb-5">
          <div>
            <h1 className="text-2xl sm:text-3xl font-bold text-white">{baseToken}-PERP</h1>
            <div className="text-xs text-gray-500 mt-0.5 font-mono">{symbol}</div>
          </div>
          {markPrice != null && (
            <div className="sm:ml-auto flex items-baseline gap-2">
              <span className="text-2xl sm:text-3xl font-bold text-white">
                ${formatPriceByTick(markPrice, quoteTick)}
              </span>
              {change24h != null && (
                <span className="text-lg font-semibold" style={{ color: changeColor }}>
                  {change24h >= 0 ? '+' : ''}
                  {change24h.toFixed(2)}%
                </span>
              )}
            </div>
          )}
        </div>

        {/* Metrics grid */}
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3">
          <MetricCard
            label="24h High"
            value={high24h != null ? `$${formatPriceByTick(high24h, quoteTick)}` : '-'}
          />
          <MetricCard
            label="24h Low"
            value={low24h != null ? `$${formatPriceByTick(low24h, quoteTick)}` : '-'}
          />
          <MetricCard label="24h Volume" value={fmtUsd(finalVolume24h)} />
          <MetricCard label="Open Interest" value={fmtUsd(openInterest)} />
          <MetricCard
            label="Funding Rate"
            value={fundingRate != null ? fmtBps(fundingRate) : '—'}
            valueColor={
              fundingRate != null ? (fundingRate >= 0 ? '#00dea3' : '#FF6390') : undefined
            }
          />
          <MetricCard
            label="Max Leverage"
            value={finalMaxLeverage != null ? `${finalMaxLeverage}x` : '—'}
          />
        </div>
      </div>
    </div>
  );
};

const MetricCard: FC<{ label: string; value: string; valueColor?: string }> = ({
  label,
  value,
  valueColor
}) => (
  <div
    className="p-3 rounded-lg"
    style={{ background: 'rgba(255,255,255,0.03)', border: '1px solid rgba(255,255,255,0.05)' }}
  >
    <div className="text-xs text-gray-500 uppercase tracking-wider mb-1">{label}</div>
    <div className="text-sm font-semibold text-white" style={{ color: valueColor }}>
      {value}
    </div>
  </div>
);
