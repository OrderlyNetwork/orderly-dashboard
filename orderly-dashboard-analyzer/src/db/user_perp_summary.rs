use bigdecimal::BigDecimal;

pub struct UserPerpSummary {
    id: i64,
    account_id: String,
    symbol: String,
    holding: BigDecimal,
    opening_cost: BigDecimal,
    cost_position: BigDecimal,

    total_trading_fee: BigDecimal,
    total_trading_volume: BigDecimal,
    total_trading_count: i64,

    total_realized_pnl: BigDecimal,
    total_liquidation_amount: BigDecimal,
    total_liquidation_count: i64,

    pulled_block_height: i64,
    pulled_block_timestamp: i64,
    created_timestamp: i64,
    updated_timestamp: i64,
}