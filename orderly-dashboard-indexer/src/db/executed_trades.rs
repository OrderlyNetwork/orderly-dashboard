use crate::db::POOL;
use crate::schema::executed_trades;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};

#[derive(Insertable, Queryable, Debug)]
#[table_name = "executed_trades"]
pub struct DbexecutedTrades {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub batch_id: BigDecimal,
    pub typ: i16,
    pub account_id: String,
    pub symbol_hash: String,
    pub fee_asset_hash: String,
    pub trade_qty: BigDecimal,
    pub notional: BigDecimal,
    pub executed_price: BigDecimal,
    pub fee: BigDecimal,
    pub sum_unitary_fundings: BigDecimal,
    pub trade_id: BigDecimal,
    pub match_id: BigDecimal,
    pub timestamp: BigDecimal,
}

pub(crate) async fn create_executed_trades(trades: Vec<DbexecutedTrades>) -> Result<usize> {
    use crate::schema::executed_trades::dsl::*;

    let num_rows = diesel::insert_into(executed_trades)
        .values(trades)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}
