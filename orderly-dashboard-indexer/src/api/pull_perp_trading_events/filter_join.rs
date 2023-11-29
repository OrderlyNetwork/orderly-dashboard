use crate::db::adl_result::query_adl_results;
use crate::db::executed_trades::{query_executed_trades, DbExecutedTrades};
use crate::db::liquidation_result::query_liquidation_results;
use crate::db::liquidation_transfer::{query_liquidation_transfers, DbLiquidationTransfer};
use crate::db::serial_batches::{query_serial_batches_with_type, SerialBatchType};
use crate::db::settings::get_last_rpc_processed_height;
use crate::db::settlement_execution::{query_settlement_executions, DbSettlementExecution};
use crate::db::settlement_result::query_settlement_results;
use crate::db::transaction_events::query_balance_transaction_executions;
use crate::formats_external::trading_events::{TradingEvent, TradingEventsResponse};
use anyhow::Result;
use std::collections::BTreeMap;

pub async fn perp_trading_join_events(
    from_block: i64,
    to_block: i64,
) -> Result<TradingEventsResponse> {
    let last_block = get_last_rpc_processed_height().await?.unwrap_or_default();
    let mut response = TradingEventsResponse::default();
    if last_block == 0 {
        return Ok(response);
    }
    let balance_trans = join_balance_transactions(from_block, to_block).await?;
    let perp_trades = join_perp_trades(from_block, to_block).await?;
    let settlements = join_settlements(from_block, to_block).await?;
    let liquidations = join_liquidations(from_block, to_block).await?;
    let adls = join_adls(from_block, to_block).await?;
    let mut trading_events = [balance_trans, perp_trades, settlements, liquidations, adls].concat();
    trading_events.sort();
    response.events = trading_events;
    response.last_block = last_block;
    Ok(response)
}

pub async fn join_perp_trades(from_block: i64, to_block: i64) -> Result<Vec<TradingEvent>> {
    let executed_trades = query_executed_trades(from_block, to_block).await?;
    let mut executed_trades_map: BTreeMap<(i64, i32), Vec<DbExecutedTrades>> = BTreeMap::new();
    executed_trades.into_iter().for_each(|trade| {
        let batch_key = trade.get_batch_key();
        if let Some(vec) = executed_trades_map.get_mut(&batch_key) {
            vec.push(trade);
        } else {
            let vec = vec![trade];
            executed_trades_map.insert(batch_key, vec);
        }
    });
    let serial_batches =
        query_serial_batches_with_type(from_block, to_block, SerialBatchType::PerpTrade).await?;
    let mut trading_event_vec: Vec<TradingEvent> = vec![];
    for serial_batch in serial_batches {
        let bath_key = serial_batch.get_batch_key();
        if let Some(trades) = executed_trades_map.remove(&bath_key) {
            trading_event_vec.push(TradingEvent::from_serial_batch_and_trades(
                serial_batch,
                trades,
            ));
        }
    }
    Ok(trading_event_vec)
}

pub async fn join_balance_transactions(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<TradingEvent>> {
    let balance_transactions = query_balance_transaction_executions(from_block, to_block).await?;
    let trading_event_vec: Vec<TradingEvent> = balance_transactions
        .into_iter()
        .map(TradingEvent::from_balance_transaction)
        .collect::<Vec<_>>();
    Ok(trading_event_vec)
}

pub async fn join_settlements(from_block: i64, to_block: i64) -> Result<Vec<TradingEvent>> {
    let settlement_executions = query_settlement_executions(from_block, to_block).await?;
    let mut settlement_execution_map: BTreeMap<(i64, i32), Vec<DbSettlementExecution>> =
        BTreeMap::new();
    settlement_executions
        .into_iter()
        .for_each(|settlement_execution| {
            let batch_key = settlement_execution.get_batch_key();
            if let Some(vec) = settlement_execution_map.get_mut(&batch_key) {
                vec.push(settlement_execution);
            } else {
                let vec = vec![settlement_execution];
                settlement_execution_map.insert(batch_key, vec);
            }
        });

    let settlement_results = query_settlement_results(from_block, to_block).await?;
    let mut settlement_result_vec: Vec<TradingEvent> = vec![];
    for settlement_result in settlement_results {
        let bath_key = settlement_result.get_batch_key();
        if let Some(executions) = settlement_execution_map.remove(&bath_key) {
            settlement_result_vec
                .push(TradingEvent::from_settlement(settlement_result, executions));
        }
    }
    Ok(settlement_result_vec)
}

pub async fn join_liquidations(from_block: i64, to_block: i64) -> Result<Vec<TradingEvent>> {
    let liquidation_transfers = query_liquidation_transfers(from_block, to_block).await?;
    let mut liquidation_transfer_map: BTreeMap<(i64, i32), Vec<DbLiquidationTransfer>> =
        BTreeMap::new();
    liquidation_transfers
        .into_iter()
        .for_each(|liquidation_transfer| {
            let batch_key = liquidation_transfer.get_batch_key();
            if let Some(vec) = liquidation_transfer_map.get_mut(&batch_key) {
                vec.push(liquidation_transfer);
            } else {
                let vec = vec![liquidation_transfer];
                liquidation_transfer_map.insert(batch_key, vec);
            }
        });

    let liquidation_results = query_liquidation_results(from_block, to_block).await?;
    let mut liquidation_result_vec: Vec<TradingEvent> = vec![];
    for liquidation_result in liquidation_results {
        let bath_key = liquidation_result.get_batch_key();
        if let Some(transfers) = liquidation_transfer_map.remove(&bath_key) {
            liquidation_result_vec.push(TradingEvent::from_liquidation(
                liquidation_result,
                transfers,
            ));
        }
    }
    Ok(liquidation_result_vec)
}

pub async fn join_adls(from_block: i64, to_block: i64) -> Result<Vec<TradingEvent>> {
    let adl_results = query_adl_results(from_block, to_block).await?;
    let adls_vec: Vec<TradingEvent> = adl_results
        .into_iter()
        .map(TradingEvent::from_adl_result)
        .collect::<Vec<_>>();
    Ok(adls_vec)
}
