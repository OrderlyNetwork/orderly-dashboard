use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::{excluded, on_constraint};
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::QueryError;
use crate::db::POOL;
use crate::db::{PrimaryKey, BATCH_UPSERT_LEN};
use crate::schema::hourly_user_token;

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "hourly_user_token"]
pub struct HourlyUserToken {
    pub account_id: String,
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

impl HourlyUserToken {
    pub fn new_emtpy_hourly_user_token(
        account_id: &str,
        token: &str,
        block_hour: NaiveDateTime,
        chain_id: &str,
    ) -> HourlyUserToken {
        HourlyUserToken {
            account_id: account_id.to_string(),
            token: token.to_string(),
            block_hour: block_hour,
            chain_id: chain_id.to_string(),
            withdraw_amount: Default::default(),
            withdraw_count: 0,
            deposit_amount: Default::default(),
            deposit_count: 0,
            pulled_block_height: 0,
            pulled_block_time: Default::default(),
        }
    }
}

impl HourlyUserToken {
    #[allow(duplicate_macro_attributes)]
    pub fn deposit(&mut self, p_deposit_amount: BigDecimal, p_block_height: i64) {
        if p_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }

        self.deposit_amount += p_deposit_amount;
        self.deposit_count += 1;
    }
    #[allow(duplicate_macro_attributes)]
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
pub struct HourlyUserTokenKey {
    pub account_id: String,
    pub token: String,
    pub block_hour: NaiveDateTime,
    pub chain_id: String,
}

impl PrimaryKey for HourlyUserTokenKey {}

pub async fn find_hourly_user_token(
    ori_account_id: String,
    ori_token: String,
    ori_block_hour: NaiveDateTime,
    ori_chain_id: String,
) -> Result<HourlyUserToken, DBException> {
    use crate::schema::hourly_user_token::dsl::*;
    let result = hourly_user_token
        .filter(account_id.eq(ori_account_id.clone()))
        .filter(token.eq(ori_token.clone()))
        .filter(block_hour.eq(ori_block_hour.clone()))
        .filter(chain_id.eq(ori_chain_id.clone()))
        .first_async::<HourlyUserToken>(&POOL)
        .await;

    match result {
        Ok(res) => Ok(res),
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                let hourly_data = HourlyUserToken {
                    account_id: ori_account_id.clone(),
                    token: ori_token.clone(),
                    block_hour: ori_block_hour.clone(),
                    chain_id: ori_chain_id.clone(),
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

pub async fn create_or_update_hourly_user_token(
    mut hourly_data_vec: Vec<HourlyUserToken>,
) -> anyhow::Result<usize> {
    #[cfg(test)]
    tracing::info!("start create_or_update_hourly_user_token....");

    if hourly_data_vec.is_empty() {
        return Ok(0);
    }
    use crate::schema::hourly_user_token::dsl::*;

    let mut row_nums = 0;
    loop {
        if hourly_data_vec.len() >= BATCH_UPSERT_LEN {
            let (values1, res) = hourly_data_vec.split_at(BATCH_UPSERT_LEN);
            let values1 = values1
                .iter()
                .map(|v| v.clone())
                .collect::<Vec<HourlyUserToken>>();
            hourly_data_vec = res.iter().cloned().collect::<Vec<HourlyUserToken>>();
            let update_result = diesel::insert_into(hourly_user_token)
                .values(values1)
                .on_conflict(on_constraint("hourly_user_token_uq"))
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
                    return Err(anyhow::anyhow!("update hourly_user_token failed: {}", err));
                }
            }
        } else {
            let update_result = diesel::insert_into(hourly_user_token)
                .values(hourly_data_vec)
                .on_conflict(on_constraint("hourly_user_token_uq"))
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
                        "update create_or_update_hourly_user_token failed: {}",
                        err
                    ));
                }
            }
        }
    }

    #[cfg(test)]
    tracing::info!("finish create_or_update_hourly_user_token....");

    Ok(row_nums)
}
