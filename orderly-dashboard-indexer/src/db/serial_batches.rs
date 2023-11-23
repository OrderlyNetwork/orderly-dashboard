use crate::db::POOL;
use crate::schema::serial_batches;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};

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
}
//     block_number      bigint                not null,
//     transaction_index integer               not null,
//     log_index         integer               not null,
//     transaction_id    text                  not null,
//     block_time        numeric               not null,
//     batch_id          bigint                not null,
//     event_type        smallint              not null,

pub(crate) async fn create_serial_batchess(batches: Vec<DbSerialBatches>) -> Result<usize> {
    use crate::schema::serial_batches::dsl::*;

    let num_rows = diesel::insert_into(serial_batches)
        .values(batches)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}
