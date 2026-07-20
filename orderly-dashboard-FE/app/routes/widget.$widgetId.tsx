import { json } from '@remix-run/node';
import { Link, useLoaderData, useSearchParams } from '@remix-run/react';
import { FC, useMemo, useState } from 'react';

import { Leaderboard } from '~/components/Leaderboard';
import { FundingChart } from '~/components/MarketDetail/FundingChart';
import { OrderbookPanel } from '~/components/MarketDetail/OrderbookPanel';
import { PlatformPositionsPanel } from '~/components/MarketDetail/PlatformPositionsPanel';
import { PriceChart } from '~/components/MarketDetail/PriceChart';
import { RecentLiquidations } from '~/components/MarketDetail/RecentLiquidations';
import { RecentTrades } from '~/components/MarketDetail/RecentTrades';
import { TopTradersPanel } from '~/components/MarketDetail/TopTradersPanel';
import { Positions } from '~/components/Positions';
import { fmtPctOfSupply } from '~/components/analytics/shared/formatters';
import {
  GranularitySelector,
  PeriodSelector,
  type Granularity,
  type Period
} from '~/components/analytics/shared/primitives';
import { AnalystKPIWidget } from '~/components/analytics/widgets/AnalystKPIWidget';
import { BuilderActiveTradersWidget } from '~/components/analytics/widgets/BuilderActiveTradersWidget';
import { BuilderRevenueWidget } from '~/components/analytics/widgets/BuilderRevenueWidget';
import { BuilderVolumeWidget } from '~/components/analytics/widgets/BuilderVolumeWidget';
import { CopyBlock } from '~/components/analytics/widgets/CopyBlock';
import { DexUsersWidget } from '~/components/analytics/widgets/DexUsersWidget';
import { DistributorsWidget } from '~/components/analytics/widgets/DistributorsWidget';
import { FeesStatsWidget } from '~/components/analytics/widgets/FeesStatsWidget';
import { FundFlowsByChainWidget } from '~/components/analytics/widgets/FundFlowsByChainWidget';
import { FundingRatesWidget } from '~/components/analytics/widgets/FundingRatesWidget';
import { InsuranceFundWidget } from '~/components/analytics/widgets/InsuranceFundWidget';
import { LiquidationHeatmapWidget } from '~/components/analytics/widgets/LiquidationHeatmapWidget';
import { LiquidationsBySymbolWidget } from '~/components/analytics/widgets/LiquidationsBySymbolWidget';
import { NetFeesWidget } from '~/components/analytics/widgets/NetFeesWidget';
import { NetFlowByBuilderWidget } from '~/components/analytics/widgets/NetFlowByBuilderWidget';
import { OmnivaultTvlWidget } from '~/components/analytics/widgets/OmnivaultTvlWidget';
import { OverviewWidget } from '~/components/analytics/widgets/OverviewWidget';
import { StakeUsersWidget } from '~/components/analytics/widgets/StakeUsersWidget';
import { StakeVsSupplyWidget } from '~/components/analytics/widgets/StakeVsSupplyWidget';
import { StakingDailyWidget } from '~/components/analytics/widgets/StakingDailyWidget';
import { TvlByChainWidget } from '~/components/analytics/widgets/TvlByChainWidget';
import { TvlByTokenWidget } from '~/components/analytics/widgets/TvlByTokenWidget';
import { VolumeChartWidget } from '~/components/analytics/widgets/VolumeChartWidget';
import { VolumeSegmentsWidget } from '~/components/analytics/widgets/VolumeSegmentsWidget';
import { WidgetWrapper } from '~/components/analytics/widgets/WidgetWrapper';
import { useStakeVsSupply } from '~/hooks/useOrderlyMetrics';
import { useMarketDetail, useMarketSummary, useSymbolInfo } from '~/hooks/usePublicInfo';
import type { DashboardData } from '~/types/dashboard';
import { fetchDashboardData } from '~/utils/data-api';

type LoaderData = DashboardData & { widgetId: string };

