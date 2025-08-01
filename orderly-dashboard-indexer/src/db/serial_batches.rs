use crate::config::get_common_cfg;
use crate::db::{DB_CONN_ERR_MSG, DB_CONTEXT, POOL};
use crate::schema::serial_batches;
use anyhow::Result;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use diesel_async::RunQueryDsl;
use std::cmp::min;
use std::collections::BTreeSet;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub enum SerialBatchType {
    EventUpload = 1,
    PerpTrade = 2,
}

impl SerialBatchType {
    pub fn value(self) -> i16 {
        self as i16
    }
}

#[derive(Insertable, Queryable, Debug)]
#[diesel(table_name = serial_batches)]
pub struct DbSerialBatches {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    pub batch_id: i64,
    pub event_type: i16,
    pub effective_gas_price: Option<BigDecimal>,
    pub gas_used: Option<BigDecimal>,
    pub l1_fee: Option<BigDecimal>,
    pub l1_fee_scalar: Option<String>,
    pub l1_gas_price: Option<BigDecimal>,
    pub l1_gas_used: Option<BigDecimal>,
}

impl DbSerialBatches {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.transaction_index)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, QueryableByName)]
pub struct DbSerialBatchesView {
    #[diesel(sql_type = BigInt)]
    pub block_number: i64,
    #[diesel(sql_type = Integer)]
    pub transaction_index: i32,
    #[diesel(sql_type = Integer)]
    pub log_index: i32,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Numeric)]
    pub block_time: BigDecimal,
    #[diesel(sql_type = BigInt)]
    pub batch_id: i64,
    #[diesel(sql_type = SmallInt)]
    pub event_type: i16,
}

impl DbSerialBatchesView {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.transaction_index)
    }
}

pub async fn create_serial_batches(batches: Vec<DbSerialBatches>) -> Result<usize> {
    use crate::schema::serial_batches::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let num_rows = diesel::insert_into(serial_batches)
        .values(batches)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}

