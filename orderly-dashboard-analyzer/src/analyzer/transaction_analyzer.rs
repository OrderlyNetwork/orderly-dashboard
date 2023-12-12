use std::str::FromStr;

use bigdecimal::BigDecimal;
use orderly_dashboard_indexer::formats_external::{trading_events::*};

use crate::db::hourly_orderly_token::{create_or_update_hourly_orderly_token, find_hourly_orderly_token};
use crate::db::hourly_user_token::{create_or_update_hourly_user_token, find_hourly_user_token};
use crate::db::orderly_token_summary::{create_or_update_orderly_token_summary, find_orderly_token_summary};
use crate::db::user_token_summary::{create_or_update_user_token_summary, find_user_token_summary};

pub async fn analyzer_transaction(account_id: String,
                                  token_hash: String,
                                  chain_id: String,
                                  side: TransactionSide,
                                  token_amount: String
                                  , block_hour: &i64, block_number: &i64, block_time: i64) {
    let mut hourly_user_token = find_hourly_user_token(account_id.clone(), token_hash.clone(), block_hour.clone(), chain_id.clone()).await.unwrap();
    let mut hourly_orderly_token = find_hourly_orderly_token(token_hash.clone(), block_hour.clone(), chain_id.clone()).await.unwrap();
    let mut user_token_summary = find_user_token_summary(account_id.clone(), token_hash.clone(), chain_id.clone()).await.unwrap();
    let mut orderly_token_summary = find_orderly_token_summary(token_hash.clone(), chain_id.clone()).await.unwrap();
    let amount = BigDecimal::from_str(token_amount.as_str()).unwrap();
    match side {
        TransactionSide::Deposit => {
            user_token_summary.deposit(amount.clone());
            hourly_user_token.deposit(amount.clone());
            hourly_orderly_token.deposit(amount.clone());
            orderly_token_summary.deposit(amount.clone());
        }
        TransactionSide::Withdraw => {
            user_token_summary.withdraw(amount.clone());
            hourly_user_token.withdraw(amount.clone());
            hourly_orderly_token.withdraw(amount.clone());
            orderly_token_summary.withdraw(amount.clone());
        }
    }

    hourly_orderly_token.pulled_block_time = block_time.clone();
    hourly_orderly_token.pulled_block_height = block_number.clone();
    let _ = create_or_update_hourly_orderly_token(vec![hourly_orderly_token]).await;

    hourly_user_token.pulled_block_time = block_time.clone();
    hourly_user_token.pulled_block_height = block_number.clone();
    let _ = create_or_update_hourly_user_token(vec![hourly_user_token]).await;


    user_token_summary.pulled_block_time = block_time.clone();
    user_token_summary.pulled_block_height = block_number.clone();
    let _ = create_or_update_user_token_summary(vec![user_token_summary]).await;


    orderly_token_summary.pulled_block_time = block_time.clone();
    orderly_token_summary.pulled_block_height = block_number.clone();
    let _ = create_or_update_orderly_token_summary(vec![orderly_token_summary]).await;
}