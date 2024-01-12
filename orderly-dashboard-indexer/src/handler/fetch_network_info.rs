use crate::db::settings::update_network_info;
use crate::eth_rpc::clone_provider;
use crate::formats_external::NetworkInfo;
use crate::service_base::runtime::spawn_future;
use anyhow::Result;
use ethers::middleware::Middleware;
use ethers::types::BlockNumber;

pub const SYNC_NETWORK_INFO_CONTEXT: &str = "sync_network_info_context";

pub fn init_fetch_network_info_task() -> Result<()> {
    let provider = clone_provider();
    spawn_future(async move {
        loop {
            if let Some(block) = provider.get_block(BlockNumber::Finalized).await.map_err(|err| tracing::warn!(target: SYNC_NETWORK_INFO_CONTEXT, "get block failed: {}", err)).ok() {
                if let Some(block) = block {
                    update_network_info(NetworkInfo{finalized_height: Some(block.number.unwrap_or_default().as_u64())}).await.ok();
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(20)).await;
        }
        #[allow(unreachable_code)]
        Ok(())
    });

    Ok(())
}
