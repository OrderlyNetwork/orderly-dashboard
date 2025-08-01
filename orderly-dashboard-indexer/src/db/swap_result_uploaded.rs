use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::swap_result_uploaded;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable};
use diesel_async::RunQueryDsl;

#[derive(Insertable, Queryable, Debug)]
#[diesel(table_name = swap_result_uploaded)]
pub struct DbSwapResultUploadedEvent {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    pub account_id: String,
    pub buy_token_hash: String,
    pub sell_token_hash: String,
    pub buy_quantity: BigDecimal,
    pub sell_quantity: BigDecimal,
    pub chain_id: BigDecimal,
    pub swap_status: i16,
}

impl DbSwapResultUploadedEvent {
    pub fn new(
        block_number: i64,
        transaction_index: i32,
        log_index: i32,
        transaction_id: String,
        block_time: BigDecimal,
        account_id: String,
        buy_token_hash: String,
        sell_token_hash: String,
        buy_quantity: BigDecimal,
        sell_quantity: BigDecimal,
        chain_id: BigDecimal,
        swap_status: i16,
    ) -> DbSwapResultUploadedEvent {
        DbSwapResultUploadedEvent {
            block_number,
            transaction_index,
            log_index,
            transaction_id,
            block_time,
            account_id,
            buy_token_hash,
            sell_token_hash,
            buy_quantity,
            sell_quantity,
            chain_id,
            swap_status,
        }
    }
}

pub async fn create_swap_result_uploaded_events(
    swap_result_uploadedes: Vec<DbSwapResultUploadedEvent>,
) -> Result<usize> {
    use crate::schema::swap_result_uploaded::dsl::*;
    if swap_result_uploadedes.is_empty() {
        return Ok(0);
    }
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let num_rows = diesel::insert_into(swap_result_uploaded)
        .values(swap_result_uploadedes)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}
