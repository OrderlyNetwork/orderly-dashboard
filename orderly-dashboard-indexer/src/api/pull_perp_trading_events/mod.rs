mod filter_join;
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
    let response =
        filter_join::perp_trading_join_events(from_block.parse()?, to_block.parse()?).await?;
    Ok(Response::Success(SuccessResponse::new(response)))
}
