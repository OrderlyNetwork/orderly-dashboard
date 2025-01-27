use crate::consume_data_task::ORDERLY_DASHBOARD_INDEXER;
use crate::db::adl_result::{query_account_adl_results, query_adl_results};
use crate::db::executed_trades::{
    query_account_executed_trades, query_executed_trades, DbExecutedTrades,
};
use crate::db::liquidation_result::{
    query_liquidation_results, query_liquidation_results_by_time_and_account,
    query_liquidation_results_by_time_and_keys,
};
use crate::db::liquidation_transfer::{
    query_account_liquidation_transfers_by_time,
    query_account_liquidation_transfers_by_time_and_result_keys, query_liquidation_transfers,
    DbLiquidationTransfer,
};
use crate::db::serial_batches::{
    query_serial_batches_by_time_and_key, query_serial_batches_with_type, SerialBatchType,
};
use crate::db::settings::{get_sol_sync_block_time, get_sol_sync_signature};
use crate::db::settlement_execution::{
    query_account_settlement_executions, query_settlement_executions, DbSettlementExecution,
    DbSettlementExecutionView,
};
use crate::db::settlement_result::query_settlement_results;
use crate::db::sol_transaction_events::{
    query_account_sol_balance_transaction_executions, query_sol_balance_transaction_executions,
};
use crate::db::transaction_events::{
    query_account_balance_transaction_executions, query_balance_transaction_executions,
};
use crate::formats_external::trading_events::{
    AccountTradingEventsResponse, TradingEvent, TradingEventType, TradingEventsResponse,
};
use anyhow::Result;
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet};

