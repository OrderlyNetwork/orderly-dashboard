use crate::trading_metrics::write_response;
use actix_web::{get, Responder, Result};
use serde_json::json;

const HEALTH_CHECK_CONTEXT: &str = "health_check_context";

#[get("/status")]
pub async fn get_status() -> Result<impl Responder> {
    if chrono::Utc::now().timestamp_millis() % 10 == 1 {
        tracing::info!(target: HEALTH_CHECK_CONTEXT, "health check ping");
    }
    Ok(write_response(json!({
        "is_ready": true
    })))
}
