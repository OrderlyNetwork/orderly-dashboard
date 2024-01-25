use clap::Parser;
use serde::Deserialize;

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
pub struct CommonConfig {
    pub port: u16,
    pub is_debug: bool,
}
