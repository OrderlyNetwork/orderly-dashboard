use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::{excluded, on_constraint};
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::{PrimaryKey, DB_CONTEXT};
use crate::db::{BATCH_UPSERT_LEN, POOL};
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
    pub fn new_empty_token_summary(
        account_id: &str,
        token: &str,
        chain_id: &str,
    ) -> UserTokenSummary {
        UserTokenSummary {
            account_id: account_id.to_string(),
            token: token.to_string(),
            chain_id: chain_id.to_string(),
            balance: Default::default(),
            total_withdraw_amount: Default::default(),
            total_deposit_amount: Default::default(),
            total_withdraw_count: 0,
            total_deposit_count: 0,
            pulled_block_height: 0,
            pulled_block_time: Default::default(),
        }
    }
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
) -> anyhow::Result<usize> {
    if user_token_summary_vec.is_empty() {
        return Ok(0);
    }
    use crate::schema::user_token_summary::dsl::*;

    let mut row_nums = 0;
    let mut user_token_summary_vec = user_token_summary_vec
        .into_iter()
        .cloned()
        .collect::<Vec<UserTokenSummary>>();
    loop {
        if user_token_summary_vec.len() >= BATCH_UPSERT_LEN {
            let (values1, res) = user_token_summary_vec.split_at(BATCH_UPSERT_LEN);
            let values1 = values1
                .iter()
                .map(|v| v.clone())
                .collect::<Vec<UserTokenSummary>>();
            user_token_summary_vec = res.iter().cloned().collect::<Vec<UserTokenSummary>>();
            let update_result = diesel::insert_into(user_token_summary)
                .values(values1)
                .on_conflict(on_constraint("user_token_summary_uq"))
                .do_update()
                .set((
                    balance.eq(excluded(balance)),
                    total_withdraw_amount.eq(excluded(total_withdraw_amount)),
                    total_withdraw_count.eq(excluded(total_withdraw_count)),
                    total_deposit_amount.eq(excluded(total_deposit_amount)),
                    total_deposit_count.eq(excluded(total_deposit_count)),
                    pulled_block_height.eq(excluded(pulled_block_height)),
                    pulled_block_time.eq(excluded(pulled_block_time)),
                ))
                .execute_async(&POOL)
                .await;

            match update_result {
                Ok(len) => {
                    row_nums += len;
                }
                Err(err) => {
                    return Err(anyhow::anyhow!("update user_token_summary failed: {}", err));
                }
            }
        } else {
            let update_result = diesel::insert_into(user_token_summary)
                .values(user_token_summary_vec)
                .on_conflict(on_constraint("user_token_summary_uq"))
                .do_update()
                .set((
                    balance.eq(excluded(balance)),
                    total_withdraw_amount.eq(excluded(total_withdraw_amount)),
                    total_withdraw_count.eq(excluded(total_withdraw_count)),
                    total_deposit_amount.eq(excluded(total_deposit_amount)),
                    total_deposit_count.eq(excluded(total_deposit_count)),
                    pulled_block_height.eq(excluded(pulled_block_height)),
                    pulled_block_time.eq(excluded(pulled_block_time)),
                ))
                .execute_async(&POOL)
                .await;

            match update_result {
                Ok(len) => {
                    row_nums += len;
                    break;
                }
                Err(err) => {
                    return Err(anyhow::anyhow!("update user_token_summary failed: {}", err));
                }
            }
        }
    }

    Ok(row_nums)
}
