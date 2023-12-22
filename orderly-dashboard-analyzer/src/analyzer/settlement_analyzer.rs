use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use orderly_dashboard_indexer::formats_external::trading_events::SettlementExecution;
use std::str::FromStr;

use crate::db::user_token_summary::{create_or_update_user_token_summary, find_user_token_summary};

pub async fn analyzer_settlement(
    account_id: String,
    settled_amount: String,
    settled_asset_hash: String,
    insurance_account_id: String,
    insurance_transfer_amount: String,
    settlement_executions: Vec<SettlementExecution>,
    block_hour: NaiveDateTime,
    pulled_block_height: i64,
    pulled_block_time: NaiveDateTime,
) {
    let mut user_token =
        find_user_token_summary(account_id, String::from("USDC"), Default::default())
            .await
            .unwrap();
    user_token.new_settlement(
        BigDecimal::from_str(&*settled_amount).unwrap(),
        pulled_block_height,
        pulled_block_time.clone(),
    );

    if !insurance_account_id.is_empty() {
        let mut insurance_token = find_user_token_summary(
            insurance_account_id,
            String::from("USDC"),
            Default::default(),
        )
        .await
        .unwrap();
        insurance_token.new_settlement(
            BigDecimal::from_str(&*insurance_transfer_amount).unwrap(),
            pulled_block_height,
            pulled_block_time.clone(),
        );
        create_or_update_user_token_summary(vec![user_token, insurance_token])
            .await
            .unwrap();
    } else {
        create_or_update_user_token_summary(vec![user_token])
            .await
            .unwrap();
    }
}
