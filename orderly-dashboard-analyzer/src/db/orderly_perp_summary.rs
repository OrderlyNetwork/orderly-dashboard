use bigdecimal::BigDecimal;
use diesel::prelude::*;

use crate::schema::orderly_perp_summary;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "orderly_perp_summary"]
pub struct OrderlyPerpSummary {
    symbol: String,

    open_interest: BigDecimal,
    total_trading_volume: BigDecimal,
    total_trading_fee: BigDecimal,

    total_trading_count: i64,
    total_trading_user_count: i64,
    total_liquidation_amount: BigDecimal,
    total_liquidation_count: i64,

    pulled_block_height: i64,
    pulled_block_time: i64,
}