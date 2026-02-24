import dayjs from 'dayjs';
import { FC, useMemo } from 'react';

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

  if (isLoading) {
    return (
      <div className="card p-4 sm:p-6">
        <h3 className="text-lg font-semibold text-white mb-4">PnL Statistics</h3>
        <p className="text-sm text-gray-400 mb-4">Updated hourly</p>
        <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
          {[1, 2, 3, 4].map((i) => (
            <div
              key={i}
              className="p-4 bg-bg-primary rounded-lg border border-border-primary animate-pulse"
            >
              <div className="h-4 bg-gray-600 rounded mb-2"></div>
              <div className="h-6 bg-gray-700 rounded"></div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  const sevenDayDisplay = formatPnL(sevenDayPnL ?? undefined);
  const thirtyDayDisplay = formatPnL(thirtyDayPnL ?? undefined);
  const ninetyDayDisplay = formatPnL(ninetyDayPnL ?? undefined);
  const allTimeDisplay = formatPnL(allTimePnL ?? undefined);

  return (
    <div className="card p-4 sm:p-6">
      <h3 className="text-lg font-semibold text-white mb-4">PnL Statistics</h3>
      <p className="text-sm text-gray-400 mb-4">Updated hourly • All stats exclude fees</p>
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <div className="p-4 bg-bg-primary rounded-lg border border-border-primary">
          <div className="text-sm text-gray-400 mb-1">7 Day PnL</div>
          <div className={`text-xl font-bold ${sevenDayDisplay.color}`}>{sevenDayDisplay.text}</div>
        </div>

        <div className="p-4 bg-bg-primary rounded-lg border border-border-primary">
          <div className="text-sm text-gray-400 mb-1">30 Day PnL</div>
          <div className={`text-xl font-bold ${thirtyDayDisplay.color}`}>
            {thirtyDayDisplay.text}
          </div>
        </div>

        <div className="p-4 bg-bg-primary rounded-lg border border-border-primary">
          <div className="text-sm text-gray-400 mb-1">90 Day PnL</div>
          <div className={`text-xl font-bold ${ninetyDayDisplay.color}`}>
            {ninetyDayDisplay.text}
          </div>
        </div>

        <div className="p-4 bg-bg-primary rounded-lg border border-border-primary">
          <div className="text-sm text-gray-400 mb-1">All Time PnL</div>
          <div className={`text-xl font-bold ${allTimeDisplay.color}`}>{allTimeDisplay.text}</div>
        </div>
      </div>
    </div>
  );
};
