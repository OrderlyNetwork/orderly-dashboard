use actix_diesel::AsyncError;
use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::BigDecimal;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::{POOL, PrimaryKey};
use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::{InsertError, QueryError};
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
    pub pulled_block_time: i64,
}

impl OrderlyTokenSummary {
    pub fn deposit(&mut self, p_deposit_amount: BigDecimal) {
        self.total_deposit_amount += p_deposit_amount.clone();
        self.total_deposit_count += 1;
        self.balance += p_deposit_amount.clone();
    }

    pub fn withdraw(&mut self, p_withdraw_amount: BigDecimal) {
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

pub async fn find_orderly_token_summary(p_token: String, p_chain_id: String) -> Result<OrderlyTokenSummary, DBException> {
    use crate::schema::orderly_token_summary::dsl::*;
    let select_result = orderly_token_summary
        .filter(token.eq(p_token.clone()))
        .filter(chain_id.eq(p_chain_id.clone()))
        .first_async::<OrderlyTokenSummary>(&POOL)
        .await;

    match select_result {
        Ok(hourly_data) => {
            Ok(hourly_data)
        }
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
                    pulled_block_time: 0,
                };
                Ok(hourly_data)
            }
            _ => {
                Err(QueryError)
            }
        }
    }
}

pub async fn create_or_update_orderly_token_summary(p_hourly_data_vec: Vec<OrderlyTokenSummary>) -> Result<usize, DBException> {
    use crate::schema::orderly_token_summary::dsl::*;
    let mut row_nums = 0;
    for summary_data in p_hourly_data_vec {
        let update_result = diesel::insert_into(orderly_token_summary)
            .values(summary_data.clone())
            .on_conflict(on_constraint("orderly_token_summary_uq"))
            .do_update()
            .set((
                balance.eq(summary_data.balance.clone()),
                total_withdraw_amount.eq(summary_data.total_withdraw_amount.clone()),
                total_withdraw_count.eq(summary_data.total_withdraw_count.clone()),
                total_deposit_amount.eq(summary_data.total_deposit_amount.clone()),
                total_deposit_count.eq(summary_data.total_deposit_count.clone()),
                pulled_block_height.eq(summary_data.pulled_block_height.clone()),
                pulled_block_time.eq(summary_data.pulled_block_time.clone()),
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
