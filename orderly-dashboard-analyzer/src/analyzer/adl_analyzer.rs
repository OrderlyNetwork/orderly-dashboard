use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use std::ops::Div;

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::calc::pnl_calc::RealizedPnl;
use crate::analyzer::{get_cost_position_prec, get_price_prec, get_qty_prec, get_unitary_prec};
use crate::db::hourly_orderly_perp::HourlyOrderlyPerpKey;
use crate::db::hourly_user_perp::HourlyUserPerpKey;
use crate::db::user_perp_summary::UserPerpSummaryKey;

const ADL_ANALYZER: &str = "adl-analyzer";
pub async fn analyzer_adl(
    account_id: String,
    symbol_hash: String,
    position_qty_transfer: String,
    cost_position_transfer: String,
    adl_price: String,
    sum_unitary_fundings: String,
    block_hour: NaiveDateTime,
    block_num: i64,
    context: &mut AnalyzeContext,
) {
    tracing::info!(target:ADL_ANALYZER,"receiver adl account_id:{},symbol:{},qty:{},cost_position:{}",account_id.clone(),symbol_hash.clone(),position_qty_transfer.clone(),cost_position_transfer.clone());
    let adl_qty: BigDecimal = position_qty_transfer.parse().unwrap();
    let adl_price: BigDecimal = adl_price.parse().unwrap();
    let cpt: BigDecimal = cost_position_transfer.parse().unwrap();
    let fsuf: BigDecimal = sum_unitary_fundings.parse().unwrap();

    let fixed_adl_qty = adl_qty.clone().div(get_qty_prec());
    let fixed_adl_perice = adl_price.clone().div(get_price_prec());
    let fixed_position_transfer = cpt.clone().div(get_cost_position_prec());
    let fixed_sum_unitary_fundings = fsuf.div(get_unitary_prec());

    {
        let key = HourlyOrderlyPerpKey::new_key(symbol_hash.clone(), block_hour.clone());
        let hourly_orderly_perp = context.get_hourly_orderly_perp(&key).await;
        hourly_orderly_perp
            .new_liquidation(fixed_adl_perice.clone() * fixed_adl_qty.clone(), block_num);
    }

    let user_perp_key = UserPerpSummaryKey {
        account_id: account_id.clone(),
        symbol: symbol_hash.clone(),
    };
    let user_perp_snap = context.get_user_perp(&user_perp_key.clone()).await.clone();
    let (open_cost_diff, pnl_diff) = RealizedPnl::calc_realized_pnl(
        fixed_adl_qty.clone(),
        fixed_position_transfer.clone(),
        user_perp_snap.holding,
        user_perp_snap.opening_cost,
    );

    {
        let key =
            HourlyUserPerpKey::new_key(account_id.clone(), symbol_hash.clone(), block_hour.clone());
        let hourly_user_perp = context.get_hourly_user_perp(&key).await;
        hourly_user_perp.new_liquidation(
            fixed_adl_perice.clone() * fixed_adl_qty.clone(),
            block_num,
            pnl_diff,
        );
    }

    {
        let user_perp_summary = context.get_user_perp(&user_perp_key).await;
        user_perp_summary.charge_funding_fee(fixed_sum_unitary_fundings.clone(), block_num);

        user_perp_summary.new_liquidation(
            adl_qty.clone(),
            adl_price.clone(),
            block_num,
            cpt.clone(),
            fixed_sum_unitary_fundings,
            open_cost_diff,
        );
    }
}
