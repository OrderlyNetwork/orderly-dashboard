use crate::config::{get_common_cfg, COMMON_CONFIGS};
use crate::contract::consume_data_on_block;
use crate::db::settings::{
    get_last_rpc_processed_height, update_last_rpc_processed_height,
    update_last_rpc_processed_timestamp,
};
use crate::eth_rpc::get_latest_block_num;
use crate::service_base::runtime::raw_spawn_future;
use anyhow::Result;
use chrono::Utc;
use futures::future::join_all;
use std::cmp::{max, min};
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::time::Duration;
pub const ORDERLY_DASHBOARD_INDEXER: &str = "orderly_dashboard_indexer";
pub(crate) static ORDERLY_PROCESSED_BLOCK_HEIGHT: AtomicU64 = AtomicU64::new(0);
pub(crate) static ORDERLY_PROCESSED_TIMESTAMP: AtomicI64 = AtomicI64::new(0);

pub async fn consume_data_task(mut start_block: Option<u64>, end_block: Option<u64>) -> Result<()> {
    tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "start consume_data_task");
    if start_block.is_none() {
        let deploy_height = {
            let config = unsafe { &COMMON_CONFIGS.get_unchecked().l2_config };
            config.contract_deploy_height
        };
        start_block = get_last_rpc_processed_height().await?;
        if start_block.is_none() && deploy_height.is_some() {
            start_block = deploy_height;
        }
    }

    let target_block = pull_target_block_until_success().await;
    consume_data_inner(
        start_block.ok_or_else(|| anyhow::anyhow!("start block should not be empty"))? as u64,
        target_block,
        end_block,
        true,
    )
    .await?;

    Ok(())
}

pub async fn pull_target_block() -> Result<u64> {
    Ok(get_latest_block_num().await?
        - unsafe {
            COMMON_CONFIGS
                .get_unchecked()
                .l2_config
                .confirm_block_num
                .unwrap_or_default()
        } as u64)
}

pub async fn pull_target_block_until_success() -> u64 {
    loop {
        match pull_target_block().await {
            Ok(block_height) => return block_height,
            Err(err) => {
                tracing::warn!(target: ORDERLY_DASHBOARD_INDEXER, "pull_target_block inner pull_target_block_util_success err: {}", err);
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
}

pub async fn consume_data_inner(
    mut start_height: u64,
    mut target_block: u64,
    end_block: Option<u64>,
    update_cursor: bool,
) -> Result<()> {
    if start_height == 0 {
        tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "start_height 0 un normal, return",);
        return Ok(());
    }
    if target_block < start_height {
        tracing::info!(
            target: ORDERLY_DASHBOARD_INDEXER,
            "start_height un normal, start_height:{},target_block:{},force update start_height",
            start_height,
            target_block
        );
        start_height = target_block;
    }
    tracing::info!(
        target: ORDERLY_DASHBOARD_INDEXER,
        "enter consume_data_inner loop,start block:{}",
        start_height
    );
    let parallel_limit = get_common_cfg().sync_block_strategy.parallel_limit;
    loop {
        if start_height >= target_block {
            match pull_target_block().await {
                Ok(height) => {
                    if height > target_block {
                        target_block = height;
                    } else {
                        tracing::info!(
                            target: ORDERLY_DASHBOARD_INDEXER,
                            "current:{}, not new blocks, sleep 1 secs",
                            target_block
                        );
                        tokio::time::sleep(Duration::from_secs(2)).await;
                        continue;
                    }
                }
                Err(err) => {
                    tracing::warn!(
                        target: ORDERLY_DASHBOARD_INDEXER,
                        "pull_target_block failed in consume_data_inner, err:{}",
                        err
                    );
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }
            }
        }
        // concurrency
        let gap = min(
            target_block - start_height,
            if parallel_limit == 0 {
                0
            } else {
                (parallel_limit - 1) as u64
            },
        );
        let last_processed = start_height + gap;
        if Utc::now().timestamp() % 60 == 0 {
            tracing::info!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "symbol block summary start_height: {} gap: {} last_processed block is: {}, target_block: {}",
                start_height, gap, last_processed, target_block
            );
        }
        match parallel_consume_blocks(start_height, gap).await {
            Err(err) => {
                tokio::time::sleep(Duration::from_secs(5)).await;
                tracing::warn!(
                    target: ORDERLY_DASHBOARD_INDEXER,
                    "parallel_consume_blocks err: {}",
                    err
                );
            }
            Ok(block_timestamp) => {
                if update_cursor {
                    if let Err(err) = update_last_rpc_processed_height(last_processed).await {
                        tracing::warn!(
                            target: ORDERLY_DASHBOARD_INDEXER,
                            "update_last_rpc_processed_height failed with err: {}",
                            err
                        );
                    }
                    if let Err(err) = update_last_rpc_processed_timestamp(block_timestamp).await {
                        tracing::warn!(
                            target: ORDERLY_DASHBOARD_INDEXER,
                            "update_last_rpc_processed_timestamp failed with err: {},block_timestamp:{}",
                            err, block_timestamp
                        );
                    }
                    ORDERLY_PROCESSED_BLOCK_HEIGHT.store(last_processed, Ordering::Relaxed);
                    ORDERLY_PROCESSED_TIMESTAMP.store(block_timestamp, Ordering::Relaxed);  
                }
                if let Some(end_block) = end_block {
                    if start_height > end_block {
                        tracing::info!(
                            target: ORDERLY_DASHBOARD_INDEXER,
                            "reach end_block height: {}, exit consume block",
                            end_block
                        );
                        break;
                    }
                }
                // may be bigger than target_block here
                start_height = last_processed + 1;
            }
        }
    }
    Ok(())
}

// returning value is block timestamp
pub async fn parallel_consume_blocks(start_height: u64, gap: u64) -> Result<i64> {
    let mut futs = Vec::with_capacity(gap as usize);
    (start_height..=(start_height + gap)).for_each(|block_height| {
        futs.push(consume_data_on_block(block_height));
    });
    let res = raw_spawn_future(join_all(futs)).await?;
    let mut timestamp = 0;
    for r in res {
        match r {
            Ok(block_timestamp) => {
                timestamp = max(timestamp, block_timestamp);
            }
            Err(err) => {
                tracing::warn!(target: ORDERLY_DASHBOARD_INDEXER, "consume block task err:{}", err);
                return Err(anyhow::anyhow!("Some task failed to be executed."));
            }
        }
    }
    Ok(timestamp)
}
