use bigdecimal::BigDecimal;

use diesel::prelude::*;

use crate::schema::hourly_user_token;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "hourly_user_token"]
pub struct HourlyUserToken {
    account_id: String,
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