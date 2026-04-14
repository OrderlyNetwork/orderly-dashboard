import dayjs from 'dayjs';
import { CSSProperties, FC, useMemo } from 'react';

import { useLeaderboard } from '~/hooks/useLeaderboard';
import { usePositions } from '~/hooks/usePositions';

const formatNumber = (value?: number): string => {
  if (value == null) return '-';
  if (value >= 1_000_000) {
    return `$${(value / 1_000_000).toFixed(1)}M`;
  }
  if (value >= 1_000) {
    return `$${(value / 1_000).toFixed(1)}K`;
  }
  return `$${value.toFixed(2)}`;
};

const formatPnL = (value?: number): { text: string; color: string } => {
  if (value == null) return { text: '-', color: 'text-gray-400' };
  const formatted = formatNumber(Math.abs(value));
  if (value > 0) {
    return { text: `+${formatted}`, color: 'text-success' };
  } else if (value < 0) {
    return { text: `-${formatted}`, color: 'text-error' };
  }
  return { text: formatted, color: 'text-gray-400' };
};

interface PnLStatsProps {
  address: string;
  brokerId?: string;
  accountId?: string;
}

export const PnLStats: FC<PnLStatsProps> = ({ address, brokerId, accountId }) => {
  const today = dayjs();

  const sevenDaysAgo = today.subtract(6, 'days');
  const thirtyDaysAgo = today.subtract(29, 'days');
  const ninetyDaysAgo = today.subtract(89, 'days');

  // Fetch data for different time periods
  const { data: sevenDayData, isLoading: isLoading7d } = useLeaderboard({
    start_date: sevenDaysAgo.format('YYYY-MM-DD'),
    end_date: today.format('YYYY-MM-DD'),
    address,
    broker_id: brokerId,
    aggregateBy: 'address_per_builder'
  });

  const { data: thirtyDayData, isLoading: isLoading30d } = useLeaderboard({
    start_date: thirtyDaysAgo.format('YYYY-MM-DD'),
    end_date: today.format('YYYY-MM-DD'),
    address,
    broker_id: brokerId,
    aggregateBy: 'address_per_builder'
  });

  const { data: ninetyDayData, isLoading: isLoading90d } = useLeaderboard({
    start_date: ninetyDaysAgo.format('YYYY-MM-DD'),
    end_date: today.format('YYYY-MM-DD'),
    address,
    broker_id: brokerId,
    aggregateBy: 'address_per_builder'
  });

  const { data: positionsData, isLoading: isLoadingPositions } = usePositions({
    account_id: accountId
  });

  const sevenDayPnL = useMemo(() => {
    if (!sevenDayData?.rows || sevenDayData.rows.length === 0) return null;
    return sevenDayData.rows.reduce((sum, row) => sum + row.realized_pnl, 0);
  }, [sevenDayData]);

  const thirtyDayPnL = useMemo(() => {
    if (!thirtyDayData?.rows || thirtyDayData.rows.length === 0) return null;
    return thirtyDayData.rows.reduce((sum, row) => sum + row.realized_pnl, 0);
  }, [thirtyDayData]);

  const ninetyDayPnL = useMemo(() => {
    if (!ninetyDayData?.rows || ninetyDayData.rows.length === 0) return null;
    return ninetyDayData.rows.reduce((sum, row) => sum + row.realized_pnl, 0);
  }, [ninetyDayData]);

  const allTimePnL = useMemo(() => {
    if (!positionsData?.rows || positionsData.rows.length === 0) return null;
    return positionsData.rows.reduce((sum, row) => sum + parseFloat(row.total_realized_pnl), 0);
  }, [positionsData]);

  const isLoading = isLoading7d || isLoading30d || isLoading90d || isLoadingPositions;

  const statItemStyle: CSSProperties = {
    flex: 1,
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    gap: '10px',
    padding: '43px 20px',
    borderRight: '1px solid rgba(255,255,255,0.15)'
  };

  const statItemLastStyle: CSSProperties = {
    ...statItemStyle,
    borderRight: 'none'
  };

  if (isLoading) {
    return (
      <div className="stats-bar" style={{ display: 'flex' }}>
        {[1, 2, 3, 4].map((i, idx) => (
          <div key={i} style={idx === 3 ? statItemLastStyle : statItemStyle}>
            <div
              className="animate-pulse"
              style={{ height: 20, width: 80, background: 'rgba(255,255,255,0.15)', borderRadius: 6 }}
            />
            <div
              className="animate-pulse"
              style={{ height: 32, width: 120, background: 'rgba(255,255,255,0.1)', borderRadius: 6 }}
            />
          </div>
        ))}
      </div>
    );
  }

  const sevenDayDisplay = formatPnL(sevenDayPnL ?? undefined);
  const thirtyDayDisplay = formatPnL(thirtyDayPnL ?? undefined);
  const ninetyDayDisplay = formatPnL(ninetyDayPnL ?? undefined);
  const allTimeDisplay = formatPnL(allTimePnL ?? undefined);

  const stats = [
    { label: '7 Day PnL', display: sevenDayDisplay },
    { label: '30 Day PnL', display: thirtyDayDisplay },
    { label: '90 Day PnL', display: ninetyDayDisplay },
    { label: 'All Time PnL', display: allTimeDisplay }
  ];

  return (
    <div
      className="stats-bar"
      style={{ display: 'flex', flexWrap: 'wrap' }}
    >
      {stats.map(({ label, display }, idx) => (
        <div key={label} style={idx === stats.length - 1 ? statItemLastStyle : statItemStyle}>
          <div
            style={{
              fontFamily: 'var(--font-display)',
              fontSize: '1rem',
              fontWeight: 500,
              color: 'rgba(255,255,255,0.8)',
              letterSpacing: '-0.02em',
              textAlign: 'center'
            }}
          >
            {label}
          </div>
          <div
            style={{
              fontFamily: 'var(--font-display)',
              fontSize: '1.75rem',
              fontWeight: 700,
              color: display.color === 'text-success'
                ? 'var(--color-success-light)'
                : display.color === 'text-error'
                  ? 'var(--color-error-light)'
                  : 'rgba(255,255,255,0.7)',
              textTransform: 'capitalize',
              textAlign: 'center'
            }}
          >
            {display.text}
          </div>
        </div>
      ))}
    </div>
  );
};
