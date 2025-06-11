use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::fee_distribution;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Insertable, Queryable, Debug)]
#[diesel(table_name = fee_distribution)]
pub struct DbFeeDistribution {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    pub event_id: BigDecimal,
    pub from_account_id: String,
    pub to_account_id: String,
    pub amount: BigDecimal,
    pub token_hash: String,
}

pub async fn create_fee_distributions(districutions: Vec<DbFeeDistribution>) -> Result<usize> {
    use crate::schema::fee_distribution::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let num_rows = diesel::insert_into(fee_distribution)
        .values(districutions)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}
