use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Insertable, Queryable};

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::{InsertError, QueryError};
use crate::db::{PrimaryKey, POOL};
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
        if p_block_height < self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.deposit_amount += p_deposit_amount;
        self.deposit_count += 1;
    }

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

pub async fn create_or_update_hourly_orderly_token(
    p_hourly_data_vec: Vec<&HourlyOrderlyToken>,
) -> Result<usize, DBException> {
    use crate::schema::hourly_orderly_token::dsl::*;
    let mut row_nums = 0;
    for hourly_data in p_hourly_data_vec {
        let update_result = diesel::insert_into(hourly_orderly_token)
            .values(hourly_data.clone())
            .on_conflict(on_constraint("hourly_orderly_token_uq"))
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

        match update_result {
            Ok(_) => {
                row_nums += 1;
            }
            Err(_) => {
                return Err(InsertError);
            }
        }
    }
    return Ok(row_nums);
}
