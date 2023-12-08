use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable};
use diesel::prelude::*;

use crate::schema::hourly_orderly_token;

#[derive(Insertable, Queryable, Debug)]
#[table_name = "hourly_orderly_token"]
pub struct HourlyOrderlyToken {
    token: String,
    block_hour: i64,

    chain_id: String,

    withdraw_amount: BigDecimal,
    withdraw_count: i64,
    deposit_amount: BigDecimal,
    deposit_count: i64,

    pulled_block_height: i64,
    pulled_block_time: i64,
}