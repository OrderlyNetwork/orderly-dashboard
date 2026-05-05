import { json } from '@remix-run/node';
import { Link, useLoaderData, useSearchParams } from '@remix-run/react';
import { useState } from 'react';

import { Leaderboard } from '~/components/Leaderboard';
import { Positions } from '~/components/Positions';
import { PeriodSelector, type Period } from '~/components/analytics/shared/primitives';
import { AnalystKPIWidget } from '~/components/analytics/widgets/AnalystKPIWidget';
import { BuilderKPIWidget } from '~/components/analytics/widgets/BuilderKPIWidget';
import { DexUsersWidget } from '~/components/analytics/widgets/DexUsersWidget';
import { DistributorsWidget } from '~/components/analytics/widgets/DistributorsWidget';
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

const NEEDS_DASHBOARD = ['volume', 'tvl-chain', 'net-fees', ...KPI_WIDGET_IDS];

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
  overview: { title: 'Protocol Overview (Weekly)' },
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
  leaderboard: { title: 'Leaderboard' },
  positions: { title: 'Positions' },
  'kpi-trader': { title: 'Key Metrics — Trader' },
  'kpi-builder': { title: 'Key Metrics — Builder' },
  'kpi-analyst': { title: 'Key Metrics — Analyst' }
};

function CopyBlock({ widgetId }: { widgetId: string }) {
  const [copied, setCopied] = useState(false);
  const origin =
    typeof window !== 'undefined' ? window.location.origin : 'https://dashboard.orderly.network';
  const iframeCode = `<iframe src="${origin}/widget/${widgetId}?embed=true" width="800" height="400" frameborder="0"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(iframeCode).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    });
  };

  return (
    <div style={{ marginTop: 24, maxWidth: 800 }}>
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          marginBottom: 8
        }}
      >
        <span style={{ fontSize: 12, fontWeight: 600, color: 'rgba(255,255,255,0.5)' }}>
          Embed Code
        </span>
        <button
          onClick={handleCopy}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: 6,
            background: copied ? 'rgba(52,211,153,0.15)' : 'rgba(156,117,255,0.1)',
            border: `1px solid ${copied ? 'rgba(52,211,153,0.3)' : 'rgba(156,117,255,0.25)'}`,
            borderRadius: 6,
            color: copied ? '#34d399' : '#9C75FF',
            fontSize: 12,
            fontWeight: 600,
            padding: '4px 10px',
            cursor: 'pointer',
            transition: 'all 0.15s'
          }}
        >
          {copied ? (
            <>
              <svg
                width="12"
                height="12"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="3"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <polyline points="20 6 9 17 4 12" />
              </svg>
              Copied
            </>
          ) : (
            <>
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
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
              </svg>
              Copy
            </>
          )}
        </button>
      </div>
      <pre
        style={{
          background: 'rgba(20,15,35,0.9)',
          border: '1px solid rgba(156,117,255,0.15)',
          borderRadius: 10,
          padding: '14px 18px',
          fontSize: 13,
          fontFamily: '"SF Mono", "Fira Code", "Fira Mono", Menlo, monospace',
          color: 'rgba(255,255,255,0.8)',
          margin: 0,
          overflowX: 'auto',
          lineHeight: 1.6
        }}
      >
        {iframeCode}
      </pre>
    </div>
  );
}

export default function WidgetRoute() {
  const loaderData = useLoaderData<typeof loader>();
  const [searchParams] = useSearchParams();
  const isEmbed = searchParams.get('embed') === 'true';
  const { widgetId } = loaderData;
  const [volPeriod, setVolPeriod] = useState<Period>('30D');

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
      widgetContent = <OverviewWidget />;
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

  const kpiStyles = isKpi ? (
    <style>{`.dash-grid-sm{display:grid;grid-template-columns:repeat(auto-fit,minmax(170px,1fr));gap:12px}`}</style>
  ) : null;

  if (isEmbed) {
    return (
      <>
        {kpiStyles}
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
      {kpiStyles}
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
