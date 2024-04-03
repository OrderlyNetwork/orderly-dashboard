use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::{InsertError, QueryError, Timeout};
use crate::db::POOL;
use crate::schema::hourly_gas_fee;

#[derive(Insertable, Queryable, QueryableByName, Debug, Clone)]
#[table_name = "hourly_gas_fee"]
pub struct HourlyGasFee {
    pub block_hour: NaiveDateTime,

    pub gas_fee: BigDecimal,
    pub event_type: String,

    pub batch_count: i64,
    pub pulled_block_height: i64,
    pub pulled_block_time: NaiveDateTime,
}

impl HourlyGasFee {
    pub fn new_event(
        &mut self,
        fee: BigDecimal,
        p_pulled_block_height: i64,
        p_pulled_block_time: NaiveDateTime,
    ) {
        self.gas_fee += fee;
        self.batch_count += 1;
        self.pulled_block_height = p_pulled_block_height;
        self.pulled_block_time = p_pulled_block_time;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct HourlyGasFeeKey {
    pub block_hour: NaiveDateTime,
    pub event_type: String,
}

impl HourlyGasFeeKey {
    pub fn new_key(event_type: String, block_hour: NaiveDateTime) -> Self {
        HourlyGasFeeKey {
            block_hour,
            event_type,
        }
    }
}

pub async fn find_hourly_gas_fee(
    p_event_type: String,
    p_block_hour: NaiveDateTime,
) -> Result<HourlyGasFee, DBException> {
    use crate::schema::hourly_gas_fee::dsl::*;
    let select_result = hourly_gas_fee
        .filter(event_type.eq(p_event_type.clone()))
        .filter(block_hour.eq(p_block_hour.clone()))
        .first_async::<HourlyGasFee>(&POOL)
        .await;

    match select_result {
        Ok(hourly_data) => Ok(hourly_data),
        Err(error) => match error {
            AsyncError::Timeout(_) => Err(Timeout),
            AsyncError::Execute(Error::NotFound) => {
                let new_hourly_data = HourlyGasFee {
                    event_type: p_event_type.clone(),
                    block_hour: p_block_hour,
                    gas_fee: Default::default(),
                    batch_count: 0,
                    pulled_block_height: 0,
                    pulled_block_time: Default::default(),
                };
                Ok(new_hourly_data)
            }
            _ => Err(QueryError),
        },
    }
}

pub async fn create_or_update_hourly_gas_fee(
    p_hourly_data_vec: Vec<&HourlyGasFee>,
) -> Result<usize, DBException> {
    use crate::schema::hourly_gas_fee::dsl::*;
    let mut row_nums = 0;
    for hourly_data in p_hourly_data_vec {
        let update_result = diesel::insert_into(hourly_gas_fee)
            .values(hourly_data.clone())
            .on_conflict(on_constraint("hourly_block_type"))
            .do_update()
            .set((
                gas_fee.eq(hourly_data.gas_fee.clone()),
                batch_count.eq(hourly_data.batch_count.clone()),
                pulled_block_height.eq(hourly_data.pulled_block_height.clone()),
                pulled_block_time.eq(hourly_data.pulled_block_time.clone()),
            ))
            .execute_async(&POOL)
            .await;

        match update_result {
            Ok(_) => {
                row_nums += 1;
            }
            Err(erro) => {
                println!(":{}", erro);
                return Err(InsertError);
            }
        }
    }
    return Ok(row_nums);
}
