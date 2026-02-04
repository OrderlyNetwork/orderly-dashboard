use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use orderly_dashboard_indexer::formats_external::trading_events::{
    MarginMode, SettlementExecution, SettlementExecutionV3,
};
use std::ops::Div;

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::calc::{USDC_CHAIN_ID, USDC_HASH};
use crate::analyzer::{get_cost_position_prec, get_unitary_prec, FUTURES_FEE_COLLECTORS};
use crate::db::iso_user_perp_summary::IsoUserPerpSummary;
use crate::db::user_perp_summary::{UserPerpSummary, UserPerpSummaryKey};
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
    context: &mut AnalyzeContext,
) {
    tracing::info!(target:SETTLEMENT_ANALYZER,"receive settlement, account:{},settlement:{:?}",account_id.clone(),settlement_executions);
    if FUTURES_FEE_COLLECTORS.contains(&account_id.as_str()) {
        return;
    }
    let key = UserTokenSummaryKey {
        account_id: account_id.clone(),
        token: String::from(USDC_HASH),
        chain_id: String::from(USDC_CHAIN_ID),
    };

    let settle_token: BigDecimal = settled_amount.parse().unwrap();
    let fixed_amount = settle_token.div(get_cost_position_prec());
    let user_token = context.get_user_token(&key).await;
    user_token.new_settlement(fixed_amount, pulled_block_height);

    for settlement in settlement_executions {
        let user_perp_summary_key = UserPerpSummaryKey {
            account_id: account_id.clone(),
            symbol: settlement.symbol_hash.clone(),
        };

        let mut user_perp_snap = context.get_user_perp(&user_perp_summary_key.clone()).await;

        handle_settlement(&mut user_perp_snap, &settlement, pulled_block_height).await;
    }
}

async fn handle_settlement(
    user_perp_snap: &mut UserPerpSummary,
    settlement: &SettlementExecution,
    pulled_block_height: i64,
) {
    tracing::info!(target:SETTLEMENT_ANALYZER,"handle_settlement, account_id: {}, user_perp_snap.cost_position:{:?}, settlement.settled_amount:{:?}",user_perp_snap.account_id,user_perp_snap.cost_position.to_string(),settlement.settled_amount.to_string());
    let sum_unitary_fundings: BigDecimal = settlement.sum_unitary_fundings.parse().unwrap();
    let fixed_sum_unitary_fundings = sum_unitary_fundings.div(get_unitary_prec());
    let cost_position_diff: BigDecimal = settlement.settled_amount.parse().unwrap();
    let fixed_cost_position_diff = cost_position_diff.div(get_cost_position_prec());

    user_perp_snap.charge_funding_fee(fixed_sum_unitary_fundings, pulled_block_height);
    user_perp_snap.new_settlemnt(fixed_cost_position_diff, pulled_block_height);
}

async fn handle_settlement_v3_legacy(
    user_perp_snap: &mut UserPerpSummary,
    settlement: &SettlementExecutionV3,
    pulled_block_height: i64,
) {
    let sum_unitary_fundings: BigDecimal = settlement.sum_unitary_fundings.parse().unwrap();
    let fixed_sum_unitary_fundings = sum_unitary_fundings.div(get_unitary_prec());
    let cost_position_diff: BigDecimal = settlement.settled_amount.parse().unwrap();
    let fixed_cost_position_diff = cost_position_diff.div(get_cost_position_prec());

    user_perp_snap.charge_funding_fee(fixed_sum_unitary_fundings, pulled_block_height);
    user_perp_snap.new_settlemnt(fixed_cost_position_diff, pulled_block_height);
}

async fn handle_settlement_v3(
    user_perp_snap: &mut IsoUserPerpSummary,
    settlement: &SettlementExecutionV3,
    pulled_block_height: i64,
) {
    let sum_unitary_fundings: BigDecimal = settlement.sum_unitary_fundings.parse().unwrap();
    let fixed_sum_unitary_fundings = sum_unitary_fundings.div(get_unitary_prec());
    let cost_position_diff: BigDecimal = settlement.settled_amount.parse().unwrap();
    let fixed_cost_position_diff = cost_position_diff.clone().div(get_cost_position_prec());

    user_perp_snap.charge_funding_fee(fixed_sum_unitary_fundings, pulled_block_height);
    user_perp_snap.new_settlemnt(fixed_cost_position_diff.clone(), pulled_block_height);
    user_perp_snap.margin_token = settlement.iso_margin_asset_hash.clone();
    user_perp_snap.margin_qty += cost_position_diff;
}

pub async fn analyzer_settlement_v3(
    account_id: String,
    settled_amount: String,
    _settled_asset_hash: String,
    _insurance_account_id: String,
    _insurance_transfer_amount: String,
    settlement_executions: Vec<SettlementExecutionV3>,
    _block_hour: NaiveDateTime,
    pulled_block_height: i64,
    context: &mut AnalyzeContext,
) {
    tracing::info!(target:SETTLEMENT_ANALYZER,"pulled_block_height: {}, receive settlement,account:{},settlement:{:?}",pulled_block_height, account_id.clone(),settlement_executions);
    if FUTURES_FEE_COLLECTORS.contains(&account_id.as_str()) {
        return;
    }
    let key = UserTokenSummaryKey {
        account_id: account_id.clone(),
        token: String::from(USDC_HASH),
        chain_id: String::from(USDC_CHAIN_ID),
    };

    let settle_token: BigDecimal = settled_amount.parse().unwrap();
    let fixed_amount = settle_token.div(get_cost_position_prec());
    let user_token = context.get_user_token(&key).await;
    user_token.new_settlement(fixed_amount, pulled_block_height);

    for settlement in settlement_executions {
        let user_perp_summary_key = UserPerpSummaryKey {
            account_id: account_id.clone(),
            symbol: settlement.symbol_hash.clone(),
        };

        match settlement.margin_mode {
            MarginMode::Cross => {
                let mut user_perp_snap =
                    context.get_user_perp(&user_perp_summary_key.clone()).await;

                handle_settlement_v3_legacy(&mut user_perp_snap, &settlement, pulled_block_height)
                    .await;
            }
            MarginMode::Isolated => {
                let mut user_perp_snap = context
                    .get_iso_user_perp(&user_perp_summary_key.clone())
                    .await;

                handle_settlement_v3(&mut user_perp_snap, &settlement, pulled_block_height).await;
            }
        }
    }
}
