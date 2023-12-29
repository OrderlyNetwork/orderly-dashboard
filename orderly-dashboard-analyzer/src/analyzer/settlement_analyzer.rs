use chrono::NaiveDateTime;
use orderly_dashboard_indexer::formats_external::trading_events::SettlementExecution;

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::calc::{USDC_CHAIN_ID, USDC_HASH};
use crate::analyzer::div_into_real;
use crate::db::user_token_summary::UserTokenSummaryKey;

const SETTLEMENT_ANALYZER: &str = "settlement-analyzer";

pub async fn analyzer_settlement(
    account_id: String,
    settled_amount: String,
    _settled_asset_hash: String,
    _insurance_account_id: String,
    _insurance_transfer_amount: String,
    _settlement_executions: Vec<SettlementExecution>,
    _block_hour: NaiveDateTime,
    pulled_block_height: i64,
    pulled_block_time: NaiveDateTime,
    context: &mut AnalyzeContext,
) {
    tracing::info!(target:SETTLEMENT_ANALYZER," receive settlement,account:{},amount:{}",account_id.clone(),settled_amount.clone());
    let key = UserTokenSummaryKey {
        account_id: account_id.clone(),
        token: String::from(USDC_HASH),
        chain_id: String::from(USDC_CHAIN_ID),
    };

    let fixed_amount = div_into_real(settled_amount.parse().unwrap(), 1000000);
    let user_token = context.get_user_token(&key).await;
    user_token.new_settlement(fixed_amount, pulled_block_height, pulled_block_time.clone());
}
