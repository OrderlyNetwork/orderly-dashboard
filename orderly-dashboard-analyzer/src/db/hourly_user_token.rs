use bigdecimal::BigDecimal;

pub struct HourlyUserPerp {
    id: i64,
    account_id:  String,
    token:  String,
    chain_hour: i64,
    chain_id:  String,
    token_address: String,
    withdraw_amount: BigDecimal,
    withdraw_count: i64,
    deposit_amount: BigDecimal,
    deposit_count: i64,
    pulled_block_height: i64,
    pulled_block_timestamp: i64,
    created_time: i64,
    updated_time: i64,
}