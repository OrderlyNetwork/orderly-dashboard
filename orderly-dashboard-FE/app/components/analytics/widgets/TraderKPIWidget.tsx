import { FC } from 'react';

import { fmtCompact, fmtDeltaPct } from '../shared/formatters';

import { KPICard } from './KPICard';

import type { DashboardData } from '~/types/dashboard';

type Props = { data: DashboardData };

export const TraderKPIWidget: FC<Props> = ({ data }) => {
  const { mainRows, marketRows } = data;
  const todayVol = mainRows[0]?.taker_volume_usd ?? 0;
  const yestVol = mainRows[1]?.taker_volume_usd ?? 0;
  const cumVol = mainRows[0]?.cumulative_volume_usd ?? 0;
  const vol30d = mainRows[0]?.rolling_30d_volume_usd ?? 0;
  const vol30dPrev = mainRows[7]?.rolling_30d_volume_usd ?? 0;
  const totalAccounts = mainRows[0]?.cumulative_accounts ?? 0;
  const prevWeekAccounts = mainRows[7]?.cumulative_accounts ?? 0;
  const latestMarkets = marketRows[0]?.markets ?? 0;
  const prevMarkets = marketRows[1]?.markets ?? 0;

  return (
    <div className="dash-grid-sm">
      <KPICard
        flat
        label="Day Volume"
        value={fmtCompact(todayVol)}
        delta={fmtDeltaPct(todayVol, yestVol)}
        subValue="vs yesterday"
      />
      <KPICard
        flat
        label="30D Volume"
        value={fmtCompact(vol30d)}
        delta={fmtDeltaPct(vol30d, vol30dPrev)}
        subValue="rolling 30-day total"
      />
      <KPICard flat label="Cumulative Volume" value={fmtCompact(cumVol)} subValue="all-time" />
      <KPICard
        flat
        label="Open Markets"
        value={`${latestMarkets}`}
        delta={fmtDeltaPct(latestMarkets, prevMarkets)}
      />
      <KPICard
        flat
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
  );
};
