use actix_diesel::AsyncError;
use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use orderly_dashboard_indexer::formats_external::trading_events::PurchaseSide;

use crate::db::POOL;
use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::{InsertError, QueryError};
use crate::schema::orderly_perp_summary;

#[derive(Queryable, Insertable, Debug, Clone)]
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
    pulled_block_time: NaiveDateTime,

    buy_amount: BigDecimal,
    sell_amount: BigDecimal,
}

impl OrderlyPerpSummary {
    pub fn new_liquidation(
        &mut self,
        liquidation_amount: BigDecimal,
        block_num: i64,
        block_time: NaiveDateTime,
    ) {
        self.total_liquidation_amount += liquidation_amount;
        self.total_liquidation_count += 1;
        self.pulled_block_height = block_num;
        self.pulled_block_time = block_time;
    }
}

impl OrderlyPerpSummary {
    pub fn new_trade(
        &mut self,
        fee: BigDecimal,
        amount: BigDecimal,
        pulled_block_height: i64,
        pulled_block_time: NaiveDateTime,
        side: PurchaseSide,
    ) {
        self.total_trading_fee += fee;
        self.total_trading_volume += amount.clone().abs();
        self.total_trading_count += 1;
        self.pulled_block_height = pulled_block_height;
        self.pulled_block_time = pulled_block_time;

        match side {
            PurchaseSide::Buy => {
                self.buy_amount += amount.clone();
            }
            PurchaseSide::Sell => {
                self.sell_amount -= amount.clone();
            }
        }
    }

    pub fn new_user(&mut self) {
        self.total_trading_user_count += 1;
    }
}

pub async fn find_orderly_perp_summary(
    p_symbol: String,
) -> Result<OrderlyPerpSummary, DBException> {
    use crate::schema::orderly_perp_summary::dsl::*;
    let select_result = orderly_perp_summary
        .filter(symbol.eq(p_symbol.clone()))
        .first_async::<OrderlyPerpSummary>(&POOL)
        .await;

    match select_result {
        Ok(perp_data) => Ok(perp_data),
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
                    pulled_block_time: Default::default(),
                    buy_amount: Default::default(),
                    sell_amount: Default::default(),
                };

                Ok(new_perp)
            }
            _ => Err(QueryError),
        },
    }
}

pub async fn create_or_update_orderly_perp_summary(
    p_orderly_perp_summary_vec: Vec<&OrderlyPerpSummary>,
) -> Result<usize, DBException> {
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
                pulled_block_time.eq(summary.pulled_block_time.clone()),
                buy_amount.eq(summary.buy_amount.clone()),
                sell_amount.eq(summary.sell_amount.clone()),
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
