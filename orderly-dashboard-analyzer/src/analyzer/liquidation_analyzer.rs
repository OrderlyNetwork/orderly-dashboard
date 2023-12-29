use std::ops::Neg;
use std::str::FromStr;

use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::NaiveDateTime;
use orderly_dashboard_indexer::formats_external::trading_events::LiquidationTransfer;

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::calc::pnl_calc::RealizedPnl;
use crate::analyzer::calc::{USDC_CHAIN_ID, USDC_HASH};
use crate::analyzer::{div_into_real, to_big_decimal};
use crate::db::hourly_orderly_perp::HourlyOrderlyPerpKey;
use crate::db::hourly_user_perp::HourlyUserPerpKey;
use crate::db::user_perp_summary::UserPerpSummaryKey;
use crate::db::user_token_summary::UserTokenSummaryKey;

const LIQUIDATION_ANALYZER: &str = "liquidation-analyzer";

pub async fn analyzer_liquidation(
    liquidated_account_id: String,
    insurance_account_id: String,
    _liquidated_asset_hash: String,
    insurance_transfer_amount: String,
    liquidation_transfers: Vec<LiquidationTransfer>,
    block_num: i64,
    block_hour: NaiveDateTime,
    block_time: NaiveDateTime,
    context: &mut AnalyzeContext,
) {
    tracing::info!(target:LIQUIDATION_ANALYZER,"receive liquidation account:{}",liquidated_account_id.clone());
    let need_transfer_amount: BigDecimal = insurance_transfer_amount
        .parse()
        .unwrap_or(BigDecimal::from(0));
    if need_transfer_amount != BigDecimal::from(0) {
        let fixed_amount = div_into_real(need_transfer_amount.to_i128().unwrap(), 1_000_000);
        {
            transfer_amount(
                liquidated_account_id.clone(),
                context,
                to_big_decimal(fixed_amount.clone()),
            )
            .await;
        }
        {
            transfer_amount(
                insurance_account_id.clone(),
                context,
                to_big_decimal(fixed_amount.clone()).neg(),
            )
            .await;
        }
    }
    for liquidation in liquidation_transfers {
        tracing::info!(target:LIQUIDATION_ANALYZER,"{:?}",liquidation.clone());
        let liquidation_qty: BigDecimal = liquidation.position_qty_transfer.parse().unwrap();
        if liquidation_qty.clone() == BigDecimal::from(0) {
            tracing::info!(target:LIQUIDATION_ANALYZER,"liquidation_qty equals zero, skipped");
            continue;
        }

        let cost_position_transfer: BigDecimal =
            liquidation.cost_position_transfer.clone().parse().unwrap();
        let liquidator_fee: BigDecimal = liquidation.liquidator_fee.parse().unwrap();
        let insurance_fee: BigDecimal = liquidation.insurance_fee.parse().unwrap();
        let mark_price: BigDecimal = liquidation.mark_price.parse().unwrap();

        let fixed_qty = div_into_real(liquidation_qty.to_i128().unwrap(), 100_000_000);
        let fixed_cost_p_transfer =
            div_into_real(cost_position_transfer.to_i128().unwrap(), 1_000_000);
        let fixed_liquidator_fee = div_into_real(liquidator_fee.to_i128().unwrap(), 1_000_000);
        let fixed_insurance_fee = div_into_real(insurance_fee.to_i128().unwrap(), 1_000_000);
        let fixed_mark_price = div_into_real(mark_price.to_i128().unwrap(), 100_000_000);

        let key = UserPerpSummaryKey::new_key(
            liquidated_account_id.clone(),
            liquidation.symbol_hash.clone(),
        );
        let (mut open_cost_diff, mut pnl_diff) = (Default::default(), Default::default());
        {
            let user_perp = context.get_user_perp(&key.clone()).await;
            let user_perp_snap = user_perp.clone();
            (open_cost_diff, pnl_diff) = RealizedPnl::calc_realized_pnl(
                to_big_decimal(fixed_qty.clone()),
                to_big_decimal(fixed_cost_p_transfer.clone())
                    - (to_big_decimal(fixed_liquidator_fee.clone())
                        + to_big_decimal(fixed_insurance_fee.clone())),
                user_perp_snap.holding.clone(),
                user_perp_snap.opening_cost.clone(),
            );
        }

        {
            let user_perp = context.get_user_perp(&key.clone()).await;
            user_perp.new_liquidation(
                to_big_decimal(fixed_qty.clone()),
                to_big_decimal(fixed_mark_price.clone()),
                block_num.clone(),
                block_time.clone(),
                to_big_decimal(fixed_cost_p_transfer.clone()),
                BigDecimal::from_str(&*liquidation.sum_unitary_fundings).unwrap(),
                open_cost_diff.clone(),
            );
        }

        {
            let h_perp_key = HourlyUserPerpKey::new_key(
                liquidated_account_id.clone(),
                liquidation.symbol_hash.clone(),
                block_hour.clone(),
            );
            let h_user_perp = context.get_hourly_user_perp(&h_perp_key).await;
            h_user_perp.new_liquidation(
                to_big_decimal(fixed_qty.clone()) * to_big_decimal(fixed_mark_price.clone()),
                block_num,
                block_time.clone(),
                pnl_diff.clone(),
            );
        }

        let liquidator_key = UserPerpSummaryKey::new_key(
            liquidation.liquidator_account_id.clone(),
            liquidation.symbol_hash.clone(),
        );

        let (mut liquidator_open_cost_diff, mut liquidator_pnl_diff) =
            (Default::default(), Default::default());
        {
            let user_perp = context.get_user_perp(&liquidator_key.clone()).await;
            let user_perp_snap = user_perp.clone();
            (liquidator_open_cost_diff, liquidator_pnl_diff) = RealizedPnl::calc_realized_pnl(
                to_big_decimal(fixed_qty.clone()),
                -(to_big_decimal(fixed_cost_p_transfer.clone())
                    - to_big_decimal(fixed_liquidator_fee.clone())),
                user_perp_snap.holding.clone(),
                user_perp_snap.opening_cost.clone(),
            );
        }

        {
            let user_perp = context.get_user_perp(&liquidator_key.clone()).await;
            user_perp.new_liquidator(
                to_big_decimal(fixed_qty.clone()),
                liquidator_open_cost_diff.clone(),
            );
        }
        {
            let h_perp_key = HourlyUserPerpKey::new_key(
                liquidation.liquidator_account_id.clone(),
                liquidation.symbol_hash.clone(),
                block_hour.clone(),
            );
            let h_user_perp = context.get_hourly_user_perp(&h_perp_key).await;
            h_user_perp.new_realized_pnl(liquidator_pnl_diff.clone());
        }

        {
            let h_orderly_perp_key =
                HourlyOrderlyPerpKey::new_key(liquidation.symbol_hash.clone(), block_hour.clone());
            let h_orderly_perp = context.get_hourly_orderly_perp(&h_orderly_perp_key).await;
            h_orderly_perp.new_liquidation(
                to_big_decimal(fixed_qty.clone()) * to_big_decimal(fixed_mark_price.clone()),
                block_num,
                block_time.clone(),
            );
        }

        {
            let orderly_perp = context
                .get_orderly_perp(&liquidation.symbol_hash.clone())
                .await;
            orderly_perp.new_liquidation(
                to_big_decimal(fixed_qty) * to_big_decimal(fixed_mark_price),
                block_num,
                block_time.clone(),
            );
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
    user_token.add_amount(amount);
}