const KPI_WIDGET_IDS = ['kpi-analyst'];

const NEEDS_DASHBOARD = ['volume', 'tvl-chain', 'net-fees', 'fees-stats', ...KPI_WIDGET_IDS];

export async function loader({ params }: { params: { widgetId: string } }) {
  const { widgetId } = params;
  const data: LoaderData = {
    widgetId,
    mainRows: [],
    tvlChains: [],
    tvlTotal: 0,
    marketRows: []
  };

  if (NEEDS_DASHBOARD.includes(widgetId)) {
    const dashboard = await fetchDashboardData(90);
    data.mainRows = dashboard.mainRows;
    data.tvlChains = dashboard.tvlChains;
    data.marketRows = dashboard.marketRows;
  }

  return json(data);
}

const WIDGET_META: Record<
  string,
  {
    title: string;
    subtitle?: string;
    height?: number;
    hasPeriodControl?: boolean;
    hasGranularityControl?: boolean;
  }
> = {
  volume: {
    title: 'Trading Volume — Daily',
    height: 260,
    hasPeriodControl: true
  },
  'tvl-chain': {
    title: 'TVL by Chain'
  },
  'net-fees': {
    title: 'Net Fees',
    subtitle: 'daily & total net fees',
    height: 260
  },
  overview: { title: 'Protocol Overview', hasGranularityControl: true },
  'dex-users': { title: 'Users by DEX' },
  'volume-segments': {
    title: 'Volume Segments',
    subtitle: 'weekly by segment (2B / 2C / MM)',
    height: 260
  },
  'omnivault-tvl': {
    title: 'Omnivault TVL',
    subtitle: 'avg weekly TVL per vault (USD millions)',
    height: 260
  },
  'stake-users': {
    title: 'Active Stakers',
    subtitle: 'weekly avg active ORDER stakers',
    height: 220
  },
  'stake-vs-supply': {
    title: 'Stake vs Circulating Supply',
    subtitle: 'weekly ORDER token staking vs circulating supply',
    height: 220
  },
  distributors: { title: 'Distributors' },
  'fees-stats': { title: 'Fees & Revenue' },
  'builder-volume': {
    title: 'Builder Volume',
    subtitle: 'daily trading volume breakdown per builder'
  },
  'builder-active-traders': {
    title: 'Daily Active Traders',
    subtitle: 'daily active trader breakdown per builder'
  },
  'builder-revenue': {
    title: 'Builder Revenue',
    subtitle: 'daily revenue breakdown per builder'
  },
  'net-flow-by-builder': {
    title: 'Net Flow by Builder',
    subtitle: 'daily net flow breakdown per builder'
  },
  'tvl-by-token': {
    title: 'TVL by Token',
    subtitle: 'daily TVL breakdown per token'
  },
  leaderboard: { title: 'Leaderboard' },
  positions: { title: 'Positions' },
  'kpi-analyst': { title: 'Key Metrics' },
  'funding-rates': {
    title: 'Funding Rates',
    subtitle: 'latest 8h funding rate per symbol'
  },
  'staking-daily': {
    title: 'Staking Activity',
    subtitle: 'daily net staked ORDER & burned ORDER'
  },
  'fund-flows-by-chain': {
    title: 'Net Flow by Chain',
    subtitle: 'daily net flow breakdown per chain'
  },
  'liquidations-by-symbol': {
    title: 'Liquidations by Symbol',
    subtitle: 'daily liquidation notional per symbol'
  },
  'liquidation-heatmap': {
    title: 'Liquidation Heatmap',
    subtitle: 'open-position notional at each estimated liquidation price level (per symbol)'
  },
  'insurance-fund': {
    title: 'Insurance Fund',
    subtitle: 'fund balance, collateral & open positions'
  },
  'market-price-chart': {
    title: 'Price Chart',
    subtitle: 'OHLCV candles',
    height: 420
  },
  'market-orderbook': {
    title: 'Orderbook',
    subtitle: 'Orderbook depth'
  },
  'market-recent-trades': {
    title: 'Recent Trades',
    subtitle: 'Latest taker-side trades'
  },
  'market-funding-chart': {
    title: 'Funding Rate History',
    subtitle: '8-hour funding rate epochs',
    height: 280
  },
  'market-recent-liquidations': {
    title: 'Recent Liquidations',
    subtitle: 'Latest liquidation events for this market'
  },
  'market-top-traders': {
    title: 'Top Traders',
    subtitle: 'Leading traders for this symbol'
  },
  'market-platform-positions': {
    title: 'Open Positions',
    subtitle: 'Platform-wide positions for this symbol'
  }
};

