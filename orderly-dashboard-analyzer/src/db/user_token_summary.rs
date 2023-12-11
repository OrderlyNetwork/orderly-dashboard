use actix_diesel::AsyncError;
use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::BigDecimal;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::DB_CONTEXT;
use crate::db::POOL;
use crate::db::user_token_summary::DBException::{InsertError, UpdateError};
use crate::schema::user_token_summary;

#[derive(Insertable, Queryable, Debug)]
#[table_name = "user_token_summary"]
pub struct UserTokenSummary {
    pub account_id: String,
    pub token: String,

    pub chain_id: String,
    pub balance: BigDecimal,

    pub total_withdraw_amount: BigDecimal,
    pub total_deposit_amount: BigDecimal,
    pub total_withdraw_count: i64,
    pub total_deposit_count: i64,

    pub pulled_block_height: i64,
    pub pulled_block_time: i64,
}

#[derive(Debug)]
pub enum DBException {
    InsertError,
    QueryError,
    UpdateError,
}

pub async fn select_user_token_summary(ori_account_id: String, ori_token: String, ori_chain_id: String) -> Option<UserTokenSummary> {
    use crate::schema::user_token_summary::dsl::*;
    let result = user_token_summary
        .filter(account_id.eq(ori_account_id))
        .filter(token.eq(ori_token))
        .filter(chain_id.eq(ori_chain_id))
        .first_async::<UserTokenSummary>(&POOL)
        .await;

    match result {
        Ok(summary) => {
            Some(summary)
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                return None;
            }
            _ => {
                tracing::warn!(target: DB_CONTEXT,"select_user_token_summary fail. err:{:?}",error);
                None
            }
        }
    }
}

pub async fn create_user_token(user_token_summary_vec: Vec<UserTokenSummary>) -> Result<usize, DBException> {
    use crate::schema::user_token_summary::dsl::*;
    let insert_result = diesel::insert_into(user_token_summary)
        .values(user_token_summary_vec)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await;

    match insert_result {
        Ok(row_nums) => {
            Ok(row_nums)
        }
        Err(err) => match err {
            _ => {
                return Err(InsertError);
            }
        }
    }
}

pub async fn update_user_token(user_token_summary_vec: Vec<UserTokenSummary>) -> Result<usize, DBException> {
    use crate::schema::user_token_summary::dsl::*;
    let mut row_nums = 0;

    for summary in user_token_summary_vec {
        let effect = diesel::update(user_token_summary)
            .filter(account_id.eq(summary.account_id))
            .filter(token.eq(summary.token))
            .filter(chain_id.eq(summary.chain_id))
            .set((
                balance.eq(summary.balance),
                total_withdraw_amount.eq(summary.total_withdraw_amount),
                total_withdraw_count.eq(summary.total_withdraw_count),
                total_deposit_amount.eq(summary.total_deposit_amount),
                total_deposit_count.eq(summary.total_deposit_count),
                pulled_block_height.eq(summary.pulled_block_height),
                pulled_block_time.eq(summary.pulled_block_time)
            ))
            .execute_async(&POOL)
            .await;

        match effect {
            Ok(affected) => {
                row_nums += affected;
            }
            Err(err) => {
                return Err(UpdateError);
            }
        }
    }

    Ok(row_nums)
}