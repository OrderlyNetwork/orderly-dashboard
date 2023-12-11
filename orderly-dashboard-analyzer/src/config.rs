use clap::Parser;
use once_cell::sync::OnceCell;
use serde::Deserialize;

pub static PULL_CONFIGS: OnceCell<AnalyzerConfig> = OnceCell::new();

pub fn init_config(config: AnalyzerConfig) {
    if PULL_CONFIGS.set(config).is_err() {
        tracing::info!(
            target: crate::ORDERLY_DASHBOARD_ANALYZER,
            "PullBlockConfig already inited"
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
}

#[derive(Clone, Deserialize, Default, Debug)]
pub struct AnalyzerConfig {
    // indexer url
    pub indexer_address: String,
    pub init_block: String,
    pub pull_interval: i32,
}
