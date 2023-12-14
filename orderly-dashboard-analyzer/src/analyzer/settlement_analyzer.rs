use orderly_dashboard_indexer::formats_external::trading_events::SettlementExecution;

pub async fn analyzer_settlement(
    account_id: String,
    settled_amount: String,
    settled_asset_hash: String,
    insurance_account_id: String,
    insurance_transfer_amount: String,
    settlement_executions: Vec<SettlementExecution>,
    block_hour: i64,
    pulled_block_height: i64,
    pulled_block_time: i64,
) {
}
