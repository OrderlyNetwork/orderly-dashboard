use std::ops::Div;
use std::ops::Neg;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use orderly_dashboard_indexer::formats_external::trading_events::{
    LiquidationTransfer, LiquidationTransferV2,
};

use crate::analyzer::calc::pnl_calc::RealizedPnl;
use crate::analyzer::calc::{USDC_CHAIN_ID, USDC_HASH};
use crate::analyzer::{analyzer_context::AnalyzeContext, INSURANCE_FUNDS};
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
    pub liquidation_fee: BigDecimal,
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
        let liquidation_fee: BigDecimal = event.liquidation_fee.parse().unwrap();
        let insurance_fee: BigDecimal = event.insurance_fee.parse().unwrap();
        let mark_price: BigDecimal = event.mark_price.parse().unwrap();
        let sum_unitary_funding: BigDecimal = event.sum_unitary_fundings.parse().unwrap();

        let fixed_qty = liquidation_qty.div(get_qty_prec());
        let fixed_cost_p_transfer = cost_position_transfer.div(get_cost_position_prec());
        let fixed_liquidator_fee = liquidator_fee.div(get_cost_position_prec());
        let fixed_liquidation_fee = liquidation_fee.div(get_cost_position_prec());
        let fixed_insurance_fee = insurance_fee.div(get_cost_position_prec());
        let fixed_mark_price = mark_price.div(get_price_prec());
        let fixed_sum_unitary_fundings = sum_unitary_funding.div(get_unitary_prec());

        Liquidation {
            liquidated_account_id,
            liquidator_account_id: event.liquidator_account_id,
            symbol_hash: event.symbol_hash,
            qty_transfer: fixed_qty,
            cost_position_transfer: fixed_cost_p_transfer,
            liquidation_fee: fixed_liquidation_fee,
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
            liquidation_fee: fixed_liquidation_fee,
            liquidator_fee: 0.into(),
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
        block_num,
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

        execute_for_liquidation_v2(&liquidation, context).await;
        statistics_for_orderly(&liquidation, context).await;
    }
}

pub async fn analyzer_liquidation_v1(
    liquidated_account_id: String,
    _insurance_account_id: String,
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
        block_num,
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
    );

    let orderly_perp = context
        .get_orderly_perp(&liquidation.symbol_hash.clone())
        .await;
    orderly_perp.new_liquidation(
        liquidation.qty_transfer.clone() * liquidation.mark_price.clone(),
        liquidation.block_num,
    );
}

async fn execute_for_liquidator(liquidation: &Liquidation, context: &mut AnalyzeContext) {
    let liquidator_key = UserPerpSummaryKey::new_key(
        liquidation.liquidator_account_id.clone(),
        liquidation.symbol_hash.clone(),
    );

    let user_perp = context.get_user_perp(&liquidator_key.clone()).await;
    user_perp.charge_funding_fee(
        liquidation.sum_unitry_funding.clone(),
        liquidation.block_num,
    );

    let user_perp_snap = user_perp.clone();

    let need_cal_avg = !INSURANCE_FUNDS.contains(&liquidation.liquidator_account_id.as_str());
    let (liquidator_open_cost_diff, liquidator_pnl_diff) = if need_cal_avg {
        RealizedPnl::calc_realized_pnl(
            liquidation.qty_transfer.clone(),
            -(liquidation.cost_position_transfer.clone() - liquidation.liquidator_fee.clone()),
            user_perp_snap.holding.clone(),
            user_perp_snap.opening_cost.clone(),
        )
    } else {
        (BigDecimal::from(0), BigDecimal::from(0))
    };

    let user_perp = context.get_user_perp(&liquidator_key.clone()).await;
    user_perp.new_liquidator(
        liquidation.qty_transfer.clone(),
        liquidation.cost_position_transfer.clone(),
        liquidation.liquidator_fee.clone(),
        liquidator_open_cost_diff.clone(),
        liquidation.block_num,
        liquidator_pnl_diff.clone(),
        need_cal_avg,
    );

    let h_perp_key = HourlyUserPerpKey::new_key(
        liquidation.liquidator_account_id.clone(),
        liquidation.symbol_hash.clone(),
        liquidation.block_hour.clone(),
    );

    let h_user_perp = context.get_hourly_user_perp(&h_perp_key).await;
    h_user_perp.new_realized_pnl(liquidator_pnl_diff.clone(), liquidation.block_num);
}

async fn execute_for_liquidated(liquidation: &Liquidation, context: &mut AnalyzeContext) {
    let key = UserPerpSummaryKey::new_key(
        liquidation.liquidated_account_id.clone(),
        liquidation.symbol_hash.clone(),
    );

    let user_perp = context.get_user_perp(&key.clone()).await;
    user_perp.charge_funding_fee(
        liquidation.sum_unitry_funding.clone(),
        liquidation.block_num,
    );

    let user_perp_snap = user_perp.clone();

    let need_cal_avg = !INSURANCE_FUNDS.contains(&liquidation.liquidated_account_id.as_str());
    let (open_cost_diff, pnl_diff) = if need_cal_avg {
        RealizedPnl::calc_realized_pnl(
            -liquidation.qty_transfer.clone(),
            liquidation.cost_position_transfer.clone()
                - (liquidation.liquidator_fee.clone() + liquidation.insurnace_fee.clone()),
            user_perp_snap.holding.clone(),
            user_perp_snap.opening_cost.clone(),
        )
    } else {
        (BigDecimal::from(0), BigDecimal::from(0))
    };

    let user_perp = context.get_user_perp(&key.clone()).await;
    user_perp.new_liquidation(
        liquidation.qty_transfer.clone(),
        liquidation.mark_price.clone(),
        liquidation.block_num,
        liquidation.cost_position_transfer.clone(),
        liquidation.liquidation_fee.clone(),
        liquidation.sum_unitry_funding.clone(),
        open_cost_diff.clone(),
        pnl_diff.clone(),
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
        pnl_diff.clone(),
    );
}

