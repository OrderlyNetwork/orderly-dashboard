use crate::trades::trades_api::QUERY_TRADES_PAGE_SIZE;
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::dsl::sql;
use diesel::sql_types::Bool;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use orderly_dashboard_indexer::db::partitioned_executed_trades::DbPartitionedExecutedTrades;
use orderly_dashboard_indexer::db::POOL;

pub const DB_TRADES_CONTEXT: &str = "indexer_db_trades_context";

/// Query trades from partitioned_executed_trades table
///
/// This function queries the indexer database for trades filtered by:
/// - broker_hash (optional)
/// - account_id (optional)
/// - time range (required)
/// - pagination cursor (optional)
///
/// Results are sorted by (block_number, transaction_index, log_index) and limited to 200 records.
pub async fn query_trades_from_db(
    broker_hash_param: Option<String>,
    account_id_param: Option<String>,
    address_param: Option<String>,
    symbol_hash_param: Option<String>,
    from_time: NaiveDateTime,
    to_time: NaiveDateTime,
    offset_block_number: Option<i64>,
    offset_transaction_index: Option<i32>,
    offset_log_index: Option<i32>,
    offset_block_time: Option<NaiveDateTime>,
) -> Result<Vec<DbPartitionedExecutedTrades>> {
    use orderly_dashboard_indexer::schema::partitioned_executed_trades::dsl::{
        account_id, address, block_number, block_time, broker_hash, log_index,
        partitioned_executed_trades, symbol_hash, transaction_index,
    };

    let mut conn = POOL
        .get()
        .await
        .expect("Couldn't get db connection from the pool");

    let mut query = partitioned_executed_trades.into_boxed();

    // Apply filters
    if let Some(ref broker) = broker_hash_param {
        query = query.filter(broker_hash.eq(broker.as_str()));
    }
    if let Some(ref account) = account_id_param {
        query = query.filter(account_id.eq(account.as_str()));
    }
    if let Some(ref addr) = address_param {
        query = query.filter(address.eq(addr.as_str()));
    }
    if let Some(ref sym_hash) = symbol_hash_param {
        query = query.filter(symbol_hash.eq(sym_hash.as_str()));
    }
    // Time range is required
    query = query.filter(block_time.ge(from_time));
    query = query.filter(block_time.le(to_time));

    // Apply pagination cursor using keyset pagination.
    // If an optional filter field exists (broker_hash / account_id / address / symbol_hash),
    // put that field at the first position in the tuple, followed by
    // (block_time, block_number, transaction_index, log_index).
    if let Some(cursor_block_time) = offset_block_time {
        // build tuple and value depending on which filter is used
        let (tuple_cols, tuple_values) = if let Some(ref broker) = broker_hash_param {
            let escaped = broker.replace('\'', "''");
            (
                "broker_hash, block_time, block_number, transaction_index, log_index",
                format!(
                    "'{}', '{}', {}, {}, {}",
                    escaped,
                    cursor_block_time,
                    offset_block_number.unwrap_or_default(),
                    offset_transaction_index.unwrap_or_default(),
                    offset_log_index.unwrap_or_default()
                ),
            )
        } else if let Some(ref account) = account_id_param {
            let escaped = account.replace('\'', "''");
            (
                "account_id, block_time, block_number, transaction_index, log_index",
                format!(
                    "'{}', '{}', {}, {}, {}",
                    escaped,
                    cursor_block_time,
                    offset_block_number.unwrap_or_default(),
                    offset_transaction_index.unwrap_or_default(),
                    offset_log_index.unwrap_or_default()
                ),
            )
        } else if let Some(ref addr) = address_param {
            let escaped = addr.replace('\'', "''");
            (
                "address, block_time, block_number, transaction_index, log_index",
                format!(
                    "'{}', '{}', {}, {}, {}",
                    escaped,
                    cursor_block_time,
                    offset_block_number.unwrap_or_default(),
                    offset_transaction_index.unwrap_or_default(),
                    offset_log_index.unwrap_or_default()
                ),
            )
        } else if let Some(ref sym_hash) = symbol_hash_param {
            let escaped = sym_hash.replace('\'', "''");
            (
                "symbol_hash, block_time, block_number, transaction_index, log_index",
                format!(
                    "'{}', '{}', {}, {}, {}",
                    escaped,
                    cursor_block_time,
                    offset_block_number.unwrap_or_default(),
                    offset_transaction_index.unwrap_or_default(),
                    offset_log_index.unwrap_or_default()
                ),
            )
        } else {
            (
                "block_time, block_number, transaction_index, log_index",
                format!(
                    "'{}', {}, {}, {}",
                    cursor_block_time,
                    offset_block_number.unwrap_or_default(),
                    offset_transaction_index.unwrap_or_default(),
                    offset_log_index.unwrap_or_default()
                ),
            )
        };

        let tuple_condition = sql::<Bool>(&format!("({}) > ({})", tuple_cols, tuple_values));
        query = query.filter(tuple_condition);
    }

    // Order by: if optional filter field exists, put it at first position in ORDER BY.
    if broker_hash_param.is_some() {
        query = query.order_by((
            broker_hash,
            block_time,
            block_number,
            transaction_index,
            log_index,
        ));
    } else if account_id_param.is_some() {
        query = query.order_by((
            account_id,
            block_time,
            block_number,
            transaction_index,
            log_index,
        ));
    } else if address_param.is_some() {
        query = query.order_by((
            address,
            block_time,
            block_number,
            transaction_index,
            log_index,
        ));
    } else if symbol_hash_param.is_some() {
        query = query.order_by((
            symbol_hash,
            block_time,
            block_number,
            transaction_index,
            log_index,
        ));
    } else {
        query = query.order_by((block_time, block_number, transaction_index, log_index));
    }

    // Limit to QUERY_TRADES_PAGE_SIZE records
    query = query.limit(QUERY_TRADES_PAGE_SIZE as i64);

    let result = query.load::<DbPartitionedExecutedTrades>(&mut conn).await;

    match result {
        Ok(trades) => Ok(trades),
        Err(diesel::NotFound) => Ok(vec![]),
        Err(err) => Err(anyhow::anyhow!("Database query failed: {}", err)),
    }
}
