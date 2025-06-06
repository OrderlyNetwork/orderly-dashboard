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
        liquidation.block_num,
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
    let (open_cost_diff, pnl_diff) = RealizedPnl::calc_realized_pnl(
        liquidation.qty_transfer.clone(),
        (liquidation.cost_position_transfer.clone())
            - ((liquidation.liquidator_fee.clone()) + (liquidation.insurnace_fee.clone())),
        user_perp_snap.holding.clone(),
        user_perp_snap.opening_cost.clone(),
    );

    let user_perp = context.get_user_perp(&key.clone()).await;
    user_perp.new_liquidation_v2(
        liquidation.qty_transfer.clone(),
        liquidation.mark_price.clone(),
        liquidation.block_num,
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
    use crate::analyzer::tests::*;
    use bigdecimal::BigDecimal;
    use chrono::NaiveDateTime;
    use num_traits::FromPrimitive;
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
        );
        context.set_user_perp_cache(
            &liquidated_eth_perp_key,
            BigDecimal::from_i128(200000000).unwrap(),
            BigDecimal::from_i128(3600000000).unwrap(),
            BigDecimal::from_i128(2000000000000002).unwrap(),
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
            assert_eq!(liquidated_btc.holding, BigDecimal::from(-2));
            println!("liquidated_btc perp summary: {:?}", liquidated_btc);
        }

        {
            let liquidated_eth = context.get_user_perp_cache(&liquidated_eth_perp_key);
            assert_eq!(liquidated_eth.holding, BigDecimal::from(1));
            println!("liquidated eth perp summary: {:?}", liquidated_eth);
        }

        {
            let liquidator_btc = context.get_user_perp_cache(&liquidator_btc_perp_key);
            assert_eq!(liquidator_btc.holding, BigDecimal::from(-2));
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
        );
        context.set_user_perp_cache(
            &liquidator_btc_perp_key,
            BigDecimal::from_i128(4_059_000_000).unwrap(),
            BigDecimal::from_i128(-2_533_031_349).unwrap(),
            BigDecimal::from_i128(40_376_000_000_000_000).unwrap(),
        );
        context.set_user_perp_cache(
            &insurance_btc_perp_key,
            BigDecimal::from_i128(0).unwrap(),
            BigDecimal::from_i128(-142_229_490).unwrap(),
            BigDecimal::from_i128(40_376_000_000_000_000).unwrap(),
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
            println!("liquidated_btc perp summary: {:?}", liquidated_btc);
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
}
