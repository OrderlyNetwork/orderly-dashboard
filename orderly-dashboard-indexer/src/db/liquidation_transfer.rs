use crate::db::POOL;
use crate::schema::liquidation_transfer;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};

#[derive(Insertable, Queryable, Debug)]
#[table_name = "liquidation_transfer"]
pub struct DbLiquidationTransfer {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub liquidation_transfer_id: BigDecimal,
    pub liquidator_account_id: String,
    pub symbol_hash: String,
    pub position_qty_transfer: BigDecimal,
    pub cost_position_transfer: BigDecimal,
    pub liquidator_fee: BigDecimal,
    pub insurance_fee: BigDecimal,
    pub mark_price: BigDecimal,
    pub sum_unitary_fundings: BigDecimal,
    pub liquidation_fee: BigDecimal,
}

pub(crate) async fn create_liquidation_trasfers(adls: Vec<DbLiquidationTransfer>) -> Result<usize> {
    use crate::schema::liquidation_transfer::dsl::*;

    let num_rows = diesel::insert_into(liquidation_transfer)
        .values(adls)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}
