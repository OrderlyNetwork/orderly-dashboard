mod api;
mod bindings;
#[allow(dead_code)]
mod client;
mod config;
mod contract;
mod db;
mod eth_rpc;
mod formats_external;
mod init;
mod schema;
mod server;
mod service_base;
mod settings;
pub mod utils;
#[macro_use]
extern crate diesel;
use crate::config::{CommonConfigs, Opts, COMMON_CONFIGS};
use crate::contract::consume_data_on_block;
use crate::db::settings::{get_last_rpc_processed_height, update_last_rpc_processed_height};
use crate::eth_rpc::get_latest_block_num;
use crate::init::init_handler;
use crate::server::webserver;
use anyhow::Result;
use clap::Parser;
use std::time::Duration;

const ORDERLY_DASHBOARD_INDEXER: &str = "orderly_dashboard_indexer";

fn main() -> Result<()> {
    let opts = Opts::parse();
    let raw_common_config =
        std::fs::read_to_string(&opts.config_path).expect("missing_common_config_file");
    let config: CommonConfigs =
        serde_json::from_str(&raw_common_config).expect("unable_to_deserialize_common_configs");
    init_handler(&config)?;

    let system = actix::System::new();
    system.block_on(async move {
        tokio::spawn(webserver(config.clone()));
        tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "Orderly Dashboard Indexer started! with opts:{:?}", opts);
        let early_stop = opts.end_block.is_some() && opts.start_block.is_none() && opts.end_block.unwrap_or_default() < opts.start_block.unwrap_or_default();
        if early_stop {
            tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "end_block: {:?} < start_block: {:?} , not need to consume data", opts.end_block, opts.start_block);
        } else {
            if let Err(err) = consume_data_task(opts.start_block, opts.end_block).await {
                tracing::warn!(
                    target: ORDERLY_DASHBOARD_INDEXER,
                    "consume_data_task err: {:?}",
                    err
                );
            }
        }

        tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "Orderly Dashboard Indexer blocked! with opts:{:?}", opts);
        std::thread::park();
        actix::System::current().stop();
    });

    system.run().unwrap();

    Ok(())
}

pub(crate) async fn consume_data_task(
    mut start_block: Option<u64>,
    end_block: Option<u64>,
) -> Result<()> {
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

    let target_block = pull_target_block().await?;
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
    loop {
        if start_height == target_block {
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
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        continue;
                    }
                }
                Err(err) => {
                    tracing::warn!(
                        target: ORDERLY_DASHBOARD_INDEXER,
                        "pull_target_block failed in consume_data_inner, err:{}",
                        err
                    );
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            }
        }
        if let Err(err) = consume_data_on_block(start_height).await {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "consume_data_on_block failed with err: {}",
                err
            );
            tokio::time::sleep(Duration::from_secs(1)).await;
        } else {
            if update_cursor && start_height % 30 == 0 {
                tracing::info!(
                    target: ORDERLY_DASHBOARD_INDEXER,
                    "consume_data_inner checked block: {}",
                    start_height
                );
                if let Err(err) = update_last_rpc_processed_height(start_height).await {
                    tracing::info!(
                        target: ORDERLY_DASHBOARD_INDEXER,
                        "update_last_rpc_processed_height failed with err: {}",
                        err
                    );
                }
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
            start_height += 1;
        }
    }
    Ok(())
}
