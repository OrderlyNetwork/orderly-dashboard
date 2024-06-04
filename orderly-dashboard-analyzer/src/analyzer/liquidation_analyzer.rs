use std::ops::Div;
use std::ops::Neg;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use orderly_dashboard_indexer::formats_external::trading_events::{
    LiquidationTransfer, LiquidationTransferV2,
};

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::calc::pnl_calc::RealizedPnl;
use crate::analyzer::calc::{USDC_CHAIN_ID, USDC_HASH};
use crate::db::hourly_orderly_perp::HourlyOrderlyPerpKey;
use crate::db::hourly_user_perp::HourlyUserPerpKey;
use crate::db::user_perp_summary::UserPerpSummaryKey;
use crate::db::user_token_summary::UserTokenSummaryKey;

use super::get_cost_position_prec;
use super::get_price_prec;
use super::get_qty_prec;
use super::get_unitary_prec;

const LIQUIDATION_ANALYZER: &str = "liquidation-analyzer";
pub struct Liquidation {
    pub liquidated_account_id: String,
    pub liquidator_account_id: String,

    pub symbol_hash: String,
    pub qty_transfer: BigDecimal,
    pub cost_position_transfer: BigDecimal,
    pub liquidator_fee: BigDecimal,
    pub insurnace_fee: BigDecimal,
    pub mark_price: BigDecimal,
    pub sum_unitry_funding: BigDecimal,

    pub block_num: i64,
    pub block_hour: NaiveDateTime,
    pub block_time: NaiveDateTime,
}

impl Liquidation {
    pub fn from_event(
        liquidated_account_id: String,
        event: LiquidationTransfer,
        block_num: i64,
        block_time: NaiveDateTime,
        block_hour: NaiveDateTime,
    ) -> Liquidation {
        let liquidation_qty: BigDecimal = event.position_qty_transfer.parse().unwrap();
        let cost_position_transfer: BigDecimal =
            event.cost_position_transfer.clone().parse().unwrap();
        let liquidator_fee: BigDecimal = event.liquidator_fee.parse().unwrap();
        let insurance_fee: BigDecimal = event.insurance_fee.parse().unwrap();
        let mark_price: BigDecimal = event.mark_price.parse().unwrap();
        let sum_unitary_funding: BigDecimal = event.sum_unitary_fundings.parse().unwrap();

        let fixed_qty = liquidation_qty.div(get_qty_prec());
        let fixed_cost_p_transfer = cost_position_transfer.div(get_cost_position_prec());
        let fixed_liquidator_fee = liquidator_fee.div(get_cost_position_prec());
        let fixed_insurance_fee = insurance_fee.div(get_cost_position_prec());
        let fixed_mark_price = mark_price.div(get_price_prec());
        let fixed_sum_unitary_fundings = sum_unitary_funding.div(get_unitary_prec());

        Liquidation {
            liquidated_account_id,
            liquidator_account_id: event.liquidator_account_id,
            symbol_hash: event.symbol_hash,
            qty_transfer: fixed_qty,
            cost_position_transfer: fixed_cost_p_transfer,
            liquidator_fee: fixed_liquidator_fee,
            insurnace_fee: fixed_insurance_fee,
            mark_price: fixed_mark_price,
            sum_unitry_funding: fixed_sum_unitary_fundings,
            block_num,
            block_hour,
            block_time,
        }
    }

    pub fn from_event_v2(
        liquidated_account_id: String,
        event: LiquidationTransferV2,
        block_num: i64,
        block_time: NaiveDateTime,
        block_hour: NaiveDateTime,
    ) -> Liquidation {
        let liquidation_qty: BigDecimal = event.position_qty_transfer.parse().unwrap();
        let cost_position_transfer: BigDecimal =
            event.cost_position_transfer.clone().parse().unwrap();
        let mark_price: BigDecimal = event.mark_price.parse().unwrap();
        let sum_unitary_funding: BigDecimal = event.sum_unitary_fundings.parse().unwrap();
        let liquidation_fee: BigDecimal = event.fee.parse().unwrap();

        let fixed_qty = liquidation_qty.div(get_qty_prec());
        let fixed_cost_p_transfer = cost_position_transfer.div(get_cost_position_prec());
        let fixed_mark_price = mark_price.div(get_price_prec());
        let fixed_sum_unitary_fundings = sum_unitary_funding.div(get_unitary_prec());
        let fixed_liquidation_fee = liquidation_fee.div(get_cost_position_prec());

        Liquidation {
            liquidated_account_id,
            liquidator_account_id: event.account_id,
            symbol_hash: event.symbol_hash,
            qty_transfer: fixed_qty,
            cost_position_transfer: fixed_cost_p_transfer,
            liquidator_fee: fixed_liquidation_fee,
            insurnace_fee: BigDecimal::from(0),
            mark_price: fixed_mark_price,
            sum_unitry_funding: fixed_sum_unitary_fundings,
            block_num,
            block_hour,
            block_time,
        }
    }
}

