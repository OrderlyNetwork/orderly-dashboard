use orderly_dashboard_indexer::formats_external::{SuccessResponse, trading_events::*};

pub trait IndexerClient {
    fn pull_block(&self, start_block: i64, end_block: i64) -> Result<SuccessResponse<TradingEventsResponse>, Err()>;
}

pub struct RestIndexerClient {}

impl IndexerClient for RestIndexerClient {
    fn pull_block(&self, start_block: i64, end_block: i64) -> Result<SuccessResponse<TradingEventsResponse>, Err()> {}
}