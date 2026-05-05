import {
  BarElement,
  CategoryScale,
  Chart as ChartJS,
  Filler,
  LinearScale,
  LineElement,
  PointElement,
  Tooltip
} from 'chart.js';
import { FC, useState } from 'react';

import { fmtCompact, fmtDeltaPct } from '../shared/formatters';
import { PeriodSelector, SectionHeading, StatCard, type Period } from '../shared/primitives';
import { DexUsersWidget } from '../widgets/DexUsersWidget';
import { DistributorsWidget } from '../widgets/DistributorsWidget';
import { KPICard } from '../widgets/KPICard';
import { NetFeesWidget } from '../widgets/NetFeesWidget';
import { OmnivaultTvlWidget } from '../widgets/OmnivaultTvlWidget';
import { OverviewWidget } from '../widgets/OverviewWidget';
import { StakeUsersWidget } from '../widgets/StakeUsersWidget';
import { StakeVsSupplyWidget } from '../widgets/StakeVsSupplyWidget';
import { TvlByChainWidget } from '../widgets/TvlByChainWidget';
import { VolumeChartWidget } from '../widgets/VolumeChartWidget';
import { VolumeSegmentsWidget } from '../widgets/VolumeSegmentsWidget';
import { WidgetWrapper } from '../widgets/WidgetWrapper';

import type { Role } from '~/components/analytics/Sidebar';
import type { DuneData } from '~/types/dune';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Filler,
  Tooltip
);

