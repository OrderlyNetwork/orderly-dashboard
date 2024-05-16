import { GroupColumnDef, createColumnHelper } from '@tanstack/react-table';
import { FixedNumber } from '@tarnadas/fixed-number';
import { useMemo } from 'react';
import { P, match } from 'ts-pattern';

import { Shortened } from './Shortened';
import { Timestamp } from './Timestamp';

import { EventTableData, EventType, EventsParams, useEvents } from '~/hooks';

export function useRenderColumns(query: EventsParams | null, eventType: EventType | 'ALL') {
  const { data, error, isLoading, setSize, mutate } = useEvents(query);
  const events = useMemo(() => data?.flat(), [data]);

  const columnHelper = createColumnHelper<EventTableData>();

  const columns = useMemo<GroupColumnDef<EventTableData, unknown>[]>(() => {
    const columns = [
      columnHelper.group({
        id: 'general',
        header: 'General',
        columns: [
          columnHelper.accessor('event.block_number', {
            header: 'Block height',
            cell: (info) => info.getValue()
          }),
          columnHelper.accessor('event.block_timestamp', {
            header: 'Block timestamp',
            cell: (info) => <Timestamp timestamp={info.getValue()} multiplier={1_000} />
          }),
          columnHelper.accessor('event.transaction_id', {
            header: 'Transaction ID',
            enableSorting: false,
            cell: (info) => <Shortened value={info.getValue()} />
          })
        ]
      })
    ];
    match(eventType)
      .with('ALL', () => {
        columns.push(
          columnHelper.display({
            header: 'Event Type',
            enableSorting: false,
            cell: (info) => {
              if (events == null) return '';
              return match(events![info.row.index])
                .with({ type: 'event', event: P.select() }, (event) =>
                  match(event.data)
                    .with(
                      {
                        Transaction: P.any
                      },
                      () => 'Transaction'
                    )
                    .with(
                      {
                        ProcessedTrades: P.any
                      },
                      () => 'Trade'
                    )
                    .with(
                      {
                        LiquidationResult: P.any
                      },
                      () => 'Liquidation'
                    )
                    .with(
                      {
                        SettlementResult: P.any
                      },
                      () => 'Pnl Settlement'
                    )
                    .with(
                      {
                        AdlResult: P.any
                      },
                      () => 'Adl'
                    )
                    .exhaustive()
                )
                .otherwise(() => undefined);
            }
          })
        );
      })
      .with('TRANSACTION', () => {
        columns.push(
          columnHelper.group({
            id: 'transaction',
            header: 'Transaction',
            columns: [
              columnHelper.accessor('event.data.Transaction.account_id', {
                header: 'Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('event.data.Transaction.broker_hash', {
                header: 'Broker Hash',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('event.data.Transaction.chain_id', {
                header: 'Chain ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.sender', {
                header: 'Sender',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('event.data.Transaction.receiver', {
                header: 'Receiver',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('event.data.Transaction.side', {
                header: 'Side',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.token_amount', {
                header: 'Token amount',
                enableSorting: false,
                cell: (info) =>
                  new FixedNumber(info.getValue(), 6).format({
                    maximumFractionDigits: 2
                  })
              }),
              columnHelper.accessor('event.data.Transaction.status', {
                header: 'Status',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.fail_reason', {
                header: 'Fail Reason',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.fee', {
                header: 'Fee',
                enableSorting: false,
                cell: (info) =>
                  new FixedNumber(info.getValue(), 6).format({
                    maximumFractionDigits: 2
                  })
              }),
              columnHelper.accessor('event.data.Transaction.withdraw_nonce', {
                header: 'Withdraw Nonce',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.token_hash', {
                header: 'Token Hash',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              })
            ]
          })
        );
      })
      .with('PERPTRADE', () => {
        columns.push(
          columnHelper.group({
            id: 'perptrade',
            header: 'Trades',
            columns: [
              columnHelper.accessor('event.data.ProcessedTrades.batch_id', {
                header: 'Batch ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.account_id', {
                header: 'Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('trade.executed_price', {
                header: 'Executed Price',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  // FIXME why API returns as 8 decimals?
                  return new FixedNumber(value, 8).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('trade.fee', {
                header: 'Fee',
                enableSorting: false,
                cell: (info) =>
                  new FixedNumber(info.getValue(), 6).format({
                    maximumFractionDigits: 2
                  })
              }),
              columnHelper.accessor('trade.fee_asset_hash', {
                header: 'Fee Asset Hash',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('trade.match_id', {
                header: 'Match ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} displayCount={3} />
              }),
              columnHelper.accessor('trade.notional', {
                header: 'Notional',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('trade.side', {
                header: 'Side',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.sum_unitary_fundings', {
                header: 'Sum Uni. Funding',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  // FIXME how many decimals?
                  return new FixedNumber(value, 8).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('trade.timestamp', {
                header: 'Timestamp',
                enableSorting: false,
                cell: (info) => <Timestamp timestamp={info.getValue()} />
              }),
              columnHelper.accessor('trade.trade_id', {
                header: 'Trade ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.trade_qty', {
                header: 'Trade Qty',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 8).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('trade.symbol_hash', {
                header: 'Symbol Hash',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              })
            ]
          })
        );
      })
      .with('SETTLEMENT', () => {
        columns.push(
          columnHelper.group({
            id: 'settlement',
            header: 'PnL Settlement',
            columns: [
              columnHelper.accessor('event.data.SettlementResult.account_id', {
                header: 'Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('event.data.SettlementResult.settled_amount', {
                header: 'Settled Amount',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('event.data.SettlementResult.insurance_transfer_amount', {
                header: 'Insurance Transfer Amount',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('event.data.SettlementResult.insurance_account_id', {
                header: 'Insurance Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('event.data.SettlementResult.settled_asset_hash', {
                header: 'Settled Asset Hash',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('settlement.mark_price', {
                header: 'Mark Price',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  // FIXMe why 8 decimals?
                  return new FixedNumber(value, 8).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('settlement.settled_amount', {
                header: 'Settled Amount',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('settlement.sum_unitary_fundings', {
                header: 'Sum Uni. Funding',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  // FIXME how many decimals?
                  return new FixedNumber(value, 11).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('settlement.symbol_hash', {
                header: 'Symbol Hash',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              })
            ]
          })
        );
      })
      .with('LIQUIDATION', () => {
        columns.push(
          columnHelper.group({
            id: 'liquidation',
            header: 'Liquidation',
            columns: [
              columnHelper.accessor('event.data.LiquidationResult.liquidated_account_id', {
                header: 'Liquidated Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('event.data.LiquidationResult.insurance_account_id', {
                header: 'Insurance Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('event.data.LiquidationResult.insurance_transfer_amount', {
                header: 'Insurance Transfer Amount',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.cost_position_transfer', {
                header: 'Cost Position Transfer',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.insurance_fee', {
                header: 'Insurance Fee',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidation_fee', {
                header: 'Liquidation Fee',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidation_transfer_id', {
                header: 'Liquidation Transfer ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidator_account_id', {
                header: 'Liquidator Account ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidator_fee', {
                header: 'Liquidator Fee',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.mark_price', {
                header: 'Mark Price',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.position_qty_transfer', {
                header: 'Position Qty Transfer',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.sum_unitary_fundings', {
                header: 'Sum Uni. Funding',
                enableSorting: false,
                cell: (info) => info.getValue()
              })
            ]
          })
        );
      })
      .with('ADL', () => {
        columns.push(
          columnHelper.group({
            id: 'adl',
            header: 'Adl',
            columns: [
              columnHelper.accessor('event.data.AdlResult.account_id', {
                header: 'Account ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.adl_price', {
                header: 'Adl Price',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.cost_position_transfer', {
                header: 'Cost Position Transfer',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.insurance_account_id', {
                header: 'Insurance Account ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.position_qty_transfer', {
                header: 'Position Qty Transfer',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.sum_unitary_fundings', {
                header: 'Sum Uni. Funding',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.symbol_hash', {
                header: 'Symbol Hash',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              })
            ]
          })
        );
      });
    return columns;
  }, [columnHelper, eventType, events]);

  return { columns, events, error, isLoading, setSize, mutate };
}
