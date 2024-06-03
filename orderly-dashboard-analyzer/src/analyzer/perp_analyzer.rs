use std::cmp::max;

use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, Timelike};
use std::ops::Div;

use orderly_dashboard_indexer::formats_external::trading_events::Trade;

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::calc::pnl_calc::RealizedPnl;
use crate::analyzer::{get_cost_position_prec, get_qty_prec, get_unitary_prec};
use crate::db::hourly_orderly_perp::HourlyOrderlyPerpKey;
use crate::db::hourly_user_perp::HourlyUserPerpKey;
use crate::db::user_perp_summary::UserPerpSummaryKey;

const PERP_ANALYZER: &str = "perp-trade-analyzer";
pub async fn analyzer_perp_trade(
    trades: Vec<Trade>,
    pulled_block_time: NaiveDateTime,
    pulled_block_height: i64,
    context: &mut AnalyzeContext,
) -> i64 {
    let mut max_perp_trade_id = 0i64;
    for perp_trade in trades {
        tracing::info!(target:PERP_ANALYZER,"receiver trade:{}",perp_trade.trade_id.clone());
        max_perp_trade_id = max(max_perp_trade_id, perp_trade.trade_id as i64);

        let trade_qty: BigDecimal = perp_trade.trade_qty.parse().unwrap();
        let fixed_qty = trade_qty.div(get_qty_prec());

        let trade_fee: BigDecimal = perp_trade.fee.parse().unwrap();
        let fixed_fee = trade_fee.div(get_cost_position_prec());

        let notional: BigDecimal = perp_trade.notional.parse().unwrap();
        let fixed_notional = notional.div(get_cost_position_prec());
        let trade_hour = convert_block_hour(perp_trade.timestamp as i64);

        // hourly_orderly
        let hour_orderly_perp_key = HourlyOrderlyPerpKey {
            symbol: perp_trade.symbol_hash.clone(),
            block_hour: trade_hour.clone(),
        };
        {
            let hourly_orderly_perp = context
                .get_hourly_orderly_perp(&hour_orderly_perp_key)
                .await;
            hourly_orderly_perp.new_trade(
                (fixed_fee.clone()).abs(),
                (fixed_notional.clone()).abs(),
                pulled_block_height.clone(),
                pulled_block_time.clone(),
            );
        }

        //orderly_summary
        let perp_symbol = perp_trade.symbol_hash.clone();
        {
            let orderly_perp_summary = context.get_orderly_perp(&perp_symbol).await;
            orderly_perp_summary.new_trade(
                (fixed_fee.clone()).abs(),
                (fixed_notional.clone()).abs(),
                pulled_block_height.clone(),
                pulled_block_time.clone(),
                perp_trade.side.clone(),
            );
        }
        let user_perp_summary_key = UserPerpSummaryKey {
            account_id: perp_trade.account_id.clone(),
            symbol: perp_symbol.clone(),
        };

        //user_summary
        let mut user_perp_snap = context
            .get_user_perp(&user_perp_summary_key.clone())
            .await
            .clone();

        let suf: BigDecimal = perp_trade.sum_unitary_fundings.parse().unwrap();

        user_perp_snap.charge_funding_fee(suf.div(get_unitary_prec()));
        let (open_cost_diff, pnl_diff) = RealizedPnl::calc_realized_pnl(
            fixed_qty.clone(),
            fixed_notional.clone(),
            user_perp_snap.holding.clone(),
            user_perp_snap.opening_cost.clone(),
        );
        {
            let user_perp_summary = context.get_user_perp(&user_perp_summary_key).await;
            let (opening, new_user) = user_perp_summary.new_trade(
                fixed_fee.clone(),
                fixed_notional.clone(),
                pulled_block_height.clone(),
                pulled_block_time.clone(),
                open_cost_diff.clone(),
                fixed_qty.clone(),
            );
            if opening {
                context
                    .get_hourly_orderly_perp(&hour_orderly_perp_key)
                    .await
                    .new_opening();
            }
            if new_user {
                context.get_orderly_perp(&perp_symbol).await.new_user();
            }
        }

        //hourly_user
        {
            let hourly_user_perp_key = HourlyUserPerpKey {
                account_id: perp_trade.account_id.clone(),
                symbol: perp_trade.symbol_hash.clone(),
                block_hour: trade_hour.clone(),
            };

            let hourly_user_perp = context.get_hourly_user_perp(&hourly_user_perp_key).await;
            let new_hourly_user = hourly_user_perp.new_trade(
                fixed_fee,
                fixed_notional,
                pulled_block_height.clone(),
                pulled_block_time.clone(),
                pnl_diff,
            );

            if new_hourly_user {
                context
                    .get_hourly_orderly_perp(&hour_orderly_perp_key)
                    .await
                    .new_user();
            }
        }
    }

    return max_perp_trade_id;
}

fn convert_block_hour(block_timestamp: i64) -> NaiveDateTime {
    let date_time = NaiveDateTime::from_timestamp_opt(block_timestamp / 1000, 0).unwrap();
    return date_time.with_second(0).unwrap().with_minute(0).unwrap();
}
