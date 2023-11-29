pub mod pull_perp_trading_events;
pub mod recovery;
use crate::formats_external::{Response, SuccessResponse};
use anyhow::Result;
pub use pull_perp_trading_events::pull_perp_trading_events;
pub async fn get_status() -> Result<Response<serde_json::Value>> {
    let data = serde_json::json!({
        "is_ready": true,
    });
    Ok(Response::Success(SuccessResponse::new(data)))
}
