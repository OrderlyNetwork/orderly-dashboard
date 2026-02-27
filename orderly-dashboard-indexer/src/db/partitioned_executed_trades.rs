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
pub async fn query_trades_with_empty_broker_hash_or_txid(
) -> Result<Vec<DbPartitionedExecutedTrades>> {
    use crate::schema::partitioned_executed_trades::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let rows = partitioned_executed_trades
        .filter(broker_hash.is_null())
        .order_by((block_number, transaction_index, log_index))
        .limit(500)
        .load::<DbPartitionedExecutedTrades>(&mut conn)
        .await?;
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
        "query_partitioned_executed_trades start",
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

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use crate::init::init_log;

    use super::*;
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
                (2025, 3),
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
                for blocknum in 0..2_000 {
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
