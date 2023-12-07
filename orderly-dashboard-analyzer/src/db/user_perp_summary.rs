use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable};
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = "user_perp_summary")]
pub struct UserPerpSummary<'a> {
    id: i64,
    account_id: &'a String,
    symbol: &'a String,
    holding: &'a BigDecimal,
    opening_cost: &'a BigDecimal,

    total_trading_fee: &'a BigDecimal,
    total_trading_volume: &'a BigDecimal,
    total_trading_count: i64,

    total_realized_pnl: &'a BigDecimal,
    total_liquidation_amount: &'a BigDecimal,
    total_liquidation_count: i64,

    pulled_block_height: i64,
    pulled_block_timestamp: i64,
    created_timestamp: i64,
    updated_timestamp: i64,
}