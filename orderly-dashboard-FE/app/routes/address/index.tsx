import { Button } from '@radix-ui/themes';
import { GroupColumnDef, createColumnHelper } from '@tanstack/react-table';
import { FixedNumber } from '@tarnadas/fixed-number';
import { Dispatch, SetStateAction, useMemo } from 'react';
import { P, match } from 'ts-pattern';

import { Shortened } from './Shortened';
import { Timestamp } from './Timestamp';
import { TransactionLink } from './TransactionLink';

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
  setEventType: Dispatch<SetStateAction<EventType | 'ALL'>>,
  aggregateTrades: boolean = false
) {
  const { events, error, isLoading, isLoadingMore, loadMore, hasMore, tradesCount, pageSizeLimit } =
    useEvents(query);

  const columnHelper = createColumnHelper<EventTableData>();

  const tokens = useTokens();
  const symbols = useSymbols();

  const processedEvents = useMemo(() => {
    if (!aggregateTrades || !events || eventType !== 'PERPTRADE') {
      return events;
    }

    const aggregatedEvents: EventTableData[] = [];
    const tradeGroups = new Map<string, EventTableData[]>();

    events.forEach((event) => {
      if (event.type === 'trade') {
        const key = `${event.transaction_id}_${event.block_number}_${event.transaction_index}_${event.trade.symbol_hash}`;
        if (!tradeGroups.has(key)) {
          tradeGroups.set(key, []);
        }
        tradeGroups.get(key)!.push(event);
      } else {
        aggregatedEvents.push(event);
      }
    });

    tradeGroups.forEach((group) => {
      if (group.length === 1) {
        aggregatedEvents.push(group[0]);
        return;
      }

      const firstTrade = group[0];
      if (firstTrade.type !== 'trade') return;
      const aggregatedTrade: EventTableData = {
        ...firstTrade,
        trade: {
          ...firstTrade.trade,
          trade_qty: group.reduce((sum, event) => {
            if (event.type !== 'trade') return sum;
            try {
              const currentQty = new FixedNumber(event.trade.trade_qty, 8);
              const sumQty = new FixedNumber(sum, 8);
              return sumQty.add(currentQty).valueOf().toString();
            } catch (error) {
              return sum;
            }
          }, '0'),
          notional: group.reduce((sum, event) => {
            if (event.type !== 'trade') return sum;
            try {
              const currentNotional = new FixedNumber(event.trade.notional, 6);
              const sumNotional = new FixedNumber(sum, 6);
              return sumNotional.add(currentNotional).valueOf().toString();
            } catch (error) {
              return sum;
            }
          }, '0'),
          fee: group.reduce((sum, event) => {
            if (event.type !== 'trade') return sum;
            try {
              const currentFee = new FixedNumber(event.trade.fee, 6);
              const sumFee = new FixedNumber(sum, 6);
              return sumFee.add(currentFee).valueOf().toString();
            } catch (error) {
              return sum;
            }
          }, '0'),
          sum_unitary_fundings: group.reduce((sum, event) => {
            if (event.type !== 'trade') return sum;
            try {
              const currentFunding = new FixedNumber(event.trade.sum_unitary_fundings, 8);
              const sumFunding = new FixedNumber(sum, 8);
              return sumFunding.add(currentFunding).valueOf().toString();
            } catch (error) {
              return sum;
            }
          }, '0'),
          executed_price: (() => {
            let totalQty = new FixedNumber('0', 8);
            let weightedPrice = new FixedNumber('0', 8);

            group.forEach((event) => {
              if (event.type !== 'trade') return;
              try {
                const qty = new FixedNumber(event.trade.trade_qty, 8);
                const price = new FixedNumber(event.trade.executed_price, 8);
                const contribution = price.mul(qty);

                totalQty = totalQty.add(qty);
                weightedPrice = weightedPrice.add(contribution);
              } catch (error) {
                console.warn('Error processing trade for aggregation:', error);
              }
            });

            try {
              if (totalQty.valueOf() <= 0n) {
                return firstTrade.trade.executed_price;
              }

              const result = weightedPrice.div(totalQty);
              return result.valueOf().toString();
            } catch (error) {
              console.error('Error calculating weighted average:', error);
              return firstTrade.trade.executed_price;
            }
          })(),
          match_id: firstTrade.trade.match_id,
          trade_id: firstTrade.trade.trade_id
        }
      };

      aggregatedEvents.push(aggregatedTrade);
    });
    return aggregatedEvents;
  }, [events, aggregateTrades, eventType]);

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
            cell: (info) => <TransactionLink value={info.getValue()} />
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
              if (processedEvents == null) return '';
              const event = processedEvents[info.row.index];
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
        const tradeColumns = [
          columnHelper.accessor('data.ProcessedTrades.batch_id', {
            header: 'Batch ID',
            enableSorting: false,
            cell: (info) => info.getValue()
          }),
          columnHelper.accessor('trade.symbol_hash', {
            header: 'Symbol',
            enableSorting: false,
            cell: (info) => {
              const symbol = getSymbolName(info.getValue(), symbols);
              const parts = symbol.split('_');
              const baseToken = parts.length >= 2 ? parts[1] : symbol;
              return <span className="font-mono text-sm">{baseToken}</span>;
            }
          }),
          columnHelper.accessor('trade.side', {
            header: 'Side',
            enableSorting: false,
            cell: (info) => {
              const value = info.getValue();
              return (
                <span className={value === 'BUY' ? 'color-green-3' : 'color-red-3'}>{value}</span>
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
            header: aggregateTrades ? 'Avg Executed Price' : 'Executed Price',
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
          })
        ];

        if (!aggregateTrades) {
          tradeColumns.push(
            columnHelper.accessor('trade.match_id', {
              header: 'Match ID',
              enableSorting: false,
              cell: (info) => <Shortened value={info.getValue()} displayCount={3} />
              // FIXME types
              // eslint-disable-next-line @typescript-eslint/no-explicit-any
            }) as any,
            columnHelper.accessor('trade.trade_id', {
              header: 'Trade ID',
              enableSorting: false,
              cell: (info) => info.getValue()
              // FIXME types
              // eslint-disable-next-line @typescript-eslint/no-explicit-any
            }) as any
          );
        }

        columns.push(
          columnHelper.group({
            id: 'perptrade',
            header: 'Trades',
            columns: tradeColumns
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
  }, [columnHelper, eventType, setEventType, processedEvents, tokens, symbols, aggregateTrades]);

  return {
    columns,
    events: processedEvents,
    error,
    isLoading,
    isLoadingMore,
    loadMore,
    hasMore,
    tradesCount,
    pageSizeLimit
  };
}
