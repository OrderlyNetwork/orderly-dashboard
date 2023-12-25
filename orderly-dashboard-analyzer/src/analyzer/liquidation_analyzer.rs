use chrono::NaiveDateTime;
use orderly_dashboard_indexer::formats_external::trading_events::LiquidationTransfer;

pub async fn analyzer_liquidation(
    _liquidated_account_id: String,
    _insurance_account_id: String,
    _liquidated_asset_hash: String,
    _insurance_transfer_amount: String,
    _liquidation_transfers: Vec<LiquidationTransfer>,
    _block_num: i64,
    _block_hour: NaiveDateTime,
    _block_time: NaiveDateTime,
) {
}
