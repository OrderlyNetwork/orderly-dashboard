use actix_web::{get, Responder, Result};

use crate::db::symbols;
use crate::db::tokens;
use crate::trading_metrics::write_response;

const LOOKUP_CONTEXT: &str = "lookup_context";

#[get("/symbols")]
pub async fn get_symbols() -> Result<impl Responder> {
    tracing::debug!(target: LOOKUP_CONTEXT, "/symbols request");
    match symbols::get_all_symbols().await {
        Ok(data) => Ok(write_response(data)),
        Err(err) => {
            tracing::warn!(target: LOOKUP_CONTEXT, "get_symbols failed: {}", err);
            Ok(crate::trading_metrics::write_failed_response(
                1,
                &err.to_string(),
            ))
        }
    }
}

#[get("/tokens")]
pub async fn get_tokens() -> Result<impl Responder> {
    tracing::debug!(target: LOOKUP_CONTEXT, "/tokens request");
    match tokens::get_all_tokens().await {
        Ok(data) => Ok(write_response(data)),
        Err(err) => {
            tracing::warn!(target: LOOKUP_CONTEXT, "get_tokens failed: {}", err);
            Ok(crate::trading_metrics::write_failed_response(
                1,
                &err.to_string(),
            ))
        }
    }
}
