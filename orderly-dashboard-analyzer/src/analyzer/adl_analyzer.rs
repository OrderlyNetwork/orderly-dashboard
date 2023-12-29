use std::str::FromStr;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use num_traits::ToPrimitive;

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::calc::pnl_calc::RealizedPnl;
use crate::analyzer::div_into_real;
use crate::db::hourly_orderly_perp::HourlyOrderlyPerpKey;
use crate::db::hourly_user_perp::HourlyUserPerpKey;
use crate::db::user_perp_summary::UserPerpSummaryKey;

const ADL_ANALYZER: &str = "adl-analyzer";

pub async fn analyzer_adl(
    account_id: String,
    _insurance_account_id: String,
    symbol_hash: String,
    position_qty_transfer: String,
    cost_position_transfer: String,
    adl_price: String,
    sum_unitary_fundings: String,
    block_hour: NaiveDateTime,
    pulled_block_time: NaiveDateTime,
    block_num: i64,
    context: &mut AnalyzeContext,
) {
    tracing::info!(target:ADL_ANALYZER,"receiver adl account_id:{},symbol:{},qty:{},cost_position:{}",account_id.clone(),symbol_hash.clone(),position_qty_transfer.clone(),cost_position_transfer.clone());
    let adl_qty: BigDecimal = position_qty_transfer.parse().unwrap();
    let adl_price: BigDecimal = adl_price.parse().unwrap();

    let fixed_adl_qty = div_into_real(adl_qty.to_i128().unwrap(), 100_000_000);
    let fixed_adl_perice = div_into_real(adl_price.to_i128().unwrap(), 100_000_000);
    let fixed_position_transfer = div_into_real(cost_position_transfer.parse().unwrap(), 1_000_000);

    {
        let key = HourlyOrderlyPerpKey::new_key(symbol_hash.clone(), block_hour.clone());
        let hourly_orderly_perp = context.get_hourly_orderly_perp(&key).await;
        hourly_orderly_perp.new_liquidation(
            fixed_adl_perice.clone() * fixed_adl_qty.clone(),
            block_num,
            pulled_block_time.clone(),
        );
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
            pulled_block_time.clone(),
            pnl_diff,
        );
    }

    {
        let user_perp_summary = context.get_user_perp(&user_perp_key).await;
        user_perp_summary.new_liquidation(
            adl_qty.clone(),
            adl_price.clone(),
            block_num,
            pulled_block_time.clone(),
            BigDecimal::from_str(&*cost_position_transfer.clone()).unwrap(),
            BigDecimal::from_str(&*sum_unitary_fundings.clone()).unwrap(),
            open_cost_diff,
        );
    }
}
