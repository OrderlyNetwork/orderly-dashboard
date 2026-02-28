//! OR-6513: Fill empty broker_hash and transaction_id in partitioned_executed_trades
//! from serial_batches (transaction_id) and analyzer_db user_info (broker_hash).

use crate::analyzer_db::user_info::{create_user_info, get_user_info, UserInfo};
use crate::cefi_client::CefiClient;
use crate::consume_data_task::ORDERLY_DASHBOARD_INDEXER;
use crate::db::partitioned_executed_trades::{
    batch_update_partitioned_executed_trades, query_trades_with_empty_broker_hash,
    PartitionedExecutedTradeUpdate,
};
use crate::db::serial_batches::query_serial_batches_by_keys;
use crate::utils::cal_broker_hash;
use anyhow::Result;
use cached::{Cached, SizedCache};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::sync::Arc;

const FILL_ACCOUNT_BROKER_HASH_CACHE_SIZE: usize = 20_000;

lazy_static! {
    /// Cache for account_id -> broker_hash when filling partitioned_executed_trades.
    static ref FILL_ACCOUNT_BROKER_HASH_CACHE: Mutex<SizedCache<String, String>> =
        Mutex::new(SizedCache::with_size(FILL_ACCOUNT_BROKER_HASH_CACHE_SIZE));
}

/// Run one round of fill: query up to 500 trades with empty broker_hash or transaction_id,
/// get transaction_id from serial_batches (event_type=2), broker_hash from analyzer_db user_info,
/// then update partitioned_executed_trades.
pub async fn run_fill_empty_broker_hash_and_txid(cefi_client: Arc<CefiClient>) -> Result<usize> {
    let trades = query_trades_with_empty_broker_hash().await?;
    if trades.is_empty() {
        return Ok(0);
    }

    // 按 (block_number, transaction_index) 去重，得到要查询的 batch keys
    let batch_keys: BTreeSet<(i64, i32)> = trades.iter().map(|t| t.get_batch_key()).collect();
    let serial_batches = query_serial_batches_by_keys(batch_keys).await?;
    let txid_map: HashMap<(i64, i32), String> = serial_batches
        .into_iter()
        .map(|b| (b.get_batch_key(), b.transaction_id))
        .collect();

    let mut account_ids_to_fetch: Vec<String> = trades
        .iter()
        .filter(|t| t.broker_hash.is_none())
        .map(|t| t.account_id.clone())
        .collect();
    account_ids_to_fetch.sort();
    account_ids_to_fetch.dedup();
    let mut account_broker_map_guard = FILL_ACCOUNT_BROKER_HASH_CACHE.lock();

    for account_id in &account_ids_to_fetch {
        if account_broker_map_guard.cache_get(account_id).is_none() {
            match get_user_info(account_id.clone()).await {
                Ok(Some(user)) => {
                    account_broker_map_guard.cache_set(account_id.clone(), user.broker_hash);
                }
                Ok(None) => {
                    let user_info = cefi_client
                        .cefi_get_account_info_with_retry(account_id)
                        .await;
                    let broker_hash = cal_broker_hash(&user_info.broker_id);
                    account_broker_map_guard.cache_set(account_id.clone(), broker_hash.clone());
                    create_user_info(&UserInfo {
                        account_id: account_id.clone(),
                        broker_id: user_info.broker_id.clone(),
                        broker_hash,
                        address: user_info.address,
                    })
                    .await?;
                }
                Err(e) => {
                    tracing::warn!(target: ORDERLY_DASHBOARD_INDEXER, "fill_partitioned_executed_trades err: {:?}", e);
                }
            }
        }
    }

    let mut updates: Vec<PartitionedExecutedTradeUpdate> = Vec::new();
    for trade in &trades {
        let key = trade.get_batch_key();
        let new_txid = trade
            .transaction_id
            .clone()
            .or_else(|| txid_map.get(&key).cloned());
        let new_broker_hash = trade.broker_hash.clone().or_else(|| {
            account_broker_map_guard
                .cache_get(&trade.account_id)
                .cloned()
        });

        updates.push(PartitionedExecutedTradeUpdate {
            block_number: trade.block_number,
            transaction_index: trade.transaction_index,
            log_index: trade.log_index,
            block_time: trade.block_time,
            broker_hash: new_broker_hash,
            transaction_id: new_txid,
        });
    }

    let updated = batch_update_partitioned_executed_trades(&updates).await?;

    Ok(updated)
}
