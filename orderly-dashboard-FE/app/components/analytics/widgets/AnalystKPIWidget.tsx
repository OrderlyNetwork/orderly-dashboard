import { FC } from 'react';

import { fmtCompact, fmtDeltaPct } from '../shared/formatters';

import { KPICard } from './KPICard';

import { useIsMobile } from '~/hooks/useMediaQuery';
import type { DashboardData } from '~/types/dashboard';

type Props = { data: DashboardData };

export const AnalystKPIWidget: FC<Props> = ({ data }) => {
  const { mainRows, marketRows, tvlTotal } = data;
  const isMobile = useIsMobile(768);

  const todayVol = mainRows[0]?.taker_volume_usd ?? 0;
  const yestVol = mainRows[1]?.taker_volume_usd ?? 0;
  const cumVol = mainRows[0]?.cumulative_volume_usd ?? 0;
  const vol30d = mainRows[0]?.rolling_30d_volume_usd ?? 0;
  const vol30dPrev = mainRows[7]?.rolling_30d_volume_usd ?? 0;
  const totalAccounts = mainRows[0]?.cumulative_accounts ?? 0;
  const prevWeekAccounts = mainRows[7]?.cumulative_accounts ?? 0;
  const latestMarkets = marketRows[0]?.markets ?? 0;
  const prevMarkets = marketRows[1]?.markets ?? 0;
  const builderFees = mainRows[0]?.cumulative_broker_fees_usd ?? 0;
  const activeBuilders = mainRows[0]?.active_builders_count ?? 0;
  const cumulativeNetFees = mainRows[0]?.cumulative_revenue_usd ?? 0;

  const cols = isMobile ? 2 : 3;
  const rowH = isMobile ? 95 : 115;

  // Desktop layout (3 cols, 5 rows):
  //  col:  1            2            3
  //  r1:   DayVol ──── DayVol       Accounts
  //  r2:   DayVol ──── DayVol       Markets
  //  r3:   Builders    30DVol ───   TVL
  //  r4:   CumDEX      30DVol ───   NetFees
  //  r5:   CumVol ─────────────── CumVol
  //
  // Mobile layout (2 cols, 7 rows):
  //  col:  1              2
  //  r1:   DayVol ─────────────── (single height)
  //  r2-3: Accounts(2r)  Markets
  //        Accounts(2r)  Builders
  //  r4:   30DVol ─────────────── (single height)
  //  r5-6: TVL           NetFees(2r)
  //        CumDEX         NetFees(2r)
  //  r7:   CumVol ───────────────

  return (
    <div style={{ display: 'grid', gridTemplateColumns: `repeat(${cols}, 1fr)`, gridAutoRows: rowH, gap: 10, maxWidth: 1080, margin: '0 auto' }}>

      {/* Day Volume: col 1-2 × row 1-2 on desktop; full-width single row on mobile */}
      <KPICard
        size={isMobile ? 'md' : 'lg'}
        bgColor="#6700CE"
        label="Day Volume"
        value={fmtCompact(todayVol)}
        delta={fmtDeltaPct(todayVol, yestVol)}
        subValue="vs yesterday"
        cardStyle={{
          gridColumn: isMobile ? 'span 2' : '1 / span 2',
          gridRow: isMobile ? '1' : '1 / span 2'
        }}
      />

      {/* 30D Volume: col 2 × row 3-4 on desktop; full-width row 4 on mobile */}
      <KPICard
        size={isMobile ? 'sm' : 'md'}
        bgColor="#3F0086"
        label="30D Volume"
        value={fmtCompact(vol30d)}
        delta={fmtDeltaPct(vol30d, vol30dPrev)}
        subValue="rolling"
        cardStyle={{
          gridColumn: isMobile ? 'span 2' : '2 / span 1',
          gridRow: isMobile ? '4' : '3 / span 2'
        }}
      />

      {/* Total Accounts: tall narrow card on left on mobile (rows 2-3) */}
      <KPICard
        size="sm"
        bgColor="#E9DEFF"
        label="Total Accounts"
        value={
          totalAccounts >= 1e6
            ? `${(totalAccounts / 1e6).toFixed(2)}M`
            : totalAccounts.toLocaleString()
        }
        delta={fmtDeltaPct(totalAccounts, prevWeekAccounts)}
        cardStyle={isMobile ? { gridColumn: '1', gridRow: '2 / span 2' } : undefined}
      />
      {/* Open Markets: auto-fills col 2 row 2 on mobile */}
      <KPICard
        size="sm"
        bgColor="#3F0086"
        label="Open Markets"
        value={`${latestMarkets}`}
        delta={fmtDeltaPct(latestMarkets, prevMarkets)}
      />
      {/* Active Builders: auto-fills col 2 row 3 on mobile */}
      <KPICard
        size="sm"
        bgColor="#9C75FF"
        label="Active Builders"
        value={`${activeBuilders}`}
        subValue={`Fees: ${fmtCompact(builderFees)}`}
      />
      {/* Total TVL: auto-fills col 1 row 5 on mobile */}
      <KPICard
        size="sm"
        bgColor="#9C75FF"
        label="Total TVL"
        value={fmtCompact(tvlTotal)}
      />
      {/* Cum. DEX Fees: auto-fills col 1 row 6 on mobile */}
      <KPICard
        size="sm"
        bgColor="#E9DEFF"
        label="Cum. DEX Fees"
        value={fmtCompact(builderFees)}
      />
      {/* Cum. Net Fees: tall narrow card on right on mobile (rows 5-6) */}
      <KPICard
        size="sm"
        bgColor="#6700CE"
        label="Cum. Orderly Net Fees"
        value={fmtCompact(cumulativeNetFees)}
        cardStyle={isMobile ? { gridColumn: '2', gridRow: '5 / span 2' } : undefined}
      />

      {/* Cumulative Volume: full-width footer row */}
      <KPICard
        size="sm"
        bgColor="#9C75FF"
        label="Cumulative Volume"
        value={fmtCompact(cumVol)}
        cardStyle={{ gridColumn: `span ${cols}` }}
      />
    </div>
  );
};
