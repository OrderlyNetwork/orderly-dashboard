use crate::schema::partitioned_executed_trades;
use crate::{
    constants::ALERT_CONTEXT,
    db::{executed_trades::DbExecutedTrades, DB_CONN_ERR_MSG, DB_CONTEXT, POOL},
};
use anyhow::Result;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::dsl::sql;
use diesel::sql_query;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use diesel_async::RunQueryDsl;
use std::time::Instant;

pub const PARTITIONED_EXECUTED_TRADES_TABLE_NAME: &str = "partitioned_executed_trades";

#[derive(Insertable, Queryable, Debug, Clone)]
#[diesel(table_name = partitioned_executed_trades)]
pub struct DbPartitionedExecutedTrades {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub typ: i16,
    pub account_id: String,
    pub symbol_hash: String,
    pub fee_asset_hash: String,
    pub trade_qty: BigDecimal,
    pub notional: BigDecimal,
    pub executed_price: BigDecimal,
    pub fee: BigDecimal,
    pub sum_unitary_fundings: BigDecimal,
    pub trade_id: BigDecimal,
    pub match_id: BigDecimal,
    pub timestamp: BigDecimal,
    pub side: bool,
    pub block_time: NaiveDateTime,
    pub broker_hash: Option<String>,
    pub transaction_id: Option<String>,
    pub margin_mode: Option<i16>,
    pub iso_margin_asset_hash: Option<String>,
    pub margin_from_cross: Option<BigDecimal>,
    pub address: Option<String>,
}

#[derive(Insertable, Queryable, Debug, Clone)]
#[diesel(table_name = partitioned_executed_trades)]
pub struct KeyDbPartitionedExecutedTrades {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub account_id: String,
    pub block_time: NaiveDateTime,
}

impl KeyDbPartitionedExecutedTrades {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.transaction_index)
    }
}

impl From<DbExecutedTrades> for DbPartitionedExecutedTrades {
    fn from(value: DbExecutedTrades) -> Self {
        DbPartitionedExecutedTrades {
            block_number: value.block_number,
            transaction_index: value.transaction_index,
            log_index: value.log_index,
            typ: value.typ,
            account_id: value.account_id,
            symbol_hash: value.symbol_hash,
            fee_asset_hash: value.fee_asset_hash,
            trade_qty: value.trade_qty,
            notional: value.notional,
            executed_price: value.executed_price,
            fee: value.fee,
            sum_unitary_fundings: value.sum_unitary_fundings,
            trade_id: value.trade_id,
            match_id: value.match_id,
            timestamp: value.timestamp,
            side: value.side,
            block_time: NaiveDateTime::from_timestamp_opt(value.block_time, 0).unwrap_or_default(),
            broker_hash: None,
            transaction_id: None,
            margin_mode: None,
            iso_margin_asset_hash: None,
            margin_from_cross: None,
            address: None,
        }
    }
}

impl DbPartitionedExecutedTrades {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.transaction_index)
    }
}

pub fn format_partition_name(year_month: u64) -> String {
    let year = year_month / 100;
    let month = year_month % 100;
    format!("executed_trades_y{}q{:02}", year, month)
}

pub async fn create_partitioned_executed_trades(
    trades: Vec<DbPartitionedExecutedTrades>,
) -> Result<usize> {
    use crate::schema::partitioned_executed_trades::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let num_rows = diesel::insert_into(partitioned_executed_trades)
        .values(trades)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}

