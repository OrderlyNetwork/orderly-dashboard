use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::{InsertError, QueryError};
use crate::db::POOL;
use crate::db::{PrimaryKey, DB_CONTEXT};
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
    #[allow(duplicate_macro_attributes)]
    pub fn deposit(&mut self, p_deposit_amount: BigDecimal, p_block_height: i64) {
        if p_block_height < self.pulled_block_height {
            // already processed this block events
            return;
        }

        self.deposit_amount += p_deposit_amount;
        self.deposit_count += 1;
    }
    #[allow(duplicate_macro_attributes)]
    pub fn withdraw(&mut self, p_withdraw_amount: BigDecimal, p_block_height: i64) {
        if p_block_height < self.pulled_block_height {
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
    hourly_data_vec: Vec<&HourlyUserToken>,
) -> Result<usize, DBException> {
    use crate::schema::hourly_user_token::dsl::*;

    let mut row_nums = 0;
    for hourly_data in hourly_data_vec {
        let result = diesel::insert_into(hourly_user_token)
            .values(hourly_data.clone())
            .on_conflict(on_constraint("hourly_user_token_uq"))
            .do_update()
            .set((
                withdraw_amount.eq(hourly_data.withdraw_amount.clone()),
                withdraw_count.eq(hourly_data.withdraw_count.clone()),
                deposit_amount.eq(hourly_data.deposit_amount.clone()),
                deposit_count.eq(hourly_data.deposit_count.clone()),
                pulled_block_height.eq(hourly_data.pulled_block_height.clone()),
                pulled_block_time.eq(hourly_data.pulled_block_time.clone()),
            ))
            .execute_async(&POOL)
            .await;

        match result {
            Ok(_) => row_nums += 1,
            Err(error) => {
                tracing::warn!(target: DB_CONTEXT,"create_or_update_hourly_data error. err:{:?}",error);
                return Err(InsertError);
            }
        }
    }
    return Ok(row_nums);
}