pub async fn analyzer_liquidation_v2(
    liquidated_account_id: String,
    _liquidated_asset_hash: String,
    insurance_transfer_amount: String,
    liquidation_transfers: Vec<LiquidationTransferV2>,
    block_num: i64,
    block_hour: NaiveDateTime,
    block_time: NaiveDateTime,
    context: &mut AnalyzeContext,
) {
    tracing::info!(target:LIQUIDATION_ANALYZER,"receive liquidation account:{}",liquidated_account_id.clone());
    transfer_insurance_fund(
        &insurance_transfer_amount,
        context,
        liquidated_account_id.clone(),
    )
    .await;

    for liquidation_event in liquidation_transfers {
        tracing::info!(target:LIQUIDATION_ANALYZER,"{:?}",liquidation_event.clone());
        let liquidation_qty: BigDecimal = liquidation_event.position_qty_transfer.parse().unwrap();
        if liquidation_qty.clone() == BigDecimal::from(0) {
            tracing::info!(target:LIQUIDATION_ANALYZER,"liquidation_qty equals zero, skipped");
            continue;
        }

        let liquidation = Liquidation::from_event_v2(
            liquidated_account_id.clone(),
            liquidation_event,
            block_num,
            block_time.clone(),
            block_hour.clone(),
        );

        execute_for_liquidated(&liquidation, context).await;
        execute_for_liquidator(&liquidation, context).await;
        statistics_for_orderly(&liquidation, context).await;
    }
}

pub async fn analyzer_liquidation_v1(
    liquidated_account_id: String,
    _liquidated_asset_hash: String,
    insurance_transfer_amount: String,
    liquidation_transfers: Vec<LiquidationTransfer>,
    block_num: i64,
    block_hour: NaiveDateTime,
    block_time: NaiveDateTime,
    context: &mut AnalyzeContext,
) {
    tracing::info!(target:LIQUIDATION_ANALYZER,"receive liquidation account:{}",liquidated_account_id.clone());
    transfer_insurance_fund(
        &insurance_transfer_amount,
        context,
        liquidated_account_id.clone(),
    )
    .await;

    for liquidation_event in liquidation_transfers {
        tracing::info!(target:LIQUIDATION_ANALYZER,"{:?}",liquidation_event.clone());
        let liquidation_qty: BigDecimal = liquidation_event.position_qty_transfer.parse().unwrap();
        if liquidation_qty.clone() == BigDecimal::from(0) {
            tracing::info!(target:LIQUIDATION_ANALYZER,"liquidation_qty equals zero, skipped");
            continue;
        }

        let liquidation = Liquidation::from_event(
            liquidated_account_id.clone(),
            liquidation_event,
            block_num,
            block_time.clone(),
            block_hour.clone(),
        );

        execute_for_liquidated(&liquidation, context).await;
        execute_for_liquidator(&liquidation, context).await;
        statistics_for_orderly(&liquidation, context).await;
    }
}

async fn statistics_for_orderly(liquidation: &Liquidation, context: &mut AnalyzeContext) {
    let h_orderly_perp_key = HourlyOrderlyPerpKey::new_key(
        liquidation.symbol_hash.clone(),
        liquidation.block_hour.clone(),
    );
    let h_orderly_perp = context.get_hourly_orderly_perp(&h_orderly_perp_key).await;
    h_orderly_perp.new_liquidation(
        (liquidation.qty_transfer.clone()) * (liquidation.mark_price.clone()),
        liquidation.block_num,
        liquidation.block_time.clone(),
    );

    let orderly_perp = context
        .get_orderly_perp(&liquidation.symbol_hash.clone())
        .await;
    orderly_perp.new_liquidation(
        liquidation.qty_transfer.clone() * liquidation.mark_price.clone(),
        liquidation.block_num,
        liquidation.block_time.clone(),
    );
}

