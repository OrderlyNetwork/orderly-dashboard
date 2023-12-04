use orderly_dashboard_indexer::formats_external::{trading_events::*};

pub trait BlockEventAnalyzer {
    fn analyzer_event(&self, block_events: Option<&TradingEventsResponse>);
}

pub struct EtherBlockEventAnalyzer {}

impl BlockEventAnalyzer for EtherBlockEventAnalyzer {
    fn analyzer_event(&self, block_events: Option<&TradingEventsResponse>) {
        todo!()
    }
}

