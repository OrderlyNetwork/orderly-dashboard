use chrono::NaiveDateTime;
use orderly_dashboard_indexer::formats_external::trading_events::LiquidationTransfer;

pub async fn analyzer_liquidation(
    liquidated_account_id: String,
    insurance_account_id: String,
    liquidated_asset_hash: String,
    insurance_transfer_amount: String,
    liquidation_transfers: Vec<LiquidationTransfer>,
    block_num: i64,
    block_hour: NaiveDateTime,
    block_time: NaiveDateTime,
) {
}
