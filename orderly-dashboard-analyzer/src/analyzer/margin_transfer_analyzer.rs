use bigdecimal::BigDecimal;

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::db::user_perp_summary::UserPerpSummaryKey;

const MARGIN_TRANSFER_ANALYZER: &str = "adl-analyzer";

pub async fn analyzer_margin_transfer(
    account_id: String,
    iso_symbol_hash: String,
    margin_asset_hash: String,
    margin_from_cross: String,
    block_num: i64,
    context: &mut AnalyzeContext,
) {
    tracing::info!(
        target:MARGIN_TRANSFER_ANALYZER,
        "block_num: {}, receiver margin transfer account_id: {},iso_symbol_hash:{},margin_asset_hash: {}, margin_from_cross:{}",
        block_num, account_id, iso_symbol_hash, margin_asset_hash, margin_from_cross
    );
    // isolated
    let user_perp_key = UserPerpSummaryKey {
        account_id: account_id.clone(),
        symbol: iso_symbol_hash.clone(),
    };
    let user_perp_snap = context.get_iso_user_perp(&user_perp_key.clone()).await;
    let margin_from_cross: BigDecimal = margin_from_cross.parse().unwrap();
    user_perp_snap.margin_deposit(block_num, margin_asset_hash, margin_from_cross);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{analyzer_margin_transfer, AnalyzeContext, UserPerpSummaryKey};
    use crate::analyzer::{calc::USDC_HASH, tests::*};
    use bigdecimal::BigDecimal;

    use num_traits::FromPrimitive;

    const ALICE: &str = "0xa11ce";
    const BOB: &str = "0xb0b";

    #[actix_web::test]
    async fn test_margin_transfer() {
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
        let alice_btc_perp_key = UserPerpSummaryKey {
            account_id: ALICE.to_string(),
            symbol: SYMBOL_HASH_ETH_USDC.to_string(),
        };

        context.set_iso_user_perp_cache(
            &alice_btc_perp_key,
            BigDecimal::from_i128(500_000_000).unwrap(),
            BigDecimal::from_i128(50_000_000).unwrap(),
            BigDecimal::from_i128(0).unwrap(),
            0.into(),
        );
        context.set_user_iso_margin_cache(
            &alice_btc_perp_key,
            TOKEN_HASH.to_string(),
            BigDecimal::from_i128(100_000_000).unwrap(),
        );

        let block_number = 1000000;
        analyzer_margin_transfer(
            ALICE.to_string(),
            SYMBOL_HASH_ETH_USDC.to_string(),
            TOKEN_HASH.to_string(),
            "200000000".to_string(),
            block_number,
            &mut context,
        )
        .await;
        let alice_btc = context.get_iso_user_perp_cache(&alice_btc_perp_key);
        println!(
            "alice_btc.holding: {:?}, alice_btc.cost_position: {}, alice_btc.margin_qty: {}",
            alice_btc.holding.to_string(),
            alice_btc.cost_position.to_string(),
            alice_btc.margin_qty.to_string(),
        );
        // assertEq(positionA.costPosition, -4992500);
        assert_eq!(
            alice_btc.margin_qty,
            BigDecimal::from_str("300000000").unwrap()
        );
    }
}
