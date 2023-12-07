use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable};
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = "hourly_order_perp")]
pub struct HourlyOrderlyPerp<'a> {
    id: i64,
    symbol: &'a String,
    chain_hour: i64,

    pulled_block_height: i64,
    pulled_block_timestamp: i64,

    trading_fee:&'a BigDecimal,
    trading_volume:&'a BigDecimal,
    trading_count: i64,
    trading_user_count: i64,
    opening_count: i64,

    liquidation_amount:&'a BigDecimal,
    liquidation_count: i64,

    created_time: i64,
    updated_time: i64,
}