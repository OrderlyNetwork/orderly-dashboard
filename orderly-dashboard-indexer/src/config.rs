use clap::Parser;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

pub static COMMON_CONFIGS: OnceCell<CommonConfigs> = OnceCell::new();

pub fn init_config(config: CommonConfigs) {
    if COMMON_CONFIGS.set(config).is_err() {
        tracing::info!(
            target: crate::ORDERLY_DASHBOARD_INDEXER,
            "COMMON_CONFIGS already inited"
        );
    }
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
pub(crate) struct Opts {
    #[clap(short, long)]
    pub config_path: std::path::PathBuf,
    #[clap(short, long)]
    pub start_block: Option<u64>,
    #[clap(short, long)]
    pub end_block: Option<u64>,
}

#[derive(Clone, Deserialize, Default)]
pub struct SubnetConfig {
    pub rpc_url: String, // evm chain rpc url
    pub rpc_url_fallback: String,
    pub is_use_ws: bool,
    pub ws_url: Option<String>,
    pub pull_check_interval: u32, // seconds
    pub ledger_address: String,
    pub operator_manager_address: String,
    pub user_ledger_abi_path: String,
    pub operator_manager_abi_path: String,
    pub market_manager_address: String,
    pub market_manager_abi_path: String,
    pub confirm_block_num: Option<u32>,
    pub contract_deploy_height: Option<u64>,
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
pub struct LayerzeroConfig {
    pub scan_url: String,
    pub graphql_url: String,
}

#[derive(Clone, Deserialize, Default)]
pub struct CommonConfigs {
    pub l2_config: SubnetConfig,
    pub cefi_server: CefiServerConfig,
    pub indexer_server: IndexerServerConfig,
    pub layerzero: LayerzeroConfig,
}

impl Display for CommonConfigs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let subnet_cfg = &self.l2_config;
        write!(
            f,
            "subnet config:[rpc_url:{}, rpc_url_fallback:{:?}, is_use_ws:{}, ws_url:{:?},pull_check_interval:{},ledger_address:{},operator_manager_address:{},user_ledger_abi_path:{},operator_manager_abi_path:{},market_manager_address:{},market_manager_abi_path:{},confirm_block_num:{:?},contract_deploy_height:{:?}];",
            subnet_cfg.rpc_url, subnet_cfg.rpc_url_fallback, subnet_cfg.is_use_ws, subnet_cfg.ws_url, subnet_cfg.pull_check_interval,subnet_cfg.ledger_address, subnet_cfg.operator_manager_address,
            subnet_cfg.user_ledger_abi_path, subnet_cfg.operator_manager_abi_path, subnet_cfg.market_manager_address, subnet_cfg.market_manager_abi_path, subnet_cfg.confirm_block_num, subnet_cfg.contract_deploy_height
        )?;
        let indexer_server = &self.cefi_server;
        write!(
            f,
            "indexer_server:[server_address:{}];",
            indexer_server.server_address
        )?;
        let cefi_config = &self.indexer_server;
        write!(
            f,
            "cefi_config:[cefi_address:{},public_key:{}].",
            cefi_config.indexer_address, cefi_config.public_key
        )?;
        let layerzero_config = &self.layerzero;
        write!(
            f,
            "layerzero_config:[scan_url:{}].",
            layerzero_config.scan_url,
        )
    }
}
