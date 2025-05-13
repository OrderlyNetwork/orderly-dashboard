use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::user_token_summary::DBException::UpdateError;
use crate::db::POOL;
use crate::db::{PrimaryKey, DB_CONTEXT};
use crate::schema::user_token_summary;

#[derive(Insertable, Queryable, Debug, Clone)]
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
    pub pulled_block_time: NaiveDateTime,
}

impl UserTokenSummary {
    pub fn add_amount(&mut self, add_amount: BigDecimal, p_block_height: i64) {
        if p_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }

        self.balance += add_amount;
    }
}

impl UserTokenSummary {
    pub fn deposit(&mut self, p_deposit_amount: BigDecimal, p_block_height: i64) {
        if p_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.total_deposit_amount += p_deposit_amount.clone();
        self.total_deposit_count += 1;
        self.balance += p_deposit_amount.clone();
    }

    pub fn withdraw(&mut self, p_withdraw_amount: BigDecimal, p_block_height: i64) {
        if p_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.total_withdraw_amount += p_withdraw_amount.clone();
        self.total_withdraw_count += 1;
        self.balance -= p_withdraw_amount.clone();
    }

    pub fn new_settlement(&mut self, p_settle_amount: BigDecimal, block_num: i64) {
        if block_num <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.balance += p_settle_amount;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UserTokenSummaryKey {
    pub account_id: String,
    pub token: String,
    pub chain_id: String,
}

impl PrimaryKey for UserTokenSummaryKey {}

#[derive(Debug)]
pub enum DBException {
    InsertError,
    QueryError,
    UpdateError,
    Timeout,
}

pub async fn find_user_token_summary(
    ori_account_id: String,
    ori_token: String,
    ori_chain_id: String,
) -> Option<UserTokenSummary> {
    use crate::schema::user_token_summary::dsl::*;

    let filter = user_token_summary
        .filter(account_id.eq(ori_account_id.clone()))
        .filter(token.eq(ori_token.clone()))
        .filter(chain_id.eq(ori_chain_id.clone()));

    let result = filter.first_async::<UserTokenSummary>(&POOL).await;

    match result {
        Ok(summary) => Some(summary),
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                let insert_sum = UserTokenSummary {
                    account_id: ori_account_id.clone(),
                    token: ori_token.clone(),
                    chain_id: ori_chain_id.clone(),
                    balance: Default::default(),
                    total_withdraw_amount: Default::default(),
                    total_deposit_amount: Default::default(),
                    total_withdraw_count: 0,
                    total_deposit_count: 0,
                    pulled_block_height: 0,
                    pulled_block_time: Default::default(),
                };
                return Some(insert_sum);
            }
            _ => {
                tracing::warn!(target: DB_CONTEXT,"select_user_token_summary fail. err:{:?}",error);
                None
            }
        },
    }
}

pub async fn create_or_update_user_token_summary(
    user_token_summary_vec: Vec<&UserTokenSummary>,
) -> Result<usize, DBException> {
    use crate::schema::user_token_summary::dsl::*;
    let mut row_nums = 0;

    for summary in user_token_summary_vec {
        let effect = diesel::insert_into(user_token_summary)
            .values(summary.clone())
            .on_conflict(on_constraint("user_token_summary_uq"))
            .do_update()
            .set((
                balance.eq(summary.balance.clone()),
                total_withdraw_amount.eq(summary.total_withdraw_amount.clone()),
                total_withdraw_count.eq(summary.total_withdraw_count.clone()),
                total_deposit_amount.eq(summary.total_deposit_amount.clone()),
                total_deposit_count.eq(summary.total_deposit_count.clone()),
                pulled_block_height.eq(summary.pulled_block_height.clone()),
                pulled_block_time.eq(summary.pulled_block_time.clone()),
            ))
            .execute_async(&POOL)
            .await;

        match effect {
            Ok(affected) => {
                row_nums += affected;
            }
            Err(_) => {
                return Err(UpdateError);
            }
        }
    }

    Ok(row_nums)
}
