use std::cmp::max;

use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::NaiveDateTime;
use orderly_dashboard_indexer::formats_external::trading_events::Trade;

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::calc::pnl_calc::RealizedPnl;
use crate::analyzer::{div_into_real, to_big_decimal};
use crate::db::hourly_orderly_perp::HourlyOrderlyPerpKey;
use crate::db::hourly_user_perp::HourlyUserPerpKey;
use crate::db::user_perp_summary::UserPerpSummaryKey;

const PERP_ANALYZER: &str = "perp-trade-analyzer";

pub async fn analyzer_perp_trade(
    trades: Vec<Trade>,
    block_hour: NaiveDateTime,
    pulled_block_time: NaiveDateTime,
    pulled_block_height: i64,
    context: &mut AnalyzeContext,
) -> i64 {
    let mut max_perp_trade_id = 0i64;
    for perp_trade in trades {
        tracing::info!(target:PERP_ANALYZER,"receiver trade:{}",perp_trade.trade_id.clone());
        max_perp_trade_id = max(max_perp_trade_id, perp_trade.trade_id as i64);

        let trade_qty: BigDecimal = perp_trade.trade_qty.parse().unwrap();
        let fixed_qty = div_into_real(trade_qty.to_i128().unwrap(), 100000000);

        let trade_fee: BigDecimal = perp_trade.fee.parse().unwrap();
        let fixed_fee = div_into_real(trade_fee.to_i128().unwrap(), 1000000);

        let notional: BigDecimal = perp_trade.notional.parse().unwrap();
        let fixed_notional = div_into_real(notional.to_i128().unwrap(), 1000000);


        // hourly_orderly
        let hour_orderly_perp_key = HourlyOrderlyPerpKey {
            symbol: perp_trade.symbol_hash.clone(),
            block_hour: block_hour.clone(),
        };
        {
            let hourly_orderly_perp = context
                .get_hourly_orderly_perp(&hour_orderly_perp_key)
                .await;
            hourly_orderly_perp.new_trade(
                to_big_decimal(fixed_fee.clone()).abs(),
                to_big_decimal(fixed_notional.clone()).abs(),
                pulled_block_height.clone(),
                pulled_block_time.clone(),
            );
        }

        //orderly_summary
        let perp_symbol = perp_trade.symbol_hash.clone();
        {
            let orderly_perp_summary = context.get_orderly_perp(&perp_symbol).await;
            orderly_perp_summary.new_trade(
                to_big_decimal(fixed_fee.clone()).abs(),
                to_big_decimal(fixed_notional.clone()).abs(),
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
        let user_perp_snap = context
            .get_user_perp(&user_perp_summary_key.clone())
            .await
            .clone();
        let (open_cost_diff, pnl_diff) = RealizedPnl::calc_realized_pnl(
            to_big_decimal(fixed_qty.clone()),
            to_big_decimal(fixed_notional.clone()),
            user_perp_snap.holding.clone(),
            user_perp_snap.opening_cost.clone(),
        );
        {
            let user_perp_summary = context.get_user_perp(&user_perp_summary_key).await;
            let (opening, new_user) = user_perp_summary.new_trade(
                to_big_decimal(fixed_fee.clone()),
                to_big_decimal(fixed_notional.clone()),
                pulled_block_height.clone(),
                pulled_block_time.clone(),
                open_cost_diff,
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
                block_hour: block_hour.clone(),
            };

            let hourly_user_perp = context.get_hourly_user_perp(&hourly_user_perp_key).await;
            let new_hourly_user = hourly_user_perp.new_trade(
                to_big_decimal(fixed_fee),
                to_big_decimal(fixed_notional),
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
