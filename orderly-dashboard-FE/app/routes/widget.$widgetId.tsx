import { json } from '@remix-run/node';
import { Link, useLoaderData, useSearchParams } from '@remix-run/react';
import { useState } from 'react';

import { Leaderboard } from '~/components/Leaderboard';
import { Positions } from '~/components/Positions';
import {
  GranularitySelector,
  PeriodSelector,
  type Granularity,
  type Period
} from '~/components/analytics/shared/primitives';
import { AnalystKPIWidget } from '~/components/analytics/widgets/AnalystKPIWidget';
import { BuilderKPIWidget } from '~/components/analytics/widgets/BuilderKPIWidget';
import { CopyBlock } from '~/components/analytics/widgets/CopyBlock';
import { DexUsersWidget } from '~/components/analytics/widgets/DexUsersWidget';
import { DistributorsWidget } from '~/components/analytics/widgets/DistributorsWidget';
import { FeesStatsWidget } from '~/components/analytics/widgets/FeesStatsWidget';
import { NetFeesWidget } from '~/components/analytics/widgets/NetFeesWidget';
import { OmnivaultTvlWidget } from '~/components/analytics/widgets/OmnivaultTvlWidget';
import { OverviewWidget } from '~/components/analytics/widgets/OverviewWidget';
import { StakeUsersWidget } from '~/components/analytics/widgets/StakeUsersWidget';
import { StakeVsSupplyWidget } from '~/components/analytics/widgets/StakeVsSupplyWidget';
import { TraderKPIWidget } from '~/components/analytics/widgets/TraderKPIWidget';
import { TvlByChainWidget } from '~/components/analytics/widgets/TvlByChainWidget';
import { VolumeChartWidget } from '~/components/analytics/widgets/VolumeChartWidget';
import { VolumeSegmentsWidget } from '~/components/analytics/widgets/VolumeSegmentsWidget';
import { WidgetWrapper } from '~/components/analytics/widgets/WidgetWrapper';
import type { DashboardData } from '~/types/dashboard';
import { fetchDashboardData } from '~/utils/data-api';

type LoaderData = DashboardData & { widgetId: string };

const KPI_WIDGET_IDS = ['kpi-trader', 'kpi-builder', 'kpi-analyst'];

const NEEDS_DASHBOARD = ['volume', 'tvl-chain', 'net-fees', 'fees-stats', ...KPI_WIDGET_IDS];

export async function loader({ params }: { params: { widgetId: string } }) {
  const { widgetId } = params;
  const data: LoaderData = {
    widgetId,
    mainRows: [],
    tvlChains: [],
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
    title: 'TVL by Chain',
    height: 260
  },
  'net-fees': {
    title: 'Net Fees (cumulative)',
    subtitle: '90-day running total',
    height: 260
  },
  overview: { title: 'Protocol Overview', hasGranularityControl: true },
  'dex-users': { title: 'DEX Users by Broker' },
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
  leaderboard: { title: 'Leaderboard' },
  positions: { title: 'Positions' },
  'kpi-trader': { title: 'Key Metrics — Trader' },
  'kpi-builder': { title: 'Key Metrics — Builder' },
  'kpi-analyst': { title: 'Key Metrics — Analyst' }
};

export default function WidgetRoute() {
  const loaderData = useLoaderData<typeof loader>();
  const [searchParams] = useSearchParams();
  const isEmbed = searchParams.get('embed') === 'true';
  const { widgetId } = loaderData;
  const [volPeriod, setVolPeriod] = useState<Period>('30D');
  const [overviewGran, setOverviewGran] = useState<Granularity>('weekly');

  const meta = WIDGET_META[widgetId];
  if (!meta) {
    return <div style={{ color: '#fff' }}>Unknown widget: {widgetId}</div>;
  }

  const isKpi = KPI_WIDGET_IDS.includes(widgetId);

  const { mainRows, tvlChains, marketRows } = loaderData;
  const tvlTotal = tvlChains.reduce((s, c) => s + c.tvl_usd, 0);
  const fmtCompact = (n: number) => {
    if (n >= 1e9) return `$${(n / 1e9).toFixed(2)}B`;
    if (n >= 1e6) return `$${(n / 1e6).toFixed(2)}M`;
    if (n >= 1e3) return `$${(n / 1e3).toFixed(1)}K`;
    return `$${n.toFixed(0)}`;
  };

  const subtitle = widgetId === 'tvl-chain' ? `Total: ${fmtCompact(tvlTotal)}` : meta.subtitle;

  const controls = meta.hasPeriodControl ? (
    <PeriodSelector period={volPeriod} onChange={setVolPeriod} />
  ) : meta.hasGranularityControl ? (
    <GranularitySelector granularity={overviewGran} onChange={setOverviewGran} />
  ) : undefined;

  let widgetContent: React.ReactNode;

  const fullData: DashboardData = { mainRows, tvlChains, marketRows };

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
      widgetContent = <DexUsersWidget />;
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
    case 'fees-stats':
      widgetContent = <FeesStatsWidget data={fullData} />;
      break;
    case 'leaderboard':
      widgetContent = <Leaderboard />;
      break;
    case 'positions':
      widgetContent = <Positions hideFilters hideTitle hideQuickActions />;
      break;
    case 'kpi-trader':
      widgetContent = <TraderKPIWidget data={fullData} />;
      break;
    case 'kpi-builder':
      widgetContent = <BuilderKPIWidget data={fullData} />;
      break;
    case 'kpi-analyst':
      widgetContent = <AnalystKPIWidget data={fullData} />;
      break;
    default:
      widgetContent = null;
  }

  const needsGrid = isKpi || widgetId === 'fees-stats';
  const gridStyles = needsGrid ? (
    <style>{`.dash-grid-sm{display:grid;grid-template-columns:repeat(auto-fit,minmax(170px,1fr));gap:12px}`}</style>
  ) : null;

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
            title={meta.title}
            subtitle={subtitle}
            height={meta.height}
            controls={controls}
            hideLink
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
          title={meta.title}
          subtitle={subtitle}
          height={meta.height}
          controls={controls}
          hideLink
        >
          {widgetContent}
        </WidgetWrapper>

        <CopyBlock widgetId={widgetId} />
      </div>
    </>
  );
}
