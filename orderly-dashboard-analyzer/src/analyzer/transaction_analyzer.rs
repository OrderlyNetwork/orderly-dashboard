use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use std::ops::Div;

use orderly_dashboard_indexer::formats_external::trading_events::*;

use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::get_cost_position_prec;
use crate::db::broker_info::find_by_broker_hash;
use crate::db::hourly_orderly_token::HourlyOrderlyTokenKey;
use crate::db::hourly_user_token::HourlyUserTokenKey;
use crate::db::orderly_token_summary::OrderlyTokenSummaryKey;
use crate::db::user_info::{create_user_info, UserInfo};
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
    context: &mut AnalyzeContext,
    sender: String,
    broker_hash: String,
) {
    tracing::info!(target:TRANSACTION_ANALYZER,"receive {:?} - account:{},amount:{}",side.clone(),account_id.clone(),token_amount.clone());
    let token_amo: BigDecimal = token_amount.clone().parse().unwrap();
    let fixed_amount = token_amo.div(get_cost_position_prec());

    let deposit;
    match side {
        TransactionSide::Deposit => {
            deposit = true;
        }
        TransactionSide::Withdraw => deposit = false,
    }

    if deposit {
        let broker_info = find_by_broker_hash(broker_hash.clone()).await;
        match broker_info {
            Ok(broker) => {
                let user: UserInfo = UserInfo {
                    account_id: account_id.clone(),
                    broker_id: broker.broker_id,
                    broker_hash: broker_hash.clone(),
                    address: sender,
                };
                create_user_info(&user).await.expect("insert user error");
            }
            Err(_) => {}
        }
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
            hourly_user_token.deposit(fixed_amount.clone(), block_number);
        } else {
            hourly_user_token.withdraw(fixed_amount.clone(), block_number);
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
            hourly_orderly_token.deposit(fixed_amount.clone(), block_number);
        } else {
            hourly_orderly_token.withdraw(fixed_amount.clone(), block_number);
        }
    }

    {
        let key = OrderlyTokenSummaryKey {
            token: token_hash.clone(),
            chain_id: chain_id.clone(),
        };
        let orderly_token = context.get_orderly_token(&key).await;
        if deposit {
            orderly_token.deposit(fixed_amount.clone(), block_number);
        } else {
            orderly_token.withdraw(fixed_amount.clone(), block_number);
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
            user_token.deposit(fixed_amount.clone(), block_number);
        } else {
            user_token.withdraw(fixed_amount.clone(), block_number);
        }
    }
}