/// Query trades where broker_hash is null, for backfill.
/// Ordered by (block_number, transaction_index, log_index), limit 500.
pub async fn query_trades_with_empty_broker_hash() -> Result<Vec<KeyDbPartitionedExecutedTrades>> {
    use crate::schema::partitioned_executed_trades::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = partitioned_executed_trades
        .select((
            block_number,
            transaction_index,
            log_index,
            account_id,
            block_time,
        ))
        .filter(broker_hash.is_null())
        // .order_by((block_number, transaction_index, log_index))
        .limit(400)
        .load::<KeyDbPartitionedExecutedTrades>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let rows = match result {
        Ok(rows) => {
            if dur_ms >= 100 {
                tracing::warn!(
                    target: ALERT_CONTEXT,
                    "query_trades_with_empty_broker_hash slow query. length:{}, used time:{} ms",
                    rows.len(),
                    dur_ms
                );
            }
            rows
        }
        Err(e) => {
            tracing::warn!(
                target: DB_CONTEXT,
                "query_trades_with_empty_broker_hash fail. err:{:?}, used time:{} ms",
                e,
                dur_ms
            );
            return Err(e.into());
        }
    };
    Ok(rows)
}

/// Query trades for filling address in a given time and block range.
/// Returns all rows in the range (no limit); may be large (e.g. hundreds of thousands).
/// Does not select address; finished is determined by target_block.
pub async fn query_trades_for_fill_address(
    from_time: NaiveDateTime,
    to_time: NaiveDateTime,
    from_block: i64,
    to_block: i64,
) -> Result<Vec<KeyDbPartitionedExecutedTrades>> {
    use crate::schema::partitioned_executed_trades::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = partitioned_executed_trades
        .select((
            block_number,
            transaction_index,
            log_index,
            account_id,
            block_time,
        ))
        .filter(block_time.ge(from_time))
        .filter(block_time.le(to_time))
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .order_by((block_number, transaction_index, log_index))
        .load::<KeyDbPartitionedExecutedTrades>(&mut conn)
        .await;

    let dur_ms = (Instant::now() - start_time).as_millis();
    let rows = match result {
        Ok(rows) => {
            if dur_ms >= 100 {
                tracing::warn!(
                    target: ALERT_CONTEXT,
                    "query_trades_for_fill_address slow query. from_block:{}, to_block:{}, rows:{}, used time:{} ms, last record: {:?}",
                    from_block,
                    to_block,
                    rows.len(),
                    dur_ms,
                    rows.last(),
                );
            } else {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_trades_for_fill_address success. rows:{}, used time:{} ms, last record: {:?}",
                    rows.len(),
                    dur_ms,
                    rows.last(),
                );
            }
            rows
        }
        Err(e) => {
            tracing::warn!(
                target: DB_CONTEXT,
                "query_trades_for_fill_address fail. err:{:?}, used time:{} ms",
                e,
                dur_ms
            );
            return Err(e.into());
        }
    };
    Ok(rows)
}

/// Update broker_hash and/or transaction_id for one row by primary key.
#[allow(dead_code)]
pub async fn update_partitioned_executed_trade_broker_hash_and_txid(
    block_number_: i64,
    transaction_index_: i32,
    log_index_: i32,
    block_time_: NaiveDateTime,
    broker_hash_: Option<String>,
    transaction_id_: Option<String>,
) -> Result<()> {
    use crate::schema::partitioned_executed_trades::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let target = partitioned_executed_trades
        .filter(block_number.eq(block_number_))
        .filter(transaction_index.eq(transaction_index_))
        .filter(log_index.eq(log_index_))
        .filter(block_time.eq(block_time_));

    diesel::update(target)
        .set((
            broker_hash.eq(broker_hash_),
            transaction_id.eq(transaction_id_),
        ))
        .execute(&mut conn)
        .await?;
    Ok(())
}

/// One row to update in batch (pk + new broker_hash and transaction_id).
#[derive(Debug, Clone)]
pub struct PartitionedExecutedTradeUpdate {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub block_time: NaiveDateTime,
    pub broker_hash: Option<String>,
    pub transaction_id: Option<String>,
}

/// One row to update address in batch (pk + new address).
#[derive(Debug, Clone)]
pub struct PartitionedExecutedTradeAddressUpdate {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub block_time: NaiveDateTime,
    pub address: Option<String>,
}

