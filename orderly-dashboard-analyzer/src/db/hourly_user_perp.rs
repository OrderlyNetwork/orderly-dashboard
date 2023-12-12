use actix_diesel::AsyncError;
use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::BigDecimal;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::{POOL, PrimaryKey};
use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::{InsertError, QueryError};
use crate::schema::hourly_user_perp;

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "hourly_user_perp"]
pub struct HourlyUserPerp {
    account_id: String,
    symbol: String,
    block_hour: i64,

    trading_fee: BigDecimal,
    trading_volume: BigDecimal,
    trading_count: i64,

    realized_pnl: BigDecimal,
    un_realized_pnl: BigDecimal,
    latest_sum_unitary_funding: BigDecimal,

    liquidation_amount: BigDecimal,
    liquidation_count: i64,

    pulled_block_height: i64,
    pulled_block_time: i64,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct HourlyUserPerpKey {
    account_id: String,
    symbol: String,
    block_hour: i64,
}

impl PrimaryKey for HourlyUserPerpKey {}


pub async fn find_hourly_user_perp(p_account_id: String, p_symbol: String, p_block_hour: i64) -> Result<HourlyUserPerp, DBException> {
    use crate::schema::hourly_user_perp::dsl::*;
    let select_result = hourly_user_perp
        .filter(account_id.eq(p_account_id.clone()))
        .filter(symbol.eq(p_symbol.clone()))
        .filter(block_hour.eq(p_block_hour.clone()))
        .first_async::<HourlyUserPerp>(&POOL)
        .await;

    match select_result {
        Ok(perp_data) => {
            Ok(perp_data)
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                let new_perp = HourlyUserPerp {
                    account_id: p_account_id.clone(),
                    symbol: p_symbol.clone(),
                    block_hour: p_block_hour.clone(),
                    trading_fee: Default::default(),
                    trading_volume: Default::default(),
                    trading_count: 0,
                    realized_pnl: Default::default(),
                    un_realized_pnl: Default::default(),
                    latest_sum_unitary_funding: Default::default(),
                    liquidation_amount: Default::default(),
                    liquidation_count: 0,
                    pulled_block_height: 0,
                    pulled_block_time: 0,
                };

                Ok(new_perp)
            }
            _ => {
                Err(QueryError)
            }
        }
    }
}

pub async fn create_or_update_hourly_user_perp(p_hourly_user_perp_vec: Vec<HourlyUserPerp>) -> Result<usize, DBException> {
    use crate::schema::hourly_user_perp::dsl::*;

    let mut row_nums = 0;
    for hourly_user_perp_data in p_hourly_user_perp_vec {
        let update_result = diesel::insert_into(hourly_user_perp)
            .values(hourly_user_perp_data.clone())
            .on_conflict(on_constraint("hourly_user_perp_uq"))
            .do_update()
            .set((
                trading_fee.eq(hourly_user_perp_data.trading_fee.clone()),
                trading_volume.eq(hourly_user_perp_data.trading_volume.clone()),
                trading_count.eq(hourly_user_perp_data.trading_count.clone()),
                realized_pnl.eq(hourly_user_perp_data.realized_pnl.clone()),
                un_realized_pnl.eq(hourly_user_perp_data.un_realized_pnl.clone()),
                latest_sum_unitary_funding.eq(hourly_user_perp_data.latest_sum_unitary_funding.clone()),
                liquidation_amount.eq(hourly_user_perp_data.liquidation_amount.clone()),
                liquidation_count.eq(hourly_user_perp_data.liquidation_count.clone()),
                pulled_block_height.eq(hourly_user_perp_data.pulled_block_height.clone()),
                pulled_block_time.eq(hourly_user_perp_data.pulled_block_time.clone())
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
    Ok(row_nums)
}



