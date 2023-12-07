use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable};
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = "hourly_user_token")]
pub struct HourlyUserPerp<'a> {
    id: i64,
    account_id: &'a String,
    token: &'a String,
    chain_hour: i64,
    chain_id: &'a String,
    token_address: &'a String,
    withdraw_amount: &'a BigDecimal,
    withdraw_count: i64,
    deposit_amount:&'a BigDecimal,
    deposit_count: i64,
    pulled_block_height: i64,
    pulled_block_timestamp: i64,
    created_time: i64,
    updated_time: i64,
}