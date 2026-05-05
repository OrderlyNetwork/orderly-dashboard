import { FC } from 'react';

import { KPICard } from './KPICard';
import { fmtCompact, fmtDeltaPct } from '../shared/formatters';

import type { DuneData } from '~/types/dune';

type Props = { data: DuneData };

export const BuilderKPIWidget: FC<Props> = ({ data }) => {
  const { volumeRows, marketRows, builderFees, activeBuilders, feeRows, tvlChains } = data;
  const latestMarkets = marketRows[0]?.markets ?? 0;
  const prevMarkets = marketRows[1]?.markets ?? 0;
  const fees30d = feeRows[0]?.rolling_total_rev ?? 0;
  const tvlTotal = tvlChains.reduce((s, c) => s + c.tvl, 0);

  return (
    <div className="dash-grid-sm">
      <KPICard flat label="Active Builders" value={`${activeBuilders}`} subValue="unique broker IDs" />
      <KPICard flat label="Builder Fees (total)" value={fmtCompact(builderFees)} subValue="cumulative broker fees" />
      <KPICard flat label="Net Fees (30D)" value={fmtCompact(fees30d)} subValue="rolling 30-day" />
      <KPICard flat label="Open Markets" value={`${latestMarkets}`} delta={fmtDeltaPct(latestMarkets, prevMarkets)} />
      <KPICard flat label="TVL (total)" value={fmtCompact(tvlTotal)} subValue={`across ${tvlChains.length} chains`} />
    </div>
  );
};
