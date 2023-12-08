use bigdecimal::BigDecimal;
use diesel::prelude::*;

use crate::schema::user_perp_summary;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "user_perp_summary"]
pub struct UserPerpSummary {
    account_id: String,
    symbol: String,

    holding: BigDecimal,
    opening_cost: BigDecimal,
    cost_position: BigDecimal,

    total_trading_volume: BigDecimal,
    total_trading_count: i64,
    total_trading_fee: BigDecimal,

    total_realized_pnl: BigDecimal,
    total_un_realized_pnl: BigDecimal,

    total_liquidation_amount: BigDecimal,
    total_liquidation_count: i64,

    pulled_block_height: i64,
    pulled_block_time: i64,
}