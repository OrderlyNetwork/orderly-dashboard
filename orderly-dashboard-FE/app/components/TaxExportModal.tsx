import { DatePicker } from '@mantine/dates';
import { Dialog } from '@radix-ui/themes';
import dayjs from 'dayjs';
import { FC, useState, useCallback } from 'react';

import { Spinner } from '.';

import { useSymbols, useTokens } from '~/hooks';
import {
  useBulkEvents,
  BulkFetchParams,
  generateCombinedCSV,
  AggregatedTrade,
  AggregatedLiquidation
} from '~/hooks/useBulkEvents';

interface TaxExportModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  accountId: string;
}

export const TaxExportModal: FC<TaxExportModalProps> = ({ open, onOpenChange, accountId }) => {
  const [dateRange, setDateRange] = useState<[string | null, string | null]>([
    dayjs(new Date()).subtract(30, 'days').format('YYYY-MM-DD'),
    dayjs(new Date()).format('YYYY-MM-DD')
  ]);

  const [exportData, setExportData] = useState<{
    trades: AggregatedTrade[];
    liquidations: AggregatedLiquidation[];
  } | null>(null);

  const { isFetching, progress, totalEvents, error, fetchAllEvents } = useBulkEvents();
  const symbols = useSymbols();
  const tokens = useTokens();

  const handleFetch = useCallback(async () => {
    if (!dateRange[0] || !dateRange[1]) return;

    try {
      const params: BulkFetchParams = {
        account_id: accountId,
        from_time: dayjs(dateRange[0]),
        to_time: dayjs(dateRange[1]).endOf('day')
      };

      const result = await fetchAllEvents(params, symbols, tokens);
      setExportData(result);
    } catch (e) {
      // Error is handled in hook
    }
  }, [dateRange, accountId, fetchAllEvents, symbols, tokens]);

  const handleDownload = useCallback(() => {
    if (!exportData) return;

    const csv = generateCombinedCSV(exportData.trades, exportData.liquidations);
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `orderly_tax_export_${accountId.substring(0, 8)}_${dateRange[0]}_${dateRange[1]}.csv`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  }, [exportData, accountId, dateRange]);

  const handleClose = useCallback(() => {
    setExportData(null);
    onOpenChange(false);
  }, [onOpenChange]);

  const totalRows = exportData ? exportData.trades.length + exportData.liquidations.length : 0;

  return (
    <Dialog.Root open={open} onOpenChange={handleClose}>
      <Dialog.Content className="card max-w-2xl w-full sm:w-[95vw] max-h-[90vh] mx-0 sm:mx-auto flex flex-col">
        <Dialog.Title className="text-lg sm:text-xl font-semibold text-white mb-4 flex-shrink-0">
          Tax Export
        </Dialog.Title>

        <div className="space-y-6 flex-1 overflow-y-auto">
          <div className="text-sm text-gray-300">
            Export your trading data for tax reporting purposes. This includes trades (aggregated by
            transaction) and liquidations within the selected date range.
          </div>

          {/* Date Range Selection */}
          <div className="space-y-3">
            <label htmlFor="date-range" className="block text-sm font-medium text-white">
              Date Range
            </label>
            <DatePicker
              id="date-range"
              type="range"
              value={dateRange}
              maxLevel="year"
              allowSingleDateInRange={true}
              maxDate={dayjs().format('YYYY-MM-DD')}
              onChange={(value) => {
                setDateRange(value);
              }}
              highlightToday={true}
            />
          </div>

          {/* Progress / Status */}
          {isFetching && (
            <div className="space-y-3 p-4 bg-bg-secondary rounded-lg border border-border-primary">
              <div className="flex items-center justify-between">
                <span className="text-sm text-white">Fetching data...</span>
                <span className="text-sm text-gray-400">{progress.toFixed(0)}%</span>
              </div>
              <div className="w-full bg-bg-primary rounded-full h-2">
                <div
                  className="bg-primary h-2 rounded-full transition-all duration-300"
                  style={{ width: `${progress}%` }}
                />
              </div>
              <div className="flex items-center gap-2 text-sm text-gray-400">
                <Spinner size="1rem" />
                <span>{totalEvents} events loaded</span>
              </div>
            </div>
          )}

          {error && (
            <div className="p-4 bg-red-900/20 border border-red-500/50 rounded-lg text-red-400 text-sm">
              Error: {error}
            </div>
          )}

          {/* Results */}
          {exportData && !isFetching && (
            <div className="space-y-4">
              <div className="p-4 bg-bg-secondary rounded-lg border border-border-primary">
                <h3 className="text-sm font-semibold text-white mb-3">Export Ready</h3>

                <div className="space-y-2">
                  <div className="flex justify-between items-center text-sm">
                    <span className="text-gray-400">Trades (aggregated):</span>
                    <span className="text-white">{exportData.trades.length} rows</span>
                  </div>
                  <div className="flex justify-between items-center text-sm">
                    <span className="text-gray-400">Liquidations:</span>
                    <span className="text-white">{exportData.liquidations.length} rows</span>
                  </div>
                  <div className="pt-2 border-t border-border-primary flex justify-between items-center text-sm">
                    <span className="text-gray-400 font-medium">Total:</span>
                    <span className="text-white font-medium">{totalRows} rows</span>
                  </div>
                </div>
              </div>

              <button
                onClick={handleDownload}
                disabled={totalRows === 0}
                className="btn btn-primary w-full disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Download CSV
                {totalRows > 0 && (
                  <span className="ml-2 text-xs opacity-75">({totalRows} rows)</span>
                )}
              </button>
            </div>
          )}
        </div>

        <div className="flex justify-end gap-2 mt-6 pt-4 border-t border-border-primary flex-shrink-0">
          <button onClick={handleClose} className="btn btn-secondary">
            Close
          </button>
          <button
            onClick={handleFetch}
            disabled={isFetching || !dateRange[0] || !dateRange[1]}
            className="btn btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isFetching ? (
              <>
                <Spinner size="1rem" />
                <span className="ml-2">Fetching...</span>
              </>
            ) : exportData ? (
              'Re-fetch Data'
            ) : (
              'Fetch Data'
            )}
          </button>
        </div>
      </Dialog.Content>
    </Dialog.Root>
  );
};
