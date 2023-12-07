use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable};
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = "hourly_user_perp")]
pub struct HourlyUserPerp<'a> {
    id: i64,
    pulled_block_height: i64,
    pulled_block_timestamp: i64,

    symbol: &'a String,
    chain_hour: i64,
    trading_fee:&'a BigDecimal,
    trading_volume:&'a BigDecimal,
    trading_count: i64,

    liquidation_amount:&'a BigDecimal,
    liquidation_count:&'a BigDecimal,
    created_time: i64,
    updated_time: i64,
}