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

#[derive(Clone, Deserialize, Default, Debug)]
pub struct AnalyzerConfig {
    // indexer url
    pub indexer_address: String,
    pub start_block: i64,
    pub start_block_timestamp: i64,
    pub pull_interval: u64,
    pub batch_block_num: u64,
    pub server_port: u16,
    pub get_broker_url: String,
}
