use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::{excluded, on_constraint};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::analyzer::{get_qty_prec, get_unitary_prec};
use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::QueryError;
use crate::db::{PrimaryKey, BATCH_UPSERT_LEN, DB_CONN_ERR_MSG, POOL};
use crate::schema::user_perp_summary;
use std::str::FromStr;

#[derive(Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = user_perp_summary)]
pub struct UserPerpSummary {
    pub account_id: String,
    pub symbol: String,

    pub holding: BigDecimal,
    pub opening_cost: BigDecimal,
    pub cost_position: BigDecimal,

    total_trading_volume: BigDecimal,
    total_trading_count: i64,
    total_trading_fee: BigDecimal,

    #[allow(dead_code)]
    total_realized_pnl: BigDecimal,
    #[allow(dead_code)]
    total_un_realized_pnl: BigDecimal,

    total_liquidation_amount: BigDecimal,
    total_liquidation_count: i64,

    pub pulled_block_height: i64,
    pub pulled_block_time: NaiveDateTime,

    pub sum_unitary_fundings: BigDecimal,
}

impl UserPerpSummary {
    pub fn new_empty_user_perp_summary(p_account_id: &str, symbol: &str) -> UserPerpSummary {
        UserPerpSummary {
            account_id: p_account_id.to_string(),
            symbol: symbol.to_string(),
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
        }
    }

    pub fn new_liquidation(
        &mut self,
        qty: BigDecimal,
        price: BigDecimal,
        block_num: i64,
        _cost_position_transfer: BigDecimal,
        _sum_unitary_funding: BigDecimal,
        open_cost_diff: BigDecimal,
    ) {
        if block_num <= self.pulled_block_height {
            // already processed this block events
            return;
        }

        self.holding -= qty.clone();
        self.total_liquidation_amount += qty.clone() * price.clone();
        self.total_liquidation_count += 1;
        self.opening_cost += open_cost_diff;
    }

    pub fn new_liquidation_v2(
        &mut self,
        qty: BigDecimal,
        price: BigDecimal,
        block_num: i64,
        _cost_position_transfer: BigDecimal,
        _sum_unitary_funding: BigDecimal,
        open_cost_diff: BigDecimal,
    ) {
        if block_num <= self.pulled_block_height {
            // already processed this block events
            return;
        }

        self.holding += qty.clone();
        self.total_liquidation_amount += qty.clone() * price.clone();
        self.total_liquidation_count += 1;
        self.opening_cost += open_cost_diff;
    }

    pub fn new_settlemnt(&mut self, settlement_amount: BigDecimal, pulled_block_height: i64) {
        if pulled_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.cost_position += settlement_amount;
    }

    pub fn new_liquidator(
        &mut self,
        holding_diff: BigDecimal,
        opening_cost_diff: BigDecimal,
        pulled_block_height: i64,
    ) {
        if pulled_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.holding += holding_diff.clone();
        self.opening_cost += opening_cost_diff;
    }

    pub fn charge_funding_fee(
        &mut self,
        new_sum_unitary_fundings: BigDecimal,
        pulled_block_height: i64,
    ) {
        if pulled_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        if new_sum_unitary_fundings.clone() == self.sum_unitary_fundings.clone() {
            return;
        }

        if self.holding == BigDecimal::from(0) {
            self.sum_unitary_fundings = new_sum_unitary_fundings;
            return;
        }
        const FUNDING_MOVE_RIGHT_PRECISIONS: i128 = 10_i128.pow(17); // 1e17

        let holding_move8 = (self.holding.clone() * get_qty_prec()).with_scale(0);
        let holding_move8 = i128::from_str(&holding_move8.to_string()).unwrap();
        let sum_unitary_fundings_diff_move = ((new_sum_unitary_fundings.clone()
            - self.sum_unitary_fundings.clone())
            * get_unitary_prec())
        .with_scale(0);
        #[cfg(test)]
        println!(
            "holding_move6: {}, sum_unitary_fundings_diff: {}",
            holding_move8, sum_unitary_fundings_diff_move
        );
        let accrued_fee_unconverted =
            holding_move8 * i128::from_str(&sum_unitary_fundings_diff_move.to_string()).unwrap();
        let mut accrued_fee = accrued_fee_unconverted / FUNDING_MOVE_RIGHT_PRECISIONS;
        let remainder = accrued_fee_unconverted - (accrued_fee * FUNDING_MOVE_RIGHT_PRECISIONS);
        if remainder > 0 {
            accrued_fee += 1;
        }
        #[cfg(test)]
        println!("accrued_fee before div: {}", accrued_fee);

        let accrued_fee = BigDecimal::from_str(&accrued_fee.to_string()).unwrap() / get_qty_prec();
        #[cfg(test)]
        println!(
            "self.cost_position: {}, accrued_fee after div: {}",
            self.cost_position, accrued_fee
        );
        self.cost_position += accrued_fee;
        self.sum_unitary_fundings = new_sum_unitary_fundings;
    }

