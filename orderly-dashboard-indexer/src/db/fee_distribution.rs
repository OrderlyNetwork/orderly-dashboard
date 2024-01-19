use crate::db::POOL;
use crate::schema::fee_distribution;
use actix_diesel::dsl::AsyncRunQueryDsl;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable};

#[derive(Insertable, Queryable, Debug)]
#[table_name = "fee_distribution"]
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

    let num_rows = diesel::insert_into(fee_distribution)
        .values(districutions)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}
