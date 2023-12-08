use bigdecimal::BigDecimal;
use diesel::prelude::*;

use crate::schema::orderly_token_summary;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "orderly_token_summary"]

pub struct OrderlyTokenSummary {
    token: String,
    chain_id: String,
    balance: BigDecimal,

    total_withdraw_amount: BigDecimal,
    total_withdraw_count: i64,

    total_deposit_amount: BigDecimal,
    total_deposit_count: i64,

    pulled_block_height: i64,
    pulled_block_time: i64,
}