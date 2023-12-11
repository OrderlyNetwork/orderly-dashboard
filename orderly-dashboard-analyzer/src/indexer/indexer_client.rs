use orderly_dashboard_indexer::formats_external::{Response, trading_events::*};

use crate::config::AnalyzerConfig;

pub trait IndexerClient {
    fn pull_block(&self, start_block: i64, end_block: i64) -> Response<TradingEventsResponse>;
    fn new_client(analyzer_config: AnalyzerConfig) -> RestIndexerClient;
}

pub struct RestIndexerClient {
    indexer_address: String,
}

impl IndexerClient for RestIndexerClient {
    fn pull_block(&self, start_block: i64, end_block: i64) -> Response<TradingEventsResponse> {
        Response::empty_success()
    }

    fn new_client(analyzer_config: AnalyzerConfig) -> RestIndexerClient {
        Self {
            indexer_address: analyzer_config.indexer_address
        }
    }
}