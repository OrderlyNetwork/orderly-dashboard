use crate::db::POOL;
use crate::schema::settlement_result;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};

#[derive(Insertable, Queryable, Debug)]
#[table_name = "settlement_result"]
pub struct DbSettlementResult {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    pub account_id: String,
    pub settled_amount: BigDecimal,
    pub settled_asset_hash: String,
    pub insurance_account_id: String,
    pub insurance_transfer_amount: BigDecimal,
    pub settlement_executions_count: BigDecimal,
}

pub(crate) async fn create_settlement_results(adls: Vec<DbSettlementResult>) -> Result<usize> {
    use crate::schema::settlement_result::dsl::*;

    let num_rows = diesel::insert_into(settlement_result)
        .values(adls)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}
