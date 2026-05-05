import { FC, useState } from 'react';

import { fmtCompact } from '../shared/formatters';
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
import type { DashboardData } from '~/types/dashboard';

type Props = { role: Role; data: DashboardData };

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
  <div
    className="rounded-[10px] py-[14px] px-4"
    style={{
      background: 'rgba(255,255,255,0.03)',
      border: `1px solid color-mix(in srgb, ${color} 12%, transparent)`
    }}
  >
    <div className="text-[10px] font-semibold text-[rgba(255,255,255,0.38)] uppercase tracking-[0.08em] mb-2">
      {label}
    </div>
    <div className="text-lg font-bold" style={{ color }}>
      {value}
    </div>
  </div>
);

export const DashboardsView: FC<Props> = ({ role, data }) => {
  const { mainRows, tvlChains } = data;

  const [volPeriod, setVolPeriod] = useState<Period>('30D');

  const cumFees = mainRows[0]?.cumulative_revenue_usd ?? 0;
  const dailyFees = mainRows[0]?.daily_revenue_usd ?? 0;
  const fees30d = mainRows[0]?.rolling_30d_revenue_usd ?? 0;
  const rollingAvgFee = (mainRows[0]?.rolling_30d_revenue_usd ?? 0) / 30;
  const builderFees = mainRows[0]?.cumulative_broker_fees_usd ?? 0;
  const tvlTotal = tvlChains.reduce((s, c) => s + c.tvl_usd, 0);
  const tvlSubtitle = `Total: ${fmtCompact(tvlTotal)}`;

  const KPI_WIDGET: Record<Role, { id: string; el: React.ReactNode }> = {
    trader: { id: 'kpi-trader', el: <TraderKPIWidget data={data} /> },
    builder: { id: 'kpi-builder', el: <BuilderKPIWidget data={data} /> },
    analyst: { id: 'kpi-analyst', el: <AnalystKPIWidget data={data} /> }
  };

  return (
    <div className="flex flex-col gap-6">
      <div className="flex items-center gap-[10px]">
        <span
          className="rounded-[20px] py-[3px] px-3 text-xs font-semibold text-[#9C75FF]"
          style={{
            background: 'rgba(156,117,255,0.12)',
            border: '1px solid rgba(156,117,255,0.25)'
          }}
        >
          {role.charAt(0).toUpperCase() + role.slice(1)} View
        </span>
        <span className="text-[13px] text-[rgba(255,255,255,0.4)]">{ROLE_TITLES[role]}</span>
      </div>

      <WidgetWrapper widgetId={KPI_WIDGET[role].id} title="Key Metrics">
        {KPI_WIDGET[role].el}
      </WidgetWrapper>

      {role === 'trader' && (
        <>
          <WidgetWrapper
            widgetId="volume"
            title="Trading Volume — Daily"
            height={260}
            controls={<PeriodSelector period={volPeriod} onChange={setVolPeriod} />}
          >
            <VolumeChartWidget rows={mainRows} period={volPeriod} />
          </WidgetWrapper>

          <WidgetWrapper widgetId="dex-users" title="DEX Users by Broker">
            <DexUsersWidget />
          </WidgetWrapper>

          <WidgetWrapper
            widgetId="volume-segments"
            title="Volume Segments"
            subtitle="weekly by segment (2B / 2C / MM)"
            height={260}
          >
            <VolumeSegmentsWidget />
          </WidgetWrapper>

          <WidgetWrapper
            widgetId="tvl-chain"
            title="TVL by Chain"
            subtitle={tvlSubtitle}
            height={260}
          >
            <TvlByChainWidget chains={tvlChains} />
          </WidgetWrapper>

          <WidgetWrapper widgetId="fees-stats" title="Fees &amp; Revenue">
            <div className="dash-grid-sm">
              <StatPill label="Net Fees (24h)" value={fmtCompact(dailyFees)} color="#34d399" />
              <StatPill label="Net Fees (30D)" value={fmtCompact(fees30d)} color="#34d399" />
              <StatPill
                label="Builder Fees (total)"
                value={fmtCompact(builderFees)}
                color="#f59e0b"
              />
              <StatPill label="Cum. Net Fees" value={fmtCompact(cumFees)} color="#9C75FF" />
            </div>
          </WidgetWrapper>
        </>
      )}

      {role === 'builder' && (
        <>
          <WidgetWrapper widgetId="fees-stats" title="Fees &amp; Revenue">
            <div className="dash-grid-sm">
              <StatPill
                label="Rolling Avg Daily Fee"
                value={`${fmtCompact(rollingAvgFee)}/day`}
                color="#f59e0b"
              />
              <StatPill label="Net Fees (24h)" value={fmtCompact(dailyFees)} color="#34d399" />
              <StatPill label="Net Fees (30D)" value={fmtCompact(fees30d)} color="#34d399" />
              <StatPill
                label="Builder Fees (total)"
                value={fmtCompact(builderFees)}
                color="#f59e0b"
              />
            </div>
          </WidgetWrapper>

          <div className="dash-grid-lg">
            <WidgetWrapper
              widgetId="tvl-chain"
              title="TVL by Chain"
              subtitle={tvlSubtitle}
              height={260}
            >
              <TvlByChainWidget chains={tvlChains} />
            </WidgetWrapper>
            <WidgetWrapper
              widgetId="net-fees"
              title="Net Fees (cumulative)"
              subtitle="90-day running total"
              height={260}
            >
              <NetFeesWidget rows={mainRows} />
            </WidgetWrapper>
          </div>

          <WidgetWrapper
            widgetId="omnivault-tvl"
            title="Omnivault TVL"
            subtitle="avg weekly TVL per vault (USD millions)"
            height={260}
          >
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

          <WidgetWrapper
            widgetId="volume"
            title="Trading Volume — Daily"
            height={260}
            controls={<PeriodSelector period={volPeriod} onChange={setVolPeriod} />}
          >
            <VolumeChartWidget rows={mainRows} period={volPeriod} />
          </WidgetWrapper>

          <div className="dash-grid-lg">
            <WidgetWrapper
              widgetId="tvl-chain"
              title="TVL by Chain"
              subtitle={tvlSubtitle}
              height={260}
            >
              <TvlByChainWidget chains={tvlChains} />
            </WidgetWrapper>
            <WidgetWrapper
              widgetId="net-fees"
              title="Net Fees (cumulative)"
              subtitle="90-day running total"
              height={260}
            >
              <NetFeesWidget rows={mainRows} />
            </WidgetWrapper>
          </div>

          <WidgetWrapper widgetId="fees-stats" title="Fees &amp; Revenue">
            <div className="dash-grid-sm">
              <StatPill label="Net Fees (24h)" value={fmtCompact(dailyFees)} color="#34d399" />
              <StatPill label="Net Fees (30D)" value={fmtCompact(fees30d)} color="#34d399" />
              <StatPill label="Cumulative Net Fees" value={fmtCompact(cumFees)} color="#34d399" />
              <StatPill
                label="Rolling Avg Daily Fee"
                value={`${fmtCompact(rollingAvgFee)}/day`}
                color="#9C75FF"
              />
            </div>
          </WidgetWrapper>

          <WidgetWrapper widgetId="dex-users" title="DEX Users by Broker">
            <DexUsersWidget />
          </WidgetWrapper>

          <div>
            <SectionHeading>Volume &amp; Liquidity</SectionHeading>
            <div className="dash-grid-lg">
              <WidgetWrapper
                widgetId="volume-segments"
                title="Volume Segments"
                subtitle="weekly by segment (2B / 2C / MM)"
                height={260}
              >
                <VolumeSegmentsWidget />
              </WidgetWrapper>
              <WidgetWrapper
                widgetId="omnivault-tvl"
                title="Omnivault TVL"
                subtitle="avg weekly TVL per vault (USD millions)"
                height={260}
              >
                <OmnivaultTvlWidget />
              </WidgetWrapper>
            </div>
          </div>

          <div>
            <SectionHeading>Staking</SectionHeading>
            <div className="dash-grid-lg">
              <WidgetWrapper
                widgetId="stake-users"
                title="Active Stakers"
                subtitle="weekly avg active ORDER stakers"
                height={220}
              >
                <StakeUsersWidget />
              </WidgetWrapper>
              <WidgetWrapper
                widgetId="stake-vs-supply"
                title="Stake vs Circulating Supply"
                subtitle="weekly ORDER token staking vs circulating supply"
                height={220}
              >
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
  );
};