const isMarketWidget = (id: string) => id.startsWith('market-');

const MarketWidgetContent: FC<{ widgetId: string; symbol: string }> = ({ widgetId, symbol }) => {
  const [candlesInterval, setCandlesInterval] = useState('1h');
  const { data, isLoading } = useMarketDetail(symbol, candlesInterval);
  const { data: marketSummary } = useMarketSummary();
  const { data: symbolInfo } = useSymbolInfo(symbol);

  const quoteTick = useMemo(() => {
    const tick = marketSummary?.markets.find((m) => m.symbol === symbol)?.quote_tick;
    const parsed = tick != null ? parseFloat(tick) : NaN;
    return Number.isFinite(parsed) ? parsed : null;
  }, [marketSummary, symbol]);

  switch (widgetId) {
    case 'market-price-chart':
      return (
        <PriceChart
          symbol={symbol}
          candles={data?.candles}
          isLoading={isLoading}
          interval={candlesInterval}
          onIntervalChange={setCandlesInterval}
          quoteTick={quoteTick}
        />
      );
    case 'market-orderbook':
      return (
        <OrderbookPanel
          symbol={symbol}
          orderbook={data?.orderbook}
          marketInfo={data?.market_info}
          symbolInfo={symbolInfo}
          isLoading={isLoading && !data}
        />
      );
    case 'market-recent-trades':
      return (
        <RecentTrades
          symbol={symbol}
          trades={data?.recent_trades}
          isLoading={isLoading && !data}
          quoteTick={quoteTick}
          standalone
        />
      );
    case 'market-funding-chart':
      return (
        <FundingChart
          symbol={symbol}
          fundingHistory={data?.funding_history}
          isLoading={isLoading && !data}
          quoteTick={quoteTick}
        />
      );
    case 'market-recent-liquidations':
      return <RecentLiquidations symbol={symbol} />;
    case 'market-top-traders':
      return <TopTradersPanel symbol={symbol} />;
    case 'market-platform-positions':
      return <PlatformPositionsPanel symbol={symbol} quoteTick={quoteTick} />;
    default:
      return null;
  }
};

const StakeVsSupplySubtitle: FC<{ base?: string }> = ({ base }) => {
  const { data } = useStakeVsSupply();
  const rows = data?.weekly ?? [];
  const pct = [...rows].reverse().find((r) => r.stake_order_perc_avg != null)?.stake_order_perc_avg;
  const text = pct != null ? `${base ?? ''} · ${fmtPctOfSupply(pct)}` : base;
  return <>{text}</>;
};

