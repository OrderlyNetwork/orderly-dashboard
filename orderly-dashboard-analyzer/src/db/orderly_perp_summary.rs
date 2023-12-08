use bigdecimal::BigDecimal;
pub struct HourlyOrderlyPerp {
    id: i64,

    pulled_block_height: i64,
    pulled_block_timestamp: i64,

    symbol: String,
    open_interest: BigDecimal,

    total_trading_fee: BigDecimal,
    total_trading_volume:  BigDecimal,

    total_trading_count: i64,
    total_trading_user_count: i64,

    total_liquidation_amount: BigDecimal,
    total_liquidation_count:  BigDecimal,

    created_time: i64,
    updated_time: i64,
}