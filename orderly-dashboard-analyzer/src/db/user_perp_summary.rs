use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::{InsertError, QueryError};
use crate::db::{PrimaryKey, POOL};
use crate::schema::user_perp_summary;

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "user_perp_summary"]
pub struct UserPerpSummary {
    pub account_id: String,
    pub symbol: String,

    pub holding: BigDecimal,
    pub opening_cost: BigDecimal,
    cost_position: BigDecimal,

    total_trading_volume: BigDecimal,
    total_trading_count: i64,
    total_trading_fee: BigDecimal,

    #[allow(dead_code)]
    total_realized_pnl: BigDecimal,
    #[allow(dead_code)]
    total_un_realized_pnl: BigDecimal,

    total_liquidation_amount: BigDecimal,
    total_liquidation_count: i64,

    pulled_block_height: i64,
    pulled_block_time: NaiveDateTime,

    pub sum_unitary_fundings: BigDecimal,
}

impl UserPerpSummary {
    pub fn new_liquidation(
        &mut self,
        qty: BigDecimal,
        price: BigDecimal,
        block_num: i64,
        block_time: NaiveDateTime,
        _cost_position_transfer: BigDecimal,
        _sum_unitary_funding: BigDecimal,
        open_cost_diff: BigDecimal,
    ) {
        self.holding -= qty.clone();
        self.total_liquidation_amount += qty.clone() * price.clone();
        self.total_liquidation_count += 1;
        self.pulled_block_height = block_num;
        self.pulled_block_time = block_time.clone();
        self.opening_cost += open_cost_diff;
    }

    pub fn new_settlemnt(&mut self, settlement_amount: BigDecimal) {
        self.cost_position += settlement_amount;
    }

    pub fn new_liquidator(&mut self, holding_diff: BigDecimal, opening_cost_diff: BigDecimal) {
        self.holding -= holding_diff.clone();
        self.opening_cost += opening_cost_diff;
    }

    pub fn charge_funding_fee(&mut self, new_sum_unitary_fundings: BigDecimal) {
        if new_sum_unitary_fundings.clone() == self.sum_unitary_fundings.clone() {
            return;
        }

        if self.holding == BigDecimal::from(0) {
            self.sum_unitary_fundings = new_sum_unitary_fundings;
            return;
        }

        let funding_fee = self.holding.clone()
            * (new_sum_unitary_fundings.clone() - self.sum_unitary_fundings.clone()).with_scale(6);

        self.cost_position += funding_fee;
        self.sum_unitary_fundings = new_sum_unitary_fundings;
    }
}

impl UserPerpSummary {
    pub fn new_trade(
        &mut self,
        fee: BigDecimal,
        amount: BigDecimal,
        pulled_block_height: i64,
        pulled_block_time: NaiveDateTime,
        open_cost_diff: BigDecimal,
        qty: BigDecimal,
    ) -> (bool, bool) {
        let is_opening = self.holding.clone() == Default::default()
            || (self.holding.clone().sign() != amount.clone().sign()
                && amount.clone().abs() > self.holding.clone().abs());

        let is_new_user = self.total_trading_count == 0;
        self.total_trading_fee += fee;
        self.total_trading_volume += amount.clone().abs();
        self.total_trading_count += 1;
        self.pulled_block_height = pulled_block_height;
        self.pulled_block_time = pulled_block_time;
        self.holding += qty;
        self.opening_cost += open_cost_diff;

        (is_opening, is_new_user)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct UserPerpSummaryKey {
    pub account_id: String,
    pub symbol: String,
}

impl UserPerpSummaryKey {
    pub fn new_key(account_id: String, symbol: String) -> UserPerpSummaryKey {
        UserPerpSummaryKey { account_id, symbol }
    }
}

impl PrimaryKey for UserPerpSummaryKey {}

pub async fn find_user_perp_summary(
    p_account_id: String,
    p_symbol: String,
) -> Result<UserPerpSummary, DBException> {
    use crate::schema::user_perp_summary::dsl::*;
    let select_result = user_perp_summary
        .filter(account_id.eq(p_account_id.clone()))
        .filter(symbol.eq(p_symbol.clone()))
        .first_async::<UserPerpSummary>(&POOL)
        .await;

    match select_result {
        Ok(perp_data) => Ok(perp_data),
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
                    pulled_block_time: Default::default(),
                    sum_unitary_fundings: BigDecimal::from(0),
                };

                Ok(new_perp)
            }
            _ => Err(QueryError),
        },
    }
}

pub async fn create_or_update_user_perp_summary(
    p_user_perp_summary_vec: Vec<&UserPerpSummary>,
) -> Result<usize, DBException> {
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
                pulled_block_time.eq(summary.pulled_block_time.clone()),
                sum_unitary_fundings.eq(summary.sum_unitary_fundings.clone()),
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
