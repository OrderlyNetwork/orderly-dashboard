mod api;
mod bindings;
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
mod tasks;
pub mod utils;
#[macro_use]
extern crate diesel;
use crate::client::{get_default_client, HttpClient};
use crate::config::{CommonConfigs, Opts, COMMON_CONFIGS};
use crate::contract::consume_data_on_block;
use crate::db::settings::{get_last_rpc_processed_height, update_last_rpc_processed_height};
use crate::eth_rpc::get_latest_block_num;
use crate::init::init_handler;
use crate::server::webserver;
use anyhow::Result;
use bindings::operator_manager::{operator_managerCalls, operator_managerEvents};
use clap::Parser;
use ethers::core::abi::{AbiDecode, RawLog};
use ethers::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::H256;
use std::convert::TryFrom;
use std::str::FromStr;
use std::time::Duration;

const ORDERLY_DASHBOARD_INDEXER: &str = "orderly_dashboard_indexer";

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let raw_common_config =
        std::fs::read_to_string(opts.config_path).expect("missing_common_config_file");
    let config: CommonConfigs =
        serde_json::from_str(&raw_common_config).expect("unable_to_deserialize_common_configs");
    init_handler(&config)?;
    let provider = Provider::<Http>::try_from(config.l2_config.rpc_url.clone())
        .expect("could not instantiate HTTP Provider");

    let http_client = get_default_client()?;
    let system = actix::System::new();
    system.block_on(async move {
        tokio::spawn(webserver(config.clone()));
        tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "Orderly Dashboard Indexer started!");

        if let Err(err) = consume_data_task(provider, http_client).await {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "consume_data_task err: {:?}",
                err
            );
        }

        actix::System::current().stop();
    });

    system.run().unwrap();

    Ok(())
}

pub(crate) async fn consume_data_task(provider: Provider<Http>, client: HttpClient) -> Result<()> {
    tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "start consume_data_task");
    let deploy_height = {
        let config = unsafe { &COMMON_CONFIGS.get_unchecked().l2_config };
        config.contract_deploy_height
    };
    let mut start_block = get_last_rpc_processed_height().await?;
    if start_block.is_none() && deploy_height.is_some() {
        start_block = deploy_height;
    }

    let target_block = pull_target_block().await?;
    consume_data_inner(
        start_block.ok_or_else(|| anyhow::anyhow!("start block should not be empty"))? as u64,
        target_block,
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

async fn consume_data_inner(mut start_height: u64, mut target_block: u64) -> Result<()> {
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
        "enter update_gas_cost_inner loop,start block:{}",
        start_height
    );
    let http_client = get_default_client()?;
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
        if let Err(err) = consume_data_on_block(start_height, http_client.clone()).await {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "consume_data_on_block failed with err: {}",
                err
            );
            tokio::time::sleep(Duration::from_secs(1)).await;
        } else {
            if start_height % 30 == 0 {
                tracing::info!(
                    target: ORDERLY_DASHBOARD_INDEXER,
                    "update_gas_cost_inner will checked block: {}",
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
            start_height += 1;
        }
    }
}
