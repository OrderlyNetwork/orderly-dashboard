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
    pulled_block_height: i64,
    context: &mut AnalyzeContext,
) -> i64 {
    if trades.is_empty() {
        return 0;
    }
    let mut max_perp_trade_id = 0i64;
    let trade_len = trades.len();
    for perp_trade in trades {
        max_perp_trade_id = max(max_perp_trade_id, perp_trade.trade_id as i64);

        let trade_qty: BigDecimal = perp_trade.trade_qty.parse().unwrap();
        let fixed_qty = trade_qty.div(get_qty_prec());

        let trade_fee: BigDecimal = perp_trade.fee.parse().unwrap();
        let fixed_fee = trade_fee.div(get_cost_position_prec());

        let notional: BigDecimal = perp_trade.notional.parse().unwrap();
        let fixed_notional = notional.div(get_cost_position_prec());
        let quoted_diff = -fixed_notional.clone();
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
                perp_trade.side.clone(),
            );
        }
        let user_perp_summary_key = UserPerpSummaryKey {
            account_id: perp_trade.account_id.clone(),
            symbol: perp_symbol.clone(),
        };

        //user_summary
        let mut user_perp_summary = context
            .get_user_perp(&user_perp_summary_key.clone())
            .await
            .clone();

        let suf: BigDecimal = perp_trade.sum_unitary_fundings.parse().unwrap();

        // should no check and update log idx for charge funding fee
        user_perp_summary.charge_funding_fee(suf.div(get_unitary_prec()), pulled_block_height);
        let (open_cost_diff, pnl_diff) = RealizedPnl::calc_realized_pnl(
            fixed_qty.clone(),
            quoted_diff.clone(),
            user_perp_summary.holding.clone(),
            user_perp_summary.opening_cost.clone(),
        );
        {
            let (opening, new_user) = user_perp_summary.new_trade(
                fixed_fee.clone(),
                fixed_notional.clone(),
                pulled_block_height.clone(),
                open_cost_diff.clone(),
                fixed_qty.clone(),
                pnl_diff.clone(),
            );
            if opening {
                context
                    .get_hourly_orderly_perp(&hour_orderly_perp_key)
                    .await
                    .new_opening();
            }
            if new_user {
                context
                    .get_orderly_perp(&perp_symbol)
                    .await
                    .new_user(pulled_block_height);
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

    tracing::info!(target:PERP_ANALYZER,"handle trade length: {}, max_perp_trade_id: {}", trade_len, max_perp_trade_id);
    return max_perp_trade_id;
}

#[allow(deprecated)]
fn convert_block_hour(block_timestamp: i64) -> NaiveDateTime {
    let date_time = NaiveDateTime::from_timestamp_opt(block_timestamp / 1000, 0).unwrap();
    return date_time.with_second(0).unwrap().with_minute(0).unwrap();
}

#[cfg(test)]
mod tests {
    use super::{analyzer_perp_trade, AnalyzeContext, UserPerpSummaryKey};
    use crate::analyzer::{calc::USDC_HASH, tests::*};
    use bigdecimal::BigDecimal;

    use num_traits::FromPrimitive;
    use orderly_dashboard_indexer::formats_external::trading_events::PurchaseSide;
    use orderly_dashboard_indexer::formats_external::trading_events::Trade;

    const ALICE: &str = "0xa11ce00000000000000000000000000000000000000000000000000000000000";
    const BOB: &str = "0xb0b0000000000000000000000000000000000000000000000000000000000000";

    #[actix_web::test]
    async fn test_perp_trades() {
        let mut context = AnalyzeContext::new_context();
        let block_time = 1748416430;

        let symbols = vec![
            SYMBOL_HASH_BTC_USDC.to_string(),
            SYMBOL_HASH_ETH_USDC.to_string(),
        ];
        let tokens = vec![TOKEN_HASH.to_string(), USDC_HASH.to_string()];
        let account_symbols = vec![
            (ALICE.to_string(), SYMBOL_HASH_BTC_USDC.to_string()),
            (ALICE.to_string(), SYMBOL_HASH_ETH_USDC.to_string()),
            (BOB.to_string(), SYMBOL_HASH_BTC_USDC.to_string()),
            (BOB.to_string(), SYMBOL_HASH_ETH_USDC.to_string()),
        ];
        let account_tokens = vec![
            (ALICE.to_string(), TOKEN_HASH.to_string()),
            (ALICE.to_string(), USDC_HASH.to_string()),
            (BOB.to_string(), TOKEN_HASH.to_string()),
            (BOB.to_string(), USDC_HASH.to_string()),
        ];

        context.init_orderly_context(block_time, symbols, tokens, account_symbols, account_tokens);
        let alice_eth_perp_key = UserPerpSummaryKey {
            account_id: ALICE.to_string(),
            symbol: SYMBOL_HASH_ETH_USDC.to_string(),
        };

        let bob_eth_perp_key = UserPerpSummaryKey {
            account_id: BOB.to_string(),
            symbol: SYMBOL_HASH_ETH_USDC.to_string(),
        };
        context.set_user_perp_cache(
            &alice_eth_perp_key,
            BigDecimal::from_i128(0).unwrap(),
            BigDecimal::from_i128(0).unwrap(),
            BigDecimal::from_i128(0).unwrap(),
        );
        context.set_user_perp_cache(
            &bob_eth_perp_key,
            BigDecimal::from_i128(0).unwrap(),
            BigDecimal::from_i128(0).unwrap(),
            BigDecimal::from_i128(0).unwrap(),
        );

        let trades = vec![
            Trade {
                account_id: ALICE.to_string(),
                symbol_hash: SYMBOL_HASH_ETH_USDC.to_string(),
                fee_asset_hash: TOKEN_HASH.to_string(),
                trade_qty: "-200000000".to_string(),
                notional: "-5000000".to_string(),
                executed_price: "250000000".to_string(),
                fee: "7500".to_string(),
                sum_unitary_fundings: "1000000000000001".to_string(),
                trade_id: 13,
                match_id: 1678975214714862536,
                timestamp: (block_time * 1000) as u64,
                side: PurchaseSide::Sell,
            },
            Trade {
                account_id: BOB.to_string(),
                symbol_hash: SYMBOL_HASH_ETH_USDC.to_string(),
                fee_asset_hash: TOKEN_HASH.to_string(),
                trade_qty: "200000000".to_string(),
                notional: "5000000".to_string(),
                executed_price: "250000000".to_string(),
                fee: "5000".to_string(),
                sum_unitary_fundings: "1000000000000001".to_string(),
                trade_id: 14,
                match_id: 1678975214714862536,
                timestamp: (block_time * 1000) as u64,
                side: PurchaseSide::Buy,
            },
        ];
        let block_number = 1000000;
        analyzer_perp_trade(trades, block_number, &mut context).await;
        {
            let alice_eth = context.get_user_perp_cache(&alice_eth_perp_key);
            assert_eq!(alice_eth.holding, BigDecimal::from(-2));
            println!("alice_eth perp summary: {:?}", alice_eth);
        }

        {
            let bob_eth = context.get_user_perp_cache(&bob_eth_perp_key);
            assert_eq!(bob_eth.holding, BigDecimal::from(2));
            println!("bob_eth perp summary: {:?}", bob_eth);
        }
    }
}
