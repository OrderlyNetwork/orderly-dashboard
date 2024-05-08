import { Tooltip } from '@radix-ui/themes';
import { GroupColumnDef, createColumnHelper } from '@tanstack/react-table';
import { FixedNumber } from '@tarnadas/fixed-number';
import { useMemo } from 'react';
import { P, match } from 'ts-pattern';

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
            cell: (info) => info.getValue()
          }),
          columnHelper.accessor('event.transaction_id', {
            header: 'Transaction ID',
            cell: (info) => {
              const value = info.getValue();
              if (value == null) return '';
              return (
                <Tooltip content={`${value} (click to copy)`}>
                  <span>
                    {value.substring(0, 4)}...{value.substr(-4)}
                  </span>
                </Tooltip>
              );
            }
          })
        ]
      })
    ];
    match(eventType)
      .with('ALL', () => {
        columns.push(
          columnHelper.display({
            header: 'Event Type',
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
                      () => 'Liqudation'
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
              columnHelper.accessor('event.data.Transaction.chain_id', {
                header: 'Chain ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.side', {
                header: 'Side',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.token_amount', {
                header: 'Token amount',
                cell: (info) =>
                  new FixedNumber(info.getValue(), 6).format({
                    maximumFractionDigits: 2
                  })
              }),
              columnHelper.accessor('event.data.Transaction.status', {
                header: 'Status',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.fail_reason', {
                header: 'Fail Reason',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.Transaction.fee', {
                header: 'Fee',
                cell: (info) =>
                  new FixedNumber(info.getValue(), 6).format({
                    maximumFractionDigits: 2
                  })
              }),
              columnHelper.accessor('event.data.Transaction.withdraw_nonce', {
                header: 'Withdraw Nonce',
                cell: (info) => info.getValue()
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
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.executed_price', {
                header: 'Executed Price',
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
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.match_id', {
                header: 'Match ID',
                cell: (info) => {
                  let value = info.getValue();
                  if (value == null) return;
                  value = String(value);
                  return (
                    <Tooltip content={`${value} (click to copy)`}>
                      <span>
                        {value.substring(0, 3)}...{value.substr(-3)}
                      </span>
                    </Tooltip>
                  );
                }
              }),
              columnHelper.accessor('trade.notional', {
                header: 'Notional',
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
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.sum_unitary_fundings', {
                header: 'Sum Uni. Funding',
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
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.trade_id', {
                header: 'Trade ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.trade_qty', {
                header: 'Trade Qty',
                cell: (info) => info.getValue()
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
              columnHelper.accessor('event.data.SettlementResult.settled_amount', {
                header: 'Settled Amount',
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
                cell: (info) => {
                  let value = info.getValue();
                  if (value == null) return;
                  value = String(value);
                  return (
                    <Tooltip content={`${value} (click to copy)`}>
                      <span>
                        {value.substring(0, 4)}...{value.substr(-4)}
                      </span>
                    </Tooltip>
                  );
                }
              }),
              columnHelper.accessor('settlement.mark_price', {
                header: 'Mark Price',
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
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  // FIXME how many decimals?
                  return new FixedNumber(value, 11).format({
                    maximumFractionDigits: 2
                  });
                }
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
                cell: (info) => {
                  let value = info.getValue();
                  if (value == null) return;
                  value = String(value);
                  return (
                    <Tooltip content={`${value} (click to copy)`}>
                      <span>
                        {value.substring(0, 4)}...{value.substr(-4)}
                      </span>
                    </Tooltip>
                  );
                }
              }),
              columnHelper.accessor('event.data.LiquidationResult.insurance_account_id', {
                header: 'Insurance Account ID',
                cell: (info) => {
                  let value = info.getValue();
                  if (value == null) return;
                  value = String(value);
                  return (
                    <Tooltip content={`${value} (click to copy)`}>
                      <span>
                        {value.substring(0, 4)}...{value.substr(-4)}
                      </span>
                    </Tooltip>
                  );
                }
              }),
              columnHelper.accessor('event.data.LiquidationResult.insurance_transfer_amount', {
                header: 'Insurance Transfer Amount',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.cost_position_transfer', {
                header: 'Cost Position Transfer',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.insurance_fee', {
                header: 'Insurance Fee',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidation_fee', {
                header: 'Liquidation Fee',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidation_transfer_id', {
                header: 'Liquidation Transfer ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidator_account_id', {
                header: 'Liquidator Account ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidator_fee', {
                header: 'Liquidator Fee',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.mark_price', {
                header: 'Mark Price',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.position_qty_transfer', {
                header: 'Position Qty Transfer',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.sum_unitary_fundings', {
                header: 'Sum Uni. Funding',
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
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.adl_price', {
                header: 'Adl Price',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.cost_position_transfer', {
                header: 'Cost Position Transfer',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.insurance_account_id', {
                header: 'Insurance Account ID',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.position_qty_transfer', {
                header: 'Position Qty Transfer',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.sum_unitary_fundings', {
                header: 'Sum Uni. Funding',
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('event.data.AdlResult.symbol_hash', {
                header: 'Symbol Hash',
                cell: (info) => info.getValue()
              })
            ]
          })
        );
      });
    return columns;
  }, [columnHelper, eventType, events]);

  return { columns, events, error, isLoading, setSize, mutate };
}