pub async fn perp_trading_join_events(
    from_block: i64,
    to_block: i64,
    event_type: Option<TradingEventType>,
) -> Result<TradingEventsResponse> {
    let last_block = crate::api::get_may_cached_orderly_last_rpc_processed_height().await?;
    let last_timestamp = crate::api::get_may_cached_orderly_last_rpc_processed_timestamp().await?;
    let to_block = min(last_block as i64, to_block);
    let mut response = TradingEventsResponse::default();
    if last_block == 0 {
        return Ok(response);
    }
    if let Some(event_type) = event_type {
        let mut trading_events: Vec<TradingEvent>;
        match event_type {
            TradingEventType::TRANSACTION => {
                trading_events = join_balance_transactions(from_block, to_block).await?;
            }
            TradingEventType::PerpTrade => {
                trading_events = join_perp_trades(from_block, to_block).await?;
            }
            TradingEventType::SETTLEMENT => {
                trading_events = join_settlements(from_block, to_block).await?;
            }
            TradingEventType::LIQUIDATION => {
                trading_events = join_liquidations(from_block, to_block).await?;
            }
            TradingEventType::ADL => {
                trading_events = join_adls(from_block, to_block).await?;
            }
        }
        trading_events.sort();
        response.events = trading_events;
        response.last_block = last_block;
        response.last_block_timestamp = last_timestamp;
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
    response.last_block_timestamp = last_timestamp;
    Ok(response)
}

pub async fn sol_join_events(
    from_block: i64,
    to_block: i64,
    event_type: Option<TradingEventType>,
) -> Result<TradingEventsResponse> {
    let last_block = get_sol_sync_signature().await?.unwrap_or_default().slot;
    let last_timestamp = get_sol_sync_block_time().await?.unwrap_or_default();
    let to_block = min(last_block as i64, to_block);
    let mut response = TradingEventsResponse::default();
    if last_block == 0 {
        return Ok(response);
    }
    if let Some(event_type) = event_type {
        let mut trading_events: Vec<TradingEvent> = vec![];
        match event_type {
            TradingEventType::TRANSACTION => {
                trading_events = join_sol_balance_transactions(from_block, to_block).await?;
            }
            _ => {}
        }
        trading_events.sort();
        response.events = trading_events;
        response.last_block = last_block;
        response.last_block_timestamp = last_timestamp;
        return Ok(response);
    }
    let mut balance_trans = join_sol_balance_transactions(from_block, to_block).await?;
    balance_trans.sort();
    response.events = balance_trans;
    response.last_block = last_block;
    response.last_block_timestamp = last_timestamp;
    Ok(response)
}

pub async fn account_perp_trading_join_events(
    account_id: &str,
    from_time: i64,
    to_time: i64,
    event_type: Option<TradingEventType>,
) -> Result<AccountTradingEventsResponse> {
    let mut response = AccountTradingEventsResponse::default();
    if let Some(event_type) = event_type {
        let mut trading_events: Vec<TradingEvent>;
        match event_type {
            TradingEventType::TRANSACTION => {
                trading_events = join_account_balance_transactions(
                    account_id.to_string(),
                    from_time,
                    to_time,
                    None,
                    None,
                )
                .await?;
            }
            TradingEventType::PerpTrade => {
                trading_events = join_account_perp_trades(
                    account_id.to_string(),
                    from_time,
                    to_time,
                    None,
                    None,
                )
                .await?
                .0;
            }
            TradingEventType::SETTLEMENT => {
                trading_events = join_account_settlements(
                    account_id.to_string(),
                    from_time,
                    to_time,
                    None,
                    None,
                )
                .await?
                .0;
            }
            TradingEventType::LIQUIDATION => {
                trading_events = join_account_liquidations(
                    account_id.to_string(),
                    from_time,
                    to_time,
                    None,
                    None,
                )
                .await?;
            }
            TradingEventType::ADL => {
                trading_events =
                    join_account_adls(account_id.to_string(), from_time, to_time, None, None)
                        .await?;
            }
        }
        trading_events.sort();
        response.events = trading_events;
        return Ok(response);
    }
    let balance_trans =
        join_account_balance_transactions(account_id.to_string(), from_time, to_time, None, None)
            .await?;
    let (perp_trades, _has_next_offset) =
        join_account_perp_trades(account_id.to_string(), from_time, to_time, None, None).await?;
    let (settlements, _has_next_offset) =
        join_account_settlements(account_id.to_string(), from_time, to_time, None, None).await?;
    let liquidations =
        join_account_liquidations(account_id.to_string(), from_time, to_time, None, None).await?;
    let adls = join_account_adls(account_id.to_string(), from_time, to_time, None, None).await?;
    let mut trading_events = [balance_trans, perp_trades, settlements, liquidations, adls].concat();
    trading_events.sort();
    response.events = trading_events;
    Ok(response)
}

pub async fn account_perp_trading_join_events_v2(
    account_id: &str,
    from_time: i64,
    to_time: i64,
    event_type: Option<TradingEventType>,
    offset: u32,
    limit: u32,
) -> Result<AccountTradingEventsResponse> {
    let mut response = AccountTradingEventsResponse::default();
    if let Some(event_type) = event_type {
        let mut trading_events: Vec<TradingEvent>;
        match event_type {
            TradingEventType::TRANSACTION => {
                {
                    trading_events = join_account_balance_transactions(
                        account_id.to_string(),
                        from_time,
                        to_time,
                        Some(offset),
                        Some(limit),
                    )
                    .await?;
                }
                if trading_events.len() >= limit as usize {
                    response.next_offset = Some(offset + limit);
                }
            }
            TradingEventType::PerpTrade => {
                let has_next_offset;
                (trading_events, has_next_offset) = join_account_perp_trades(
                    account_id.to_string(),
                    from_time,
                    to_time,
                    Some(offset),
                    Some(limit),
                )
                .await?;
                if has_next_offset {
                    response.next_offset = Some(offset + limit);
                }
            }
            TradingEventType::SETTLEMENT => {
                let has_next_offset;
                (trading_events, has_next_offset) = join_account_settlements(
                    account_id.to_string(),
                    from_time,
                    to_time,
                    Some(offset),
                    Some(limit),
                )
                .await?;
                if has_next_offset {
                    response.next_offset = Some(offset + limit);
                }
            }
            TradingEventType::LIQUIDATION => {
                trading_events = join_account_liquidations(
                    account_id.to_string(),
                    from_time,
                    to_time,
                    Some(offset),
                    Some(limit),
                )
                .await?;
            }
            TradingEventType::ADL => {
                trading_events = join_account_adls(
                    account_id.to_string(),
                    from_time,
                    to_time,
                    Some(offset),
                    Some(limit),
                )
                .await?;
            }
        }
        trading_events.sort();
        response.events = trading_events;
        return Ok(response);
    }
    let balance_trans = join_account_balance_transactions(
        account_id.to_string(),
        from_time,
        to_time,
        Some(offset),
        Some(limit),
    )
    .await?;
    let (perp_trades, has_next_offset) = join_account_perp_trades(
        account_id.to_string(),
        from_time,
        to_time,
        Some(offset),
        Some(limit),
    )
    .await?;
    if has_next_offset {
        response.next_offset = Some(offset + limit);
    }
    let (settlements, has_next_offset) = join_account_settlements(
        account_id.to_string(),
        from_time,
        to_time,
        Some(offset),
        Some(limit),
    )
    .await?;
    if has_next_offset {
        response.next_offset = Some(offset + limit);
    }
    let liquidations = join_account_liquidations(
        account_id.to_string(),
        from_time,
        to_time,
        Some(offset),
        Some(limit),
    )
    .await?;
    let adls = join_account_adls(
        account_id.to_string(),
        from_time,
        to_time,
        Some(offset),
        Some(limit),
    )
    .await?;
    let mut trading_events = [balance_trans, perp_trades, settlements, liquidations, adls].concat();
    trading_events.sort();
    response.events = trading_events;
    Ok(response)
}

pub async fn account_sol_join_events(
    account_id: &str,
    from_time: i64,
    to_time: i64,
    event_type: Option<TradingEventType>,
) -> Result<AccountTradingEventsResponse> {
    let mut response = AccountTradingEventsResponse::default();
    if let Some(event_type) = event_type {
        let mut trading_events: Vec<TradingEvent> = vec![];
        match event_type {
            TradingEventType::TRANSACTION => {
                trading_events = join_sol_account_balance_transactions(
                    account_id.to_string(),
                    from_time,
                    to_time,
                )
                .await?;
            }
            _ => {}
        }
        trading_events.sort();
        response.events = trading_events;
        return Ok(response);
    }
    let mut balance_trans =
        join_sol_account_balance_transactions(account_id.to_string(), from_time, to_time).await?;
    balance_trans.sort();
    response.events = balance_trans;
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

/// return (trading_events, has_next_offset)
pub async fn join_account_perp_trades(
    account_id: String,
    from_time: i64,
    to_time: i64,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<(Vec<TradingEvent>, bool)> {
    let executed_trades =
        query_account_executed_trades(account_id.clone(), from_time, to_time, offset, limit)
            .await?;
    let has_next_offset = if let Some(limit) = limit {
        executed_trades.len() >= limit as usize
    } else {
        false
    };
    tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "executed_trades length: {}", executed_trades.len());
    let mut executed_trades_map: BTreeMap<(i64, i32), Vec<DbExecutedTrades>> = BTreeMap::new();
    let mut batch_keys: BTreeSet<(i64, i32)> = BTreeSet::new();
    executed_trades.into_iter().for_each(|trade| {
        let batch_key = trade.get_batch_key();
        batch_keys.insert(batch_key);
        if let Some(vec) = executed_trades_map.get_mut(&batch_key) {
            vec.push(trade);
        } else {
            let vec = vec![trade];
            executed_trades_map.insert(batch_key, vec);
        }
    });
    let serial_batches =
        query_serial_batches_by_time_and_key(from_time, to_time, batch_keys).await?;
    let mut trading_event_vec: Vec<TradingEvent> = vec![];
    for serial_batch in serial_batches {
        let bath_key = serial_batch.get_batch_key();
        if let Some(trades) = executed_trades_map.remove(&bath_key) {
            trading_event_vec.push(TradingEvent::from_view_serial_batch_and_trades(
                serial_batch,
                trades,
            ));
        }
    }
    Ok((trading_event_vec, has_next_offset))
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

pub async fn join_sol_balance_transactions(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<TradingEvent>> {
    let balance_transactions =
        query_sol_balance_transaction_executions(from_block, to_block).await?;
    let trading_event_vec: Vec<TradingEvent> = balance_transactions
        .into_iter()
        .map(TradingEvent::from_sol_balance_transaction)
        .collect::<Vec<_>>();
    Ok(trading_event_vec)
}

pub async fn join_account_balance_transactions(
    account_id: String,
    from_time: i64,
    to_time: i64,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<TradingEvent>> {
    let balance_transactions =
        query_account_balance_transaction_executions(account_id, from_time, to_time, offset, limit)
            .await?;
    let trading_event_vec: Vec<TradingEvent> = balance_transactions
        .into_iter()
        .map(TradingEvent::from_balance_transaction)
        .collect::<Vec<_>>();
    Ok(trading_event_vec)
}

pub async fn join_sol_account_balance_transactions(
    account_id: String,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<TradingEvent>> {
    let balance_transactions =
        query_account_sol_balance_transaction_executions(account_id, from_time, to_time).await?;
    let trading_event_vec: Vec<TradingEvent> = balance_transactions
        .into_iter()
        .map(TradingEvent::from_sol_balance_transaction)
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

/// return (trading_events, has_next_offset)
pub async fn join_account_settlements(
    account_id: String,
    from_time: i64,
    to_time: i64,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<(Vec<TradingEvent>, bool)> {
    let settlement_executions =
        query_account_settlement_executions(account_id.clone(), from_time, to_time, offset, limit)
            .await?;
    let has_next_offset = if let Some(limit) = limit {
        settlement_executions.len() >= limit as usize
    } else {
        false
    };
    let mut settlement_execution_map: BTreeMap<(i64, i32), Vec<DbSettlementExecutionView>> =
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

    let mut settlement_result_vec: Vec<TradingEvent> = vec![];
    for (_, executions) in settlement_execution_map {
        settlement_result_vec.push(TradingEvent::from_settlement_view(
            account_id.clone(),
            executions,
        ));
    }
    Ok((settlement_result_vec, has_next_offset))
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

pub async fn join_account_liquidations(
    account_id: String,
    from_time: i64,
    to_time: i64,
    offset: Option<u32>,
    _limit: Option<u32>,
) -> Result<Vec<TradingEvent>> {
    // as account role
    let liquidation_transfers =
        query_account_liquidation_transfers_by_time(from_time, to_time, account_id.clone(), offset)
            .await?;
    if liquidation_transfers.is_empty() {
        return Ok(vec![]);
    }
    let mut liquidation_res_keys: BTreeSet<(i64, i32)> = BTreeSet::new();
    let mut liquidation_transfer_map: BTreeMap<(i64, i32), Vec<DbLiquidationTransfer>> =
        BTreeMap::new();
    liquidation_transfers
        .into_iter()
        .for_each(|liquidation_transfer| {
            let batch_key = liquidation_transfer.get_batch_key();
            liquidation_res_keys.insert(batch_key);
            if let Some(vec) = liquidation_transfer_map.get_mut(&batch_key) {
                vec.push(liquidation_transfer);
            } else {
                let vec = vec![liquidation_transfer];
                liquidation_transfer_map.insert(batch_key, vec);
            }
        });

    let liquidation_results =
        query_liquidation_results_by_time_and_keys(from_time, to_time, liquidation_res_keys)
            .await?;
    let mut liquidation_result_map: BTreeMap<(i64, i32), TradingEvent> = BTreeMap::new();
    for liquidation_result in liquidation_results {
        let bath_key = liquidation_result.get_batch_key();
        if let Some(transfers) = liquidation_transfer_map.remove(&bath_key) {
            liquidation_result_map.insert(
                bath_key,
                TradingEvent::from_liquidation(liquidation_result, transfers),
            );
        }
    }

    // all data are liquidation v2 data, return directly
    if to_time < 1725120000 {
        return Ok(liquidation_result_map.values().cloned().collect());
    }

    // as liquidator role
    let liquidation_results =
        query_liquidation_results_by_time_and_account(from_time, to_time, account_id).await?;
    let mut liquidation_result_keys: BTreeSet<(i64, i32)> = BTreeSet::new();
    liquidation_results.iter().for_each(|liquidation_res| {
        liquidation_result_keys.insert(liquidation_res.get_batch_key());
    });
    let liquidation_transfers = query_account_liquidation_transfers_by_time_and_result_keys(
        from_time,
        to_time,
        liquidation_result_keys,
    )
    .await?;
    let mut liquidation_transfer_map2: BTreeMap<(i64, i32), Vec<DbLiquidationTransfer>> =
        BTreeMap::new();
    liquidation_transfers
        .into_iter()
        .for_each(|liquidation_transfer| {
            let batch_key = liquidation_transfer.get_batch_key();
            if let Some(vec) = liquidation_transfer_map.get_mut(&batch_key) {
                vec.push(liquidation_transfer);
            } else {
                let vec = vec![liquidation_transfer];
                liquidation_transfer_map2.insert(batch_key, vec);
            }
        });
    for liquidation_result in liquidation_results {
        if let Some(transfers) =
            liquidation_transfer_map2.remove(&liquidation_result.get_batch_key())
        {
            liquidation_result_map.insert(
                liquidation_result.get_batch_key(),
                TradingEvent::from_liquidation(liquidation_result, transfers),
            );
        }
    }

    Ok(liquidation_result_map.values().cloned().collect())
}

pub async fn join_adls(from_block: i64, to_block: i64) -> Result<Vec<TradingEvent>> {
    let adl_results = query_adl_results(from_block, to_block).await?;
    let adls_vec: Vec<TradingEvent> = adl_results
        .into_iter()
        .map(TradingEvent::from_adl_result)
        .collect::<Vec<_>>();
    Ok(adls_vec)
}

pub async fn join_account_adls(
    account_id: String,
    from_time: i64,
    to_time: i64,
    offset: Option<u32>,
    _limit: Option<u32>,
) -> Result<Vec<TradingEvent>> {
    if offset.unwrap_or_default() != 0 {
        return Ok(vec![]);
    }
    let adl_results = query_account_adl_results(account_id, from_time, to_time).await?;
    let adls_vec: Vec<TradingEvent> = adl_results
        .into_iter()
        .map(TradingEvent::from_adl_result)
        .collect::<Vec<_>>();
    Ok(adls_vec)
}
