use actix_diesel::AsyncError;
use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::POOL;
use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::{InsertError, QueryError};
use crate::schema::orderly_perp_summary;

#[derive(Queryable, Insertable, Debug,Clone)]
#[table_name = "orderly_perp_summary"]
pub struct OrderlyPerpSummary {
    symbol: String,

    open_interest: BigDecimal,
    total_trading_volume: BigDecimal,
    total_trading_fee: BigDecimal,

    total_trading_count: i64,
    total_trading_user_count: i64,
    total_liquidation_amount: BigDecimal,
    total_liquidation_count: i64,

    pulled_block_height: i64,
    pulled_block_time: i64,
}


pub async fn find_hourly_user_perp(p_symbol: String) -> Result<OrderlyPerpSummary, DBException> {
    use crate::schema::orderly_perp_summary::dsl::*;
    let select_result = orderly_perp_summary
        .filter(symbol.eq(p_symbol.clone()))
        .first_async::<OrderlyPerpSummary>(&POOL)
        .await;

    match select_result {
        Ok(perp_data) => {
            Ok(perp_data)
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                let new_perp = OrderlyPerpSummary {
                    symbol: p_symbol.clone(),
                    open_interest: Default::default(),
                    total_trading_volume: Default::default(),
                    total_trading_fee: Default::default(),
                    total_trading_count: 0,
                    total_trading_user_count: 0,
                    total_liquidation_amount: Default::default(),
                    total_liquidation_count: 0,
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

pub async fn create_or_update_hourly_user_perp(p_orderly_perp_summary_vec: Vec<OrderlyPerpSummary>) -> Result<usize, DBException> {
    use crate::schema::orderly_perp_summary::dsl::*;

    let mut row_nums = 0;
    for summary in p_orderly_perp_summary_vec {
        let update_result = diesel::insert_into(orderly_perp_summary)
            .values(summary.clone())
            .on_conflict(symbol)
            .do_update()
            .set((
                open_interest.eq(summary.open_interest.clone()),
                total_trading_volume.eq(summary.total_trading_volume.clone()),
                total_trading_fee.eq(summary.total_trading_fee.clone()),
                total_trading_count.eq(summary.total_trading_count.clone()),
                total_trading_user_count.eq(summary.total_trading_user_count.clone()),
                total_liquidation_amount.eq(summary.total_liquidation_amount.clone()),
                total_liquidation_count.eq(summary.total_liquidation_count.clone()),
                pulled_block_height.eq(summary.pulled_block_height.clone()),
                pulled_block_time.eq(summary.pulled_block_time.clone())
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