pub async fn query_serial_batches_with_type(
    from_block: i64,
    to_block: i64,
    serial_typ: SerialBatchType,
) -> Result<Vec<DbSerialBatches>> {
    use crate::schema::serial_batches::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_serial_batches_with_type start",
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = serial_batches
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .filter(event_type.eq(serial_typ.value()))
        .load::<DbSerialBatches>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_serial_batches_with_type success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_serial_batches_with_type success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_serial_batches_with_type fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

#[allow(dead_code)]
pub async fn query_serial_batches_with_type_and_time(
    from_block: i64,
    to_block: i64,
    from_time: i64,
    to_time: i64,
    serial_typ: SerialBatchType,
) -> Result<Vec<DbSerialBatches>> {
    use crate::schema::serial_batches::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_serial_batches_with_type start",
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = serial_batches
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .filter(block_time.ge(BigDecimal::from(from_time)))
        .filter(block_time.le(BigDecimal::from(to_time)))
        .filter(event_type.eq(serial_typ.value()))
        .load::<DbSerialBatches>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_serial_batches_with_type success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_serial_batches_with_type success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_serial_batches_with_type fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

#[allow(dead_code)]
pub async fn query_serial_batches_with_type_by_time(
    from_time: i64,
    to_time: i64,
    account_id_: String,
    serial_typ: SerialBatchType,
) -> Result<Vec<DbSerialBatchesView>> {
    use diesel::sql_query;
    tracing::info!(
        target: DB_CONTEXT,
        "query_serial_batches_with_type_by_time start",
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result = sql_query(
        "select
                serial_t.block_number as block_number,
                serial_t.transaction_index as transaction_index,
                serial_t.log_index as log_index,
                serial_t.transaction_id as transaction_id,
                serial_t.block_time as block_time,
                serial_t.batch_id as batch_id,
                serial_t.event_type as event_type
              from
                serial_batches serial_t
                left join executed_trades exec_t on serial_t.block_number = exec_t.block_number
                and serial_t.transaction_index = exec_t.transaction_index
              where
                serial_t.block_time >= $1
                and serial_t.block_time <= $2
                and serial_t.event_type = $3
                and exec_t.account_id = $4
                and exec_t.block_time >= $5
                and exec_t.block_time <= $6;",
    )
    .bind::<diesel::sql_types::Numeric, _>(BigDecimal::from(from_time))
    .bind::<diesel::sql_types::Numeric, _>(BigDecimal::from(to_time))
    .bind::<diesel::sql_types::SmallInt, _>(serial_typ.value())
    .bind::<diesel::sql_types::Text, _>(account_id_)
    .bind::<diesel::sql_types::BigInt, _>(from_time)
    .bind::<diesel::sql_types::BigInt, _>(to_time)
    .get_results::<DbSerialBatchesView>(&mut conn)
    .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_serial_batches_with_type_by_time success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_serial_batches_with_type_by_time success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_serial_batches_with_type_by_time fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_serial_batches_by_time_and_key(
    from_time: i64,
    to_time: i64,
    blcoknum_tx_idx_vec: BTreeSet<(i64, i32)>,
) -> Result<Vec<DbSerialBatchesView>> {
    tracing::info!(
        target: DB_CONTEXT,
        "query_serial_batches_by_time_and_key start with length:{}",
        blcoknum_tx_idx_vec.len(),
    );
    if blcoknum_tx_idx_vec.is_empty() {
        return Ok(vec![]);
    }
    let start_time = Instant::now();

    let page_size = get_common_cfg().db_query_limit;
    if blcoknum_tx_idx_vec.len() > page_size {
        // page query
        let mut page_idx = 0;
        let mut events: Vec<DbSerialBatchesView> = vec![];
        while page_idx < blcoknum_tx_idx_vec.len() {
            let page_size = min(page_size, blcoknum_tx_idx_vec.len() - page_idx);
            let page_vec: Vec<(i64, i32)> = blcoknum_tx_idx_vec
                .iter()
                .skip(page_idx)
                .take(page_size)
                .cloned()
                .collect();
            let page_events = query_serial_batches_by_time_and_key_page(
                from_time,
                to_time,
                page_vec.into_iter().collect(),
            )
            .await?;
            events.extend(page_events);
            page_idx += page_size;
        }
        tracing::info!(
            target: DB_CONTEXT,
            "query_serial_batches_by_time_and_key success. length:{}, used time:{} ms",
            events.len(),
            (Instant::now() - start_time).as_millis()
        );
        return Ok(events);
    }

    let events =
        query_serial_batches_by_time_and_key_page(from_time, to_time, blcoknum_tx_idx_vec).await?;
    tracing::info!(
        target: DB_CONTEXT,
        "query_serial_batches_by_time_and_key success. length:{}, used time:{} ms",
        events.len(),
        (Instant::now() - start_time).as_millis()
    );
    Ok(events)
}

async fn query_serial_batches_by_time_and_key_page(
    from_time: i64,
    to_time: i64,
    blcoknum_tx_idx_vec: BTreeSet<(i64, i32)>,
) -> Result<Vec<DbSerialBatchesView>> {
    use diesel::sql_query;
    tracing::info!(
        target: DB_CONTEXT,
        "query_serial_batches_by_time_and_key_page start",
    );
    if blcoknum_tx_idx_vec.is_empty() {
        return Ok(vec![]);
    }
    let start_time = Instant::now();

    let conditions = blcoknum_tx_idx_vec
        .iter()
        .map(|(block, tx)| format!("({}, {})", block, tx))
        .collect::<Vec<_>>()
        .join(",");

    let query = format!(
        "select block_number, transaction_index, log_index, transaction_id, block_time, batch_id, event_type 
         from serial_batches 
         where block_time >= $1 and block_time <= $2 
         and (block_number, transaction_index) in ({})", 
        conditions
    );
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = sql_query(&query)
        .bind::<Numeric, _>(BigDecimal::from(from_time))
        .bind::<Numeric, _>(BigDecimal::from(to_time))
        .get_results::<DbSerialBatchesView>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_serial_batches_by_time_and_key_page success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_serial_batches_by_time_and_key_page success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_serial_batches_by_time_and_key_page fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_serial_batches_joined_partitioned_trades_with_type_by_time(
    from_time: i64,
    to_time: i64,
    account_id_: String,
    serial_typ: SerialBatchType,
) -> Result<Vec<DbSerialBatchesView>> {
    use diesel::sql_query;
    tracing::info!(
        target: DB_CONTEXT,
        "query_serial_batches_with_type_by_time start",
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result = sql_query(
        "select
                serial_t.block_number as block_number,
                serial_t.transaction_index as transaction_index,
                serial_t.log_index as log_index,
                serial_t.transaction_id as transaction_id,
                serial_t.block_time as block_time,
                serial_t.batch_id as batch_id,
                serial_t.event_type as event_type
              from
                serial_batches serial_t
                left join partitioned_executed_trades exec_t on serial_t.block_number = exec_t.block_number
                and serial_t.transaction_index = exec_t.transaction_index
              where
                serial_t.block_time >= $1
                and serial_t.block_time <= $2
                and serial_t.event_type = $3
                and exec_t.block_time >= $4
                and exec_t.block_time <= $5
                and exec_t.account_id = $6;",
    )
        .bind::<diesel::sql_types::Numeric, _>(BigDecimal::from(from_time))
        .bind::<diesel::sql_types::Numeric, _>(BigDecimal::from(to_time))
        .bind::<diesel::sql_types::SmallInt, _>(serial_typ.value())
        .bind::<diesel::sql_types::Timestamp, _>(
            NaiveDateTime::from_timestamp_opt(from_time, 0)
            .ok_or_else(|| anyhow::anyhow!("timestamp of from_time should be valid"))?
        )
        .bind::<diesel::sql_types::Timestamp, _>(NaiveDateTime::from_timestamp_opt(to_time, 0)
        .ok_or_else(|| anyhow::anyhow!("timestamp of from_time should be valid"))?)
        .bind::<diesel::sql_types::Text, _>(account_id_)
        .get_results::<DbSerialBatchesView>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_serial_batches_with_type_by_time success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_serial_batches_with_type_by_time success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_serial_batches_with_type_by_time fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn get_serial_batches(from_block: i64, to_block: i64) -> Result<Vec<DbSerialBatches>> {
    use crate::schema::serial_batches::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_serial_batches_with_type start",
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = serial_batches
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load::<DbSerialBatches>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "get_serial_batches success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "get_serial_batches success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "get_serial_batches fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}