fn escape_sql_text(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\'', "''")
}

fn sql_value_text(opt: &Option<String>) -> String {
    match opt {
        None => "NULL::text".to_string(),
        Some(s) => format!("'{}'::text", escape_sql_text(s)),
    }
}

const BATCH_UPDATE_CHUNK_SIZE: usize = 100;

/// Batch update broker_hash and transaction_id for multiple rows in one or few queries.
pub async fn batch_update_partitioned_executed_trades(
    updates: &[PartitionedExecutedTradeUpdate],
) -> Result<usize> {
    if updates.is_empty() {
        return Ok(0);
    }
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let mut total = 0usize;
    for chunk in updates.chunks(BATCH_UPDATE_CHUNK_SIZE) {
        let values: Vec<String> = chunk
            .iter()
            .map(|u| {
                let bt = u.block_time.format("%Y-%m-%d %H:%M:%S").to_string();
                format!(
                    "({}, {}, {}, '{}'::timestamp, {}, {})",
                    u.block_number,
                    u.transaction_index,
                    u.log_index,
                    escape_sql_text(&bt),
                    sql_value_text(&u.broker_hash),
                    sql_value_text(&u.transaction_id),
                )
            })
            .collect();
        let values_clause = values.join(", ");
        let sql = format!(
            "UPDATE partitioned_executed_trades t SET broker_hash = v.broker_hash, transaction_id = v.transaction_id \
             FROM (VALUES {}) AS v(block_number, transaction_index, log_index, block_time, broker_hash, transaction_id) \
             WHERE t.block_number = v.block_number AND t.transaction_index = v.transaction_index \
             AND t.log_index = v.log_index AND t.block_time = v.block_time",
            values_clause
        );
        let n = sql_query(&sql).execute(&mut conn).await?;
        total += n;
    }
    let dur_ms = (Instant::now() - start_time).as_millis();
    if dur_ms >= 100 {
        tracing::warn!(
            target: ALERT_CONTEXT,
            "batch_update_partitioned_executed_trades slow query. updated_rows:{}, count:{}, used time:{} ms",
            total,
            updates.len(),
            dur_ms
        );
    }

    Ok(total)
}

/// Batch update address for multiple rows in one or few queries.
pub async fn batch_update_partitioned_executed_trades_address(
    updates: &[PartitionedExecutedTradeAddressUpdate],
) -> Result<usize> {
    if updates.is_empty() {
        return Ok(0);
    }
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let mut total = 0usize;
    const BATCH_UPDATE_ADDRESS_CHUNK_SIZE: usize = 2000;
    for chunk in updates.chunks(BATCH_UPDATE_ADDRESS_CHUNK_SIZE) {
        let values: Vec<String> = chunk
            .iter()
            .map(|u| {
                let bt = u.block_time.format("%Y-%m-%d %H:%M:%S").to_string();
                format!(
                    "({}, {}, {}, '{}'::timestamp, {})",
                    u.block_number,
                    u.transaction_index,
                    u.log_index,
                    escape_sql_text(&bt),
                    sql_value_text(&u.address),
                )
            })
            .collect();
        let values_clause = values.join(", ");
        let sql = format!(
            "UPDATE partitioned_executed_trades t SET address = v.address \
             FROM (VALUES {}) AS v(block_number, transaction_index, log_index, block_time, address) \
             WHERE t.block_number = v.block_number AND t.transaction_index = v.transaction_index \
             AND t.log_index = v.log_index AND t.block_time = v.block_time",
            values_clause
        );
        let n = sql_query(&sql).execute(&mut conn).await?;
        total += n;
        if chunk.len() == BATCH_UPDATE_ADDRESS_CHUNK_SIZE {
            tokio::time::sleep(std::time::Duration::from_millis(25)).await;
        }
    }
    let dur_ms = (Instant::now() - start_time).as_millis();
    if dur_ms >= 100 {
        tracing::warn!(
            target: ALERT_CONTEXT,
            "batch_update_partitioned_executed_trades_address slow query. updated_rows:{}, count:{}, used time:{} ms",
            total,
            updates.len(),
            dur_ms
        );
    }
    Ok(total)
}

