pub mod calculate_gas;
pub mod network_info;
pub mod pull_perp_trading_events;
#[allow(dead_code)]
pub mod recovery;
pub mod symbols_config;
use std::sync::atomic::Ordering;

use crate::{consume_data_task::{ORDERLY_PROCESSED_BLOCK_HEIGHT, ORDERLY_PROCESSED_TIMESTAMP}, formats_external::{Response, SuccessResponse}, db::settings::{get_last_rpc_processed_height, get_last_rpc_processed_timestamp}};
use anyhow::Result;
pub use network_info::get_network_info;
pub use pull_perp_trading_events::{
    pull_perp_trading_events, pull_perp_trading_events_by_account,
    pull_perp_trading_events_by_account_v2, pull_sol_events, pull_sol_events_by_account,
};
pub use symbols_config::get_symbols_data;

pub async fn get_status() -> Result<Response<serde_json::Value>> {
    let data = serde_json::json!({
        "is_ready": true,
    });
    Ok(Response::Success(SuccessResponse::new(data)))
}

pub(crate) async fn get_may_cached_orderly_last_rpc_processed_height() -> Result<u64> {
    let height = ORDERLY_PROCESSED_BLOCK_HEIGHT.load(Ordering::Relaxed);
    let height = if height == 0 {
        get_last_rpc_processed_height().await?.unwrap_or_default()
    } else {
        height
    };

    Ok(height)
}

pub(crate) async fn get_may_cached_orderly_last_rpc_processed_timestamp() -> Result<i64> {
    let timestamp = ORDERLY_PROCESSED_TIMESTAMP.load(Ordering::Relaxed);
    let timestamp = if timestamp == 0 {
        get_last_rpc_processed_timestamp().await?.unwrap_or_default()
    } else {
        timestamp
    };
    Ok(timestamp)
}