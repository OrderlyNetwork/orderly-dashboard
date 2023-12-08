use bigdecimal::BigDecimal;

pub struct UserTokenSummary {
    id: i64,
    account_id: String,
    token: String,
    chain_id: String,
    token_address: String,
    balance: BigDecimal,

    total_withdraw_amount: BigDecimal,
    total_deposit_amount: BigDecimal,
    total_withdraw_count: i64,
    total_deposit_count: i64,

    pulled_block_height: i64,
    pulled_block_timestamp: i64,
    created_time: i64,
    updated_time: i64,
}