async fn execute_for_liquidation_v2(liquidation: &Liquidation, context: &mut AnalyzeContext) {
    let key = UserPerpSummaryKey::new_key(
        liquidation.liquidated_account_id.clone(),
        liquidation.symbol_hash.clone(),
    );

    let user_perp = context.get_user_perp(&key.clone()).await;
    user_perp.charge_funding_fee(
        liquidation.sum_unitry_funding.clone(),
        liquidation.block_num,
    );

    let user_perp_snap = user_perp.clone();
    let quoted_diff = -liquidation.cost_position_transfer.clone()
        - liquidation.liquidation_fee.clone()
        - liquidation.insurnace_fee.clone();
    #[cfg(test)]
    println!(
        "execute_for_liquidation_v2 key: {:?}, qty_transfer: {}, quoted_diff: {}, total_realized_pnl: {}, holding: {}, opening_cost: {}", 
        key, liquidation.qty_transfer.to_string(), quoted_diff.to_string(), user_perp_snap.total_realized_pnl.to_string(), user_perp_snap.holding.to_string(),  user_perp_snap.opening_cost.to_string(),
    );

    let need_cal_avg = !INSURANCE_FUNDS.contains(&liquidation.liquidated_account_id.as_str());
    let (open_cost_diff, pnl_diff) = if need_cal_avg {
        RealizedPnl::calc_realized_pnl(
            liquidation.qty_transfer.clone(),
            quoted_diff,
            user_perp_snap.holding.clone(),
            user_perp_snap.opening_cost.clone(),
        )
    } else {
        (BigDecimal::from(0), BigDecimal::from(0))
    };
    #[cfg(test)]
    println!(
        "execute_for_liquidation_v2 open_cost_diff: {:?}, pnl_diff: {}",
        open_cost_diff.to_string(),
        pnl_diff.to_string()
    );

    let user_perp = context.get_user_perp(&key.clone()).await;
    user_perp.new_liquidation_v2(
        liquidation.qty_transfer.clone(),
        liquidation.mark_price.clone(),
        liquidation.block_num,
        liquidation.cost_position_transfer.clone(),
        liquidation.liquidation_fee.clone(),
        liquidation.sum_unitry_funding.clone(),
        open_cost_diff.clone(),
        pnl_diff.clone(),
        need_cal_avg,
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
        pnl_diff.clone(),
    );
}

async fn transfer_insurance_fund(
    insurance_transfer_amount: &String,
    context: &mut AnalyzeContext,
    liquidated_account_id: String,
    p_block_height: i64,
) {
    let need_transfer_amount: BigDecimal = insurance_transfer_amount
        .parse()
        .unwrap_or(BigDecimal::from(0));
    if need_transfer_amount != BigDecimal::from(0) {
        let fixed_amount = need_transfer_amount.div(get_cost_position_prec());
        {
            transfer_amount(
                liquidated_account_id.clone(),
                context,
                fixed_amount.clone(),
                p_block_height,
            )
            .await;
        }
    }
}

async fn transfer_amount(
    account_id: String,
    context: &mut AnalyzeContext,
    amount: BigDecimal,
    p_block_height: i64,
) {
    let key = UserTokenSummaryKey {
        account_id: account_id.clone(),
        token: String::from(USDC_HASH),
        chain_id: String::from(USDC_CHAIN_ID),
    };

    let user_token = context.get_user_token(&key).await;
    if p_block_height < user_token.pulled_block_height {
        return;
    }
    user_token.add_amount(amount.abs().neg(), p_block_height);
}

#[cfg(test)]
mod tests {
    use crate::{
        analyzer::{analyzer_job::parse_and_analyzer, tests::*},
        db::user_perp_summary::UserPerpSummary,
    };
    use bigdecimal::BigDecimal;
    use chrono::NaiveDateTime;
    use num_traits::FromPrimitive;
    use orderly_dashboard_indexer::formats_external::{
        trading_events::TradingEventsResponse, IndexerQueryResponse, SuccessResponse,
    };
    use std::str::FromStr;

    use super::{
        analyzer_liquidation_v1, analyzer_liquidation_v2, AnalyzeContext, LiquidationTransfer,
        LiquidationTransferV2, UserPerpSummaryKey, USDC_HASH,
    };

