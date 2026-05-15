import { FC, useState } from 'react';

import { fmtCompact, fmtPctOfSupply } from '../shared/formatters';
import {
  GranularitySelector,
  PeriodSelector,
  SectionHeading,
  type Granularity,
  type Period
} from '../shared/primitives';
import { AnalystKPIWidget } from '../widgets/AnalystKPIWidget';
import { DexUsersWidget } from '../widgets/DexUsersWidget';
import { DistributorsWidget } from '../widgets/DistributorsWidget';
import { FeesStatsWidget } from '../widgets/FeesStatsWidget';
import { NetFeesWidget } from '../widgets/NetFeesWidget';
import { OmnivaultTvlWidget } from '../widgets/OmnivaultTvlWidget';
import { OverviewWidget } from '../widgets/OverviewWidget';
import { StakeUsersWidget } from '../widgets/StakeUsersWidget';
import { StakeVsSupplyWidget } from '../widgets/StakeVsSupplyWidget';
import { TvlByChainWidget } from '../widgets/TvlByChainWidget';
import { VolumeChartWidget } from '../widgets/VolumeChartWidget';
import { VolumeSegmentsWidget } from '../widgets/VolumeSegmentsWidget';
import { WidgetWrapper } from '../widgets/WidgetWrapper';

import { useStakeVsSupply } from '~/hooks/useOrderlyMetrics';
import type { DashboardData } from '~/types/dashboard';

type Props = { data: DashboardData };

export const DashboardsView: FC<Props> = ({ data }) => {
  const { mainRows, tvlChains, tvlTotal } = data;

  const [volPeriod, setVolPeriod] = useState<Period>('30D');
  const [overviewGran, setOverviewGran] = useState<Granularity>('weekly');

  const tvlSubtitle = `Total: ${fmtCompact(tvlTotal)}`;

  const { data: stakeVsSupplyData } = useStakeVsSupply();
  const stakeRows = stakeVsSupplyData?.weekly ?? [];
  const latestStakePct = [...stakeRows]
    .reverse()
    .find((r) => r.stake_order_perc_avg != null)?.stake_order_perc_avg;
  const stakeSubtitle =
    latestStakePct != null
      ? `weekly ORDER token staking vs circulating supply · ${fmtPctOfSupply(latestStakePct)}`
      : 'weekly ORDER token staking vs circulating supply';

  return (
    <div className="flex flex-col gap-6">
      <WidgetWrapper widgetId="kpi-analyst" title="Key Metrics">
        <AnalystKPIWidget data={data} />
      </WidgetWrapper>

      <WidgetWrapper
        widgetId="overview"
        title="Protocol Overview"
        controls={<GranularitySelector granularity={overviewGran} onChange={setOverviewGran} />}
      >
        <OverviewWidget granularity={overviewGran} />
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
        <WidgetWrapper widgetId="tvl-chain" title="TVL by Chain" subtitle={tvlSubtitle}>
          <TvlByChainWidget chains={tvlChains} />
        </WidgetWrapper>
        <WidgetWrapper
          widgetId="net-fees"
          title="Net Fees (cumulative)"
          subtitle="90-day running total"
        >
          <NetFeesWidget rows={mainRows} />
        </WidgetWrapper>
      </div>

      <WidgetWrapper widgetId="fees-stats" title="Fees &amp; Revenue">
        <FeesStatsWidget data={data} />
      </WidgetWrapper>

      <WidgetWrapper widgetId="dex-users" title="Users by DEX">
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
            subtitle={stakeSubtitle}
            height={220}
          >
            <StakeVsSupplyWidget />
          </WidgetWrapper>
        </div>
      </div>

      <WidgetWrapper widgetId="distributors" title="Distributors">
        <DistributorsWidget />
      </WidgetWrapper>
    </div>
  );
};
