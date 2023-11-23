use crate::db::POOL;
use crate::schema::adl_result;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};

#[derive(Insertable, Queryable, Debug)]
#[table_name = "adl_result"]
pub struct DbAdlResult {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    pub account_id: String,
    pub insurance_account_id: String,
    pub symbol_hash: String,
    pub position_qty_transfer: BigDecimal,
    pub cost_position_transfer: BigDecimal,
    pub adl_price: BigDecimal,
    pub sum_unitary_fundings: BigDecimal,
}

pub(crate) async fn create_adl_results(adls: Vec<DbAdlResult>) -> Result<usize> {
    use crate::schema::adl_result::dsl::*;

    let num_rows = diesel::insert_into(adl_result)
        .values(adls)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}
