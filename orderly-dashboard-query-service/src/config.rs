use crate::ORDERLY_DASHBOARD_CONTEXT;
use clap::Parser;
use once_cell::sync::OnceCell;
use serde::Deserialize;

pub static COMMON_CONFIGS: OnceCell<CommonConfig> = OnceCell::new();

pub fn init_config(config: CommonConfig) {
    if COMMON_CONFIGS.set(config).is_err() {
        tracing::info!(
            target: ORDERLY_DASHBOARD_CONTEXT,
            "COMMON_CONFIGS already inited"
        );
    }
}

pub fn get_common_cfg() -> &'static CommonConfig {
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
}

#[derive(Clone, Deserialize, Default, Debug)]
pub struct CommonConfig {
    pub port: u16,
    pub is_debug: bool,
    pub indexer_address: String,
    pub is_raw_query_allow: bool,
}
