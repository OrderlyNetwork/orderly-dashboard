use clap::Parser;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

use crate::consume_data_task::ORDERLY_DASHBOARD_INDEXER;

pub static COMMON_CONFIGS: OnceCell<CommonConfigs> = OnceCell::new();

pub fn init_config(config: CommonConfigs) {
    if COMMON_CONFIGS.set(config).is_err() {
        tracing::info!(
            target: ORDERLY_DASHBOARD_INDEXER,
            "COMMON_CONFIGS already inited"
        );
    }
}

pub fn get_common_cfg() -> &'static CommonConfigs {
    unsafe { COMMON_CONFIGS.get_unchecked() }
}
#[derive(Parser, Debug)]
#[clap(
    version,
    author,
    about,
    disable_help_subcommand(true),
    propagate_version(true),
    next_line_help(true)
)]
pub struct Opts {
    #[clap(short, long)]
    pub config_path: std::path::PathBuf,
    #[clap(short, long)]
    pub start_block: Option<u64>,
    #[clap(short, long)]
    pub end_block: Option<u64>,
}

#[allow(dead_code)]
#[derive(Clone, Deserialize, Default)]
pub struct SubnetConfig {
    pub rpc_url: String, // evm chain rpc url
    // pub rpc_url_fallback: String,
    // pub is_use_ws: bool,
    // pub ws_url: Option<String>,
    pub pull_check_interval: u32, // seconds
    pub ledger_address: String,
    pub operator_manager_address: String,
    pub user_ledger_abi_path: String,
    pub operator_manager_abi_path: String,
    pub market_manager_address: String,
    pub market_manager_abi_path: String,
    pub confirm_block_num: Option<u32>,
    pub contract_deploy_height: Option<u64>,
    pub contract_deploy_timestamp: i64,
    pub explorer_url: String,
    pub vault_manager_address: String,
    pub vault_manager_abi_path: String,
    pub upgrade_height: u64,
}

#[derive(Clone, Deserialize, Default)]
pub struct CefiServerConfig {
    pub server_address: String, // server url
    pub private_key: String,
}

#[derive(Clone, Deserialize, Default)]
pub struct IndexerServerConfig {
    pub indexer_address: String, // indexer url
    pub public_key: String,
}

#[derive(Clone, Deserialize, Default)]
pub struct SyncBlockStrategy {
    pub parallel_limit: u32,
}

#[allow(dead_code)]
#[derive(Clone, Deserialize, Default)]
pub struct LayerzeroConfig {
    pub scan_url: String,
    pub graphql_url: String,
}

fn empty_string() -> String {
    "".to_string()
}

fn default_db_query_limit() -> usize {
    1000
}

fn default_option_query_from_partitioning_executed_trades() -> bool {
    false
}

#[allow(dead_code)]
#[derive(Clone, Deserialize, Default, Debug)]
pub struct SolChainConfig {
    pub is_enable: bool,
    pub chain_id: u64,
    pub rpc_url: String,
    pub start_sig: String,
    pub program_address: String,
    #[serde(default = "empty_string")]
    pub vault_usdc: String,
}

#[derive(Clone, Deserialize, Default)]
pub struct CommonConfigs {
    pub l2_config: SubnetConfig,
    pub indexer_server: IndexerServerConfig,
    pub sync_block_strategy: SyncBlockStrategy,
    pub layerzero: LayerzeroConfig,
    pub perp_symbols_config: Vec<String>,
    pub sol_chain_config: SolChainConfig,
    #[serde(default = "default_db_query_limit")]
    pub db_query_limit: usize,
    #[serde(default = "default_option_query_from_partitioning_executed_trades")]
    pub option_query_from_partitioning_executed_trades: bool,
    pub be_api_base_url: String,
}

impl Display for CommonConfigs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let subnet_cfg = &self.l2_config;
        write!(
            f,
            "subnet config:[rpc_url:{}, pull_check_interval:{},ledger_address:{},operator_manager_address:{},user_ledger_abi_path:{},operator_manager_abi_path:{},market_manager_address:{},market_manager_abi_path:{},confirm_block_num:{:?},contract_deploy_height:{:?},contract_deploy_timestamp:{},explorer_url: {}, upgrade_height: {}];",
            subnet_cfg.rpc_url, subnet_cfg.pull_check_interval,subnet_cfg.ledger_address, subnet_cfg.operator_manager_address,
            subnet_cfg.user_ledger_abi_path, subnet_cfg.operator_manager_abi_path, subnet_cfg.market_manager_address, subnet_cfg.market_manager_abi_path, subnet_cfg.confirm_block_num, subnet_cfg.contract_deploy_height, subnet_cfg.contract_deploy_timestamp, subnet_cfg.explorer_url, subnet_cfg.upgrade_height,
        )?;
        let indexer_server = &self.indexer_server;
        write!(
            f,
            "indexer_server:[indexer_address:{},public_key:{}];",
            indexer_server.indexer_address, indexer_server.public_key
        )?;
        write!(
            f,
            "sync_block_strategy: [parallel_limit: {}]",
            &self.sync_block_strategy.parallel_limit
        )?;
        let layerzero_config = &self.layerzero;
        write!(
            f,
            "layerzero_config:[scan_url:{}];",
            layerzero_config.scan_url,
        )?;
        write!(f, "perp_symbols:{:?};", self.perp_symbols_config,)?;
        write!(f, "sol_chain_config:{:?}.", self.sol_chain_config,)?;
        write!(
            f,
            "db_query_limit: {}, option_query_from_partitioning_executed_trades: {}",
            self.db_query_limit, self.option_query_from_partitioning_executed_trades
        )?;
        write!(f, "be_api_base_url: {}", self.be_api_base_url,)
    }
}
