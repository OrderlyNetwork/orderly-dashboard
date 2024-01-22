use crate::trading_metrics::write_response;
use actix_web::{get, Responder, Result};
use serde_json::json;

#[get("/status")]
pub async fn get_status() -> Result<impl Responder> {
    Ok(write_response(json!({
        "is_ready": true
    })))
}
