use std::hash::Hash;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::{excluded, on_constraint};
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use diesel_async::RunQueryDsl;
use orderly_dashboard_indexer::formats_external::trading_events::PurchaseSide;

use crate::db::{BATCH_UPSERT_LEN, DB_CONN_ERR_MSG, POOL};
use crate::schema::hourly_orderly_perp;

#[derive(Insertable, Queryable, QueryableByName, Debug, Clone)]
#[diesel(table_name = hourly_orderly_perp)]
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
    pub fn new_empty_hourly_orderly_perp(
        symbol: &str,
        block_hour: NaiveDateTime,
    ) -> HourlyOrderlyPerp {
        HourlyOrderlyPerp {
            symbol: symbol.to_string(),
            block_hour: block_hour,
            trading_fee: Default::default(),
            trading_volume: Default::default(),
            trading_count: 0,
            trading_user_count: 0,
            opening_count: 0,
            liquidation_amount: Default::default(),
            liquidation_count: 0,
            pulled_block_height: 0,
            pulled_block_time: Default::default(),
        }
    }
}

impl HourlyOrderlyPerp {
    pub fn new_liquidation(&mut self, liquidation_amount: BigDecimal, block_num: i64) {
        if block_num <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.liquidation_count += 1;
        self.liquidation_amount += liquidation_amount.abs();
    }
}

impl HourlyOrderlyPerp {
    pub fn new_trade(
        &mut self,
        fee: BigDecimal,
        amount: BigDecimal,
        pulled_block_height: i64,
        side: PurchaseSide,
    ) {
        if pulled_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.trading_fee += fee;
        // only count the buy part
        if side == PurchaseSide::Buy {
            self.trading_volume += amount.abs();
        }
        self.trading_count += 1;
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
) -> anyhow::Result<HourlyOrderlyPerp> {
    use crate::schema::hourly_orderly_perp::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result = hourly_orderly_perp
        .filter(symbol.eq(p_symbol.clone()))
        .filter(block_hour.eq(p_block_hour.clone()))
        .first::<HourlyOrderlyPerp>(&mut conn)
        .await;

    match select_result {
        Ok(hourly_data) => Ok(hourly_data),
        Err(error) => match error {
            diesel::NotFound => {
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
            _ => Err(anyhow::anyhow!(
                "find_hourly_orderly_perp execute err: {}",
                error
            )),
        },
    }
}

pub async fn create_or_update_hourly_orderly_perp(
    p_hourly_data_vec: Vec<&HourlyOrderlyPerp>,
) -> anyhow::Result<usize> {
    if p_hourly_data_vec.is_empty() {
        return Ok(0);
    }
    use crate::schema::hourly_orderly_perp::dsl::*;

    let mut row_nums = 0;
    let mut p_hourly_data_vec_ref = p_hourly_data_vec.as_slice();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    loop {
        if p_hourly_data_vec_ref.len() >= BATCH_UPSERT_LEN {
            let values1: &[&HourlyOrderlyPerp];
            (values1, p_hourly_data_vec_ref) = p_hourly_data_vec_ref.split_at(BATCH_UPSERT_LEN);
            #[allow(suspicious_double_ref_op)]
            let values1 = values1
                .iter()
                .map(|v| v.clone().clone())
                .collect::<Vec<_>>();
            let update_result = diesel::insert_into(hourly_orderly_perp)
                .values(values1)
                .on_conflict(on_constraint("hourly_orderly_perp_uq"))
                .do_update()
                .set((
                    trading_fee.eq(excluded(trading_fee)),
                    trading_volume.eq(excluded(trading_volume)),
                    trading_count.eq(excluded(trading_count)),
                    trading_user_count.eq(excluded(trading_user_count)),
                    opening_count.eq(excluded(opening_count)),
                    liquidation_amount.eq(excluded(liquidation_amount)),
                    liquidation_count.eq(excluded(liquidation_count)),
                    pulled_block_height.eq(excluded(pulled_block_height)),
                    pulled_block_time.eq(excluded(pulled_block_time)),
                ))
                .execute(&mut conn)
                .await;

            match update_result {
                Ok(len) => {
                    row_nums += len;
                }
                Err(err) => {
                    return Err(anyhow::anyhow!(
                        "update hourly_orderly_perp failed: {}",
                        err
                    ));
                }
            }
        } else {
            #[allow(suspicious_double_ref_op)]
            let values1 = p_hourly_data_vec_ref
                .iter()
                .map(|v| v.clone().clone())
                .collect::<Vec<_>>();
            let update_result = diesel::insert_into(hourly_orderly_perp)
                .values(values1)
                .on_conflict(on_constraint("hourly_orderly_perp_uq"))
                .do_update()
                .set((
                    trading_fee.eq(excluded(trading_fee)),
                    trading_volume.eq(excluded(trading_volume)),
                    trading_count.eq(excluded(trading_count)),
                    trading_user_count.eq(excluded(trading_user_count)),
                    opening_count.eq(excluded(opening_count)),
                    liquidation_amount.eq(excluded(liquidation_amount)),
                    liquidation_count.eq(excluded(liquidation_count)),
                    pulled_block_height.eq(excluded(pulled_block_height)),
                    pulled_block_time.eq(excluded(pulled_block_time)),
                ))
                .execute(&mut conn)
                .await;

            match update_result {
                Ok(len) => {
                    row_nums += len;
                    break;
                }
                Err(err) => {
                    return Err(anyhow::anyhow!(
                        "update hourly_orderly_perp failed: {}",
                        err
                    ));
                }
            }
        }
    }

    Ok(row_nums)
}
