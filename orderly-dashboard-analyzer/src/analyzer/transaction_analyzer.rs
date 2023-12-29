use chrono::NaiveDateTime;
use orderly_dashboard_indexer::formats_external::trading_events::*;

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::{div_into_real, to_big_decimal};
use crate::db::hourly_orderly_token::HourlyOrderlyTokenKey;
use crate::db::hourly_user_token::HourlyUserTokenKey;
use crate::db::orderly_token_summary::OrderlyTokenSummaryKey;
use crate::db::user_token_summary::UserTokenSummaryKey;

const TRANSACTION_ANALYZER: &str = "transaction-analyzer";

pub async fn analyzer_transaction(
    account_id: String,
    token_hash: String,
    chain_id: String,
    side: TransactionSide,
    token_amount: String,
    block_hour: &NaiveDateTime,
    block_number: i64,
    block_time: NaiveDateTime,
    context: &mut AnalyzeContext,
) {
    tracing::info!(target:TRANSACTION_ANALYZER,"receive {:?} - account:{},amount:{}",side.clone(),account_id.clone(),token_amount.clone());
    let fixed_amount = div_into_real(token_amount.parse().unwrap(), 1000000);

    let deposit;
    match side {
        TransactionSide::Deposit => {
            deposit = true;
        }
        TransactionSide::Withdraw => deposit = false,
    }

    {
        let key = HourlyUserTokenKey {
            account_id: account_id.clone(),
            token: token_hash.clone(),
            block_hour: block_hour.clone(),
            chain_id: chain_id.clone(),
        };
        let hourly_user_token = context.get_hourly_user_token(&key).await;
        if deposit {
            hourly_user_token.deposit(
                to_big_decimal(fixed_amount.clone()),
                block_number,
                block_time,
            );
        } else {
            hourly_user_token.withdraw(
                to_big_decimal(fixed_amount.clone()),
                block_number,
                block_time,
            );
        }
    }

    {
        let key = HourlyOrderlyTokenKey {
            token: token_hash.clone(),
            block_hour: block_hour.clone(),
            chain_id: chain_id.clone(),
        };

        let hourly_orderly_token = context.get_hourly_orderly_token(&key).await;
        if deposit {
            hourly_orderly_token.deposit(
                to_big_decimal(fixed_amount.clone()),
                block_number,
                block_time,
            );
        } else {
            hourly_orderly_token.withdraw(
                to_big_decimal(fixed_amount.clone()),
                block_number,
                block_time,
            );
        }
    }

    {
        let key = OrderlyTokenSummaryKey {
            token: token_hash.clone(),
            chain_id: chain_id.clone(),
        };
        let orderly_token = context.get_orderly_token(&key).await;
        if deposit {
            orderly_token.deposit(
                to_big_decimal(fixed_amount.clone()),
                block_number,
                block_time,
            );
        } else {
            orderly_token.withdraw(
                to_big_decimal(fixed_amount.clone()),
                block_number,
                block_time,
            );
        }
    }
    {
        let key = UserTokenSummaryKey {
            account_id: account_id.clone(),
            token: token_hash.clone(),
            chain_id: chain_id.clone(),
        };
        let user_token = context.get_user_token(&key).await;
        if deposit {
            user_token.deposit(
                to_big_decimal(fixed_amount.clone()),
                block_number,
                block_time,
            );
        } else {
            user_token.withdraw(
                to_big_decimal(fixed_amount.clone()),
                block_number,
                block_time,
            );
        }
    }
}
