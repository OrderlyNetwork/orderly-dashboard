use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::excluded;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use orderly_dashboard_indexer::formats_external::trading_events::PurchaseSide;

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::QueryError;
use crate::db::{BATCH_UPSERT_LEN, DB_CONN_ERR_MSG, POOL};
use crate::schema::orderly_perp_summary;

#[derive(Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = orderly_perp_summary)]
pub struct OrderlyPerpSummary {
    symbol: String,

    open_interest: BigDecimal,
    total_trading_volume: BigDecimal,
    total_trading_fee: BigDecimal,

    total_trading_count: i64,
    total_trading_user_count: i64,
    total_liquidation_amount: BigDecimal,
    total_liquidation_count: i64,

    pub pulled_block_height: i64,
    pub pulled_block_time: NaiveDateTime,

    buy_amount: BigDecimal,
    sell_amount: BigDecimal,
}

impl OrderlyPerpSummary {
    pub fn new_empty_orderly_perp_summary(symbol: &str) -> OrderlyPerpSummary {
        OrderlyPerpSummary {
            symbol: symbol.to_string(),
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
        }
    }
}

impl OrderlyPerpSummary {
    pub fn new_liquidation(&mut self, liquidation_amount: BigDecimal, block_num: i64) {
        if block_num <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.total_liquidation_amount += liquidation_amount;
        self.total_liquidation_count += 1;
    }
}

impl OrderlyPerpSummary {
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
        self.total_trading_fee += fee;
        self.total_trading_volume += amount.clone().abs();
        self.total_trading_count += 1;

        match side {
            PurchaseSide::Buy => {
                self.buy_amount += amount.clone();
            }
            PurchaseSide::Sell => {
                self.sell_amount -= amount.clone();
            }
        }
    }

    pub fn new_user(&mut self, pulled_block_height: i64) {
        if pulled_block_height <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.total_trading_user_count += 1;
    }
}

pub async fn find_orderly_perp_summary(
    p_symbol: String,
) -> Result<OrderlyPerpSummary, DBException> {
    use crate::schema::orderly_perp_summary::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let select_result = orderly_perp_summary
        .filter(symbol.eq(p_symbol.clone()))
        .first::<OrderlyPerpSummary>(&mut conn)
        .await;

    match select_result {
        Ok(perp_data) => Ok(perp_data),
        Err(error) => match error {
            diesel::NotFound => {
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
) -> anyhow::Result<usize> {
    if p_orderly_perp_summary_vec.is_empty() {
        return Ok(0);
    }
    use crate::schema::orderly_perp_summary::dsl::*;

    let mut row_nums = 0;
    let mut p_orderly_perp_summary_vec_ref = p_orderly_perp_summary_vec.as_slice();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    loop {
        if p_orderly_perp_summary_vec.len() >= BATCH_UPSERT_LEN {
            let values1: &[&OrderlyPerpSummary];
            (values1, p_orderly_perp_summary_vec_ref) =
                p_orderly_perp_summary_vec_ref.split_at(BATCH_UPSERT_LEN);
            #[allow(suspicious_double_ref_op)]
            let values1 = values1
                .iter()
                .map(|v| v.clone().clone())
                .collect::<Vec<OrderlyPerpSummary>>();
            let update_result = diesel::insert_into(orderly_perp_summary)
                .values(values1)
                .on_conflict(symbol)
                .do_update()
                .set((
                    open_interest.eq(excluded(open_interest)),
                    total_trading_volume.eq(excluded(total_trading_volume)),
                    total_trading_fee.eq(excluded(total_trading_fee)),
                    total_trading_count.eq(excluded(total_trading_count)),
                    total_trading_user_count.eq(excluded(total_trading_user_count)),
                    total_liquidation_amount.eq(excluded(total_liquidation_amount)),
                    total_liquidation_count.eq(excluded(total_liquidation_count)),
                    pulled_block_height.eq(excluded(pulled_block_height)),
                    pulled_block_time.eq(excluded(pulled_block_time)),
                    buy_amount.eq(excluded(buy_amount)),
                    sell_amount.eq(excluded(sell_amount)),
                ))
                .execute(&mut conn)
                .await;

            match update_result {
                Ok(len) => {
                    row_nums += len;
                }
                Err(err) => {
                    return Err(anyhow::anyhow!(
                        "update orderly_perp_summary failed: {}",
                        err
                    ));
                }
            }
        } else {
            #[allow(suspicious_double_ref_op)]
            let values1 = p_orderly_perp_summary_vec_ref
                .iter()
                .map(|v| v.clone().clone())
                .collect::<Vec<OrderlyPerpSummary>>();
            let update_result = diesel::insert_into(orderly_perp_summary)
                .values(values1)
                .on_conflict(symbol)
                .do_update()
                .set((
                    open_interest.eq(excluded(open_interest)),
                    total_trading_volume.eq(excluded(total_trading_volume)),
                    total_trading_fee.eq(excluded(total_trading_fee)),
                    total_trading_count.eq(excluded(total_trading_count)),
                    total_trading_user_count.eq(excluded(total_trading_user_count)),
                    total_liquidation_amount.eq(excluded(total_liquidation_amount)),
                    total_liquidation_count.eq(excluded(total_liquidation_count)),
                    pulled_block_height.eq(excluded(pulled_block_height)),
                    pulled_block_time.eq(excluded(pulled_block_time)),
                    buy_amount.eq(excluded(buy_amount)),
                    sell_amount.eq(excluded(sell_amount)),
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
                        "update orderly_perp_summary failed: {}",
                        err
                    ));
                }
            }
        }
    }

    Ok(row_nums)
}
