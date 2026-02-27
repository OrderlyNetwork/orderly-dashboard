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
    from_time: NaiveDateTime,
    to_time: NaiveDateTime,
    offset_block_number: Option<i64>,
    offset_transaction_index: Option<i32>,
    offset_log_index: Option<i32>,
    _offset_block_time: Option<NaiveDateTime>,
) -> Result<Vec<DbPartitionedExecutedTrades>> {
    use orderly_dashboard_indexer::schema::partitioned_executed_trades::dsl::{
        account_id, block_number, block_time, broker_hash, log_index, partitioned_executed_trades,
        transaction_index,
    };

    let mut conn = POOL
        .get()
        .await
        .expect("Couldn't get db connection from the pool");

    let mut query = partitioned_executed_trades.into_boxed();

    // Apply filters
    if let Some(broker) = broker_hash_param {
        query = query.filter(broker_hash.eq(broker));
    }
    if let Some(account) = account_id_param {
        query = query.filter(account_id.eq(account));
    }
    // Time range is required
    query = query.filter(block_time.ge(from_time));
    query = query.filter(block_time.le(to_time));

    // Apply pagination cursor
    if let Some(offset_block) = offset_block_number {
        let tuple_condition = sql::<Bool>(&format!(
            "(block_number, transaction_index, log_index) > ({}, {}, {})",
            offset_block,
            offset_transaction_index.unwrap_or_default(),
            offset_log_index.unwrap_or_default()
        ));
        query = query.filter(tuple_condition);
    }

    // Order by (block_number, transaction_index, log_index)
    query = query.order_by((block_number, transaction_index, log_index));

    // Limit to QUERY_TRADES_PAGE_SIZE records
    query = query.limit(QUERY_TRADES_PAGE_SIZE as i64);

    let result = query.load::<DbPartitionedExecutedTrades>(&mut conn).await;

    match result {
        Ok(trades) => Ok(trades),
        Err(diesel::NotFound) => Ok(vec![]),
        Err(err) => Err(anyhow::anyhow!("Database query failed: {}", err)),
    }
}

/// Count total trades from partitioned_executed_trades table
///
/// This function counts the total number of trades in the indexer database filtered by:
/// - broker_hash (optional)
/// - account_id (optional)
/// - time range (required)
///
/// This is used to get the total count for pagination purposes.
pub async fn count_trades_from_db(
    broker_hash_param: Option<String>,
    account_id_param: Option<String>,
    from_time: NaiveDateTime,
    to_time: NaiveDateTime,
) -> Result<i64> {
    use orderly_dashboard_indexer::schema::partitioned_executed_trades::dsl::{
        account_id, block_time, broker_hash, partitioned_executed_trades,
    };

    let mut conn = POOL
        .get()
        .await
        .expect("Couldn't get db connection from the pool");

    let mut query = partitioned_executed_trades.into_boxed();

    // Apply filters
    if let Some(broker) = broker_hash_param {
        query = query.filter(broker_hash.eq(broker));
    }
    if let Some(account) = account_id_param {
        query = query.filter(account_id.eq(account));
    }
    // Time range is required
    query = query.filter(block_time.ge(from_time));
    query = query.filter(block_time.le(to_time));

    let result = query.count().get_result::<i64>(&mut conn).await;

    match result {
        Ok(count) => Ok(count),
        Err(err) => Err(anyhow::anyhow!("Database count query failed: {}", err)),
    }
}
