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
        "receiver margin transfer account_id:{},iso_symbol_hash:{},margin_asset_hash: {}, margin_from_cross:{}",
        account_id, iso_symbol_hash, margin_asset_hash, margin_from_cross
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
