use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use orderly_dashboard_indexer::formats_external::trading_events::SettlementExecution;
use std::ops::Div;

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::calc::{USDC_CHAIN_ID, USDC_HASH};
use crate::analyzer::{get_cost_position_prec, get_unitary_prec};
use crate::db::user_perp_summary::UserPerpSummaryKey;
use crate::db::user_token_summary::UserTokenSummaryKey;

const SETTLEMENT_ANALYZER: &str = "settlement-analyzer";
pub async fn analyzer_settlement(
    account_id: String,
    settled_amount: String,
    _settled_asset_hash: String,
    _insurance_account_id: String,
    _insurance_transfer_amount: String,
    settlement_executions: Vec<SettlementExecution>,
    _block_hour: NaiveDateTime,
    pulled_block_height: i64,
    pulled_block_time: NaiveDateTime,
    context: &mut AnalyzeContext,
) {
    tracing::info!(target:SETTLEMENT_ANALYZER," receive settlement,account:{},settlement:{:?}",account_id.clone(),settlement_executions);
    let key = UserTokenSummaryKey {
        account_id: account_id.clone(),
        token: String::from(USDC_HASH),
        chain_id: String::from(USDC_CHAIN_ID),
    };

    let settle_token: BigDecimal = settled_amount.parse().unwrap();
    let fixed_amount = settle_token.div(get_cost_position_prec());
    let user_token = context.get_user_token(&key).await;
    user_token.new_settlement(fixed_amount, pulled_block_height, pulled_block_time.clone());

    for settlement in settlement_executions {
        let user_perp_summary_key = UserPerpSummaryKey {
            account_id: account_id.clone(),
            symbol: settlement.symbol_hash.clone(),
        };

        let mut user_perp_snap = context
            .get_user_perp(&user_perp_summary_key.clone())
            .await
            .clone();

        let sum_unitary_fundings: BigDecimal = settlement.sum_unitary_fundings.parse().unwrap();
        let fixed_sum_unitary_fundings = sum_unitary_fundings.div(get_unitary_prec());
        let cost_position_diff: BigDecimal = settlement.settled_amount.parse().unwrap();
        let fixed_cost_position_diff = cost_position_diff.div(get_cost_position_prec());

        user_perp_snap.charge_funding_fee(fixed_sum_unitary_fundings);
        user_perp_snap.new_settlemnt(fixed_cost_position_diff);
    }
}
