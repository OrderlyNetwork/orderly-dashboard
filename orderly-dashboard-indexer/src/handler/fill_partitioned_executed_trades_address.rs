//! Fill empty address in partitioned_executed_trades
//! from analyzer_db user_info (account_id -> address).

use crate::analyzer_db::user_info::get_user_info;
use crate::config::get_common_cfg;
use crate::consume_data_task::ORDERLY_DASHBOARD_INDEXER;
use crate::db::partitioned_executed_trades::{
    batch_update_partitioned_executed_trades_address, query_trades_for_fill_address,
    PartitionedExecutedTradeAddressUpdate,
};
use crate::db::settings::{
    get_fill_partitioned_executed_trades_address_progress, get_last_rpc_processed_height,
    update_fill_partitioned_executed_trades_address_progress,
};
use crate::eth_rpc::get_blockheader_by_number;
use anyhow::Result;
use cached::{Cached, SizedCache};
use chrono::{Duration, NaiveDateTime};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;

const FILL_ACCOUNT_ADDRESS_CACHE_SIZE: usize = 20_000;
const ADDRESS_FILL_BLOCK_SPAN: i64 = 1000;
const ADDRESS_FILL_TIME_WINDOW_DAYS: i64 = 10;

lazy_static! {
    /// Cache for account_id -> address when filling partitioned_executed_trades.
    static ref FILL_ACCOUNT_ADDRESS_CACHE: Mutex<SizedCache<String, String>> =
        Mutex::new(SizedCache::with_size(FILL_ACCOUNT_ADDRESS_CACHE_SIZE));
}

/// Run one round of filling address.
///
/// Pagination strategy:
/// - Start from progress cursor (timestamp/block) or contract deploy (height/timestamp)
/// - Query within [ts, ts+10days] and [block, block+999]
/// - target_block is from settings LastRpcProcessHeight; finished when last trade's block_number >= target_block
pub async fn run_fill_empty_address_in_partitioned_executed_trades() -> Result<usize> {
    let mut progress = get_fill_partitioned_executed_trades_address_progress().await?;
    if progress.finished {
        return Ok(0);
    }

    if progress.target_block <= 0 {
        progress.target_block = get_last_rpc_processed_height().await?.unwrap_or_default() as i64;
        update_fill_partitioned_executed_trades_address_progress(&progress).await?;
    }

    let cfg = get_common_cfg();
    let deploy_ts = cfg.l2_config.contract_deploy_timestamp;
    let deploy_height = cfg.l2_config.contract_deploy_height.unwrap_or_default() as i64;

    if progress.current_timestamp <= 0 {
        progress.current_timestamp = deploy_ts;
    }
    if progress.current_block <= 0 {
        progress.current_block = deploy_height;
    }

    let from_ts = progress.current_timestamp.max(deploy_ts);
    let from_time =
        NaiveDateTime::from_timestamp_opt(from_ts, 0).unwrap_or_else(|| NaiveDateTime::MIN);
    let to_time = from_time
        .checked_add_signed(Duration::days(ADDRESS_FILL_TIME_WINDOW_DAYS))
        .unwrap_or(from_time);

    let from_block = progress.current_block.max(deploy_height);
    let to_block = from_block + (ADDRESS_FILL_BLOCK_SPAN - 1);

    let trades = query_trades_for_fill_address(from_time, to_time, from_block, to_block).await?;

    if trades.is_empty() {
        progress.current_block = to_block;
        progress.current_timestamp = get_blockheader_by_number(to_block as u64)
            .await?
            .timestamp
            .as_u128() as i64;
        update_fill_partitioned_executed_trades_address_progress(&progress).await?;
        return Ok(0);
    }

    let mut account_ids: Vec<String> = trades.iter().map(|t| t.account_id.clone()).collect();
    account_ids.sort();
    account_ids.dedup();

    let mut cache_guard = FILL_ACCOUNT_ADDRESS_CACHE.lock();
    for account_id in &account_ids {
        if cache_guard.cache_get(account_id).is_some() {
            continue;
        }
        match get_user_info(account_id.clone()).await {
            Ok(Some(user)) => {
                if !user.address.trim().is_empty() {
                    cache_guard.cache_set(account_id.clone(), user.address);
                }
            }
            Ok(None) => {
                tracing::warn!(
                    target: ORDERLY_DASHBOARD_INDEXER,
                    "fill_address: user_info not found for account_id:{}",
                    account_id
                );
            }
            Err(e) => {
                tracing::warn!(
                    target: ORDERLY_DASHBOARD_INDEXER,
                    "fill_address: get_user_info err for account_id:{} err:{:?}",
                    account_id,
                    e
                );
            }
        }
    }

    let mut updates: Vec<PartitionedExecutedTradeAddressUpdate> = Vec::new();
    let mut addr_map: HashMap<String, String> = HashMap::new();
    for account_id in &account_ids {
        if let Some(addr) = cache_guard.cache_get(account_id).cloned() {
            addr_map.insert(account_id.clone(), addr);
        }
    }

    for trade in &trades {
        if let Some(new_addr) = addr_map.get(&trade.account_id).cloned() {
            updates.push(PartitionedExecutedTradeAddressUpdate {
                block_number: trade.block_number,
                transaction_index: trade.transaction_index,
                log_index: trade.log_index,
                block_time: trade.block_time,
                address: Some(new_addr),
            });
        }
    }

    let updated = batch_update_partitioned_executed_trades_address(&updates).await?;

    // Move cursor forward; finish when last trade's block_number >= target_block.
    if let Some(last) = trades.last() {
        progress.current_timestamp = last.block_time.timestamp();
        progress.finished = last.block_number >= progress.target_block;
    }
    progress.current_block = to_block;

    update_fill_partitioned_executed_trades_address_progress(&progress).await?;

    Ok(updated)
}