export default function WidgetRoute() {
  const loaderData = useLoaderData<typeof loader>();
  const [searchParams] = useSearchParams();
  const isEmbed = searchParams.get('embed') === 'true';
  const symbol = searchParams.get('symbol') ?? '';
  const { widgetId } = loaderData;
  const [volPeriod, setVolPeriod] = useState<Period>('30D');
  const [overviewGran, setOverviewGran] = useState<Granularity>('weekly');
  const [dexSearch, setDexSearch] = useState('');

  const meta = WIDGET_META[widgetId];
  if (!meta) {
    return <div style={{ color: '#fff' }}>Unknown widget: {widgetId}</div>;
  }

  if (isMarketWidget(widgetId) && !symbol) {
    return (
      <div
        style={{
          color: '#fff',
          padding: 24,
          fontFamily:
            '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif'
        }}
      >
        Symbol required: append <code>?symbol=PERP_BTC_USDC</code> to the URL.
      </div>
    );
  }

  const isKpi = KPI_WIDGET_IDS.includes(widgetId);

  const { mainRows, tvlChains, tvlTotal, marketRows } = loaderData;
  const fmtCompact = (n: number) => {
    if (n >= 1e9) return `$${(n / 1e9).toFixed(2)}B`;
    if (n >= 1e6) return `$${(n / 1e6).toFixed(2)}M`;
    if (n >= 1e3) return `$${(n / 1e3).toFixed(1)}K`;
    return `$${n.toFixed(0)}`;
  };

  const subtitle = (() => {
    if (widgetId === 'tvl-chain') return `Total: ${fmtCompact(tvlTotal)}`;
    if (widgetId === 'stake-vs-supply') {
      return <StakeVsSupplySubtitle base={meta.subtitle} />;
    }
    return meta.subtitle;
  })();

  const titleSuffix = (() => {
    if (!symbol) return '';
    const parts = symbol.split('_');
    const base = parts.length >= 2 ? parts[1] : symbol;
    return ` — ${base}-PERP`;
  })();
  const titleWithSymbol = `${meta.title}${titleSuffix}`;

  const dexSearchInput = (
    <input
      type="text"
      value={dexSearch}
      onChange={(e) => setDexSearch(e.target.value)}
      placeholder="Search broker…"
      className="rounded text-xs px-2.5 py-1 outline-none"
      style={{
        width: 140,
        background: '#130E1D',
        border: '1px solid rgba(156,117,255,0.18)',
        color: '#ffffff'
      }}
    />
  );

  const controls =
    widgetId === 'dex-users' ? (
      dexSearchInput
    ) : meta.hasPeriodControl ? (
      <PeriodSelector period={volPeriod} onChange={setVolPeriod} />
    ) : meta.hasGranularityControl ? (
      <GranularitySelector granularity={overviewGran} onChange={setOverviewGran} />
    ) : undefined;

  let widgetContent: React.ReactNode;

  const fullData: DashboardData = { mainRows, tvlChains, tvlTotal, marketRows };

  switch (widgetId) {
    case 'volume':
      widgetContent = <VolumeChartWidget rows={mainRows} period={volPeriod} />;
      break;
    case 'tvl-chain':
      widgetContent = <TvlByChainWidget chains={tvlChains} />;
      break;
    case 'net-fees':
      widgetContent = <NetFeesWidget rows={mainRows} />;
      break;
    case 'overview':
      widgetContent = <OverviewWidget granularity={overviewGran} />;
      break;
    case 'dex-users':
      widgetContent = <DexUsersWidget search={dexSearch} onSearchChange={setDexSearch} />;
      break;
    case 'volume-segments':
      widgetContent = <VolumeSegmentsWidget />;
      break;
    case 'omnivault-tvl':
      widgetContent = <OmnivaultTvlWidget />;
      break;
    case 'stake-users':
      widgetContent = <StakeUsersWidget />;
      break;
    case 'stake-vs-supply':
      widgetContent = <StakeVsSupplyWidget />;
      break;
    case 'distributors':
      widgetContent = <DistributorsWidget />;
      break;
    case 'builder-volume':
      widgetContent = <BuilderVolumeWidget />;
      break;
    case 'builder-active-traders':
      widgetContent = <BuilderActiveTradersWidget />;
      break;
    case 'builder-revenue':
      widgetContent = <BuilderRevenueWidget />;
      break;
    case 'net-flow-by-builder':
      widgetContent = <NetFlowByBuilderWidget />;
      break;
    case 'tvl-by-token':
      widgetContent = <TvlByTokenWidget />;
      break;
    case 'fees-stats':
      widgetContent = <FeesStatsWidget data={fullData} />;
      break;
    case 'leaderboard':
      widgetContent = <Leaderboard />;
      break;
    case 'positions':
      widgetContent = <Positions hideFilters hideTitle hideQuickActions />;
      break;
    case 'kpi-analyst':
      widgetContent = <AnalystKPIWidget data={fullData} />;
      break;
    case 'funding-rates':
      widgetContent = <FundingRatesWidget />;
      break;
    case 'staking-daily':
      widgetContent = <StakingDailyWidget />;
      break;
    case 'fund-flows-by-chain':
      widgetContent = <FundFlowsByChainWidget />;
      break;
    case 'liquidations-by-symbol':
      widgetContent = <LiquidationsBySymbolWidget />;
      break;
    case 'liquidation-heatmap':
      widgetContent = <LiquidationHeatmapWidget symbol={symbol || undefined} />;
      break;
    case 'insurance-fund':
      widgetContent = <InsuranceFundWidget />;
      break;
    case 'market-price-chart':
    case 'market-orderbook':
    case 'market-recent-trades':
    case 'market-funding-chart':
    case 'market-recent-liquidations':
    case 'market-top-traders':
    case 'market-platform-positions':
      widgetContent = <MarketWidgetContent widgetId={widgetId} symbol={symbol} />;
      break;
    default:
      widgetContent = null;
  }

  const needsGrid = isKpi || widgetId === 'fees-stats';
  const gridStyles = needsGrid ? (
    <style>{`.dash-grid-sm{display:grid;grid-template-columns:repeat(auto-fit,minmax(170px,1fr));gap:12px}`}</style>
  ) : null;

  // Market-detail panels ship with their own card chrome + header (incl. share button),
  // so they must not be wrapped in another WidgetWrapper.
  if (isMarketWidget(widgetId)) {
    if (isEmbed) {
      return (
        <>
          {gridStyles}
          <div
            style={{
              padding: 24,
              minHeight: '100vh',
              color: '#fff',
              fontFamily:
                '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif'
            }}
          >
            {widgetContent}
          </div>
        </>
      );
    }

    return (
      <>
        {gridStyles}
        <div style={{ display: 'flex', flexDirection: 'column', gap: 20 }}>
          <Link
            to="/markets"
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: 6,
              fontSize: 12,
              color: 'rgba(156,117,255,0.6)',
              textDecoration: 'none',
              marginBottom: -8
            }}
          >
            <svg
              width="12"
              height="12"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <line x1="19" y1="12" x2="5" y2="12" />
              <polyline points="12 19 5 12 12 5" />
            </svg>
            Back to Markets
          </Link>

          {widgetContent}

          <CopyBlock widgetId={widgetId} symbol={symbol || undefined} />
        </div>
      </>
    );
  }

  if (isEmbed) {
    return (
      <>
        {gridStyles}
        <div
          style={{
            padding: 24,
            minHeight: '100vh',
            color: '#fff',
            fontFamily:
              '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif'
          }}
        >
          <WidgetWrapper
            widgetId={widgetId}
            title={titleWithSymbol}
            subtitle={subtitle}
            height={meta.height}
            controls={controls}
            hideLink
            symbol={symbol || undefined}
          >
            {widgetContent}
          </WidgetWrapper>
        </div>
      </>
    );
  }

  return (
    <>
      {gridStyles}
      <div style={{ display: 'flex', flexDirection: 'column', gap: 20 }}>
        <Link
          to="/"
          style={{
            display: 'inline-flex',
            alignItems: 'center',
            gap: 6,
            fontSize: 12,
            color: 'rgba(156,117,255,0.6)',
            textDecoration: 'none',
            marginBottom: -8
          }}
        >
          <svg
            width="12"
            height="12"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <line x1="19" y1="12" x2="5" y2="12" />
            <polyline points="12 19 5 12 12 5" />
          </svg>
          Back to Dashboard
        </Link>

        <WidgetWrapper
          widgetId={widgetId}
          title={titleWithSymbol}
          subtitle={subtitle}
          height={meta.height}
          controls={controls}
          hideLink
          symbol={symbol || undefined}
        >
          {widgetContent}
        </WidgetWrapper>

        <CopyBlock widgetId={widgetId} symbol={symbol || undefined} />
      </div>
    </>
  );
}
