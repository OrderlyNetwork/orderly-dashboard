use orderly_dashboard_indexer::formats_external::{trading_events::*};

pub trait BlockEventAnalyzer {
    fn analyze_event(&self, block_events: Option<&TradingEventsResponse>);
}

pub struct EtherBlockEventAnalyzer {}

impl BlockEventAnalyzer for EtherBlockEventAnalyzer {
    fn analyze_event(&self, block_events: Option<&TradingEventsResponse>) {
        todo!()
    }
}

