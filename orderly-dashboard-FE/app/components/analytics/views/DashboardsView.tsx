import { FC, useState } from 'react';

import { fmtCompact, fmtDeltaPct } from '../shared/formatters';
import { PeriodSelector, SectionHeading, type Period } from '../shared/primitives';
import { AnalystKPIWidget } from '../widgets/AnalystKPIWidget';
import { BuilderKPIWidget } from '../widgets/BuilderKPIWidget';
import { DexUsersWidget } from '../widgets/DexUsersWidget';
import { DistributorsWidget } from '../widgets/DistributorsWidget';
import { NetFeesWidget } from '../widgets/NetFeesWidget';
import { OmnivaultTvlWidget } from '../widgets/OmnivaultTvlWidget';
import { OverviewWidget } from '../widgets/OverviewWidget';
import { StakeUsersWidget } from '../widgets/StakeUsersWidget';
import { StakeVsSupplyWidget } from '../widgets/StakeVsSupplyWidget';
import { TraderKPIWidget } from '../widgets/TraderKPIWidget';
import { TvlByChainWidget } from '../widgets/TvlByChainWidget';
import { VolumeChartWidget } from '../widgets/VolumeChartWidget';
import { VolumeSegmentsWidget } from '../widgets/VolumeSegmentsWidget';
import { WidgetWrapper } from '../widgets/WidgetWrapper';

import type { Role } from '~/components/analytics/Sidebar';
import type { DuneData } from '~/types/dune';

type Props = { role: Role; data: DuneData };

const ROLE_TITLES: Record<Role, string> = {
  trader: 'Markets · Volume · Users',
  builder: 'Revenue · Ecosystem · Liquidity',
  analyst: 'Full Protocol Overview'
};

const StatPill: FC<{ label: string; value: string; color?: string }> = ({
  label,
  value,
  color = '#34d399'
}) => (
  <div className="dash-stat-pill" style={{ '--pill-color': color } as React.CSSProperties}>
    <div className="dash-stat-pill__label">{label}</div>
    <div className="dash-stat-pill__value">{value}</div>
  </div>
);

