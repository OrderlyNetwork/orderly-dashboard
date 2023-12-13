use actix_diesel::AsyncError;
use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::BigDecimal;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::{POOL, PrimaryKey};
use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::{InsertError, QueryError};
use crate::schema::user_perp_summary;

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "user_perp_summary"]
pub struct UserPerpSummary {
    account_id: String,
    symbol: String,

    holding: BigDecimal,
    opening_cost: BigDecimal,
    cost_position: BigDecimal,

    total_trading_volume: BigDecimal,
    total_trading_count: i64,
    total_trading_fee: BigDecimal,

    total_realized_pnl: BigDecimal,
    total_un_realized_pnl: BigDecimal,

    total_liquidation_amount: BigDecimal,
    total_liquidation_count: i64,

    pulled_block_height: i64,
    pulled_block_time: i64,
}

impl UserPerpSummary {
    pub fn new_trade(&mut self, fee: BigDecimal, amount: BigDecimal, pulled_block_height: i64, pulled_block_time: i64) -> (bool, bool) {
        let is_opening = self.holding.clone() == Default::default()
            || (self.holding.clone().sign() != amount.clone().sign()
            && amount.clone().abs() > self.holding.clone().abs()
        );

        let is_new_user = self.total_trading_count == 0;
        self.total_trading_fee += fee;
        self.total_trading_volume += amount.clone().abs();
        self.total_trading_count += 1;
        self.pulled_block_height = pulled_block_height;
        self.pulled_block_time = pulled_block_time;
        self.holding += amount;

        (is_opening, is_new_user)
    }
}


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct UserPerpSummaryKey {
    pub account_id: String,
    pub symbol: String,
}

impl PrimaryKey for UserPerpSummaryKey {}


pub async fn find_user_perp_summary(p_account_id: String, p_symbol: String) -> Result<UserPerpSummary, DBException> {
    use crate::schema::user_perp_summary::dsl::*;
    let select_result = user_perp_summary
        .filter(account_id.eq(p_account_id.clone()))
        .filter(symbol.eq(p_symbol.clone()))
        .first_async::<UserPerpSummary>(&POOL)
        .await;

    match select_result {
        Ok(perp_data) => {
            Ok(perp_data)
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                let new_perp = UserPerpSummary {
                    account_id: p_account_id.clone(),
                    symbol: p_symbol.clone(),
                    holding: Default::default(),
                    opening_cost: Default::default(),
                    cost_position: Default::default(),
                    total_trading_volume: Default::default(),
                    total_trading_fee: Default::default(),
                    total_realized_pnl: Default::default(),
                    total_un_realized_pnl: Default::default(),
                    total_trading_count: 0,
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

pub async fn create_or_update_user_perp_summary(p_user_perp_summary_vec: Vec<&UserPerpSummary>) -> Result<usize, DBException> {
    use crate::schema::user_perp_summary::dsl::*;

    let mut row_nums = 0;
    for summary in p_user_perp_summary_vec {
        let update_result = diesel::insert_into(user_perp_summary)
            .values(summary.clone())
            .on_conflict(on_constraint("user_perp_summary_uq"))
            .do_update()
            .set((
                holding.eq(summary.holding.clone()),
                opening_cost.eq(summary.opening_cost.clone()),
                cost_position.eq(summary.cost_position.clone()),
                total_trading_volume.eq(summary.total_trading_volume.clone()),
                total_trading_fee.eq(summary.total_trading_fee.clone()),
                total_trading_count.eq(summary.total_trading_count.clone()),
                total_realized_pnl.eq(summary.total_realized_pnl.clone()),
                total_un_realized_pnl.eq(summary.total_un_realized_pnl.clone()),
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
