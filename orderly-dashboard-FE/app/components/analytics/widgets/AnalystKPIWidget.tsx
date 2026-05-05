import { FC } from 'react';

import { KPICard } from './KPICard';
import { fmtCompact, fmtDeltaPct } from '../shared/formatters';

import type { DuneData } from '~/types/dune';

type Props = { data: DuneData };

export const AnalystKPIWidget: FC<Props> = ({ data }) => {
  const { volumeRows, accountRows, marketRows, builderFees, activeBuilders } = data;
  const todayVol = volumeRows[0]?.volume ?? 0;
  const yestVol = volumeRows[1]?.volume ?? 0;
  const cumVol = volumeRows[0]?.cumulative_volume ?? 0;
  const vol30d = volumeRows[0]?.rolling_total_volume ?? 0;
  const vol30dPrev = volumeRows[7]?.rolling_total_volume ?? 0;
  const totalAccounts = accountRows[0]?.cumulative_accounts ?? 0;
  const prevWeekAccounts = accountRows[7]?.cumulative_accounts ?? 0;
  const latestMarkets = marketRows[0]?.markets ?? 0;
  const prevMarkets = marketRows[1]?.markets ?? 0;

  return (
    <div className="dash-grid-sm">
      <KPICard flat label="Day Volume" value={fmtCompact(todayVol)} delta={fmtDeltaPct(todayVol, yestVol)} subValue="vs yesterday" />
      <KPICard flat label="30D Volume" value={fmtCompact(vol30d)} delta={fmtDeltaPct(vol30d, vol30dPrev)} subValue="rolling" />
      <KPICard flat label="Cumulative Volume" value={fmtCompact(cumVol)} />
      <KPICard flat label="Total Accounts" value={totalAccounts >= 1e6 ? `${(totalAccounts / 1e6).toFixed(2)}M` : totalAccounts.toLocaleString()} delta={fmtDeltaPct(totalAccounts, prevWeekAccounts)} />
      <KPICard flat label="Open Markets" value={`${latestMarkets}`} delta={fmtDeltaPct(latestMarkets, prevMarkets)} />
      <KPICard flat label="Active Builders" value={`${activeBuilders}`} subValue={`Fees: ${fmtCompact(builderFees)}`} />
    </div>
  );
};
