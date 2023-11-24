use crate::db::POOL;
use crate::schema::liquidation_result;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};

#[derive(Insertable, Queryable, Debug)]
#[table_name = "liquidation_result"]
pub struct DbLiquidationResult {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    pub liquidated_account_id: String,
    pub insurance_account_id: String,
    pub liquidated_asset_hash: String,
    pub insurance_transfer_amount: BigDecimal,
}

pub(crate) async fn create_liquidation_results(
    liquidations: Vec<DbLiquidationResult>,
) -> Result<usize> {
    use crate::schema::liquidation_result::dsl::*;

    let num_rows = diesel::insert_into(liquidation_result)
        .values(liquidations)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}