type Props = { role: Role; data: DuneData };

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
  const tvlTotalStr = fmtCompact(tvlTotal);
  const tvlSubtitle = `Total: ${tvlTotalStr}`;

  const ROLE_TITLES: Record<Role, string> = {
    trader: 'Markets · Volume · Users',
    builder: 'Revenue · Ecosystem · Liquidity',
    analyst: 'Full Protocol Overview'
  };

  return (
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

      {role === 'trader' && (
        <>
          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))',
              gap: 16
            }}
          >
            <KPICard
              label="Day Volume"
              value={fmtCompact(todayVol)}
              delta={fmtDeltaPct(todayVol, yestVol)}
              subValue="vs yesterday"
            />
            <KPICard
              label="30D Volume"
              value={fmtCompact(vol30d)}
              delta={fmtDeltaPct(vol30d, vol30dPrev)}
              subValue="rolling 30-day total"
            />
            <KPICard label="Cumulative Volume" value={fmtCompact(cumVol)} subValue="all-time" />
            <KPICard
              label="Open Markets"
              value={`${latestMarkets}`}
              delta={fmtDeltaPct(latestMarkets, prevMarkets)}
            />
            <KPICard
              label="Total Accounts"
              value={
                totalAccounts >= 1e6
                  ? `${(totalAccounts / 1e6).toFixed(2)}M`
                  : totalAccounts.toLocaleString()
              }
              delta={fmtDeltaPct(totalAccounts, prevWeekAccounts)}
              subValue="vs last week"
            />
          </div>

          <WidgetWrapper
            widgetId="volume"
            title="Trading Volume — Daily"
            height={260}
            controls={<PeriodSelector period={volPeriod} onChange={setVolPeriod} />}
          >
            <VolumeChartWidget rows={volumeRows} period={volPeriod} />
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

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
              gap: 16
            }}
          >
            <StatCard
              label="Net Fees (24h)"
              value={fmtCompact(dailyFees)}
              sub={`30D total: ${fmtCompact(fees30d)}`}
              color="#34d399"
            />
            <StatCard
              label="Builder Fees (total)"
              value={fmtCompact(builderFees)}
              sub={`${activeBuilders} active builders`}
              color="#f59e0b"
            />
            <StatCard
              label="Cum. Net Fees"
              value={fmtCompact(cumFees)}
              sub="all-time protocol revenue"
              color="#9C75FF"
            />
          </div>
        </>
      )}

      {role === 'builder' && (
        <>
          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))',
              gap: 16
            }}
          >
            <KPICard
              label="Active Builders"
              value={`${activeBuilders}`}
              subValue="unique broker IDs"
            />
            <KPICard
              label="Builder Fees (total)"
              value={fmtCompact(builderFees)}
              subValue="cumulative broker fees"
            />
            <KPICard label="Net Fees (30D)" value={fmtCompact(fees30d)} subValue="rolling 30-day" />
            <KPICard
              label="Open Markets"
              value={`${latestMarkets}`}
              delta={fmtDeltaPct(latestMarkets, prevMarkets)}
            />
            <KPICard
              label="TVL (total)"
              value={fmtCompact(tvlTotal)}
              subValue={`across ${tvlChains.length} chains`}
            />
          </div>

          <div
            style={{
              background: 'rgba(20,15,35,.9)',
              border: '1px solid rgba(245,158,11,0.2)',
              borderRadius: 14,
              padding: '16px 24px',
              display: 'flex',
              alignItems: 'center',
              gap: 24
            }}
          >
            <div>
              <div
                style={{
                  fontSize: 11,
                  fontWeight: 600,
                  color: 'rgba(255,255,255,0.38)',
                  textTransform: 'uppercase',
                  letterSpacing: '0.08em',
                  marginBottom: 4
                }}
              >
                Rolling Avg Daily Fee
              </div>
              <div style={{ fontSize: 22, fontWeight: 700, color: '#f59e0b' }}>
                {fmtCompact(rollingAvgFee)}
                <span
                  style={{
                    fontSize: 12,
                    color: 'rgba(255,255,255,0.35)',
                    fontWeight: 400,
                    marginLeft: 6
                  }}
                >
                  /day
                </span>
              </div>
            </div>
            <div style={{ flex: 1, height: 1, background: 'rgba(245,158,11,0.15)' }} />
            <div style={{ textAlign: 'right' }}>
              <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)', marginBottom: 2 }}>
                Net Fees (24h)
              </div>
              <div style={{ fontSize: 18, fontWeight: 700, color: '#34d399' }}>
                {fmtCompact(dailyFees)}
              </div>
            </div>
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
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
              <NetFeesWidget rows={feeRows} />
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

          <div
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))',
              gap: 16
            }}
          >
            <KPICard
              label="Day Volume"
              value={fmtCompact(todayVol)}
              delta={fmtDeltaPct(todayVol, yestVol)}
              subValue="vs yesterday"
            />
            <KPICard
              label="30D Volume"
              value={fmtCompact(vol30d)}
              delta={fmtDeltaPct(vol30d, vol30dPrev)}
              subValue="rolling"
            />
            <KPICard label="Cumulative Volume" value={fmtCompact(cumVol)} />
            <KPICard
              label="Total Accounts"
              value={
                totalAccounts >= 1e6
                  ? `${(totalAccounts / 1e6).toFixed(2)}M`
                  : totalAccounts.toLocaleString()
              }
              delta={fmtDeltaPct(totalAccounts, prevWeekAccounts)}
            />
            <KPICard
              label="Open Markets"
              value={`${latestMarkets}`}
              delta={fmtDeltaPct(latestMarkets, prevMarkets)}
            />
            <KPICard
              label="Active Builders"
              value={`${activeBuilders}`}
              subValue={`Fees: ${fmtCompact(builderFees)}`}
            />
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: 12 }}>
            {[
              { label: 'Net Fees (24h)', value: fmtCompact(dailyFees) },
              { label: 'Net Fees (30D)', value: fmtCompact(fees30d) },
              { label: 'Cumulative Net Fees', value: fmtCompact(cumFees) }
            ].map((s) => (
              <div
                key={s.label}
                style={{
                  background: 'rgba(20,15,35,.9)',
                  border: '1px solid rgba(52,211,153,0.12)',
                  borderRadius: 14,
                  padding: '14px 20px',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'space-between'
                }}
              >
                <div
                  style={{
                    fontSize: 11,
                    fontWeight: 600,
                    color: 'rgba(255,255,255,0.38)',
                    textTransform: 'uppercase',
                    letterSpacing: '0.08em'
                  }}
                >
                  {s.label}
                </div>
                <div style={{ fontSize: 18, fontWeight: 700, color: '#34d399' }}>{s.value}</div>
              </div>
            ))}
          </div>

          <WidgetWrapper
            widgetId="volume"
            title="Trading Volume — Daily"
            height={260}
            controls={<PeriodSelector period={volPeriod} onChange={setVolPeriod} />}
          >
            <VolumeChartWidget rows={volumeRows} period={volPeriod} />
          </WidgetWrapper>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
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
              <NetFeesWidget rows={feeRows} />
            </WidgetWrapper>
          </div>

          <WidgetWrapper widgetId="dex-users" title="DEX Users by Broker">
            <DexUsersWidget />
          </WidgetWrapper>

          <div>
            <SectionHeading>Volume &amp; Liquidity</SectionHeading>
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
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
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
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

          <div
            style={{
              background: 'rgba(20,15,35,.9)',
              border: '1px solid rgba(156,117,255,0.12)',
              borderRadius: 14,
              padding: '16px 24px',
              display: 'flex',
              alignItems: 'center',
              gap: 24
            }}
          >
            <div>
              <div
                style={{
                  fontSize: 11,
                  fontWeight: 600,
                  color: 'rgba(255,255,255,0.38)',
                  textTransform: 'uppercase',
                  letterSpacing: '0.08em',
                  marginBottom: 4
                }}
              >
                Rolling Avg Daily Fee
              </div>
              <div style={{ fontSize: 13, color: 'rgba(255,255,255,0.5)' }}>
                {fmtCompact(rollingAvgFee)}/day
              </div>
            </div>
            <div style={{ flex: 1 }} />
            <div style={{ textAlign: 'right' }}>
              <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)', marginBottom: 2 }}>
                Builder fees (total)
              </div>
              <div style={{ fontSize: 18, fontWeight: 700, color: '#f59e0b' }}>
                {fmtCompact(builderFees)}
              </div>
            </div>
          </div>
        </>
      )}
    </div>
  );
};
