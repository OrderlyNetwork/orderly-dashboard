import { Button } from '@radix-ui/themes';
import { GroupColumnDef, createColumnHelper } from '@tanstack/react-table';
import { FixedNumber } from '@tarnadas/fixed-number';
import { Dispatch, SetStateAction, useMemo } from 'react';
import { P, match } from 'ts-pattern';

import { Shortened } from './Shortened';
import { Timestamp } from './Timestamp';

import {
  EventTableData,
  EventType,
  EventsParams,
  getSymbolName,
  getTokenName,
  useEvents,
  useSymbols,
  useTokens
} from '~/hooks';

export function useRenderColumns(
  query: EventsParams | null,
  eventType: EventType | 'ALL',
  setEventType: Dispatch<SetStateAction<EventType | 'ALL'>>
) {
  const { data, error, isLoading, setSize, mutate } = useEvents(query);
  const events = useMemo(() => data?.flat(), [data]);

  const columnHelper = createColumnHelper<EventTableData>();

  const tokens = useTokens();
  const symbols = useSymbols();
  const columns = useMemo<GroupColumnDef<EventTableData, unknown>[]>(() => {
    const columns = [
      columnHelper.group({
        id: 'general',
        header: 'General',
        columns: [
          columnHelper.accessor('block_number', {
            header: 'Block height',
            cell: (info) => info.getValue()
          }),
          columnHelper.accessor('block_timestamp', {
            header: 'Block timestamp',
            cell: (info) => <Timestamp timestamp={info.getValue()} multiplier={1_000} />
          }),
          columnHelper.accessor('transaction_id', {
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
              const event = events[info.row.index];
              const [renderedValue, newEventType] = match(event.data)
                .with(
                  {
                    Transaction: P.any
                  },
                  () => ['Transaction', 'TRANSACTION']
                )
                .with(
                  {
                    ProcessedTrades: P.any
                  },
                  () => ['Trade', 'PERPTRADE']
                )
                .with(
                  {
                    LiquidationResult: P.any
                  },
                  () => ['Liquidation', 'LIQUIDATION']
                )
                .with(
                  {
                    LiquidationResultV2: P.any
                  },
                  () => ['LiquidationV2', 'LIQUIDATIONV2']
                )
                .with(
                  {
                    SettlementResult: P.any
                  },
                  () => ['Pnl Settlement', 'SETTLEMENT']
                )
                .with(
                  {
                    AdlResult: P.any
                  },
                  () => ['Adl', 'ADL']
                )
                .with(
                  {
                    AdlResultV2: P.any
                  },
                  () => ['AdlV2', 'ADLV2']
                )
                .exhaustive() as [string | undefined, EventType | 'ALL'];
              return (
                <Button
                  className="p-1 h-auto bg-[--accent-5] hover:bg-[--accent-4]"
                  onClick={() => {
                    setEventType(newEventType);
                  }}
                >
                  {renderedValue}
                </Button>
              );
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
              columnHelper.accessor('data.Transaction.account_id', {
                header: 'Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('data.Transaction.broker_hash', {
                header: 'Broker Hash',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('data.Transaction.chain_id', {
                header: 'Chain ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.Transaction.sender', {
                header: 'Sender',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('data.Transaction.receiver', {
                header: 'Receiver',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('data.Transaction.side', {
                header: 'Side',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.Transaction.token_amount', {
                header: 'Token amount',
                enableSorting: false,
                cell: (info) =>
                  new FixedNumber(info.getValue(), 6).format({
                    maximumFractionDigits: 2
                  })
              }),
              columnHelper.accessor('data.Transaction.status', {
                header: 'Status',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.Transaction.fail_reason', {
                header: 'Fail Reason',
                enableSorting: false,

                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.Transaction.fee', {
                header: 'Fee',
                enableSorting: false,
                cell: (info) =>
                  new FixedNumber(info.getValue(), 6).format({
                    maximumFractionDigits: 2
                  })
              }),
              columnHelper.accessor('data.Transaction.withdraw_nonce', {
                header: 'Withdraw Nonce',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.Transaction.token_hash', {
                header: 'Token',
                enableSorting: false,
                cell: (info) => getTokenName(info.getValue(), tokens)
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
              columnHelper.accessor('data.ProcessedTrades.batch_id', {
                header: 'Batch ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('trade.symbol_hash', {
                header: 'Symbol',
                enableSorting: false,
                cell: (info) => getSymbolName(info.getValue(), symbols)
              }),
              columnHelper.accessor('trade.side', {
                header: 'Side',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  return (
                    <span className={value === 'BUY' ? 'color-green-3' : 'color-red-3'}>
                      {value}
                    </span>
                  );
                }
              }),
              columnHelper.accessor('trade.trade_qty', {
                header: 'Trade Qty',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  const res = new FixedNumber(value, 8);
                  return (
                    <span className={res.valueOf() > 0n ? 'color-green-3' : 'color-red-3'}>
                      {res.format({
                        maximumFractionDigits: 2
                      })}
                    </span>
                  );
                }
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
                header: 'Fee Asset',
                enableSorting: false,
                cell: (info) => getTokenName(info.getValue(), tokens)
              }),
              columnHelper.accessor('trade.timestamp', {
                header: 'Timestamp',
                enableSorting: false,
                cell: (info) => <Timestamp timestamp={info.getValue()} />
              }),
              columnHelper.accessor('trade.account_id', {
                header: 'Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('trade.match_id', {
                header: 'Match ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} displayCount={3} />
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
              columnHelper.accessor('trade.trade_id', {
                header: 'Trade ID',
                enableSorting: false,
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
              columnHelper.accessor('settlement.symbol_hash', {
                header: 'Symbol',
                enableSorting: false,
                cell: (info) => getSymbolName(info.getValue(), symbols)
              }),
              columnHelper.accessor('data.SettlementResult.account_id', {
                header: 'Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('data.SettlementResult.settled_amount', {
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
              columnHelper.accessor('data.SettlementResult.insurance_transfer_amount', {
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
              columnHelper.accessor('data.SettlementResult.insurance_account_id', {
                header: 'Insurance Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('data.SettlementResult.settled_asset_hash', {
                header: 'Settled Asset',
                enableSorting: false,
                cell: (info) => getTokenName(info.getValue(), tokens)
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
                  return new FixedNumber(value, 8).format({
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
              columnHelper.accessor('data.LiquidationResult.liquidated_account_id', {
                header: 'Liquidated Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('data.LiquidationResult.insurance_account_id', {
                header: 'Insurance Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('data.LiquidationResult.insurance_transfer_amount', {
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
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('liquidation.liquidation_fee', {
                header: 'Liquidation Fee',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('liquidation.liquidation_transfer_id', {
                header: 'Liquidation Transfer ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidation.liquidator_account_id', {
                header: 'Liquidator Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('liquidation.liquidator_fee', {
                header: 'Liquidator Fee',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('liquidation.mark_price', {
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
              columnHelper.accessor('liquidation.position_qty_transfer', {
                header: 'Position Qty Transfer',
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
              columnHelper.accessor('liquidation.sum_unitary_fundings', {
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
              })
            ]
          })
        );
      })
      .with('LIQUIDATIONV2', () => {
        columns.push(
          columnHelper.group({
            id: 'liquidation',
            header: 'Liquidation',
            columns: [
              columnHelper.accessor('data.LiquidationResultV2.account_id', {
                header: 'Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('liquidationv2.symbol_hash', {
                header: 'Symbol',
                enableSorting: false,
                cell: (info) => getSymbolName(info.getValue(), symbols)
              }),
              columnHelper.accessor('liquidationv2.mark_price', {
                header: 'Mark Price',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 8).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('liquidationv2.fee', {
                header: 'Fee',
                enableSorting: false,
                cell: (info) => {
                  const value = info.getValue();
                  if (value == null) return '';
                  return new FixedNumber(value, 6).format({
                    maximumFractionDigits: 2
                  });
                }
              }),
              columnHelper.accessor('data.LiquidationResultV2.insurance_transfer_amount', {
                header: 'Insurance Transfer Amount',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('liquidationv2.cost_position_transfer', {
                header: 'Cost Position Transfer',
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
              columnHelper.accessor('liquidationv2.position_qty_transfer', {
                header: 'Position Qty Transfer',
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
              columnHelper.accessor('liquidationv2.account_id', {
                header: 'Liquidated Account ID',
                enableSorting: false,
                cell: (info) => <Shortened value={info.getValue()} />
              }),
              columnHelper.accessor('liquidationv2.sum_unitary_fundings', {
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
              columnHelper.accessor('data.AdlResult.symbol_hash', {
                header: 'Symbol',
                enableSorting: false,
                cell: (info) => getSymbolName(info.getValue(), symbols)
              }),
              columnHelper.accessor('data.AdlResult.account_id', {
                header: 'Account ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.AdlResult.adl_price', {
                header: 'Adl Price',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.AdlResult.cost_position_transfer', {
                header: 'Cost Position Transfer',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.AdlResult.insurance_account_id', {
                header: 'Insurance Account ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.AdlResult.position_qty_transfer', {
                header: 'Position Qty Transfer',
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
              columnHelper.accessor('data.AdlResult.sum_unitary_fundings', {
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
              })
            ]
          })
        );
      })
      .with('ADLV2', () => {
        columns.push(
          columnHelper.group({
            id: 'adl',
            header: 'Adl',
            columns: [
              columnHelper.accessor('data.AdlResultV2.symbol_hash', {
                header: 'Symbol',
                enableSorting: false,
                cell: (info) => getSymbolName(info.getValue(), symbols)
              }),
              columnHelper.accessor('data.AdlResultV2.account_id', {
                header: 'Account ID',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.AdlResultV2.adl_price', {
                header: 'Adl Price',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.AdlResultV2.cost_position_transfer', {
                header: 'Cost Position Transfer',
                enableSorting: false,
                cell: (info) => info.getValue()
              }),
              columnHelper.accessor('data.AdlResultV2.position_qty_transfer', {
                header: 'Position Qty Transfer',
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
              columnHelper.accessor('data.AdlResultV2.sum_unitary_fundings', {
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
              })
            ]
          })
        );
      })
      .exhaustive();
    return columns;
  }, [columnHelper, eventType, setEventType, events, tokens, symbols]);

  return { columns, events, error, isLoading, setSize, mutate };
}
