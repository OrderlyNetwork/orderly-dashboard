use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::{excluded, on_constraint};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Insertable, Queryable};

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::QueryError;
use crate::db::{PrimaryKey, BATCH_UPSERT_LEN, POOL};
use crate::schema::hourly_orderly_token;

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name = "hourly_orderly_token"]
pub struct HourlyOrderlyToken {
    pub token: String,
    pub block_hour: NaiveDateTime,
    pub chain_id: String,

    pub withdraw_amount: BigDecimal,
    pub withdraw_count: i64,
    pub deposit_amount: BigDecimal,
    pub deposit_count: i64,

    pub pulled_block_height: i64,
    pub pulled_block_time: NaiveDateTime,
}

impl HourlyOrderlyToken {
    pub fn deposit(&mut self, p_deposit_amount: BigDecimal, p_block_height: i64) {
        if p_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.deposit_amount += p_deposit_amount;
        self.deposit_count += 1;
    }

    pub fn withdraw(&mut self, p_withdraw_amount: BigDecimal, p_block_height: i64) {
        if p_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.withdraw_amount += p_withdraw_amount;
        self.deposit_count += 1;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct HourlyOrderlyTokenKey {
    pub token: String,
    pub block_hour: NaiveDateTime,
    pub chain_id: String,
}

impl PrimaryKey for HourlyOrderlyTokenKey {}

pub async fn find_hourly_orderly_token(
    p_token: String,
    p_block_hour: NaiveDateTime,
    p_chain_id: String,
) -> Result<HourlyOrderlyToken, DBException> {
    use crate::schema::hourly_orderly_token::dsl::*;

    let select_result = hourly_orderly_token
        .filter(token.eq(p_token.clone()))
        .filter(block_hour.eq(p_block_hour.clone()))
        .filter(chain_id.eq(p_chain_id.clone()))
        .first_async::<HourlyOrderlyToken>(&POOL)
        .await;

    match select_result {
        Ok(hourly_data) => Ok(hourly_data),
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                let hourly_data = HourlyOrderlyToken {
                    token: p_token.clone(),
                    block_hour: p_block_hour.clone(),
                    chain_id: p_chain_id.clone(),
                    withdraw_amount: Default::default(),
                    withdraw_count: 0,
                    deposit_amount: Default::default(),
                    deposit_count: 0,
                    pulled_block_height: 0,
                    pulled_block_time: Default::default(),
                };
                Ok(hourly_data)
            }
            _ => Err(QueryError),
        },
    }
}

#[allow(dead_code)]
pub async fn create_or_update_hourly_orderly_token(
    p_hourly_data_vec: Vec<&HourlyOrderlyToken>,
) -> anyhow::Result<usize> {
    if p_hourly_data_vec.is_empty() {
        return Ok(0);
    }
    use crate::schema::hourly_orderly_token::dsl::*;

    let mut row_nums = 0;
    let mut p_hourly_data_vec = p_hourly_data_vec
        .into_iter()
        .cloned()
        .collect::<Vec<HourlyOrderlyToken>>();
    loop {
        if p_hourly_data_vec.len() >= BATCH_UPSERT_LEN {
            let (values1, res) = p_hourly_data_vec.split_at(BATCH_UPSERT_LEN);
            let values1 = values1
                .iter()
                .map(|v| v.clone())
                .collect::<Vec<HourlyOrderlyToken>>();
            p_hourly_data_vec = res.iter().cloned().collect::<Vec<HourlyOrderlyToken>>();
            let update_result = diesel::insert_into(hourly_orderly_token)
                .values(values1)
                .on_conflict(on_constraint("hourly_orderly_token_uq"))
                .do_update()
                .set((
                    withdraw_amount.eq(excluded(withdraw_amount)),
                    withdraw_count.eq(excluded(withdraw_count)),
                    deposit_amount.eq(excluded(deposit_amount)),
                    deposit_count.eq(excluded(deposit_count)),
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
                        "update hourly_orderly_token failed: {}",
                        err
                    ));
                }
            }
        } else {
            let update_result = diesel::insert_into(hourly_orderly_token)
                .values(p_hourly_data_vec)
                .on_conflict(on_constraint("hourly_orderly_token_uq"))
                .do_update()
                .set((
                    withdraw_amount.eq(excluded(withdraw_amount)),
                    withdraw_count.eq(excluded(withdraw_count)),
                    deposit_amount.eq(excluded(deposit_amount)),
                    deposit_count.eq(excluded(deposit_count)),
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
                        "update hourly_orderly_token failed: {}",
                        err
                    ));
                }
            }
        }
    }

    Ok(row_nums)
}
