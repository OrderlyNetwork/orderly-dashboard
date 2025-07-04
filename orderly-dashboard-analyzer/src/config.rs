use clap::Parser;
use serde::Deserialize;

#[allow(duplicate_macro_attributes)]
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

#[derive(Clone, Deserialize, Debug)]
pub struct AnalyzerConfig {
    // indexer url
    pub indexer_address: String,
    pub start_block: i64,
    #[allow(dead_code)]
    pub start_block_timestamp: i64,
    pub pull_interval: u64,
    pub batch_block_num: u64,
    pub server_port: u16,
    pub get_broker_url: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}

fn default_base_url() -> String {
    "https://api.orderly.org".to_string()
}
