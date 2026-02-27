use crate::indexer_db::trades::{count_trades_from_db, query_trades_from_db};
use crate::trades::{CONTRACT_DEPLOY_HEIGHT, LAST_RPC_PROCESS_TIMESTAMP};
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDateTime;
use orderly_dashboard_analyzer::sync_broker::cal_broker_hash;
use orderly_dashboard_indexer::db::partitioned_executed_trades::DbPartitionedExecutedTrades;
use serde_derive::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use std::time::Instant;
use utoipa::ToSchema;

pub(crate) const QUERY_TRADES_CONTEXT: &str = "query_trades_context";

/// Page size for query trades API (number of records per page)
pub const QUERY_TRADES_PAGE_SIZE: u64 = 200;

/// Maximum time range for query trades API (24 hours in seconds)
pub const MAX_TIME_RANGE_SECONDS: i64 = 24 * 3600;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct PaginationCursorRequest {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub block_time: i64,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct QueryTradesRequest {
    pub broker_id: Option<String>,
    pub account_id: Option<String>,
    pub from_time: i64,
    pub to_time: i64,
    pub next_cursor: Option<PaginationCursorRequest>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TradeItem {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub typ: i16,
    pub account_id: String,
    pub symbol_hash: String,
    pub fee_asset_hash: String,
    pub trade_qty: String,
    pub notional: String,
    pub executed_price: String,
    pub fee: String,
    pub sum_unitary_fundings: String,
    pub trade_id: String,
    pub match_id: String,
    pub timestamp: String,
    pub side: bool,
    pub block_time: i64,
    pub broker_hash: Option<String>,
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PaginationCursor {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub block_time: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct QueryTradesResponse {
    pub trades: Vec<TradeItem>,
    pub next_cursor: Option<PaginationCursor>,
    pub total_trades: Option<i64>,
}

impl From<DbPartitionedExecutedTrades> for TradeItem {
    fn from(trade: DbPartitionedExecutedTrades) -> Self {
        TradeItem {
            block_number: trade.block_number,
            transaction_index: trade.transaction_index,
            log_index: trade.log_index,
            typ: trade.typ,
            account_id: trade.account_id,
            symbol_hash: trade.symbol_hash,
            fee_asset_hash: trade.fee_asset_hash,
            trade_qty: trade.trade_qty.to_string(),
            notional: trade.notional.to_string(),
            executed_price: trade.executed_price.to_string(),
            fee: trade.fee.to_string(),
            sum_unitary_fundings: trade.sum_unitary_fundings.to_string(),
            trade_id: trade.trade_id.to_string(),
            match_id: trade.match_id.to_string(),
            timestamp: trade.timestamp.to_string(),
            side: trade.side,
            block_time: trade.block_time.timestamp(),
            broker_hash: trade.broker_hash,
            transaction_id: trade.transaction_id,
        }
    }
}

/// Query trades from partitioned_executed_trades table
///
/// This API allows querying trades filtered by broker_id or account_id (must provide exactly one) with required time range.
/// Results are sorted by (block_number, transaction_index, log_index) and paginated with 200 records per page.
///
/// Time Range: The time range between `from_time` and `to_time` must not exceed 24 hours (86400 seconds).
///
/// Pagination: If next_cursor is returned, use it in the next request to get the next page.
///
/// total_trades: This field is only returned when `next_cursor` is not provided in the request (i.e., the first page query).
/// It represents the total number of trades matching the filter criteria (broker_id/account_id and time range).
/// When querying subsequent pages using `next_cursor`, this field will be `null` to avoid unnecessary count queries.
#[utoipa::path(
    responses(
        (status = 200, description = "Query trades response. `total_trades` is only returned when `next_cursor` is not provided in the request (first page query).", body = QueryTradesResponse),
        (status = 1000, description = "Invalid Request")
    ),
    request_body(
        content = QueryTradesRequest,
        content_type = "application/json",
        description = "Query trades request. Must provide either broker_id or account_id (but not both)."
    ),
)]
#[post("/trades")]
pub async fn query_trades(param: web::Json<QueryTradesRequest>) -> actix_web::Result<HttpResponse> {
    let param = param.into_inner();
    // Validate that either broker_id or account_id is provided, but not both
    match (&param.broker_id, &param.account_id) {
        (None, None) => {
            return Ok(HttpResponse::Ok().json(
                orderly_dashboard_indexer::formats_external::FailureResponse::new(
                    1000,
                    "Either broker_id or account_id must be provided".to_string(),
                ),
            ));
        }
        (Some(_), Some(_)) => {
            return Ok(HttpResponse::Ok().json(
                orderly_dashboard_indexer::formats_external::FailureResponse::new(
                    1000,
                    "broker_id and account_id cannot be provided at the same time. Please provide only one of them.".to_string(),
                )
            ));
        }
        _ => {}
    }

    tracing::info!(
        target: QUERY_TRADES_CONTEXT,
        "query trades start broker_id: {:?}, account_id: {:?}, from_time: {:?}, to_time: {:?}, cursor: {:?}",
        param.broker_id, param.account_id, param.from_time, param.to_time, param.next_cursor,
    );

    let start_time = Instant::now();

    // Convert broker_id to broker_hash if provided
    let broker_hash = param.broker_id.as_ref().map(|id| cal_broker_hash(id));

    // Convert timestamps to NaiveDateTime
    let from_time = NaiveDateTime::from_timestamp_opt(param.from_time, 0)
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid from_time timestamp"))?;
    let to_time = NaiveDateTime::from_timestamp_opt(param.to_time, 0)
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid to_time timestamp"))?;

    // Validate time range: maximum 24 hours (24 * 3600 seconds)
    let time_range_seconds = (to_time - from_time).num_seconds();
    if time_range_seconds > MAX_TIME_RANGE_SECONDS {
        return Ok(HttpResponse::Ok().json(
            orderly_dashboard_indexer::formats_external::FailureResponse::new(
                1000,
                format!(
                    "Time range exceeds maximum allowed duration of {} seconds (24 hours). Please reduce the time range.",
                    MAX_TIME_RANGE_SECONDS
                ),
            ),
        ));
    }

    // Validate that to_time is after from_time
    if to_time < from_time {
        return Ok(HttpResponse::Ok().json(
            orderly_dashboard_indexer::formats_external::FailureResponse::new(
                1000,
                "to_time must be greater than or equal to from_time".to_string(),
            ),
        ));
    }

    // Calculate total_trades only if this is the first page (no cursor provided)
    let total_trades = if param.next_cursor.is_none() {
        match count_trades_from_db(
            broker_hash.clone(),
            param.account_id.clone(),
            from_time,
            to_time,
        )
        .await
        {
            Ok(total) => Some(total),
            Err(err) => {
                tracing::warn!(
                    target: QUERY_TRADES_CONTEXT,
                    "Failed to count total trades: {}", err
                );
                return Ok(HttpResponse::Ok().json(
                    orderly_dashboard_indexer::formats_external::FailureResponse::new(
                        1000,
                        format!("Query trades count failed: {}", err),
                    ),
                ));
            }
        }
    } else {
        None
    };

    match query_trades_from_db(
        broker_hash,
        param.account_id.clone(),
        from_time,
        to_time,
        param.next_cursor.as_ref().map(|c| c.block_number),
        param.next_cursor.as_ref().map(|c| c.transaction_index),
        param.next_cursor.as_ref().map(|c| c.log_index),
        param
            .next_cursor
            .as_ref()
            .and_then(|c| NaiveDateTime::from_timestamp_opt(c.block_time, 0)),
    )
    .await
    {
        Ok(trades) => {
            let mut response = QueryTradesResponse {
                trades: trades.iter().map(|t| TradeItem::from(t.clone())).collect(),
                next_cursor: None,
                total_trades,
            };

            // If we got exactly QUERY_TRADES_PAGE_SIZE records, there might be more, so set next_cursor
            if trades.len() == QUERY_TRADES_PAGE_SIZE as usize {
                if let Some(last_trade) = trades.last() {
                    response.next_cursor = Some(PaginationCursor {
                        block_number: last_trade.block_number,
                        transaction_index: last_trade.transaction_index,
                        log_index: last_trade.log_index,
                        block_time: last_trade.block_time.timestamp(),
                    });
                }
            }

            let elapsed_ms = start_time.elapsed().as_millis();
            tracing::info!(
                target: QUERY_TRADES_CONTEXT,
                "query trades success. broker_id: {:?}, account_id: {:?}, result len: {}, cost: {} ms",
                param.broker_id, param.account_id, response.trades.len(), elapsed_ms,
            );

            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            let elapsed_ms = start_time.elapsed().as_millis();
            tracing::warn!(
                target: QUERY_TRADES_CONTEXT,
                "query trades failed. broker_id: {:?}, account_id: {:?}, err: {}, cost: {} ms",
                param.broker_id, param.account_id, err, elapsed_ms,
            );
            Ok(HttpResponse::Ok().json(
                orderly_dashboard_indexer::formats_external::FailureResponse::new(
                    1000,
                    format!("Query trades failed: {}", err),
                ),
            ))
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TradesStatusResponse {
    pub engine_start_time: u64,
    pub last_rpc_process_timestamp: u64,
    pub query_trades_page_size: u64,
    pub max_time_range_seconds: i64,
}

/// Get trades status including EngineStartTime and LastRpcProcessTimeStamp
///
/// This API returns the current values of:
/// - engine_start_time: The engine start time (contract deployment height) from indexer database settings
/// - last_rpc_process_timestamp: The last RPC processed timestamp from indexer database settings
/// - query_trades_page_size: The page size for query trades API (number of records per page)
/// - max_time_range_seconds: The maximum allowed time range for query trades API (in seconds)
#[utoipa::path(
    responses(
        (status = 200, description = "Trades status response", body = TradesStatusResponse),
    ),
)]
#[get("/trades_status")]
pub async fn get_trades_status() -> actix_web::Result<HttpResponse> {
    let engine_start_time = CONTRACT_DEPLOY_HEIGHT.load(Ordering::Relaxed);
    let last_rpc_process_timestamp = LAST_RPC_PROCESS_TIMESTAMP.load(Ordering::Relaxed);

    let response = TradesStatusResponse {
        engine_start_time,
        last_rpc_process_timestamp,
        query_trades_page_size: QUERY_TRADES_PAGE_SIZE,
        max_time_range_seconds: MAX_TIME_RANGE_SECONDS,
    };

    tracing::info!(
        target: QUERY_TRADES_CONTEXT,
        "get_trades_status: engine_start_time={}, last_rpc_process_timestamp={}, query_trades_page_size={}, max_time_range_seconds={}",
        engine_start_time,
        last_rpc_process_timestamp,
        QUERY_TRADES_PAGE_SIZE,
        MAX_TIME_RANGE_SECONDS
    );

    Ok(HttpResponse::Ok().json(response))
}
