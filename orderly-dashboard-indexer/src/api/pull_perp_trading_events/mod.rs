mod filter_join;

use crate::config::get_common_cfg;
use crate::consume_data_task::ORDERLY_DASHBOARD_INDEXER;
use crate::formats_external::trading_events::{AccountTradingEventsResponse, TradingEventType};
use crate::formats_external::{
    trading_events::TradingEventsResponse, FailureResponse, Response, SuccessResponse,
};
use anyhow::{Context, Result};
use chrono::Utc;
use std::cmp::min;
use std::collections::HashMap;
use std::str::FromStr;

// 31 days, one month
pub const QUERY_RANGE_S: i64 = 31 * 24 * 3600;

pub async fn pull_perp_trading_events(
    params: &HashMap<String, String>,
) -> Result<Response<TradingEventsResponse>> {
    let from_block = params
        .get("from_block")
        .context("param from_block not found")?;
    let to_block = params.get("to_block").context("param to_block not found")?;
    let e_type = if let Some(event_type) = params.get("event_type") {
        let event_type = "\"".to_string() + event_type + "\"";
        match serde_json::from_str::<TradingEventType>(&event_type) {
            Ok(event_type) => Some(event_type),
            Err(err) => {
                return Ok(Response::Failure(FailureResponse::new(
                    1000,
                    format!("parse event_type failed with err: {}", err),
                )))
            }
        }
    } else {
        None
    };
    let response =
        filter_join::perp_trading_join_events(from_block.parse()?, to_block.parse()?, e_type)
            .await?;
    Ok(Response::Success(SuccessResponse::new(response)))
}

pub async fn pull_sol_events(
    params: &HashMap<String, String>,
) -> Result<Response<TradingEventsResponse>> {
    let from_block = params
        .get("from_block")
        .context("param from_block not found")?;
    let to_block = params.get("to_block").context("param to_block not found")?;
    let e_type = if let Some(event_type) = params.get("event_type") {
        let event_type = "\"".to_string() + event_type + "\"";
        match serde_json::from_str::<TradingEventType>(&event_type) {
            Ok(event_type) => Some(event_type),
            Err(err) => {
                return Ok(Response::Failure(FailureResponse::new(
                    1000,
                    format!("parse event_type failed with err: {}", err),
                )))
            }
        }
    } else {
        None
    };
    let response =
        filter_join::sol_join_events(from_block.parse()?, to_block.parse()?, e_type).await?;
    Ok(Response::Success(SuccessResponse::new(response)))
}

pub async fn pull_perp_trading_events_by_account(
    params: &HashMap<String, String>,
) -> Result<Response<AccountTradingEventsResponse>> {
    let account_id = params
        .get("account_id")
        .context("param account_id not found")?;
    let now = Utc::now().timestamp();
    let from_time = if let Some(from_time) = params.get("from_time") {
        i64::from_str(from_time)?
    } else {
        now - QUERY_RANGE_S
    };
    let mut to_time = if let Some(to_time) = params.get("to_time") {
        i64::from_str(to_time)?
    } else {
        now
    };
    if to_time < from_time {
        tracing::info!(
            target: ORDERLY_DASHBOARD_INDEXER,
            "to_time: {} smaller than from_time: {}",
            to_time, from_time,
        );
        return Ok(Response::Success(SuccessResponse::new(
            AccountTradingEventsResponse::default(),
        )));
    }
    if to_time - from_time > QUERY_RANGE_S {
        return Ok(Response::Failure(FailureResponse::new(
            1000,
            format!(
                "to_time - from_time should less than {} days",
                QUERY_RANGE_S as f64 / (24 * 3600) as f64
            ),
        )));
    }
    let e_type = if let Some(event_type) = params.get("event_type") {
        let event_type = "\"".to_string() + event_type + "\"";
        match serde_json::from_str::<TradingEventType>(&event_type) {
            Ok(event_type) => Some(event_type),
            Err(err) => {
                return Ok(Response::Failure(FailureResponse::new(
                    1000,
                    format!("parse event_type failed with err: {}", err),
                )))
            }
        }
    } else {
        None
    };
    let orderly_processed_time =
        crate::api::get_may_cached_orderly_last_rpc_processed_timestamp().await?;
    
        to_time = min(orderly_processed_time, to_time);
    
    tracing::info!(target: ORDERLY_DASHBOARD_INDEXER,
        "account_id events: {}, from_time: {}, to_time: {} e_type: {:?}", account_id, from_time, to_time, e_type
    );

    let response =
        filter_join::account_perp_trading_join_events(account_id, from_time, to_time, e_type)
            .await?;

    Ok(Response::Success(SuccessResponse::new(response)))
}

