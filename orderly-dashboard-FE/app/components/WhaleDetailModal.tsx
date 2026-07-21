import { Dialog, Table } from '@radix-ui/themes';
import dayjs from 'dayjs';
import { FC, useMemo } from 'react';

import { Spinner } from '.';

import { BrokerBadge } from '~/components/BrokerBadge';
import { useMarketSummary, useWhaleContext } from '~/hooks/usePublicInfo';
import { getBaseToken, getBroker } from '~/hooks/useSymbols';
import { formatPriceByTick } from '~/utils/format';

interface WhaleDetailModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  address: string;
  brokerId?: string;
}

export const WhaleDetailModal: FC<WhaleDetailModalProps> = ({
  open,
  onOpenChange,
  address,
  brokerId
}) => {
  const { data, isLoading, error } = useWhaleContext({
    address,
    broker_id: brokerId,
    recent_trades_limit: 20
  });
  const { data: marketSummary } = useMarketSummary();

  const quoteTickFor = useMemo(() => {
    return (symbol: string): number | null => {
      const tick = marketSummary?.markets.find((m) => m.symbol === symbol)?.quote_tick;
      const parsed = tick != null ? parseFloat(tick) : NaN;
      return Number.isFinite(parsed) ? parsed : null;
    };
  }, [marketSummary]);

  const formatNumber = (value: string | number | null | undefined) => {
    if (value === null || value === undefined) return '-';
    const num = typeof value === 'string' ? parseFloat(value) : value;
    if (isNaN(num)) return '-';
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      notation: 'compact',
      maximumFractionDigits: 2
    }).format(num);
  };

  const formatAddress = (addr: string) => {
    return `${addr.substring(0, 8)}...${addr.substring(addr.length - 6)}`;
  };

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content style={{ maxWidth: 800, maxHeight: '90vh' }}>
        <Dialog.Title className="text-white flex items-center gap-2">
          <span>Whale Detail</span>
          <span className="font-address text-sm text-gray-400 font-normal">
            {formatAddress(address)}
          </span>
        </Dialog.Title>
        <Dialog.Description size="2" mb="4" className="text-gray-400">
          Account overview, open positions, and recent trades.
        </Dialog.Description>

        {isLoading && (
          <div className="flex justify-center py-12">
            <Spinner size="2.5rem" />
          </div>
        )}

        {error && (
          <div className="text-red-500 py-4">Error loading whale data: {error.message}</div>
        )}

        {data && !isLoading && (
          <div className="space-y-6 overflow-y-auto max-h-[60vh]">
            {data.account && (
              <div>
                <h3 className="text-sm font-semibold text-white mb-3">Account Summary</h3>
                <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
                  <div className="rounded-lg p-3" style={{ background: '#1A1525' }}>
                    <div className="text-xs text-gray-400 mb-1">Collateral</div>
                    <div className="text-sm font-semibold text-white">
                      {formatNumber(data.account.total_collateral)}
                    </div>
                  </div>
                  <div className="rounded-lg p-3" style={{ background: '#1A1525' }}>
                    <div className="text-xs text-gray-400 mb-1">Free Collateral</div>
                    <div className="text-sm font-semibold text-white">
                      {formatNumber(data.account.free_collateral)}
                    </div>
                  </div>
                  <div className="rounded-lg p-3" style={{ background: '#1A1525' }}>
                    <div className="text-xs text-gray-400 mb-1">Unrealized PnL</div>
                    <div
                      className="text-sm font-semibold"
                      style={{
                        color:
                          parseFloat(data.account.total_unrealized_pnl) >= 0 ? '#00dea3' : '#FF6390'
                      }}
                    >
                      {formatNumber(data.account.total_unrealized_pnl)}
                    </div>
                  </div>
                  <div className="rounded-lg p-3" style={{ background: '#1A1525' }}>
                    <div className="text-xs text-gray-400 mb-1">Realized PnL</div>
                    <div
                      className="text-sm font-semibold"
                      style={{
                        color:
                          parseFloat(data.account.total_realized_pnl) >= 0 ? '#00dea3' : '#FF6390'
                      }}
                    >
                      {formatNumber(data.account.total_realized_pnl)}
                    </div>
                  </div>
                </div>
              </div>
            )}

            <div>
              <h3 className="text-sm font-semibold text-white mb-3">
                Open Positions ({data.positions.length})
              </h3>
              {data.positions.length === 0 ? (
                <div className="text-gray-500 text-sm py-4 text-center">No open positions</div>
              ) : (
                <div className="overflow-x-auto">
                  <Table.Root size="1">
                    <Table.Header>
                      <Table.Row>
                        <Table.ColumnHeaderCell>Symbol</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Side</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Size</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Notional</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Entry</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Mark</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Unrealized PnL</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Liq. Price</Table.ColumnHeaderCell>
                      </Table.Row>
                    </Table.Header>
                    <Table.Body>
                      {data.positions.map((pos, idx) => (
                        <Table.Row key={`${pos.symbol}-${idx}`}>
                          <Table.Cell className="text-white font-medium">
                            <span className="inline-flex items-center gap-1.5">
                              {getBaseToken(pos.symbol)}
                              <BrokerBadge broker={getBroker(pos.symbol)} />
                            </span>
                          </Table.Cell>
                          <Table.Cell>
                            <span
                              style={{
                                color: pos.side === 'LONG' ? '#00dea3' : '#FF6390',
                                fontWeight: 600
                              }}
                            >
                              {pos.side}
                            </span>
                          </Table.Cell>
                          <Table.Cell>{parseFloat(pos.position_qty).toFixed(4)}</Table.Cell>
                          <Table.Cell>{formatNumber(pos.notional)}</Table.Cell>
                          <Table.Cell>
                            {formatPriceByTick(pos.average_open_price, quoteTickFor(pos.symbol))}
                          </Table.Cell>
                          <Table.Cell>
                            {formatPriceByTick(pos.mark_price, quoteTickFor(pos.symbol))}
                          </Table.Cell>
                          <Table.Cell>
                            <span
                              style={{
                                color: parseFloat(pos.unrealized_pnl) >= 0 ? '#00dea3' : '#FF6390'
                              }}
                            >
                              {formatNumber(pos.unrealized_pnl)}
                            </span>
                          </Table.Cell>
                          <Table.Cell>
                            {pos.est_liq_price
                              ? formatPriceByTick(pos.est_liq_price, quoteTickFor(pos.symbol))
                              : '-'}
                          </Table.Cell>
                        </Table.Row>
                      ))}
                    </Table.Body>
                  </Table.Root>
                </div>
              )}
            </div>

            <div>
              <h3 className="text-sm font-semibold text-white mb-3">
                Recent Trades ({data.recent_trades.length})
              </h3>
              {data.recent_trades.length === 0 ? (
                <div className="text-gray-500 text-sm py-4 text-center">No recent trades</div>
              ) : (
                <div className="overflow-x-auto">
                  <Table.Root size="1">
                    <Table.Header>
                      <Table.Row>
                        <Table.ColumnHeaderCell>Time</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Symbol</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Side</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Price</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Qty</Table.ColumnHeaderCell>
                        <Table.ColumnHeaderCell>Fee</Table.ColumnHeaderCell>
                      </Table.Row>
                    </Table.Header>
                    <Table.Body>
                      {data.recent_trades.map((trade) => (
                        <Table.Row key={trade.id}>
                          <Table.Cell className="text-gray-400">
                            {dayjs(trade.executed_timestamp).format('MM/DD HH:mm:ss')}
                          </Table.Cell>
                          <Table.Cell className="text-white">
                            <span className="inline-flex items-center gap-1.5">
                              {getBaseToken(trade.symbol)}
                              <BrokerBadge broker={getBroker(trade.symbol)} />
                            </span>
                          </Table.Cell>
                          <Table.Cell>
                            <span
                              style={{
                                color: trade.side === 'BUY' ? '#00dea3' : '#FF6390',
                                fontWeight: 600
                              }}
                            >
                              {trade.side}
                            </span>
                          </Table.Cell>
                          <Table.Cell>
                            {formatPriceByTick(trade.executed_price, quoteTickFor(trade.symbol))}
                          </Table.Cell>
                          <Table.Cell>{parseFloat(trade.executed_quantity).toFixed(4)}</Table.Cell>
                          <Table.Cell className="text-gray-400">
                            {formatNumber(trade.fee)} {trade.fee_asset}
                          </Table.Cell>
                        </Table.Row>
                      ))}
                    </Table.Body>
                  </Table.Root>
                </div>
              )}
            </div>
          </div>
        )}
      </Dialog.Content>
    </Dialog.Root>
  );
};