    pub fn new_user_adl_v1(
        &mut self,
        qty: BigDecimal,
        price: BigDecimal,
        block_num: i64,
        _cost_position_transfer: BigDecimal,
        _sum_unitary_funding: BigDecimal,
        open_cost_diff: BigDecimal,
    ) {
        if block_num <= self.pulled_block_height {
            // already processed this block events
            return;
        }

        self.holding += qty.clone();
        self.total_liquidation_amount += qty.clone() * price.clone();
        self.total_liquidation_count += 1;
        self.opening_cost += open_cost_diff;
    }

    pub fn new_insurance_adl_v1(
        &mut self,
        qty: BigDecimal,
        price: BigDecimal,
        block_num: i64,
        _cost_position_transfer: BigDecimal,
        _sum_unitary_funding: BigDecimal,
    ) {
        if block_num <= self.pulled_block_height {
            // already processed this block events
            return;
        }

        self.holding -= qty.clone();
        self.total_liquidation_amount += qty.clone() * price.clone();
        self.total_liquidation_count += 1;
    }

    pub fn new_user_adl_v2(
        &mut self,
        qty: BigDecimal,
        price: BigDecimal,
        block_num: i64,
        _cost_position_transfer: BigDecimal,
        _sum_unitary_funding: BigDecimal,
        open_cost_diff: BigDecimal,
    ) {
        if block_num <= self.pulled_block_height {
            // already processed this block events
            return;
        }

        self.holding += qty.clone();
        self.total_liquidation_amount += qty.clone() * price.clone();
        self.total_liquidation_count += 1;
        self.opening_cost += open_cost_diff;
    }
}

