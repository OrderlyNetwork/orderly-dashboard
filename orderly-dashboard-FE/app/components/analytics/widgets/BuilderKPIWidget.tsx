import { FC } from 'react';

import { fmtCompact, fmtDeltaPct } from '../shared/formatters';

import { KPICard } from './KPICard';

import type { DashboardData } from '~/types/dashboard';

type Props = { data: DashboardData };

export const BuilderKPIWidget: FC<Props> = ({ data }) => {
  const { mainRows, tvlChains, marketRows } = data;
  const latestMarkets = marketRows[0]?.markets ?? 0;
  const prevMarkets = marketRows[1]?.markets ?? 0;
  const fees30d = mainRows[0]?.rolling_30d_revenue_usd ?? 0;
  const tvlTotal = tvlChains.reduce((s, c) => s + c.tvl_usd, 0);
  const builderFees = mainRows[0]?.cumulative_broker_fees_usd ?? 0;
  const activeBuilders = mainRows[0]?.active_builders_count ?? 0;

  return (
    <div className="dash-grid-sm">
      <KPICard
        flat
        label="Active Builders"
        value={`${activeBuilders}`}
        subValue="unique broker IDs"
      />
      <KPICard
        flat
        label="Builder Fees (total)"
        value={fmtCompact(builderFees)}
        subValue="cumulative broker fees"
      />
      <KPICard flat label="Net Fees (30D)" value={fmtCompact(fees30d)} subValue="rolling 30-day" />
      <KPICard
        flat
        label="Open Markets"
        value={`${latestMarkets}`}
        delta={fmtDeltaPct(latestMarkets, prevMarkets)}
      />
      <KPICard
        flat
        label="TVL (total)"
        value={fmtCompact(tvlTotal)}
        subValue={`across ${tvlChains.length} chains`}
      />
    </div>
  );
};