#[allow(dead_code)]
pub async fn query_partitioned_executed_trades(
    from_time: NaiveDateTime,
    to_time: NaiveDateTime,
    from_block: i64,
    to_block: i64,
) -> Result<Vec<DbPartitionedExecutedTrades>> {
    use crate::schema::partitioned_executed_trades::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_partitioned_executed_trades start, from_time:{}, to_time:{}, from_block:{}, to_block:{}",
        from_time.timestamp(), to_time.timestamp(), from_block, to_block,
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = partitioned_executed_trades
        .filter(block_time.ge(from_time))
        .filter(block_time.le(to_time))
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load::<DbPartitionedExecutedTrades>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            if dur_ms >= 100 {
                tracing::warn!(
                    target: ALERT_CONTEXT,
                    "query_partitioned_executed_trades slow query. from:{}, to:{} length:{}, used time:{} ms",
                        from_block,
                        to_block,
                    events.len(),
                    dur_ms
                );
            }
            tracing::info!(
                target: DB_CONTEXT,
                "query_partitioned_executed_trades success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_partitioned_executed_trades success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_partitioned_executed_trades fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn count_user_in_time_range(
    account: String,
    from_time: NaiveDateTime,
    to_time: NaiveDateTime,
) -> Result<i64> {
    use crate::schema::partitioned_executed_trades::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "count_user_in_range start with account: {},from_time: {}, to_time: {}",
        account, from_time, to_time,
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let count = partitioned_executed_trades
        .filter(block_time.ge(from_time))
        .filter(block_time.le(to_time))
        .filter(account_id.eq(account.clone()))
        .count()
        .get_result::<i64>(&mut conn)
        .await?;

    let dur_ms = (Instant::now() - start_time).as_millis();
    if dur_ms >= 1000 {
        tracing::warn!(
            target: ALERT_CONTEXT,
            "count_user_in_time_range slow query. account_id: {}, from_time:{}, to_time:{}, used time:{} ms",
            account,
                from_time,
                to_time,
            dur_ms
        );
    }

    Ok(count)
}

// time of executed trades need to be convert from secs to millisecond
#[allow(dead_code)]
pub async fn query_account_partitioned_executed_trades(
    account: String,
    from_time: NaiveDateTime,
    to_time: NaiveDateTime,
    limit: Option<u32>,
    offset_block_number: Option<i64>,
    offset_transaction_index: Option<i32>,
    offset_log_index: Option<i32>,
) -> Result<Vec<DbPartitionedExecutedTrades>> {
    use crate::schema::partitioned_executed_trades::dsl::*;
    use diesel::sql_types::Bool;
    tracing::info!(
        target: DB_CONTEXT,
        "query_account_partitioned_executed_trades start",
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let query = partitioned_executed_trades
        .filter(block_time.ge(from_time))
        .filter(block_time.le(to_time))
        .filter(account_id.eq(account.clone()));
    let result = if let Some(offset_block_number) = offset_block_number {
        let tuple_condition = sql::<Bool>(&format!(
            "(block_number, transaction_index, log_index) > ({}, {}, {})",
            offset_block_number,
            offset_transaction_index.unwrap_or_default(),
            offset_log_index.unwrap_or_default()
        ));
        query
            // .filter(block_number.le(offset_block_number.unwrap_or_default()))
            // .filter(transaction_index.le(offset_transaction_index.unwrap_or_default()))
            // .filter(log_index.gt(offset_log_index.unwrap_or_default()))
            // .filter((block_number, transaction_index, log_index).gt(offset_block_number.unwrap_or_default(), offset_transaction_index.unwrap_or_default(), offset_log_index.unwrap_or_default()))
            .filter(tuple_condition)
            .order_by((block_time, block_number, transaction_index, log_index))
            .limit(limit.unwrap_or_default() as i64)
            .load::<DbPartitionedExecutedTrades>(&mut conn)
            .await
    } else {
        query
            .order_by((block_time, block_number, transaction_index, log_index))
            .limit(limit.unwrap_or_default() as i64)
            .load::<DbPartitionedExecutedTrades>(&mut conn)
            .await
    };
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            if dur_ms >= 100 {
                tracing::warn!(
                    target: ALERT_CONTEXT,
                    "query_account_partitioned_executed_trades slow query. account_id: {}, from_time:{}, to_time:{} length:{}, used time:{} ms",
                    account,
                        from_time,
                        to_time,
                    events.len(),
                    dur_ms
                );
            }
            tracing::info!(
                target: DB_CONTEXT,
                "query_account_partitioned_executed_trades success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_account_partitioned_executed_trades success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_account_partitioned_executed_trades fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

/// Generate mock data: deterministically produce one row from global row_index.
/// Used to insert ~112M rows into partitioned_executed_trades, with each partition >10M and 2M extra on 2024-07-22.
#[cfg(test)]
fn mock_trade_for_index(
    row_index: u64,
    brokers: &[&str],
    addresses: &[&str],
    account_ids: &[String],
) -> DbPartitionedExecutedTrades {
    use bigdecimal::FromPrimitive;
    const TX_PER_BLOCK: u64 = 11; // transaction_index 0..=10
    const LOG_PER_TX: u64 = 1001; // log_index 0..=1000
    const SLOT: u64 = TX_PER_BLOCK * LOG_PER_TX; // 11011

    let block_number = (1 + (row_index / SLOT) % 3_831_371) as i64;
    let transaction_index = ((row_index / LOG_PER_TX) % TX_PER_BLOCK) as i32;
    let log_index = (row_index % LOG_PER_TX) as i32;

    let broker_i = (row_index % 10) as usize;
    let addr_i = (row_index / 10) % 20;
    let account_i = (broker_i * 10 + (addr_i as usize % 10)) % 100;

    let broker_hash = brokers
        .get(broker_i)
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("broker_{}", broker_i));
    let address = addresses
        .get(addr_i as usize)
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("addr_{}", addr_i));
    let account_id = account_ids
        .get(account_i)
        .cloned()
        .unwrap_or_else(|| format!("account_{}", account_i));

    let (block_time, partition_index) = mock_block_time_for_row_index(row_index);

    let _ = partition_index;
    let ts_secs = block_time.timestamp();
    let ts_ms = ts_secs.saturating_mul(1000);

    DbPartitionedExecutedTrades {
        block_number,
        transaction_index,
        log_index,
        typ: 0,
        account_id,
        symbol_hash: "0xa".to_string(),
        fee_asset_hash: "0xb".to_string(),
        trade_qty: BigDecimal::from(1u32),
        notional: BigDecimal::from(1u32),
        executed_price: BigDecimal::from(1u32),
        fee: BigDecimal::from(0u32),
        sum_unitary_fundings: BigDecimal::from(0u32),
        trade_id: BigDecimal::from(1u32),
        match_id: BigDecimal::from(1u32),
        timestamp: BigDecimal::from_i64(ts_ms).unwrap_or_default(),
        side: true,
        block_time,
        broker_hash: Some(broker_hash),
        transaction_id: None,
        margin_mode: None,
        iso_margin_asset_hash: None,
        margin_from_cross: None,
        address: Some(address),
    }
}

/// Determine partition and block_time from row_index (deterministic).
/// Partitions 0..=10 map to before_y2024 through y2026q01; first 2M rows in partition 3 fall on 2024-07-22.
#[cfg(test)]
fn mock_block_time_for_row_index(row_index: u64) -> (NaiveDateTime, usize) {
    const ROWS_PER_PARTITION_BASE: u64 = 10_200_000;
    const EXTRA_20240722: u64 = 2_000_000;
    const PARTITION_3_EXTRA_START: u64 = 30_600_000;
    const PARTITION_3_EXTRA_END: u64 = 32_600_000;

    let partition_bounds: [(NaiveDateTime, NaiveDateTime); 11] = [
        (
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-04-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            NaiveDateTime::parse_from_str("2024-04-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-07-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            NaiveDateTime::parse_from_str("2024-07-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-10-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            NaiveDateTime::parse_from_str("2024-10-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            NaiveDateTime::parse_from_str("2025-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2025-04-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            NaiveDateTime::parse_from_str("2025-04-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2025-07-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            NaiveDateTime::parse_from_str("2025-07-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2025-10-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            NaiveDateTime::parse_from_str("2025-10-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2026-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            NaiveDateTime::parse_from_str("2026-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2026-04-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        (
            NaiveDateTime::parse_from_str("2026-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2026-04-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
    ];

    const TOTAL_MOCK_ROWS: u64 = 114_200_000;
    let cumulative: [u64; 12] = [
        0,
        ROWS_PER_PARTITION_BASE,
        ROWS_PER_PARTITION_BASE * 2,
        ROWS_PER_PARTITION_BASE * 3,
        ROWS_PER_PARTITION_BASE * 3 + EXTRA_20240722,
        ROWS_PER_PARTITION_BASE * 4 + EXTRA_20240722,
        ROWS_PER_PARTITION_BASE * 5 + EXTRA_20240722,
        ROWS_PER_PARTITION_BASE * 6 + EXTRA_20240722,
        ROWS_PER_PARTITION_BASE * 7 + EXTRA_20240722,
        ROWS_PER_PARTITION_BASE * 8 + EXTRA_20240722,
        ROWS_PER_PARTITION_BASE * 9 + EXTRA_20240722,
        TOTAL_MOCK_ROWS,
    ];

    let partition_index = cumulative
        .windows(2)
        .position(|w| row_index >= w[0] && row_index < w[1])
        .unwrap_or(10);

    let (start, end) = partition_bounds[partition_index];
    let partition_start_row = cumulative[partition_index];
    let partition_end_row = cumulative[partition_index + 1];
    let partition_len = partition_end_row - partition_start_row;

    let block_time = if partition_index == 3
        && row_index >= PARTITION_3_EXTRA_START
        && row_index < PARTITION_3_EXTRA_END
    {
        let day_start =
            NaiveDateTime::parse_from_str("2024-07-22 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let day_end =
            NaiveDateTime::parse_from_str("2024-07-22 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
        let offset = row_index - PARTITION_3_EXTRA_START;
        let total_secs = (day_end - day_start).num_seconds().max(1) as u64;
        let secs = (offset * total_secs / EXTRA_20240722) as i64;
        day_start + chrono::Duration::seconds(secs)
    } else if partition_index == 3 && row_index >= PARTITION_3_EXTRA_END {
        let offset_in_partition = row_index - PARTITION_3_EXTRA_END;
        let sub_len = partition_end_row - PARTITION_3_EXTRA_END;
        let range_secs = (end - start).num_seconds().max(1);
        let secs = (offset_in_partition as i64 * range_secs) / sub_len as i64;
        start + chrono::Duration::seconds(secs)
    } else {
        let offset_in_partition = row_index - partition_start_row;
        let range_secs = (end - start).num_seconds().max(1);
        let secs = (offset_in_partition as i64 * range_secs) / partition_len as i64;
        start + chrono::Duration::seconds(secs)
    };

    (block_time, partition_index)
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use crate::init::init_log;

    use super::*;

    /// Insert ~112M deterministic mock rows into partitioned_executed_trades.
    /// Partitions executed_trades_before_y2024 through executed_trades_y2026q01 each have >10M rows;
    /// 2M extra on 2024-07-22. No inserts into y2026q02 or y2026q03.
    #[ignore]
    #[test]
    fn test_insert_mock_partitioned_executed_trades_100m() {
        dotenv::dotenv().ok();
        init_log();

        let brokers: Vec<String> = (0..10).map(|i| format!("broker_{}", i)).collect();
        let broker_refs: Vec<&str> = brokers.iter().map(|s| s.as_str()).collect();

        let addresses: Vec<String> = (0..20).map(|i| format!("addr_{}", i)).collect();
        let address_refs: Vec<&str> = addresses.iter().map(|s| s.as_str()).collect();

        let account_ids: Vec<String> = (0..100).map(|i| format!("account_{}", i)).collect();

        const TOTAL_ROWS: u64 = 114_200_000;
        const BATCH_SIZE: usize = 1_000;

        let system = actix::System::new();
        system.block_on(async move {
            let mut inserted = 0u64;
            for start in (0..TOTAL_ROWS).step_by(BATCH_SIZE) {
                let end = (start + BATCH_SIZE as u64).min(TOTAL_ROWS);
                let mut batch = Vec::with_capacity(BATCH_SIZE);
                for row_index in start..end {
                    batch.push(mock_trade_for_index(
                        row_index,
                        &broker_refs,
                        &address_refs,
                        &account_ids,
                    ));
                }
                let n = create_partitioned_executed_trades(batch)
                    .await
                    .expect("insert batch");
                inserted += n as u64;
                if inserted % 1_000_000 == 0 || start + BATCH_SIZE as u64 >= TOTAL_ROWS {
                    tracing::info!("mock insert progress: {} / {}", inserted, TOTAL_ROWS);
                }
            }
            tracing::info!("mock insert done: total {}", inserted);
        });
    }

    #[ignore]
    #[test]
    fn test_insert_partitioning_trades() {
        dotenv::dotenv().ok();
        init_log();
        let system = actix::System::new();
        system.block_on(async move {
            let data_time = chrono::Utc::now();
            for (year, q) in [
                (2023, 1),
                (2024, 1),
                (2024, 2),
                (2024, 3),
                (2024, 4),
                (2025, 1),
                (2025, 2),
            ] {
                let data_time = data_time.with_year(year).unwrap();
                let data_time = data_time
                    .with_month(q * 3)
                    .unwrap()
                    .with_hour(10)
                    .unwrap()
                    .with_minute(30)
                    .unwrap()
                    .with_second(36)
                    .unwrap();
                let timestamp = data_time.timestamp();
                for blocknum in 0..1_000 {
                    let inst = Instant::now();
                    let mut trades = vec![];
                    for i in 0..100 {
                        trades.push(DbPartitionedExecutedTrades {
                            block_number: blocknum,
                            transaction_index: 1,
                            log_index: i,
                            typ: 0,
                            account_id: "0x100000".to_string() + &i.to_string(),
                            symbol_hash: "0xaaaaaaaaaaa".to_string(),
                            fee_asset_hash: "0xbbbbbbbb".to_string(),
                            trade_qty: 100000.into(),
                            notional: 1000.into(),
                            executed_price: 1900000.into(),
                            fee: 1900.into(),
                            sum_unitary_fundings: 2000.into(),
                            trade_id: 1.into(),
                            match_id: 1000.into(),
                            timestamp: timestamp.into(),
                            side: true,
                            block_time: NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(),
                            broker_hash: None,
                            transaction_id: None,
                            margin_mode: None,
                            iso_margin_asset_hash: None,
                            margin_from_cross: None,
                            address: None,
                        });
                    }
                    create_partitioned_executed_trades(trades).await.unwrap();

                    let elapsed_ms = inst.elapsed().as_millis();
                    tracing::info!(
                        "elapsed_ms: {} for year: {}, q: {}, blocknum: {}",
                        elapsed_ms,
                        year,
                        q,
                        blocknum
                    );
                }
            }
        });
    }
}