    #[actix_web::test]
    async fn test_liquidation_v1() {
        let mut context = AnalyzeContext::new_context();
        let block_time = 1748416430;

        let symbols = vec![
            SYMBOL_HASH_BTC_USDC.to_string(),
            SYMBOL_HASH_ETH_USDC.to_string(),
        ];
        let tokens = vec![TOKEN_HASH.to_string(), USDC_HASH.to_string()];
        let account_symbols = vec![
            (
                LIQUIDATED_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_BTC_USDC.to_string(),
            ),
            (
                LIQUIDATED_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_ETH_USDC.to_string(),
            ),
            (
                LIQUIDATOR_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_BTC_USDC.to_string(),
            ),
            (
                LIQUIDATOR_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_ETH_USDC.to_string(),
            ),
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_BTC_USDC.to_string()),
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_ETH_USDC.to_string()),
        ];
        let account_tokens = vec![
            (LIQUIDATED_ACCOUNT_ID.to_string(), TOKEN_HASH.to_string()),
            (LIQUIDATED_ACCOUNT_ID.to_string(), USDC_HASH.to_string()),
            (LIQUIDATOR_ACCOUNT_ID.to_string(), TOKEN_HASH.to_string()),
            (LIQUIDATOR_ACCOUNT_ID.to_string(), USDC_HASH.to_string()),
            (INSURANCE_FUND.to_string(), TOKEN_HASH.to_string()),
            (INSURANCE_FUND.to_string(), USDC_HASH.to_string()),
        ];

        context.init_orderly_context(block_time, symbols, tokens, account_symbols, account_tokens);
        let liquidated_btc_perp_key = UserPerpSummaryKey {
            account_id: LIQUIDATED_ACCOUNT_ID.to_string(),
            symbol: SYMBOL_HASH_BTC_USDC.to_string(),
        };
        let liquidated_eth_perp_key = UserPerpSummaryKey {
            account_id: LIQUIDATED_ACCOUNT_ID.to_string(),
            symbol: SYMBOL_HASH_ETH_USDC.to_string(),
        };
        let liquidator_btc_perp_key = UserPerpSummaryKey {
            account_id: LIQUIDATOR_ACCOUNT_ID.to_string(),
            symbol: SYMBOL_HASH_BTC_USDC.to_string(),
        };
        let liquidator_eth_perp_key = UserPerpSummaryKey {
            account_id: LIQUIDATOR_ACCOUNT_ID.to_string(),
            symbol: SYMBOL_HASH_ETH_USDC.to_string(),
        };
        context.set_user_perp_cache(
            &liquidated_btc_perp_key,
            BigDecimal::from_i128(-400000000).unwrap(),
            BigDecimal::from_i128(-8000000).unwrap(),
            BigDecimal::from_i128(2000000000000002).unwrap(),
            0.into(),
        );
        context.set_user_perp_cache(
            &liquidated_eth_perp_key,
            BigDecimal::from_i128(200000000).unwrap(),
            BigDecimal::from_i128(3600000000).unwrap(),
            BigDecimal::from_i128(2000000000000002).unwrap(),
            0.into(),
        );

        let liquidation_transfers = vec![
            LiquidationTransfer {
                liquidation_transfer_id: 100.to_string(),
                liquidator_account_id: LIQUIDATOR_ACCOUNT_ID.to_string(),
                symbol_hash: SYMBOL_HASH_BTC_USDC.to_string(),
                position_qty_transfer: (-200000000).to_string(),
                cost_position_transfer: (-4000000).to_string(),
                liquidator_fee: 15000.to_string(),
                insurance_fee: 15000.to_string(),
                liquidation_fee: 30000.to_string(),
                mark_price: 200000000.to_string(),
                sum_unitary_fundings: "3000000000000003".to_string(),
            },
            LiquidationTransfer {
                liquidation_transfer_id: 101.to_string(),
                liquidator_account_id: LIQUIDATOR_ACCOUNT_ID.to_string(),
                symbol_hash: SYMBOL_HASH_ETH_USDC.to_string(),
                position_qty_transfer: 100000000.to_string(),
                cost_position_transfer: 1800000000.to_string(),
                liquidator_fee: 18000000.to_string(),
                insurance_fee: 18000000.to_string(),
                liquidation_fee: 36000000.to_string(),
                mark_price: "180000000000".to_string(),
                sum_unitary_fundings: "3000000000000003".to_string(),
            },
        ];

        let block_number = 1000000;
        let block_hour = convert_block_hour(block_time);
        let block_time = NaiveDateTime::from_timestamp_opt(block_time, 0).unwrap();

        analyzer_liquidation_v1(
            LIQUIDATED_ACCOUNT_ID.to_string(),
            "insurance_fund".to_string(),
            USDC_HASH.to_string(),
            0.to_string(),
            liquidation_transfers,
            block_number,
            block_hour,
            block_time,
            &mut context,
        )
        .await;

        {
            let liquidated_btc = context.get_user_perp_cache(&liquidated_btc_perp_key);
            println!(
                "liquidated_btc perp summary: {:?}, holding: {}, cost_position: {}",
                liquidated_btc,
                liquidated_btc.holding.to_string(),
                liquidated_btc.cost_position.to_string()
            );
            // assertEq(LiquidatedBtcPosition.positionQty, -200000000);
            assert_eq!(liquidated_btc.holding, BigDecimal::from(-2));
            assert_eq!(
                liquidated_btc.cost_position,
                BigDecimal::from_str("-7.97").unwrap()
            );
        }

        {
            let liquidated_eth = context.get_user_perp_cache(&liquidated_eth_perp_key);
            assert_eq!(liquidated_eth.holding, BigDecimal::from(1));
            println!("liquidated eth perp summary: {:?}", liquidated_eth);
        }

        {
            let liquidator_btc = context.get_user_perp_cache(&liquidator_btc_perp_key);
            assert_eq!(liquidator_btc.holding, BigDecimal::from(-2));
            assert_eq!(
                liquidator_btc.cost_position,
                BigDecimal::from_str("-4.015").unwrap()
            );
            println!("liquidator btc perp summary: {:?}", liquidator_btc);
        }

        {
            let liquidator_eth = context.get_user_perp_cache(&liquidator_eth_perp_key);
            assert_eq!(liquidator_eth.holding, BigDecimal::from(1));
            println!("liquidator eth perp summary: {:?}", liquidator_eth);
        }
    }

    #[actix_web::test]
    async fn test_liquidation_v2_1() {
        let mut context = AnalyzeContext::new_context();
        let block_time = 1748416430;

        let symbols = vec![
            SYMBOL_HASH_BTC_USDC.to_string(),
            SYMBOL_HASH_ETH_USDC.to_string(),
        ];
        let tokens = vec![TOKEN_HASH.to_string(), USDC_HASH.to_string()];
        let account_symbols = vec![
            (
                LIQUIDATED_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_BTC_USDC.to_string(),
            ),
            (
                LIQUIDATED_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_ETH_USDC.to_string(),
            ),
            (
                LIQUIDATOR_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_BTC_USDC.to_string(),
            ),
            (
                LIQUIDATOR_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_ETH_USDC.to_string(),
            ),
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_BTC_USDC.to_string()),
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_ETH_USDC.to_string()),
        ];
        let account_tokens = vec![
            (LIQUIDATED_ACCOUNT_ID.to_string(), TOKEN_HASH.to_string()),
            (LIQUIDATED_ACCOUNT_ID.to_string(), USDC_HASH.to_string()),
            (LIQUIDATOR_ACCOUNT_ID.to_string(), TOKEN_HASH.to_string()),
            (LIQUIDATOR_ACCOUNT_ID.to_string(), USDC_HASH.to_string()),
            (INSURANCE_FUND.to_string(), TOKEN_HASH.to_string()),
            (INSURANCE_FUND.to_string(), USDC_HASH.to_string()),
        ];

        context.init_orderly_context(block_time, symbols, tokens, account_symbols, account_tokens);
        let liquidated_btc_perp_key = UserPerpSummaryKey {
            account_id: LIQUIDATED_ACCOUNT_ID.to_string(),
            symbol: SYMBOL_HASH_BTC_USDC.to_string(),
        };
        let liquidator_btc_perp_key = UserPerpSummaryKey {
            account_id: LIQUIDATOR_ACCOUNT_ID.to_string(),
            symbol: SYMBOL_HASH_BTC_USDC.to_string(),
        };
        let insurance_btc_perp_key = UserPerpSummaryKey {
            account_id: INSURANCE_FUND.to_string(),
            symbol: SYMBOL_HASH_BTC_USDC.to_string(),
        };
        context.set_user_perp_cache(
            &liquidated_btc_perp_key,
            BigDecimal::from_i128(-289_000_000).unwrap(),
            BigDecimal::from_i128(-435_175_502).unwrap(),
            BigDecimal::from_i128(40_331_900_000_000_000).unwrap(),
            BigDecimal::from_i128(100_000_000).unwrap(),
        );
        context.set_user_perp_cache(
            &liquidator_btc_perp_key,
            BigDecimal::from_i128(4_059_000_000).unwrap(),
            BigDecimal::from_i128(-2_533_031_349).unwrap(),
            BigDecimal::from_i128(40_376_000_000_000_000).unwrap(),
            0.into(),
        );
        context.set_user_perp_cache(
            &insurance_btc_perp_key,
            BigDecimal::from_i128(0).unwrap(),
            BigDecimal::from_i128(-142_229_490).unwrap(),
            BigDecimal::from_i128(40_376_000_000_000_000).unwrap(),
            0.into(),
        );

        let liquidation_transfers1 = vec![LiquidationTransferV2 {
            account_id: LIQUIDATED_ACCOUNT_ID.to_string(),
            symbol_hash: SYMBOL_HASH_BTC_USDC.to_string(),
            position_qty_transfer: (254_000_000).to_string(),
            cost_position_transfer: (402_465_540).to_string(),
            fee: 14_086_294.to_string(),
            mark_price: "15845100000".to_string(),
            sum_unitary_fundings: "40376000000000000".to_string(),
        }];
        let liquidation_transfers2 = vec![LiquidationTransferV2 {
            account_id: LIQUIDATOR_ACCOUNT_ID.to_string(),
            symbol_hash: SYMBOL_HASH_BTC_USDC.to_string(),
            position_qty_transfer: (-254_000_000).to_string(),
            cost_position_transfer: (-402_465_540).to_string(),
            fee: (-7_043_147).to_string(),
            mark_price: "15845100000".to_string(),
            sum_unitary_fundings: "40376000000000000".to_string(),
        }];
        let liquidation_transfers3 = vec![LiquidationTransferV2 {
            account_id: INSURANCE_FUND.to_string(),
            symbol_hash: SYMBOL_HASH_ETH_USDC.to_string(),
            position_qty_transfer: 0.to_string(),
            cost_position_transfer: 0.to_string(),
            fee: (-7_043_147).to_string(),
            mark_price: "15845100000".to_string(),
            sum_unitary_fundings: "40376000000000000".to_string(),
        }];

        let block_number = 1000000;
        let block_hour = convert_block_hour(block_time);
        let block_time = NaiveDateTime::from_timestamp_opt(block_time, 0).unwrap();

        analyzer_liquidation_v2(
            LIQUIDATED_ACCOUNT_ID.to_string(),
            USDC_HASH.to_string(),
            0.to_string(),
            liquidation_transfers1,
            block_number,
            block_hour,
            block_time,
            &mut context,
        )
        .await;

        analyzer_liquidation_v2(
            LIQUIDATOR_ACCOUNT_ID.to_string(),
            USDC_HASH.to_string(),
            0.to_string(),
            liquidation_transfers2,
            block_number,
            block_hour,
            block_time,
            &mut context,
        )
        .await;

        analyzer_liquidation_v2(
            INSURANCE_FUND.to_string(),
            USDC_HASH.to_string(),
            0.to_string(),
            liquidation_transfers3,
            block_number,
            block_hour,
            block_time,
            &mut context,
        )
        .await;

        {
            let liquidated_btc = context.get_user_perp_cache(&liquidated_btc_perp_key);
            assert_eq!(
                liquidated_btc.holding,
                BigDecimal::from_str("-0.35000000").unwrap()
            );
            assert_eq!(
                liquidated_btc.cost_position,
                BigDecimal::from_str("-18.751117").unwrap()
            );
        }

        {
            let liquidator_btc = context.get_user_perp_cache(&liquidator_btc_perp_key);
            assert_eq!(
                liquidator_btc.holding,
                BigDecimal::from_str("38.05000000").unwrap()
            );
            println!("liquidator eth perp summary: {:?}", liquidator_btc);
        }

        {
            let insurance_btc = context.get_user_perp_cache(&insurance_btc_perp_key);
            assert_eq!(insurance_btc.holding, BigDecimal::from(0));
            println!("insurance btc perp summary: {:?}", insurance_btc);
        }
    }

    const TRADE_AND_LIQ_DATA: &str = r#"
{
    "success": true,
    "data": {
        "events": [
            {
                "block_number": 29607613,
                "transaction_index": 2,
                "log_index": 7,
                "transaction_id": "0x444d3119bf970bf8f6febe879eb4a7103266831a64bc3ee4afc2dba62936ffe9",
                "block_timestamp": 1750747598,
                "data": {
                    "ProcessedTrades": {
                        "batch_id": 544501,
                        "trades": [
                            {
                                "account_id": "0xa8fbdad84f63facd3363563445137ae903f41ffdab304d24f1db95c7ba2cfe84",
                                "symbol_hash": "0x7e83089239db756ee233fa8972addfea16ae653db0f692e4851aed546b21caeb",
                                "fee_asset_hash": "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa",
                                "trade_qty": "870000",
                                "notional": "21078012",
                                "executed_price": "242276000000",
                                "fee": "12647",
                                "sum_unitary_fundings": "1831480000000000000",
                                "trade_id": 10076132133,
                                "match_id": 1750745787747772649,
                                "timestamp": 1750745787747,
                                "side": "BUY"
                            },
                            {
                                "account_id": "0xa8fbdad84f63facd3363563445137ae903f41ffdab304d24f1db95c7ba2cfe84",
                                "symbol_hash": "0x7e83089239db756ee233fa8972addfea16ae653db0f692e4851aed546b21caeb",
                                "fee_asset_hash": "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa",
                                "trade_qty": "-870000",
                                "notional": "-20999973",
                                "executed_price": "241379000000",
                                "fee": "12600",
                                "sum_unitary_fundings": "1831480000000000000",
                                "trade_id": 10076132134,
                                "match_id": 1750745906031410525,
                                "timestamp": 1750745906031,
                                "side": "SELL"
                            }
                        ]
                    }
                }
            },
            {
                "block_number": 29643682,
                "transaction_index": 1,
                "log_index": 3,
                "transaction_id": "0xbea889aa594cce839fc158eecaa028a5c260880ddb6eb0af6714716d66ea006d",
                "block_timestamp": 1750819736,
                "data": {
                    "ProcessedTrades": {
                        "batch_id": 544666,
                        "trades": [
                            {
                                "account_id": "0xa8fbdad84f63facd3363563445137ae903f41ffdab304d24f1db95c7ba2cfe84",
                                "symbol_hash": "0x7e83089239db756ee233fa8972addfea16ae653db0f692e4851aed546b21caeb",
                                "fee_asset_hash": "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa",
                                "trade_qty": "2000000",
                                "notional": "49334400",
                                "executed_price": "246672000000",
                                "fee": "29601",
                                "sum_unitary_fundings": "1832200000000000000",
                                "trade_id": 10076132861,
                                "match_id": 1750817981088355795,
                                "timestamp": 1750817981088,
                                "side": "BUY"
                            },
                            {
                                "account_id": "0xa8fbdad84f63facd3363563445137ae903f41ffdab304d24f1db95c7ba2cfe84",
                                "symbol_hash": "0x7e83089239db756ee233fa8972addfea16ae653db0f692e4851aed546b21caeb",
                                "fee_asset_hash": "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa",
                                "trade_qty": "16050000",
                                "notional": "396048195",
                                "executed_price": "246759000000",
                                "fee": "237629",
                                "sum_unitary_fundings": "1832200000000000000",
                                "trade_id": 10076132862,
                                "match_id": 1750817981088372622,
                                "timestamp": 1750817981088,
                                "side": "BUY"
                            }
                        ]
                    }
                }
            },
            {
                "block_number": 29647199,
                "transaction_index": 3,
                "log_index": 7,
                "transaction_id": "0x1a5d656bcb01a5d145f94d67684c233316defda4540a53d5f88f1c1cdb6b9d09",
                "block_timestamp": 1750826770,
                "data": {
                    "LiquidationResultV2": {
                        "account_id": "0xa8fbdad84f63facd3363563445137ae903f41ffdab304d24f1db95c7ba2cfe84",
                        "liquidated_asset_hash": "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa",
                        "insurance_transfer_amount": "0",
                        "liquidation_transfers": [
                            {
                                "account_id": "0xa8fbdad84f63facd3363563445137ae903f41ffdab304d24f1db95c7ba2cfe84",
                                "symbol_hash": "0x7e83089239db756ee233fa8972addfea16ae653db0f692e4851aed546b21caeb",
                                "position_qty_transfer": "-18050000",
                                "cost_position_transfer": "-438726910",
                                "fee": "2545055",
                                "mark_price": "243062000000",
                                "sum_unitary_fundings": "1832200000000000000"
                            }
                        ]
                    }
                }
            }
        ],
        "last_block": 29647199,
        "last_block_timestamp": 1750826770,
        "next_offset": null
    }
}
        "#;

    #[ignore]
    #[actix_web::test]
    async fn test_liquidation_with_data() {
        let result: IndexerQueryResponse<TradingEventsResponse> =
            serde_json::from_str(TRADE_AND_LIQ_DATA).unwrap();
        let mut result_data = IndexerQueryResponse::<TradingEventsResponse>::empty_success();
        println!("result: {:?}", result);
        match &result {
            IndexerQueryResponse::Success(data) => {
                println!(
                    "result data length: {:?}",
                    data.as_data().clone().unwrap().events.len()
                );
                let mut data = data.as_data().cloned().unwrap();
                // data.events.remove(2);
                result_data = IndexerQueryResponse::Success(SuccessResponse::new(data))
            }
            _ => {}
        }
        let mut context = AnalyzeContext::new_context();
        parse_and_analyzer(result_data, &mut context).await;
    }

    const TRADE_AND_LIQ_DATA2: &str = r#"
{
    "success": true,
    "data": {
        "events": [
            {
                "block_number": 2247044,
                "transaction_index": 1,
                "log_index": 0,
                "transaction_id": "0x3c60a58338eb779cb6a12715918cc8af034fdcea8ae2dc8fbf775f2ff7479d6b",
                "block_timestamp": 1696026460,
                "data": {
                    "ProcessedTrades": {
                        "batch_id": 36454,
                        "trades": [
                            {
                                "account_id": "0x199265c7b18b200fbe2fb5813ca524e1762be91fcd44f19ce40f998b016426fe",
                                "symbol_hash": "0x5a8133e52befca724670dbf2cade550c522c2410dd5b1189df675e99388f509d",
                                "fee_asset_hash": "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa",
                                "trade_qty": "-27000",
                                "notional": "-7252740",
                                "executed_price": "2686200000000",
                                "fee": "5803",
                                "sum_unitary_fundings": "787400000000000000",
                                "trade_id": 498153,
                                "match_id": 1696026448099928000,
                                "timestamp": 1696026460000,
                                "side": "SELL"
                            },
                            {
                                "account_id": "0x199265c7b18b200fbe2fb5813ca524e1762be91fcd44f19ce40f998b016426fe",
                                "symbol_hash": "0x5a8133e52befca724670dbf2cade550c522c2410dd5b1189df675e99388f509d",
                                "fee_asset_hash": "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa",
                                "trade_qty": "27000",
                                "notional": "7252740",
                                "executed_price": "2686200000000",
                                "fee": "2902",
                                "sum_unitary_fundings": "787400000000000000",
                                "trade_id": 498155,
                                "match_id": 1696026448099928000,
                                "timestamp": 1696026460000,
                                "side": "BUY"
                            },
                            {
                                "account_id": "0x199265c7b18b200fbe2fb5813ca524e1762be91fcd44f19ce40f998b016426fe",
                                "symbol_hash": "0x5a8133e52befca724670dbf2cade550c522c2410dd5b1189df675e99388f509d",
                                "fee_asset_hash": "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa",
                                "trade_qty": "-3000",
                                "notional": "-805842",
                                "executed_price": "2686140000000",
                                "fee": "645",
                                "sum_unitary_fundings": "787400000000000000",
                                "trade_id": 498156,
                                "match_id": 1696026448100051966,
                                "timestamp": 1696026460000,
                                "side": "SELL"
                            }
                        ]
                    }
                }
            },
            {
                "block_number": 3300687,
                "transaction_index": 1,
                "log_index": 3,
                "transaction_id": "0x77ccdb6effe618df3cb4c781fa4f8bc56429788b873c4c463d9e25961711f8ac",
                "block_timestamp": 1696026460,
                "data": {
                    "LiquidationResult": {
                        "liquidated_account_id": "0x199265c7b18b200fbe2fb5813ca524e1762be91fcd44f19ce40f998b016426fe",
                        "insurance_account_id": "0xe042ae0d7f1cb85245360af73a383d643e43401f64fa56c2c072dbbf200554d7",
                        "liquidated_asset_hash": "0x199265c7b18b200fbe2fb5813ca524e1762be91fcd44f19ce40f998b016426fe",
                        "insurance_transfer_amount": "1000000000",
                        "liquidation_transfers": [
                            {
                                "liquidation_transfer_id": "72678",
                                "liquidator_account_id": "0xe042ae0d7f1cb85245360af73a383d643e43401f64fa56c2c072dbbf200554d7",
                                "symbol_hash": "0x5a8133e52befca724670dbf2cade550c522c2410dd5b1189df675e99388f509d",
                                "position_qty_transfer": "-3000",
                                "cost_position_transfer": "-798934",
                                "liquidator_fee": "0",
                                "insurance_fee": "0",
                                "liquidation_fee": "0",
                                "mark_price": "3444430000000",
                                "sum_unitary_fundings": "868800000000000000"
                            }
                        ]
                    }
                }
            }
        ],
        "last_block": 29647199,
        "last_block_timestamp": 1696026460,
        "next_offset": null
    }
}
            "#;

    #[actix_web::test]
    async fn test_trade_and_liquidation_with_data() {
        let mut context = AnalyzeContext::new_context();
        let block_time = 1696026460;

        const P_ACCOUNT_ID: &str =
            "0x199265c7b18b200fbe2fb5813ca524e1762be91fcd44f19ce40f998b016426fe";
        const P_LIQUIDATOR_ID: &str =
            "0xe042ae0d7f1cb85245360af73a383d643e43401f64fa56c2c072dbbf200554d7";

        const SYMBOL: &str = "0x5a8133e52befca724670dbf2cade550c522c2410dd5b1189df675e99388f509d";

        let symbols = vec![
            SYMBOL.to_string(),
            SYMBOL_HASH_BTC_USDC.to_string(),
            SYMBOL_HASH_ETH_USDC.to_string(),
        ];
        let tokens = vec![TOKEN_HASH.to_string(), USDC_HASH.to_string()];
        let account_symbols = vec![
            (P_ACCOUNT_ID.to_string(), SYMBOL.to_string()),
            (P_LIQUIDATOR_ID.to_string(), SYMBOL.to_string()),
            (
                LIQUIDATED_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_BTC_USDC.to_string(),
            ),
            (
                LIQUIDATED_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_ETH_USDC.to_string(),
            ),
            (
                LIQUIDATOR_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_BTC_USDC.to_string(),
            ),
            (
                LIQUIDATOR_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_ETH_USDC.to_string(),
            ),
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_BTC_USDC.to_string()),
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_ETH_USDC.to_string()),
        ];
        let account_tokens = vec![
            (P_ACCOUNT_ID.to_string(), TOKEN_HASH.to_string()),
            (P_LIQUIDATOR_ID.to_string(), TOKEN_HASH.to_string()),
            (LIQUIDATED_ACCOUNT_ID.to_string(), TOKEN_HASH.to_string()),
            (LIQUIDATED_ACCOUNT_ID.to_string(), USDC_HASH.to_string()),
            (LIQUIDATOR_ACCOUNT_ID.to_string(), TOKEN_HASH.to_string()),
            (LIQUIDATOR_ACCOUNT_ID.to_string(), USDC_HASH.to_string()),
            (INSURANCE_FUND.to_string(), TOKEN_HASH.to_string()),
            (INSURANCE_FUND.to_string(), USDC_HASH.to_string()),
        ];
        context.init_orderly_context(block_time, symbols, tokens, account_symbols, account_tokens);

        let result: IndexerQueryResponse<TradingEventsResponse> =
            serde_json::from_str(TRADE_AND_LIQ_DATA2).unwrap();
        let mut result_data = IndexerQueryResponse::<TradingEventsResponse>::empty_success();
        println!("result2: {:?}", result);
        match &result {
            IndexerQueryResponse::Success(data) => {
                let mut data = data.as_data().cloned().unwrap();
                // match data.events[0].data.clone() {
                //     TradingEventInnerData::ProcessedTrades { batch_id, trades } => {
                //         data.events = vec![data.events[0].clone()];
                //         data.events[0].data = TradingEventInnerData::ProcessedTrades { batch_id, trades: trades[0..3].to_vec() };
                //     },
                //     _ => {}
                // }
                // println!(
                //     "------result data length: {}, data: {:?}",
                //     data.events.len(), data.events
                // );
                result_data = IndexerQueryResponse::Success(SuccessResponse::new(data))
            }
            _ => {}
        }
        let p_account_id =
            "0x199265c7b18b200fbe2fb5813ca524e1762be91fcd44f19ce40f998b016426fe".to_string();
        let symbol =
            "0x5a8133e52befca724670dbf2cade550c522c2410dd5b1189df675e99388f509d".to_string();
        let perp_key = UserPerpSummaryKey {
            account_id: p_account_id.clone(),
            symbol: symbol.clone(),
        };
        context.user_perp_cache.insert(
            perp_key.clone(),
            UserPerpSummary::new_empty_user_perp_summary(&p_account_id, &symbol),
        );
        println!("1111111111111goooooooo");
        parse_and_analyzer(result_data, &mut context).await;
        let user_perp = context.get_user_perp(&perp_key).await;
        println!(
            "total_realized_pnl: {}",
            user_perp.total_realized_pnl.to_string()
        );
        assert_eq!(
            user_perp.total_realized_pnl,
            BigDecimal::from_str("0.006908").unwrap()
        );
    }

    const TRADE_AND_LIQ_DATA3: &str = r#"
{
    "success": true,
    "data": {
        "events": [
            {
                "block_number": 1187648,
                "transaction_index": 1,
                "log_index": 0,
                "transaction_id": "0x725c821aec213d0b57c12b45229db7e17bea0727ac047b0837bd3bc8b4991e7c",
                "block_timestamp": 1722820471,
                "data": {
                    "ProcessedTrades": {
                        "batch_id": 63,
                        "trades": [
                            {
                                "account_id": "0xe2f47fd3fde3585df923998726dfbca5f4214299998f1d992156d035709f6074",
                                "symbol_hash": "0x7e83089239db756ee233fa8972addfea16ae653db0f692e4851aed546b21caeb",
                                "fee_asset_hash": "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa",
                                "trade_qty": "5440000",
                                "notional": "97845472",
                                "executed_price": "179863000000",
                                "fee": "58708",
                                "sum_unitary_fundings": "4250000000000000",
                                "trade_id": 10281,
                                "match_id": 1698982326734489998,
                                "timestamp": 1722820471000,
                                "side": "BUY"
                            }
                        ]
                    }
                }
            },
            {
                "block_number": 13106122,
                "transaction_index": 1,
                "log_index": 9,
                "transaction_id": "0xbc4eb7288804f483e255ed35b525dfc9942edb5d94b6eabbde9985101d202579",
                "block_timestamp": 1722820471,
                "data": {
                    "LiquidationResultV2": {
                        "account_id": "0xe2f47fd3fde3585df923998726dfbca5f4214299998f1d992156d035709f6074",
                        "liquidated_asset_hash": "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa",
                        "insurance_transfer_amount": "-10000000",
                        "liquidation_transfers": [
                            {
                                "account_id": "0xe2f47fd3fde3585df923998726dfbca5f4214299998f1d992156d035709f6074",
                                "symbol_hash": "0x7e83089239db756ee233fa8972addfea16ae653db0f692e4851aed546b21caeb",
                                "position_qty_transfer": "-5440000",
                                "cost_position_transfer": "-137648276",
                                "fee": "0",
                                "mark_price": "231050000000",
                                "sum_unitary_fundings": "734840000000000000"
                            }
                        ]
                    }
                }
            }
        ],
        "last_block": 13106122,
        "last_block_timestamp": 1722820471,
        "next_offset": null
    }
}
    "#;
    #[actix_web::test]
    async fn test_prod_trade_and_liquidation_with_data() {
        let result: IndexerQueryResponse<TradingEventsResponse> =
            serde_json::from_str(TRADE_AND_LIQ_DATA3).unwrap();
        let mut result_data = IndexerQueryResponse::<TradingEventsResponse>::empty_success();
        println!("result3: {:?}", result);
        match &result {
            IndexerQueryResponse::Success(data) => {
                let mut data = data.as_data().cloned().unwrap();
                // match data.events[0].data.clone() {
                //     TradingEventInnerData::ProcessedTrades { batch_id, trades } => {
                //         data.events = vec![data.events[0].clone()];
                //         data.events[0].data = TradingEventInnerData::ProcessedTrades { batch_id, trades: trades[0..3].to_vec() };
                //     },
                //     _ => {}
                // }
                // println!(
                //     "------result data length: {}, data: {:?}",
                //     data.events.len(), data.events
                // );
                result_data = IndexerQueryResponse::Success(SuccessResponse::new(data))
            }
            _ => {}
        }
        let mut context = AnalyzeContext::new_context();
        let block_time = 1722820471;

        const P_ACCOUNT_ID: &str =
            "0xe2f47fd3fde3585df923998726dfbca5f4214299998f1d992156d035709f6074";
        const P_LIQUIDATOR_ID: &str =
            "0xe042ae0d7f1cb85245360af73a383d643e43401f64fa56c2c072dbbf200554d7";

        const SYMBOL: &str = "0x7e83089239db756ee233fa8972addfea16ae653db0f692e4851aed546b21caeb";

        let symbols = vec![
            SYMBOL.to_string(),
            SYMBOL_HASH_BTC_USDC.to_string(),
            SYMBOL_HASH_ETH_USDC.to_string(),
        ];
        let tokens = vec![TOKEN_HASH.to_string(), USDC_HASH.to_string()];
        let account_symbols = vec![
            (P_ACCOUNT_ID.to_string(), SYMBOL.to_string()),
            (P_LIQUIDATOR_ID.to_string(), SYMBOL.to_string()),
            (
                LIQUIDATED_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_BTC_USDC.to_string(),
            ),
            (
                LIQUIDATED_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_ETH_USDC.to_string(),
            ),
            (
                LIQUIDATOR_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_BTC_USDC.to_string(),
            ),
            (
                LIQUIDATOR_ACCOUNT_ID.to_string(),
                SYMBOL_HASH_ETH_USDC.to_string(),
            ),
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_BTC_USDC.to_string()),
            (INSURANCE_FUND.to_string(), SYMBOL_HASH_ETH_USDC.to_string()),
        ];
        let account_tokens = vec![
            (P_ACCOUNT_ID.to_string(), TOKEN_HASH.to_string()),
            (P_LIQUIDATOR_ID.to_string(), TOKEN_HASH.to_string()),
            (LIQUIDATED_ACCOUNT_ID.to_string(), TOKEN_HASH.to_string()),
            (LIQUIDATED_ACCOUNT_ID.to_string(), USDC_HASH.to_string()),
            (LIQUIDATOR_ACCOUNT_ID.to_string(), TOKEN_HASH.to_string()),
            (LIQUIDATOR_ACCOUNT_ID.to_string(), USDC_HASH.to_string()),
            (INSURANCE_FUND.to_string(), TOKEN_HASH.to_string()),
            (INSURANCE_FUND.to_string(), USDC_HASH.to_string()),
        ];
        context.init_orderly_context(block_time, symbols, tokens, account_symbols, account_tokens);
        let perp_key = UserPerpSummaryKey {
            account_id: P_ACCOUNT_ID.to_string(),
            symbol: SYMBOL.to_string(),
        };
        parse_and_analyzer(result_data, &mut context).await;
        let user_perp = context.get_user_perp(&perp_key).await;
        println!(
            "total_realized_pnl: {}",
            user_perp.total_realized_pnl.to_string()
        );
        assert_eq!(
            user_perp.total_realized_pnl,
            BigDecimal::from_str("39.802804").unwrap()
        );
    }
}
