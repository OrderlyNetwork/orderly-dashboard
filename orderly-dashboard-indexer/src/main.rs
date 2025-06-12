#![allow(non_local_definitions)]

mod api;
mod bindings;
#[allow(dead_code)]
mod client;
pub(crate) mod config;
pub mod constants;
mod consume_data_task;
mod contract;
mod db;
mod eth_rpc;
mod formats_external;
mod handler;
mod init;
mod schema;
mod server;
mod service_base;
mod settings;
pub mod transform;
pub mod utils;
extern crate diesel;

use crate::config::{CommonConfigs, Opts};
use crate::init::init_handler;
use crate::server::webserver;
use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

use crate::handler::check_or_create_new_partition::{
    check_or_create_executed_trades_partition, migrate_executed_trades_data,
};
use crate::{
    consume_data_task::{consume_data_task, ORDERLY_DASHBOARD_INDEXER},
    db::settings::update_last_sol_syn_signature,
    handler::solana::solana_program_log_processor::{
        get_starting_signature, handle_sol_program_logs, SolanaProgramLogProcessor,
    },
};

fn main() -> Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv().ok();

    let opts = Opts::parse();
    let raw_common_config =
        std::fs::read_to_string(&opts.config_path).expect("missing_common_config_file");
    let config: CommonConfigs =
        serde_json::from_str(&raw_common_config).expect("unable_to_deserialize_common_configs");
    init_handler(&config)?;

    let system = actix::System::new();
    system.block_on(async move {
        tokio::spawn(webserver(config.clone()));
        tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "Orderly Dashboard Indexer started! with opts:{:?}, config: {}", opts, config);
        let sol_config = &config.sol_chain_config;
        if sol_config.is_enable {
            actix::spawn(handler::solana::solana_program_log_processor::process_solana_logs(
                SolanaProgramLogProcessor::new(
                    sol_config,
                    sol_config.program_address.clone(),
                    get_starting_signature,
                    handle_sol_program_logs,
                    update_last_sol_syn_signature,
                )
            ));
        }
        if let Err(err) = check_or_create_executed_trades_partition().await {
            tracing::warn!(target: ORDERLY_DASHBOARD_INDEXER, "check_or_create_executed_trades_partition failed with err: {}, exit", err);
            return;
        }
        if let Err(err) = migrate_executed_trades_data(config.option_query_from_partitioning_executed_trades).await {
            tracing::warn!(target: ORDERLY_DASHBOARD_INDEXER, "migrate_executed_trades_data failed with err: {}, exit", err);
            return;
        }
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_range_iter() {
        let start = 3;
        let end = 6;
        let s = end - start;
        let mut res = Vec::with_capacity(s + 1);
        (start..=end).for_each(|v| {
            res.push(v);
        });
        assert_eq!(res, vec![3, 4, 5, 6]);
    }
}
