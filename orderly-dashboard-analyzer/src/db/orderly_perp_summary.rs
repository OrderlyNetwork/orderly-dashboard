use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable};
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = "orderly_perp_summary")]
pub struct HourlyOrderlyPerp<'a> {
    id: i64,

    pulled_block_height: i64,
    pulled_block_timestamp: i64,

    symbol: &'a String,
    open_interest: &'a BigDecimal,

    total_trading_fee:&'a BigDecimal,
    total_trading_volume:&'a BigDecimal,

    total_trading_count: i64,
    total_trading_user_count: i64,

    total_liquidation_amount:&'a BigDecimal,
    total_liquidation_count:&'a BigDecimal,

    created_time: i64,
    updated_time: i64,
}