mod filter_join;
use crate::formats_external::trading_events::TradingEventType;
use crate::formats_external::{trading_events::TradingEventsResponse, Response, SuccessResponse};
use anyhow::{Context, Result};
use std::collections::HashMap;

pub async fn pull_perp_trading_events(
    params: &HashMap<String, String>,
) -> Result<Response<TradingEventsResponse>> {
    let from_block = params
        .get("from_block")
        .context("param from_block not found")?;
    let to_block = params.get("to_block").context("param to_block not found")?;
    let e_type = if let Some(event_type) = params.get("event_type") {
        let event_type = "\"".to_string() + event_type + "\"";
        Some(serde_json::from_str::<TradingEventType>(&event_type)?)
    } else {
        None
    };
    let response =
        filter_join::perp_trading_join_events(from_block.parse()?, to_block.parse()?, e_type)
            .await?;
    Ok(Response::Success(SuccessResponse::new(response)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize() {
        let s = TradingEventType::TRANSACTION.to_string();
        let s = "\"".to_string() + &s + "\"";
        serde_json::from_str::<TradingEventType>(&s).unwrap();
    }
}
