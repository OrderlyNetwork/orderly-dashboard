use std::hash::Hash;

use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Insertable, Queryable};

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::{InsertError, QueryError, Timeout};
use crate::db::POOL;
use crate::schema::hourly_orderly_perp;

#[derive(Insertable, Queryable, QueryableByName, Debug, Clone)]
#[table_name = "hourly_orderly_perp"]
pub struct HourlyOrderlyPerp {
    pub symbol: String,
    pub block_hour: NaiveDateTime,

    pub trading_fee: BigDecimal,
    pub trading_volume: BigDecimal,

    pub trading_count: i64,
    pub trading_user_count: i64,
    pub opening_count: i64,

    pub liquidation_amount: BigDecimal,
    pub liquidation_count: i64,

    pub pulled_block_height: i64,
    pub pulled_block_time: NaiveDateTime,
}

impl HourlyOrderlyPerp {
    pub fn new_adl(
        &mut self,
        adl_qty: BigDecimal,
        adl_price: BigDecimal,
        block_num: i64,
        block_time: NaiveDateTime,
    ) {
        self.liquidation_count += 1;
        self.liquidation_amount += adl_qty * adl_price;
        self.pulled_block_time = block_time;
        self.pulled_block_height = block_num;
    }
}

impl HourlyOrderlyPerp {
    pub fn new_trade(
        &mut self,
        fee: BigDecimal,
        amount: BigDecimal,
        pulled_block_height: i64,
        pulled_block_time: NaiveDateTime,
    ) {
        self.trading_fee += fee;
        self.trading_volume += amount.abs();
        self.trading_count += 1;
        self.pulled_block_height = pulled_block_height;
        self.pulled_block_time = pulled_block_time;
    }

    pub fn new_opening(&mut self) {
        self.opening_count += 1;
    }

    pub fn new_user(&mut self) {
        self.trading_user_count += 1;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct HourlyOrderlyPerpKey {
    pub symbol: String,
    pub block_hour: NaiveDateTime,
}

impl HourlyOrderlyPerpKey {
    pub fn new_key(symbol: String, block_hour: NaiveDateTime) -> Self {
        HourlyOrderlyPerpKey { symbol, block_hour }
    }
}

pub async fn find_hourly_orderly_perp(
    p_symbol: String,
    p_block_hour: NaiveDateTime,
) -> Result<HourlyOrderlyPerp, DBException> {
    use crate::schema::hourly_orderly_perp::dsl::*;
    let select_result = hourly_orderly_perp
        .filter(symbol.eq(p_symbol.clone()))
        .filter(block_hour.eq(p_block_hour.clone()))
        .first_async::<HourlyOrderlyPerp>(&POOL)
        .await;

    match select_result {
        Ok(hourly_data) => Ok(hourly_data),
        Err(error) => match error {
            AsyncError::Timeout(_) => Err(Timeout),
            AsyncError::Execute(Error::NotFound) => {
                let new_hourly_data = HourlyOrderlyPerp {
                    symbol: p_symbol.clone(),
                    block_hour: p_block_hour,
                    trading_fee: Default::default(),
                    trading_volume: Default::default(),
                    trading_count: 0,
                    trading_user_count: 0,
                    opening_count: 0,
                    liquidation_amount: Default::default(),
                    liquidation_count: 0,
                    pulled_block_height: 0,
                    pulled_block_time: Default::default(),
                };
                Ok(new_hourly_data)
            }
            _ => Err(QueryError),
        },
    }
}

pub async fn create_or_update_hourly_orderly_perp(
    p_hourly_data_vec: Vec<&HourlyOrderlyPerp>,
) -> Result<usize, DBException> {
    use crate::schema::hourly_orderly_perp::dsl::*;
    let mut row_nums = 0;
    for hourly_data in p_hourly_data_vec {
        let update_result = diesel::insert_into(hourly_orderly_perp)
            .values(hourly_data.clone())
            .on_conflict(on_constraint("hourly_orderly_perp_uq"))
            .do_update()
            .set((
                trading_fee.eq(hourly_data.trading_fee.clone()),
                trading_volume.eq(hourly_data.trading_volume.clone()),
                trading_count.eq(hourly_data.trading_count.clone()),
                trading_user_count.eq(hourly_data.trading_user_count.clone()),
                opening_count.eq(hourly_data.opening_count),
                liquidation_amount.eq(hourly_data.liquidation_amount.clone()),
                liquidation_count.eq(hourly_data.liquidation_count.clone()),
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
