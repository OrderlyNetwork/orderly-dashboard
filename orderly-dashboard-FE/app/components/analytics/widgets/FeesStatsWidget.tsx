import { FC } from 'react';

import { fmtCompact } from '../shared/formatters';

import type { DashboardData } from '~/types/dashboard';

type Props = { data: DashboardData };

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
      <StatPill label="Net Fees (24h)" value={fmtCompact(dailyFees)} color="#34d399" />
      <StatPill label="Net Fees (30D)" value={fmtCompact(fees30d)} color="#34d399" />
      <StatPill label="Total Net Fees" value={fmtCompact(cumFees)} color="#9C75FF" />
      <StatPill label="Builder Fees (total)" value={fmtCompact(builderFees)} color="#f59e0b" />
      <StatPill
        label="Rolling Avg Daily Fee"
        value={`${fmtCompact(rollingAvgFee)}/day`}
        color="#9C75FF"
      />
    </div>
  );
};
