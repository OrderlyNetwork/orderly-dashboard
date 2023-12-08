use bigdecimal::BigDecimal;

use diesel::prelude::*;

use crate::schema::hourly_user_perp;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "hourly_user_perp"]
pub struct HourlyUserPerp {
    symbol: String,
    block_hour: i64,

    trading_fee: BigDecimal,
    trading_volume: BigDecimal,
    trading_count: i64,

    realized_pnl: BigDecimal,
    un_realized_pnl: BigDecimal,
    latest_sum_unitary_funding: BigDecimal,

    liquidation_amount: BigDecimal,
    liquidation_count: i64,

    pulled_block_height: i64,
    pulled_block_time: i64,
}