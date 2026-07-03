import { FC } from 'react';

import { fmtCompact, fmtDeltaPct } from '../shared/formatters';

import { KPICard } from './KPICard';

import { useIsMobile, useMediaQuery } from '~/hooks/useMediaQuery';
import type { DashboardData } from '~/types/dashboard';

type Props = { data: DashboardData };

export const AnalystKPIWidget: FC<Props> = ({ data }) => {
  const { mainRows, marketRows, tvlTotal } = data;
  const isMobile = useIsMobile(768);
  const isTablet = useMediaQuery('(min-width: 768px) and (max-width: 1279px)');
  const isWide = useMediaQuery('(min-width: 1600px)');
  const isDesktop = !isMobile && !isTablet && !isWide; // 1280–1599px

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

  const cols = isMobile ? 2 : isWide ? 9 : isTablet ? 3 : 4;
  const rowH = isMobile ? 110 : isWide ? 125 : 115;

  // Wide layout (9 cols, 2 rows):
  //  col:  1        2       3       4         5        6       7         8       9
  //  r1:   DayVol──────────────── Accounts  30DVol──────── Builders  CumVol──────
  //  r2:   Markets  TVL    CumDEX  Accounts  NetFees──────── Builders  CumVol──────
  //
  // Desktop layout (4 cols, 3 rows):  1280–1599px
  //  col:  1            2            3          4
  //  r1:   DayVol ─────────────    Builders   Accounts
  //  r2:   CumDEX←     TVL        NetFees     Markets
  //  r3:   CumDEX←     30DVol     CumVol ─────────
  //
  // Tablet layout (3 cols, 5 rows):  768–1279px
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
    <div
      style={{
        display: 'grid',
        gridTemplateColumns: `repeat(${cols}, 1fr)`,
        gridAutoRows: rowH,
        gap: 10
      }}
    >
      {/* Day Volume: spans 2 rows on tablet → wrapBadge there */}
      <KPICard
        size={isMobile ? 'md' : isWide ? 'md' : isDesktop ? 'md' : 'lg'}
        bgColor="#6700CE"
        label="Day Volume"
        value={fmtCompact(todayVol)}
        delta={fmtDeltaPct(todayVol, yestVol)}
        subValue="vs yesterday"
        wrapBadge={isTablet}
        cardStyle={{
          gridColumn: isMobile ? 'span 2' : isWide ? '1 / span 3' : '1 / span 2',
          gridRow: isMobile ? '1' : isWide ? '1' : isTablet ? '1 / span 2' : '1'
        }}
      />

      {/* 30D Volume: spans 2 rows on tablet → wrapBadge there */}
      <KPICard
        size={isMobile ? 'sm' : 'md'}
        bgColor="#3F0086"
        label="30D Volume"
        value={fmtCompact(vol30d)}
        delta={fmtDeltaPct(vol30d, vol30dPrev)}
        subValue="rolling"
        wrapBadge={isTablet}
        cardStyle={
          isMobile
            ? { gridColumn: 'span 2', gridRow: '4' }
            : isWide
              ? { gridColumn: '5 / span 2', gridRow: '1' }
              : isTablet
                ? { gridColumn: '2 / span 1', gridRow: '3 / span 2' }
                : isDesktop
                  ? { gridColumn: '2', gridRow: '3' }
                  : undefined
        }
      />

      {/* Total Accounts: tall on wide + mobile → wrapBadge */}
      <KPICard
        size={isWide ? 'lg' : isMobile ? 'lg' : 'sm'}
        bgColor="#E9DEFF"
        label="Total Accounts"
        value={
          totalAccounts >= 1e6
            ? `${(totalAccounts / 1e6).toFixed(2)}M`
            : totalAccounts.toLocaleString()
        }
        delta={fmtDeltaPct(totalAccounts, prevWeekAccounts)}
        wrapBadge={isWide || isMobile}
        cardStyle={
          isMobile
            ? { gridColumn: '1', gridRow: '2 / span 2' }
            : isWide
              ? { gridColumn: '7', gridRow: '1 / span 2' }
              : undefined
        }
      />
      {/* Open Markets: desktop→ r2c2 */}
      <KPICard
        size={isWide ? 'lg' : isMobile ? 'lg' : 'sm'}
        bgColor="#3F0086"
        label="Open Markets"
        value={`${latestMarkets}`}
        delta={fmtDeltaPct(latestMarkets, prevMarkets)}
        cardStyle={isDesktop ? { gridColumn: '4', gridRow: '2' } : undefined}
      />
      {/* Active Builders: wide→ tall; desktop→ r3c2 */}
      <KPICard
        size={isWide ? 'xl' : isMobile ? 'lg' : 'sm'}
        bgColor="#9C75FF"
        label="Active Builders"
        value={`${activeBuilders}`}
        subValue={isMobile ? undefined : `Fees: ${fmtCompact(builderFees)}`}
        wrapBadge={isWide}
        cardStyle={
          isWide
            ? { gridColumn: '4', gridRow: '1 / span 2' }
            : isDesktop
              ? { gridColumn: '3', gridRow: '1' }
              : undefined
        }
      />
      {/* Total TVL: desktop→ r2c2 */}
      <KPICard
        size={isWide ? 'lg' : isMobile ? 'md' : 'sm'}
        bgColor="#9C75FF"
        label="Total TVL"
        value={fmtCompact(tvlTotal)}
        cardStyle={isDesktop ? { gridColumn: '2', gridRow: '2' } : undefined}
      />
      {/* Cum. DEX Fees: desktop→ r2-3 c1 */}
      <KPICard
        size={isWide ? 'lg' : isDesktop ? 'lg' : isMobile ? 'md' : 'sm'}
        bgColor="#E9DEFF"
        label="Cum. DEX Fees"
        value={fmtCompact(builderFees)}
        cardStyle={isDesktop ? { gridColumn: '1', gridRow: '2 / span 2' } : undefined}
      />
      {/* Cum. Net Fees: mobile→ tall; wide→ r2; desktop→ r2c3 */}
      <KPICard
        size={isWide ? 'md' : isMobile ? 'lg' : 'sm'}
        bgColor="#6700CE"
        label="Cum. Orderly Net Fees"
        value={fmtCompact(cumulativeNetFees)}
        wrapBadge={isMobile}
        cardStyle={
          isMobile
            ? { gridColumn: '2', gridRow: '5 / span 2' }
            : isWide
              ? { gridColumn: '5 / span 2', gridRow: '2' }
              : isDesktop
                ? { gridColumn: '3', gridRow: '2' }
                : undefined
        }
      />

      {/* Cumulative Volume: tall on wide → wrapBadge */}
      <KPICard
        size={isWide ? 'lg' : isDesktop ? 'md' : 'sm'}
        bgColor="#9C75FF"
        label="Cumulative Volume"
        value={fmtCompact(cumVol)}
        wrapBadge={isWide}
        cardStyle={
          isWide
            ? { gridColumn: '8 / span 2', gridRow: '1 / span 2' }
            : isDesktop
              ? { gridColumn: '3 / span 2', gridRow: '3' }
              : { gridColumn: `span ${cols}` }
        }
      />
    </div>
  );
};