impl UserPerpSummary {
    pub fn new_trade(
        &mut self,
        fee: BigDecimal,
        amount: BigDecimal,
        pulled_block_height: i64,
        open_cost_diff: BigDecimal,
        qty: BigDecimal,
    ) -> (bool, bool) {
        if pulled_block_height <= self.pulled_block_height {
            tracing::warn!("deprecated trade for pulled_block_height: {}, self.pulled_block_height: {}, fee: {}", pulled_block_height, self.pulled_block_height, fee);
            // already processed this block events
            return (false, false);
        }

        let is_opening = self.holding.clone() == Default::default()
            || (self.holding.clone().sign() != amount.clone().sign()
                && amount.clone().abs() > self.holding.clone().abs());

        let is_new_user = self.total_trading_count == 0;
        self.total_trading_fee += fee;
        self.total_trading_volume += amount.clone().abs();
        self.total_trading_count += 1;
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
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result = user_perp_summary
        .filter(account_id.eq(p_account_id.clone()))
        .filter(symbol.eq(p_symbol.clone()))
        .first::<UserPerpSummary>(&mut conn)
        .await;

    match select_result {
        Ok(perp_data) => Ok(perp_data),
        Err(error) => match error {
            diesel::NotFound => {
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
) -> anyhow::Result<usize> {
    if p_user_perp_summary_vec.is_empty() {
        return Ok(0);
    }
    use crate::schema::user_perp_summary::dsl::*;

    let mut row_nums = 0;
    let mut user_perp_summary_vec = p_user_perp_summary_vec
        .into_iter()
        .cloned()
        .collect::<Vec<UserPerpSummary>>();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    loop {
        if user_perp_summary_vec.len() >= BATCH_UPSERT_LEN {
            let (values1, res) = user_perp_summary_vec.split_at(BATCH_UPSERT_LEN);
            let values1 = values1
                .iter()
                .map(|v| v.clone())
                .collect::<Vec<UserPerpSummary>>();
            user_perp_summary_vec = res.iter().cloned().collect::<Vec<UserPerpSummary>>();
            let update_result = diesel::insert_into(user_perp_summary)
                .values(values1)
                .on_conflict(on_constraint("user_perp_summary_uq"))
                .do_update()
                .set((
                    holding.eq(excluded(holding)),
                    opening_cost.eq(excluded(opening_cost)),
                    cost_position.eq(excluded(cost_position)),
                    total_trading_volume.eq(excluded(total_trading_volume)),
                    total_trading_fee.eq(excluded(total_trading_fee)),
                    total_trading_count.eq(excluded(total_trading_count)),
                    total_realized_pnl.eq(excluded(total_realized_pnl)),
                    total_un_realized_pnl.eq(excluded(total_un_realized_pnl)),
                    total_liquidation_amount.eq(excluded(total_liquidation_amount)),
                    total_liquidation_count.eq(excluded(total_liquidation_count)),
                    pulled_block_height.eq(excluded(pulled_block_height)),
                    pulled_block_time.eq(excluded(pulled_block_time)),
                    sum_unitary_fundings.eq(excluded(sum_unitary_fundings)),
                ))
                .execute(&mut conn)
                .await;

            match update_result {
                Ok(len) => {
                    row_nums += len;
                }
                Err(err) => {
                    return Err(anyhow::anyhow!("update user_perp_summary failed: {}", err));
                }
            }
        } else {
            let update_result = diesel::insert_into(user_perp_summary)
                .values(user_perp_summary_vec)
                .on_conflict(on_constraint("user_perp_summary_uq"))
                .do_update()
                .set((
                    holding.eq(excluded(holding)),
                    opening_cost.eq(excluded(opening_cost)),
                    cost_position.eq(excluded(cost_position)),
                    total_trading_volume.eq(excluded(total_trading_volume)),
                    total_trading_fee.eq(excluded(total_trading_fee)),
                    total_trading_count.eq(excluded(total_trading_count)),
                    total_realized_pnl.eq(excluded(total_realized_pnl)),
                    total_un_realized_pnl.eq(excluded(total_un_realized_pnl)),
                    total_liquidation_amount.eq(excluded(total_liquidation_amount)),
                    total_liquidation_count.eq(excluded(total_liquidation_count)),
                    pulled_block_height.eq(excluded(pulled_block_height)),
                    pulled_block_time.eq(excluded(pulled_block_time)),
                    sum_unitary_fundings.eq(excluded(sum_unitary_fundings)),
                ))
                .execute(&mut conn)
                .await;

            match update_result {
                Ok(len) => {
                    row_nums += len;
                    break;
                }
                Err(err) => {
                    return Err(anyhow::anyhow!("update user_perp_summary failed: {}", err));
                }
            }
        }
    }

    Ok(row_nums)
}

#[allow(dead_code)]
pub async fn legacy_create_or_update_user_perp_summary(
    p_user_perp_summary_vec: Vec<&UserPerpSummary>,
) -> anyhow::Result<usize> {
    if p_user_perp_summary_vec.is_empty() {
        return Ok(0);
    }
    use crate::schema::user_perp_summary::dsl::*;

    let mut row_nums = 0;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
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
            .execute(&mut conn)
            .await;

        match update_result {
            Ok(_) => {
                row_nums += 1;
            }
            Err(err) => {
                return Err(anyhow::anyhow!("update user_perp_summary failed: {}", err));
            }
        }
    }
    Ok(row_nums)
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, time::Instant};

    use super::*;
    use chrono::Utc;

    fn init_log() {
        tracing_subscriber::fmt::Subscriber::builder()
            .with_writer(std::io::stderr)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
    }
    #[ignore]
    #[actix_web::test]
    async fn test_create_or_update_user_perp_summary() {
        dotenv::dotenv().ok();
        init_log();
        let mut data = vec![];
        let update_time = NaiveDateTime::from_timestamp_opt(
            Utc::now().timestamp(),
            Utc::now().timestamp_subsec_nanos() as u32,
        )
        .unwrap();
        for i in 0..20 {
            data.push(UserPerpSummary {
                account_id: (i).to_string(),
                symbol: "0xaaaaa".to_string(),
                holding: (i * 10000).into(),
                opening_cost: (i * 10000).into(),
                cost_position: (i * 10000).into(),
                total_trading_volume: (i * 10000).into(),
                total_trading_count: (i * 10000).into(),
                total_trading_fee: (i * 10000).into(),
                total_realized_pnl: (i * 10000).into(),
                total_un_realized_pnl: (i * 10000).into(),
                total_liquidation_amount: (i * 10000).into(),
                total_liquidation_count: (i * 10000).into(),
                pulled_block_height: (i * 10000).into(),
                pulled_block_time: update_time,
                sum_unitary_fundings: (i * 10000).into(),
            });
        }
        let val = Vec::from_iter(data.iter());
        let inst1 = Instant::now();
        create_or_update_user_perp_summary(val.clone())
            .await
            .unwrap();
        let elapse1_ms = inst1.elapsed().as_millis();

        tracing::info!(
            "test_create_or_update_user_perp_summary elapse1_ms: {}",
            elapse1_ms,
        );

        // let inst2 = Instant::now();
        // legacy_create_or_update_user_perp_summary(val.clone())
        //     .await
        //     .unwrap();
        // let elapse2_ms = inst2.elapsed().as_millis();

        // tracing::info!(
        //     "test_create_or_update_user_perp_summary elapse1_ms: {}, elapse2_ms: {}",
        //     elapse1_ms,
        //     elapse2_ms
        // );
    }

    #[test]
    fn test_bigdecimal_scal() {
        let v = BigDecimal::from_str("3.213").unwrap();
        let v = v.with_scale(0);
        println!("test_bigdecimal_scal: {}", v);
    }
}
