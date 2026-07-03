import { FC } from 'react';

import { fmtCompact } from '../shared/formatters';
import { StatCard } from '../shared/primitives';

import type { DashboardData } from '~/types/dashboard';

type Props = { data: DashboardData };

export const FeesStatsWidget: FC<Props> = ({ data }) => {
  const { mainRows } = data;
  const row = mainRows[0];
  const dailyFees = row?.daily_revenue_usd ?? 0;
  const fees30d = row?.rolling_30d_revenue_usd ?? 0;
  const cumFees = row?.cumulative_revenue_usd ?? 0;
  const builderFees = row?.cumulative_broker_fees_usd ?? 0;
  const rollingAvgFee = fees30d / 30;

  return (
    <div className="dash-grid-sm">
      <StatCard label="Net Fees (24h)" value={fmtCompact(dailyFees)} color="#00dea3" selected />
      <StatCard label="Net Fees (30D)" value={fmtCompact(fees30d)} color="#00dea3" selected />
      <StatCard label="Total Net Fees" value={fmtCompact(cumFees)} color="#9C75FF" selected />
      <StatCard
        label="Builder Fees (total)"
        value={fmtCompact(builderFees)}
        color="#f59e0b"
        selected
      />
      <StatCard
        label="Rolling Avg Daily Fee"
        value={fmtCompact(rollingAvgFee)}
        color="#9C75FF"
        selected
      />
    </div>
  );
};