async fn execute_for_liquidator(liquidation: &Liquidation, context: &mut AnalyzeContext) {
    let liquidator_key = UserPerpSummaryKey::new_key(
        liquidation.liquidator_account_id.clone(),
        liquidation.symbol_hash.clone(),
    );

    let user_perp = context.get_user_perp(&liquidator_key.clone()).await;
    user_perp.charge_funding_fee(liquidation.sum_unitry_funding.clone());

    let user_perp_snap = user_perp.clone();
    let (liquidator_open_cost_diff, liquidator_pnl_diff) = RealizedPnl::calc_realized_pnl(
        liquidation.qty_transfer.clone(),
        -(liquidation.cost_position_transfer.clone() - (liquidation.liquidator_fee.clone())),
        user_perp_snap.holding.clone(),
        user_perp_snap.opening_cost.clone(),
    );

    let user_perp = context.get_user_perp(&liquidator_key.clone()).await;
    user_perp.new_liquidator(
        liquidation.qty_transfer.clone(),
        liquidator_open_cost_diff.clone(),
    );

    let h_perp_key = HourlyUserPerpKey::new_key(
        liquidation.liquidator_account_id.clone(),
        liquidation.symbol_hash.clone(),
        liquidation.block_hour.clone(),
    );

    let h_user_perp = context.get_hourly_user_perp(&h_perp_key).await;
    h_user_perp.new_realized_pnl(liquidator_pnl_diff.clone());
}

async fn execute_for_liquidated(liquidation: &Liquidation, context: &mut AnalyzeContext) {
    let key = UserPerpSummaryKey::new_key(
        liquidation.liquidated_account_id.clone(),
        liquidation.symbol_hash.clone(),
    );

    let user_perp = context.get_user_perp(&key.clone()).await;
    user_perp.charge_funding_fee(liquidation.sum_unitry_funding.clone());

    let user_perp_snap = user_perp.clone();
    let (open_cost_diff, pnl_diff) = RealizedPnl::calc_realized_pnl(
        liquidation.qty_transfer.clone(),
        (liquidation.cost_position_transfer.clone())
            - ((liquidation.liquidator_fee.clone()) + (liquidation.insurnace_fee.clone())),
        user_perp_snap.holding.clone(),
        user_perp_snap.opening_cost.clone(),
    );

    let user_perp = context.get_user_perp(&key.clone()).await;
    user_perp.new_liquidation(
        liquidation.qty_transfer.clone(),
        liquidation.mark_price.clone(),
        liquidation.block_num,
        liquidation.block_time.clone(),
        liquidation.cost_position_transfer.clone(),
        liquidation.sum_unitry_funding.clone(),
        open_cost_diff.clone(),
    );

    let h_perp_key = HourlyUserPerpKey::new_key(
        liquidation.liquidated_account_id.clone(),
        liquidation.symbol_hash.clone(),
        liquidation.block_hour.clone(),
    );
    let h_user_perp = context.get_hourly_user_perp(&h_perp_key).await;
    h_user_perp.new_liquidation(
        (liquidation.qty_transfer.clone()) * (liquidation.mark_price.clone()),
        liquidation.block_num,
        liquidation.block_time.clone(),
        pnl_diff.clone(),
    );
}

async fn transfer_insurance_fund(
    insurance_transfer_amount: &String,
    context: &mut AnalyzeContext,
    liquidated_account_id: String,
) {
    let need_transfer_amount: BigDecimal = insurance_transfer_amount
        .parse()
        .unwrap_or(BigDecimal::from(0));
    if need_transfer_amount != BigDecimal::from(0) {
        let fixed_amount = need_transfer_amount.div(get_cost_position_prec());
        {
            transfer_amount(liquidated_account_id.clone(), context, fixed_amount.clone()).await;
        }
    }
}

async fn transfer_amount(account_id: String, context: &mut AnalyzeContext, amount: BigDecimal) {
    let key = UserTokenSummaryKey {
        account_id: account_id.clone(),
        token: String::from(USDC_HASH),
        chain_id: String::from(USDC_CHAIN_ID),
    };

    let user_token = context.get_user_token(&key).await;
    user_token.add_amount(amount.abs().neg());
}
