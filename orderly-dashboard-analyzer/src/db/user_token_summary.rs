use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable};
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = "user_token_summary")]
pub struct UserTokenSummary<'a> {
    id: i64,
    account_id: &'a String,
    token: &'a String,
    chain_id: &'a String,
    token_address: &'a String,
    balance:&'a BigDecimal,

    total_withdraw_amount:&'a BigDecimal,
    total_deposit_amount:&'a BigDecimal,
    total_withdraw_count: i64,
    total_deposit_count: i64,

    pulled_block_height: i64,
    pulled_block_timestamp: i64,
    created_time: i64,
    updated_time: i64,
}