use crate::db::{DB_CONTEXT, POOL};
use crate::schema::serial_batches;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
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
#[table_name = "serial_batches"]
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
    pub l1_fee_scalar: Option<BigDecimal>,
    pub l1_gas_price: Option<BigDecimal>,
    pub l1_gas_used: Option<BigDecimal>,
}

impl DbSerialBatches {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.transaction_index)
    }
}

pub async fn create_serial_batches(batches: Vec<DbSerialBatches>) -> Result<usize> {
    use crate::schema::serial_batches::dsl::*;

    let num_rows = diesel::insert_into(serial_batches)
        .values(batches)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
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

    let result = serial_batches
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .filter(event_type.eq(serial_typ.value()))
        .load_async::<DbSerialBatches>(&POOL)
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
            AsyncError::Execute(Error::NotFound) => {
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

pub async fn get_serial_batches(from_block: i64, to_block: i64) -> Result<Vec<DbSerialBatches>> {
    use crate::schema::serial_batches::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_serial_batches_with_type start",
    );
    let start_time = Instant::now();

    let result = serial_batches
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load_async::<DbSerialBatches>(&POOL)
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
            AsyncError::Execute(Error::NotFound) => {
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
