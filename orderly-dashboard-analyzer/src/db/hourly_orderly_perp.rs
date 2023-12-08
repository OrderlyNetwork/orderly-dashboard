use bigdecimal::BigDecimal;

pub struct HourlyOrderlyPerp {
    id: i64,
    symbol: String,
    chain_hour: i64,

    pulled_block_height: i64,
    pulled_block_timestamp: i64,

    trading_fee: BigDecimal,
    trading_volume:  BigDecimal,
    trading_count: i64,
    trading_user_count: i64,
    opening_count: i64,

    liquidation_amount:  BigDecimal,
    liquidation_count: i64,

    created_time: i64,
    updated_time: i64,
}