export const DashboardsView: FC<Props> = ({ role, data }) => {
  const { volumeRows, tvlChains, feeRows, accountRows, marketRows, builderFees, activeBuilders } =
    data;

  const [volPeriod, setVolPeriod] = useState<Period>('30D');

  const todayVol = volumeRows[0]?.volume ?? 0;
  const yestVol = volumeRows[1]?.volume ?? 0;
  const cumVol = volumeRows[0]?.cumulative_volume ?? 0;
  const vol30d = volumeRows[0]?.rolling_total_volume ?? 0;
  const vol30dPrev = volumeRows[7]?.rolling_total_volume ?? 0;
  const totalAccounts = accountRows[0]?.cumulative_accounts ?? 0;
  const prevWeekAccounts = accountRows[7]?.cumulative_accounts ?? 0;
  const latestMarkets = marketRows[0]?.markets ?? 0;
  const prevMarkets = marketRows[1]?.markets ?? 0;
  const cumFees = feeRows[0]?.cum_rev ?? 0;
  const dailyFees = feeRows[0]?.daily_rev ?? 0;
  const fees30d = feeRows[0]?.rolling_total_rev ?? 0;
  const rollingAvgFee = feeRows[0]?.rolling_rev ?? 0;
  const tvlTotal = tvlChains.reduce((s, c) => s + c.tvl, 0);
  const tvlSubtitle = `Total: ${fmtCompact(tvlTotal)}`;

  const KPI_WIDGET: Record<Role, { id: string; el: React.ReactNode }> = {
    trader: { id: 'kpi-trader', el: <TraderKPIWidget data={data} /> },
    builder: { id: 'kpi-builder', el: <BuilderKPIWidget data={data} /> },
    analyst: { id: 'kpi-analyst', el: <AnalystKPIWidget data={data} /> }
  };

  return (
    <>
      <style>{`
        .dash-grid-sm {
          display: grid;
          grid-template-columns: repeat(auto-fit, minmax(170px, 1fr));
          gap: 12px;
        }
        .dash-grid-lg {
          display: grid;
          grid-template-columns: repeat(2, 1fr);
          gap: 16px;
        }
        @media (max-width: 900px) {
          .dash-grid-lg { grid-template-columns: 1fr; }
        }
        .dash-stat-pill {
          background: rgba(255,255,255,0.03);
          border: 1px solid color-mix(in srgb, var(--pill-color) 12%, transparent);
          border-radius: 10px;
          padding: 14px 16px;
        }
        .dash-stat-pill__label {
          font-size: 10px;
          font-weight: 600;
          color: rgba(255,255,255,0.38);
          text-transform: uppercase;
          letter-spacing: 0.08em;
          margin-bottom: 8px;
        }
        .dash-stat-pill__value {
          font-size: 18px;
          font-weight: 700;
          color: var(--pill-color);
        }
      `}</style>

      <div style={{ display: 'flex', flexDirection: 'column', gap: 24 }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
          <span
            style={{
              background: 'rgba(156,117,255,0.12)',
              border: '1px solid rgba(156,117,255,0.25)',
              borderRadius: 20,
              padding: '3px 12px',
              fontSize: 12,
              fontWeight: 600,
              color: '#9C75FF'
            }}
          >
            {role.charAt(0).toUpperCase() + role.slice(1)} View
          </span>
          <span style={{ fontSize: 13, color: 'rgba(255,255,255,0.4)' }}>{ROLE_TITLES[role]}</span>
        </div>

        <WidgetWrapper widgetId={KPI_WIDGET[role].id} title="Key Metrics">
          {KPI_WIDGET[role].el}
        </WidgetWrapper>

        {role === 'trader' && (
          <>
            <WidgetWrapper widgetId="volume" title="Trading Volume — Daily" height={260} controls={<PeriodSelector period={volPeriod} onChange={setVolPeriod} />}>
              <VolumeChartWidget rows={volumeRows} period={volPeriod} />
            </WidgetWrapper>

            <WidgetWrapper widgetId="dex-users" title="DEX Users by Broker">
              <DexUsersWidget />
            </WidgetWrapper>

            <WidgetWrapper widgetId="volume-segments" title="Volume Segments" subtitle="weekly by segment (2B / 2C / MM)" height={260}>
              <VolumeSegmentsWidget />
            </WidgetWrapper>

            <WidgetWrapper widgetId="tvl-chain" title="TVL by Chain" subtitle={tvlSubtitle} height={260}>
              <TvlByChainWidget chains={tvlChains} />
            </WidgetWrapper>

            <WidgetWrapper widgetId="fees-stats" title="Fees &amp; Revenue">
              <div className="dash-grid-sm">
                <StatPill label="Net Fees (24h)" value={fmtCompact(dailyFees)} color="#34d399" />
                <StatPill label="Net Fees (30D)" value={fmtCompact(fees30d)} color="#34d399" />
                <StatPill label="Builder Fees (total)" value={fmtCompact(builderFees)} color="#f59e0b" />
                <StatPill label="Cum. Net Fees" value={fmtCompact(cumFees)} color="#9C75FF" />
              </div>
            </WidgetWrapper>
          </>
        )}

        {role === 'builder' && (
          <>
            <WidgetWrapper widgetId="fees-stats" title="Fees &amp; Revenue">
              <div className="dash-grid-sm">
                <StatPill label="Rolling Avg Daily Fee" value={`${fmtCompact(rollingAvgFee)}/day`} color="#f59e0b" />
                <StatPill label="Net Fees (24h)" value={fmtCompact(dailyFees)} color="#34d399" />
                <StatPill label="Net Fees (30D)" value={fmtCompact(fees30d)} color="#34d399" />
                <StatPill label="Builder Fees (total)" value={fmtCompact(builderFees)} color="#f59e0b" />
              </div>
            </WidgetWrapper>

            <div className="dash-grid-lg">
              <WidgetWrapper widgetId="tvl-chain" title="TVL by Chain" subtitle={tvlSubtitle} height={260}>
                <TvlByChainWidget chains={tvlChains} />
              </WidgetWrapper>
              <WidgetWrapper widgetId="net-fees" title="Net Fees (cumulative)" subtitle="90-day running total" height={260}>
                <NetFeesWidget rows={feeRows} />
              </WidgetWrapper>
            </div>

            <WidgetWrapper widgetId="omnivault-tvl" title="Omnivault TVL" subtitle="avg weekly TVL per vault (USD millions)" height={260}>
              <OmnivaultTvlWidget />
            </WidgetWrapper>

            <WidgetWrapper widgetId="distributors" title="Distributors">
              <DistributorsWidget />
            </WidgetWrapper>
          </>
        )}

        {role === 'analyst' && (
          <>
            <WidgetWrapper widgetId="overview" title="Protocol Overview (Weekly)">
              <OverviewWidget />
            </WidgetWrapper>

            <WidgetWrapper widgetId="volume" title="Trading Volume — Daily" height={260} controls={<PeriodSelector period={volPeriod} onChange={setVolPeriod} />}>
              <VolumeChartWidget rows={volumeRows} period={volPeriod} />
            </WidgetWrapper>

            <div className="dash-grid-lg">
              <WidgetWrapper widgetId="tvl-chain" title="TVL by Chain" subtitle={tvlSubtitle} height={260}>
                <TvlByChainWidget chains={tvlChains} />
              </WidgetWrapper>
              <WidgetWrapper widgetId="net-fees" title="Net Fees (cumulative)" subtitle="90-day running total" height={260}>
                <NetFeesWidget rows={feeRows} />
              </WidgetWrapper>
            </div>

            <WidgetWrapper widgetId="fees-stats" title="Fees &amp; Revenue">
              <div className="dash-grid-sm">
                <StatPill label="Net Fees (24h)" value={fmtCompact(dailyFees)} color="#34d399" />
                <StatPill label="Net Fees (30D)" value={fmtCompact(fees30d)} color="#34d399" />
                <StatPill label="Cumulative Net Fees" value={fmtCompact(cumFees)} color="#34d399" />
                <StatPill label="Rolling Avg Daily Fee" value={`${fmtCompact(rollingAvgFee)}/day`} color="#9C75FF" />
              </div>
            </WidgetWrapper>

            <WidgetWrapper widgetId="dex-users" title="DEX Users by Broker">
              <DexUsersWidget />
            </WidgetWrapper>

            <div>
              <SectionHeading>Volume &amp; Liquidity</SectionHeading>
              <div className="dash-grid-lg">
                <WidgetWrapper widgetId="volume-segments" title="Volume Segments" subtitle="weekly by segment (2B / 2C / MM)" height={260}>
                  <VolumeSegmentsWidget />
                </WidgetWrapper>
                <WidgetWrapper widgetId="omnivault-tvl" title="Omnivault TVL" subtitle="avg weekly TVL per vault (USD millions)" height={260}>
                  <OmnivaultTvlWidget />
                </WidgetWrapper>
              </div>
            </div>

            <div>
              <SectionHeading>Staking</SectionHeading>
              <div className="dash-grid-lg">
                <WidgetWrapper widgetId="stake-users" title="Active Stakers" subtitle="weekly avg active ORDER stakers" height={220}>
                  <StakeUsersWidget />
                </WidgetWrapper>
                <WidgetWrapper widgetId="stake-vs-supply" title="Stake vs Circulating Supply" subtitle="weekly ORDER token staking vs circulating supply" height={220}>
                  <StakeVsSupplyWidget />
                </WidgetWrapper>
              </div>
            </div>

            <WidgetWrapper widgetId="distributors" title="Distributors">
              <DistributorsWidget />
            </WidgetWrapper>
          </>
        )}
      </div>
    </>
  );
};
