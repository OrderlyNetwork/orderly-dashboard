use actix_diesel::AsyncError;
use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::BigDecimal;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::DB_CONTEXT;
use crate::db::POOL;
use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::{InsertError, QueryError};
use crate::schema::hourly_user_token;

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "hourly_user_token"]
pub struct HourlyUserToken {
    pub account_id: String,
    pub token: String,
    pub block_hour: i64,
    pub chain_id: String,

    pub withdraw_amount: BigDecimal,
    pub withdraw_count: i64,
    pub deposit_amount: BigDecimal,
    pub deposit_count: i64,

    pub pulled_block_height: i64,
    pub pulled_block_time: i64,
}

pub async fn select_by_key(ori_account_id: String, ori_token: String, ori_block_hour: i64, ori_chain_id: String) -> Result<Option<HourlyUserToken>, DBException> {
    use crate::schema::hourly_user_token::dsl::*;
    let result = hourly_user_token
        .filter(account_id.eq(ori_account_id))
        .filter(token.eq(ori_token))
        .filter(block_hour.eq(ori_block_hour))
        .filter(chain_id.eq(ori_chain_id))
        .first_async::<HourlyUserToken>(&POOL)
        .await;

    match result {
        Ok(res) => {
            Ok(Some(res))
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                Ok(None)
            }
            _ => {
                Err(QueryError)
            }
        }
    }
}

pub async fn create_or_update_hourly_data(hourly_data_vec: Vec<HourlyUserToken>) -> Result<usize, DBException> {
    use crate::schema::hourly_user_token::dsl::*;

    let mut row_nums = 0;
    for hourly_data in hourly_data_vec {
        let result = diesel::insert_into(hourly_user_token)
            .values(hourly_data.clone())
            .on_conflict(on_constraint("hourly_user_token_uq"))
            .do_update()
            .set((
                withdraw_amount.eq(hourly_data.withdraw_amount),
                withdraw_count.eq(hourly_data.withdraw_count),
                deposit_amount.eq(hourly_data.deposit_amount),
                deposit_count.eq(hourly_data.deposit_count),
                pulled_block_height.eq(hourly_data.pulled_block_height),
                pulled_block_time.eq(hourly_data.pulled_block_time)
            ))
            .execute_async(&POOL)
            .await;

        match result {
            Ok(_) => { row_nums += 1 }
            Err(error) => {
                tracing::warn!(target: DB_CONTEXT,"create_or_update_hourly_data error. err:{:?}",error);
                return Err(InsertError);
            }
        }
    }
    return Ok(row_nums);
}