pub async fn pull_perp_trading_events_by_account_v2(
    params: &HashMap<String, String>,
) -> Result<Response<AccountTradingEventsResponse>> {
    let account_id = params
        .get("account_id")
        .context("param account_id not found")?;
    let now = Utc::now().timestamp();
    let from_time = if let Some(from_time) = params.get("from_time") {
        i64::from_str(from_time)?
    } else {
        now - QUERY_RANGE_S
    };
    let mut to_time = if let Some(to_time) = params.get("to_time") {
        i64::from_str(to_time)?
    } else {
        now
    };
    if to_time < from_time {
        tracing::info!(
            target: ORDERLY_DASHBOARD_INDEXER,
            "to_time: {} smaller than from_time: {}",
            to_time, from_time,
        );
        return Ok(Response::Success(SuccessResponse::new(
            AccountTradingEventsResponse::default(),
        )));
    }
    if to_time - from_time > QUERY_RANGE_S {
        return Ok(Response::Failure(FailureResponse::new(
            1000,
            format!(
                "to_time - from_time should less than {} days",
                QUERY_RANGE_S as f64 / (24 * 3600) as f64
            ),
        )));
    }
    let e_type = if let Some(event_type) = params.get("event_type") {
        let event_type = "\"".to_string() + event_type + "\"";
        match serde_json::from_str::<TradingEventType>(&event_type) {
            Ok(event_type) => Some(event_type),
            Err(err) => {
                return Ok(Response::Failure(FailureResponse::new(
                    1000,
                    format!("parse event_type failed with err: {}", err),
                )))
            }
        }
    } else {
        None
    };
    let offset = if let Some(offset) = params.get("offset") {
        u32::from_str(offset)?
    } else {
        0
    };
    let limit = get_common_cfg().db_query_limit;

    let orderly_processed_time=
        crate::api::get_may_cached_orderly_last_rpc_processed_timestamp().await?;
        to_time = min(orderly_processed_time, to_time);
    tracing::info!(target: ORDERLY_DASHBOARD_INDEXER,
        "events v2 account_id: {}, from_time: {}, to_time: {} e_type: {:?}, offset: {}, limit: {}", account_id, from_time, to_time, e_type, offset, limit as u32
    );

    let response = filter_join::account_perp_trading_join_events_v2(
        account_id,
        from_time,
        to_time,
        e_type,
        offset,
        limit as u32,
    )
    .await?;

    Ok(Response::Success(SuccessResponse::new(response)))
}

pub async fn pull_sol_events_by_account(
    params: &HashMap<String, String>,
) -> Result<Response<AccountTradingEventsResponse>> {
    let account_id = params
        .get("account_id")
        .context("param account_id not found")?;
    let now = Utc::now().timestamp();
    let from_time = if let Some(from_time) = params.get("from_time") {
        i64::from_str(from_time)?
    } else {
        now - QUERY_RANGE_S
    };
    let mut to_time = if let Some(to_time) = params.get("to_time") {
        i64::from_str(to_time)?
    } else {
        now
    };
    if to_time < from_time {
        tracing::info!(
            target: ORDERLY_DASHBOARD_INDEXER,
            "to_time: {} smaller than from_time: {}",
            to_time, from_time,
        );
        return Ok(Response::Success(SuccessResponse::new(
            AccountTradingEventsResponse::default(),
        )));
    }
    if to_time - from_time > QUERY_RANGE_S {
        return Ok(Response::Failure(FailureResponse::new(
            1000,
            format!(
                "to_time - from_time should less than {} days",
                QUERY_RANGE_S as f64 / (24 * 3600) as f64
            ),
        )));
    }
    let e_type = if let Some(event_type) = params.get("event_type") {
        let event_type = "\"".to_string() + event_type + "\"";
        match serde_json::from_str::<TradingEventType>(&event_type) {
            Ok(event_type) => Some(event_type),
            Err(err) => {
                return Ok(Response::Failure(FailureResponse::new(
                    1000,
                    format!("parse event_type failed with err: {}", err),
                )))
            }
        }
    } else {
        None
    };
    let sol_block_time = crate::db::settings::get_sol_sync_block_time().await?;
    tracing::info!(target: ORDERLY_DASHBOARD_INDEXER,
        "account_id: {}, from_time: {}, to_time: {}, sol_block_time: {:?}, e_type: {:?}", account_id, from_time, to_time, sol_block_time, e_type
    );
    if let Some(sol_block_time) = sol_block_time {
        to_time = min(sol_block_time, to_time);
    }
    let response =
        filter_join::account_sol_join_events(account_id, from_time, to_time, e_type).await?;

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
