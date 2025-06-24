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
    insurance_fund_id: String,
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
        -fixed_position_transfer.clone(),
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
            pnl_diff.clone(),
        );
    }

    {
        let user_perp_summary = context.get_user_perp(&user_perp_key).await;
        user_perp_summary.charge_funding_fee(fixed_sum_unitary_fundings.clone(), block_num);

        user_perp_summary.new_user_adl_v1(
            fixed_adl_qty.clone(),
            adl_price.clone(),
            block_num,
            cpt.clone(),
            fixed_sum_unitary_fundings.clone(),
            open_cost_diff,
            pnl_diff.clone(),
        );

        let insurance_perp_key = UserPerpSummaryKey {
            account_id: insurance_fund_id.clone(),
            symbol: symbol_hash.clone(),
        };
        let insurance_perp_summary = context.get_user_perp(&insurance_perp_key).await;
        insurance_perp_summary.charge_funding_fee(fixed_sum_unitary_fundings.clone(), block_num);

        insurance_perp_summary.new_insurance_adl_v1(
            fixed_adl_qty,
            adl_price,
            block_num,
            cpt,
            fixed_sum_unitary_fundings,
        );
    }
}

pub async fn analyzer_adl_v2(
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
        -fixed_position_transfer.clone(),
        user_perp_snap.holding.clone(),
        user_perp_snap.opening_cost.clone(),
    );

    {
        let key =
            HourlyUserPerpKey::new_key(account_id.clone(), symbol_hash.clone(), block_hour.clone());
        let hourly_user_perp = context.get_hourly_user_perp(&key).await;
        hourly_user_perp.new_liquidation(
            fixed_adl_perice.clone() * fixed_adl_qty.clone(),
            block_num,
            pnl_diff.clone(),
        );
    }

    {
        let user_perp_summary = context.get_user_perp(&user_perp_key).await;
        user_perp_summary.charge_funding_fee(fixed_sum_unitary_fundings.clone(), block_num);

        user_perp_summary.new_user_adl_v2(
            fixed_adl_qty.clone(),
            adl_price.clone(),
            block_num,
            cpt.clone(),
            fixed_sum_unitary_fundings.clone(),
            open_cost_diff,
            pnl_diff,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{analyzer_adl, analyzer_adl_v2, AnalyzeContext, UserPerpSummaryKey};
    use crate::analyzer::{calc::USDC_HASH, tests::*};
    use bigdecimal::BigDecimal;
    use num_traits::FromPrimitive;
    use std::str::FromStr;

    const ALICE: &str = "0xa11ce00000000000000000000000000000000000000000000000000000000000";
    const BOB: &str = "0xb0b0000000000000000000000000000000000000000000000000000000000000";

    #[actix_web::test]
    async fn test_insurance_fund_adl_positive_position() {
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
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_BTC_USDC.to_string()),
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_ETH_USDC.to_string()),
        ];
        let account_tokens = vec![
            (ALICE.to_string(), TOKEN_HASH.to_string()),
            (ALICE.to_string(), USDC_HASH.to_string()),
            (BOB.to_string(), TOKEN_HASH.to_string()),
            (BOB.to_string(), USDC_HASH.to_string()),
            (INSURANCE_FUND.to_string(), TOKEN_HASH.to_string()),
            (INSURANCE_FUND.to_string(), USDC_HASH.to_string()),
        ];

        context.init_orderly_context(block_time, symbols, tokens, account_symbols, account_tokens);

        let bob_eth_perp_key = UserPerpSummaryKey {
            account_id: BOB.to_string(),
            symbol: SYMBOL_HASH_ETH_USDC.to_string(),
        };

        let insurance_eth_perp_key = UserPerpSummaryKey {
            account_id: INSURANCE_FUND.to_string(),
            symbol: SYMBOL_HASH_ETH_USDC.to_string(),
        };
        context.set_user_perp_cache(
            &insurance_eth_perp_key,
            BigDecimal::from_i128(2_000_000_000).unwrap(),
            BigDecimal::from_i128(100_000_000).unwrap(),
            BigDecimal::from_i128(10_000_000_000_000_000).unwrap(),
        );

        context.set_user_perp_cache(
            &bob_eth_perp_key,
            BigDecimal::from_i128(-1_000_000_000).unwrap(),
            BigDecimal::from_i128(100_000_000).unwrap(),
            BigDecimal::from_i128(10_000_000_000_000_000).unwrap(),
        );

        let block_num = 1000000;
        let block_hour = convert_block_hour(block_time);

        analyzer_adl(
            BOB.to_string(),
            INSURANCE_FUND.to_string(),
            SYMBOL_HASH_ETH_USDC.to_string(),
            1_000_000_000_i128.to_string(),
            100_000_000_i128.to_string(),
            30_000_000.to_string(),
            20_000_000_000_000_000_i128.to_string(),
            block_hour,
            block_num,
            &mut context,
        )
        .await;

        {
            let bob_eth = context.get_user_perp_cache(&bob_eth_perp_key);
            assert_eq!(bob_eth.holding, BigDecimal::from_str("0").unwrap());
            println!("bob eth perp summary: {:?}", bob_eth);
        }

        {
            let insurance_eth = context.get_user_perp_cache(&insurance_eth_perp_key);
            assert_eq!(insurance_eth.holding, BigDecimal::from_str("10").unwrap());
            println!("insurance eth perp summary: {:?}", insurance_eth);
        }
    }

    #[actix_web::test]
    async fn test_adl_v2() {
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
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_BTC_USDC.to_string()),
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_ETH_USDC.to_string()),
        ];
        let account_tokens = vec![
            (ALICE.to_string(), TOKEN_HASH.to_string()),
            (ALICE.to_string(), USDC_HASH.to_string()),
            (BOB.to_string(), TOKEN_HASH.to_string()),
            (BOB.to_string(), USDC_HASH.to_string()),
            (INSURANCE_FUND.to_string(), TOKEN_HASH.to_string()),
            (INSURANCE_FUND.to_string(), USDC_HASH.to_string()),
        ];

        context.init_orderly_context(block_time, symbols, tokens, account_symbols, account_tokens);

        let alice_btc_perp_key = UserPerpSummaryKey {
            account_id: ALICE.to_string(),
            symbol: SYMBOL_HASH_BTC_USDC.to_string(),
        };

        let bob_btc_perp_key = UserPerpSummaryKey {
            account_id: BOB.to_string(),
            symbol: SYMBOL_HASH_BTC_USDC.to_string(),
        };
        context.set_user_perp_cache(
            &alice_btc_perp_key,
            BigDecimal::from_i128(10_000_000).unwrap(),
            BigDecimal::from_i128(1_340_402).unwrap(),
            BigDecimal::from_i128(-623_710_000_000_000).unwrap(),
        );
        context.set_user_perp_cache(
            &bob_btc_perp_key,
            BigDecimal::from_i128(-10_000_000).unwrap(),
            BigDecimal::from_i128(-1_057_610).unwrap(),
            BigDecimal::from_i128(-97_240_000_000_000).unwrap(),
        );

        let block_num = 1000000;
        let block_hour = convert_block_hour(block_time);

        analyzer_adl_v2(
            ALICE.to_string(),
            SYMBOL_HASH_BTC_USDC.to_string(),
            (-10_000_000_i128).to_string(),
            (-1_070_650).to_string(),
            1_070_650_000_i128.to_string(),
            (-97_240_000_000_000_i128).to_string(),
            block_hour,
            block_num,
            &mut context,
        )
        .await;

        analyzer_adl_v2(
            BOB.to_string(),
            SYMBOL_HASH_BTC_USDC.to_string(),
            (10_000_000_i128).to_string(),
            (1_070_650).to_string(),
            1_070_650_000_i128.to_string(),
            (-97_240_000_000_000_i128).to_string(),
            block_hour,
            block_num,
            &mut context,
        )
        .await;

        {
            let alice_btc = context.get_user_perp_cache(&alice_btc_perp_key);
            println!(
                "alice eth perp alice_btc.holding: {:?}",
                alice_btc.holding.to_string()
            );
            assert_eq!(alice_btc.holding, BigDecimal::from_str("0").unwrap());
            println!("alice eth perp summary: {:?}", alice_btc);
        }

        {
            let bob_btc = context.get_user_perp_cache(&bob_btc_perp_key);
            assert_eq!(bob_btc.holding, BigDecimal::from_str("0").unwrap());
            println!("bob btc perp summary: {:?}", bob_btc);
        }
    }
}
