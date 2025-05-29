use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::{excluded, on_constraint};
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::QueryError;
use crate::db::{PrimaryKey, BATCH_UPSERT_LEN, POOL};
use crate::schema::orderly_token_summary;

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "orderly_token_summary"]
pub struct OrderlyTokenSummary {
    pub token: String,
    pub chain_id: String,
    pub balance: BigDecimal,

    pub total_withdraw_amount: BigDecimal,
    pub total_withdraw_count: i64,

    pub total_deposit_amount: BigDecimal,
    pub total_deposit_count: i64,

    pub pulled_block_height: i64,
    pub pulled_block_time: NaiveDateTime,
}

impl OrderlyTokenSummary {
    pub fn new_empty_orderly_token_summary(token: &str, chain_id: &str) -> OrderlyTokenSummary {
        OrderlyTokenSummary {
            token: token.to_string(),
            chain_id: chain_id.to_string(),
            balance: Default::default(),
            total_withdraw_amount: Default::default(),
            total_withdraw_count: 0,
            total_deposit_amount: Default::default(),
            total_deposit_count: 0,
            pulled_block_height: 0,
            pulled_block_time: Default::default(),
        }
    }
}

impl OrderlyTokenSummary {
    #[allow(duplicate_macro_attributes)]
    pub fn deposit(&mut self, p_deposit_amount: BigDecimal, p_block_height: i64) {
        if p_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.total_deposit_amount += p_deposit_amount.clone();
        self.total_deposit_count += 1;
        self.balance += p_deposit_amount.clone().abs();
    }

    #[allow(duplicate_macro_attributes)]
    pub fn withdraw(&mut self, p_withdraw_amount: BigDecimal, p_block_height: i64) {
        if p_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.total_withdraw_amount += p_withdraw_amount.clone();
        self.total_withdraw_count += 1;
        self.balance -= p_withdraw_amount.clone();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct OrderlyTokenSummaryKey {
    pub token: String,
    pub chain_id: String,
}

impl PrimaryKey for OrderlyTokenSummaryKey {}

pub async fn find_orderly_token_summary(
    p_token: String,
    p_chain_id: String,
) -> Result<OrderlyTokenSummary, DBException> {
    use crate::schema::orderly_token_summary::dsl::*;
    let select_result = orderly_token_summary
        .filter(token.eq(p_token.clone()))
        .filter(chain_id.eq(p_chain_id.clone()))
        .first_async::<OrderlyTokenSummary>(&POOL)
        .await;

    match select_result {
        Ok(hourly_data) => Ok(hourly_data),
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                let hourly_data = OrderlyTokenSummary {
                    token: p_token.clone(),
                    chain_id: p_chain_id.clone(),
                    balance: Default::default(),
                    total_withdraw_amount: Default::default(),
                    total_withdraw_count: 0,
                    total_deposit_amount: Default::default(),
                    total_deposit_count: 0,
                    pulled_block_height: 0,
                    pulled_block_time: Default::default(),
                };
                Ok(hourly_data)
            }
            _ => Err(QueryError),
        },
    }
}

pub async fn create_or_update_orderly_token_summary(
    p_hourly_data_vec: Vec<&OrderlyTokenSummary>,
) -> anyhow::Result<usize> {
    if p_hourly_data_vec.is_empty() {
        return Ok(0);
    }
    use crate::schema::orderly_token_summary::dsl::*;

    let mut row_nums = 0;
    let mut p_hourly_data_vec = p_hourly_data_vec
        .into_iter()
        .cloned()
        .collect::<Vec<OrderlyTokenSummary>>();
    loop {
        if p_hourly_data_vec.len() >= BATCH_UPSERT_LEN {
            let (values1, res) = p_hourly_data_vec.split_at(BATCH_UPSERT_LEN);
            let values1 = values1
                .iter()
                .map(|v| v.clone())
                .collect::<Vec<OrderlyTokenSummary>>();
            p_hourly_data_vec = res.iter().cloned().collect::<Vec<OrderlyTokenSummary>>();
            let update_result = diesel::insert_into(orderly_token_summary)
                .values(values1)
                .on_conflict(on_constraint("orderly_token_summary_uq"))
                .do_update()
                .set((
                    #[allow(duplicate_macro_attributes)]
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
                    return Err(anyhow::anyhow!(
                        "update orderly_token_summary failed: {}",
                        err
                    ));
                }
            }
        } else {
            let update_result = diesel::insert_into(orderly_token_summary)
                .values(p_hourly_data_vec)
                .on_conflict(on_constraint("orderly_token_summary_uq"))
                .do_update()
                .set((
                    #[allow(duplicate_macro_attributes)]
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
                    return Err(anyhow::anyhow!(
                        "update orderly_token_summary failed: {}",
                        err
                    ));
                }
            }
        }
    }

    Ok(row_nums)
}
