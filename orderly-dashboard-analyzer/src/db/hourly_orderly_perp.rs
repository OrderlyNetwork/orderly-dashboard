use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable};
use diesel::prelude::*;

use crate::schema::hourly_orderly_perp;

#[derive(Insertable, Queryable, Debug)]
#[table_name = "hourly_orderly_perp"]
pub struct HourlyOrderlyPerp {
    symbol: String,
    block_hour: i64,

    trading_fee: BigDecimal,
    trading_volume: BigDecimal,

    trading_count: i64,
    trading_user_count: i64,
    opening_count: i64,

    liquidation_amount: BigDecimal,
    liquidation_count: i64,

    pulled_block_height: i64,
    pulled_block_time: i64,
}