pub mod calculate_gas;
pub mod network_info;
pub mod pull_perp_trading_events;
#[allow(dead_code)]
pub mod recovery;
pub mod symbols_config;
use crate::formats_external::{Response, SuccessResponse};
use anyhow::Result;
pub use network_info::get_network_info;
pub use pull_perp_trading_events::{
    pull_perp_trading_events, pull_perp_trading_events_by_account, pull_sol_events,
    pull_sol_events_by_account,
};
pub use symbols_config::get_symbols_data;
pub async fn get_status() -> Result<Response<serde_json::Value>> {
    let data = serde_json::json!({
        "is_ready": true,
    });
    Ok(Response::Success(SuccessResponse::new(data)))
}
