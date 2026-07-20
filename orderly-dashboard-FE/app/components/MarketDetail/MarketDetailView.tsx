import { ArrowLeftIcon } from '@radix-ui/react-icons';
import { Link } from '@remix-run/react';
import { FC, useState } from 'react';

import { FundingChart } from './FundingChart';
import { MarketHeader } from './MarketHeader';
import { OrderbookPanel } from './OrderbookPanel';
import { PlatformPositionsPanel } from './PlatformPositionsPanel';
import { PriceChart } from './PriceChart';
import { RecentTrades } from './RecentTrades';
import { TopTradersPanel } from './TopTradersPanel';

import { Spinner } from '~/components';
import { useMarketDetail, useFuturesSymbol, useSymbolInfo } from '~/hooks/usePublicInfo';

export type MarketDetailViewProps = {
  symbol: string;
  baseToken: string;
};

export const MarketDetailView: FC<MarketDetailViewProps> = ({ symbol, baseToken }) => {
  const [candlesInterval, setCandlesInterval] = useState('1h');
  const { data, error, isLoading } = useMarketDetail(symbol, candlesInterval);
  const { data: futuresData } = useFuturesSymbol(symbol);
  const { data: symbolInfoData } = useSymbolInfo(symbol);

  if (error) {
    return (
      <div className="flex flex-col items-center gap-6 py-16">
        <div className="text-gray-400 text-sm">
          {error.message || 'Failed to fetch market data'}
        </div>
        <Link
          to="/markets"
          className="inline-flex items-center gap-2 text-gray-400 hover:text-white transition-colors duration-200 no-underline"
        >
          <ArrowLeftIcon width="16" height="16" />
          <span className="text-sm">Back to Markets</span>
        </Link>
      </div>
    );
  }

  if (isLoading && !data) {
    return (
      <div className="flex justify-center py-12">
        <Spinner size="2.5rem" />
      </div>
    );
  }

  return (
    <div className="space-y-4 sm:space-y-6 animate-fade-in">
      {/* Back Button */}
      <div>
        <Link
          to="/markets"
          className="inline-flex items-center gap-2 text-gray-400 hover:text-white transition-colors duration-200 no-underline"
        >
          <ArrowLeftIcon width="16" height="16" />
          <span className="text-sm">Back to Markets</span>
        </Link>
      </div>

      {/* Header with key metrics */}
      <MarketHeader
        symbol={symbol}
        baseToken={baseToken}
        marketInfo={data?.market_info}
        futuresInfo={futuresData}
        symbolInfo={symbolInfoData}
      />

      {/* Price Chart - full width */}
      <PriceChart
        symbol={symbol}
        candles={data?.candles}
        isLoading={isLoading}
        interval={candlesInterval}
        onIntervalChange={setCandlesInterval}
      />

      {/* Orderbook + Recent Trades side-by-side (stacked on mobile) */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 sm:gap-6 items-start">
        {' '}
        <OrderbookPanel
          orderbook={data?.orderbook}
          marketInfo={data?.market_info}
          symbolInfo={symbolInfoData}
          isLoading={isLoading && !data}
        />
        <RecentTrades trades={data?.recent_trades} isLoading={isLoading && !data} />
      </div>

      {/* Funding History */}
      <FundingChart fundingHistory={data?.funding_history} isLoading={isLoading && !data} />

      {/* Top Traders + Platform Positions side-by-side (stacked on mobile) */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 sm:gap-6">
        <TopTradersPanel symbol={symbol} />
        <PlatformPositionsPanel symbol={symbol} />
      </div>
    </div>
  );
};
