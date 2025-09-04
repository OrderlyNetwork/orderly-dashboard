use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::balance_transfer;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable};
use diesel_async::RunQueryDsl;

/*
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        transaction_id -> Text,
        block_time -> Numeric,
        from_account_id -> Text,
        to_account_id -> Text,
        amount -> Numeric,
        token_hash -> Text,
        is_from_account_id -> Bool,
        transfer_type -> Int2,
        transfer_id -> Numeric,
*/
#[derive(Insertable, Queryable, Debug)]
#[diesel(table_name = balance_transfer)]
pub struct DbBalanceTransferEvent {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    pub from_account_id: String,
    pub to_account_id: String,
    pub amount: BigDecimal,
    pub token_hash: String,
    pub is_from_account_id: bool,
    pub transfer_type: i16,
    pub transfer_id: BigDecimal,
}

pub async fn create_balance_transfer_events(
    balance_transfers: Vec<DbBalanceTransferEvent>,
) -> Result<usize> {
    use crate::schema::balance_transfer::dsl::*;
    if balance_transfers.is_empty() {
        return Ok(0);
    }
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let num_rows = diesel::insert_into(balance_transfer)
        .values(balance_transfers)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}
