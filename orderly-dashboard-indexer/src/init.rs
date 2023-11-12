use crate::config::{init_config, CommonConfigs};
use crate::contract::init_addr_set;
use crate::eth_rpc::init_provider;
use crate::service_base::runtime::init_pool_workers_num;

pub(crate) fn init_log() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(std::io::stderr)
        // .with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}

pub fn init_handler(config: &CommonConfigs) -> anyhow::Result<()> {
    init_log();
    init_pool_workers_num(8);
    init_config(config.clone());
    init_addr_set()?;
    init_provider()?;
    Ok(())
}
