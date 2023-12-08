
pub struct BlockSummary {
    id: i64,

    pulled_block_height: i64,
    pulled_block_timestamp: i64,
    pulled_block_hash: String,

    pulled_event_id: i64,
    pulled_spot_trade_id: i64,
    pulled_perp_trade_id: i64,

    created_time: i64,
    updated_time: i64,
}