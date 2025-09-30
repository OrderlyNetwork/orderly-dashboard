use crate::db::{DB_CONN_ERR_MSG, DB_CONTEXT, POOL};
use crate::formats_external::trading_events::AccoutTradingCursor;
use crate::schema::liquidation_transfer;
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::Numeric;
use diesel::QueryDsl;
use diesel::{sql_query, ExpressionMethods};
use diesel::{Insertable, Queryable};
use diesel_async::RunQueryDsl;
use num_traits::ToPrimitive;
use std::collections::BTreeSet;
use std::time::Instant;

#[derive(Insertable, Queryable, QueryableByName, Debug, Clone)]
#[diesel(table_name = liquidation_transfer)]
pub struct DbLiquidationTransfer {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub liquidation_result_log_idx: i32,
    pub transaction_id: String,
    pub liquidation_transfer_id: BigDecimal,
    // reused as accountId in LiquidationTransferV2
    pub liquidator_account_id: String,
    pub symbol_hash: String,
    pub position_qty_transfer: BigDecimal,
    pub cost_position_transfer: BigDecimal,
    pub liquidator_fee: BigDecimal,
    pub insurance_fee: BigDecimal,
    pub mark_price: BigDecimal,
    pub sum_unitary_fundings: BigDecimal,
    // reuse as fee in LiquidationTransferV2
    pub liquidation_fee: BigDecimal,
    pub block_time: Option<BigDecimal>,
    pub version: Option<i16>,
}

#[derive(Debug, Copy, Clone)]
pub enum LiquidationTransferVersion {
    V1 = 1,
    V2 = 2,
}

impl LiquidationTransferVersion {
    pub fn value(&self) -> i16 {
        *self as i16
    }
}

impl TryFrom<i16> for LiquidationTransferVersion {
    type Error = anyhow::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 | 1 => Ok(Self::V1),
            2 => Ok(Self::V2),
            _ => Err(anyhow::anyhow!(
                "cannot convert integer:{} to LiquidationTransferVersion",
                value
            )),
        }
    }
}

impl DbLiquidationTransfer {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.liquidation_result_log_idx)
    }

    pub fn is_result_log_set(&self) -> bool {
        self.liquidation_result_log_idx >= 0
    }
}

pub async fn create_liquidation_transfers(
    liquidation_transfers: Vec<DbLiquidationTransfer>,
) -> Result<usize> {
    use crate::schema::liquidation_transfer::dsl::*;
    if liquidation_transfers.is_empty() {
        return Ok(0);
    }
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let num_rows = diesel::insert_into(liquidation_transfer)
        .values(liquidation_transfers)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}

pub async fn query_liquidation_transfers(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<DbLiquidationTransfer>> {
    use crate::schema::liquidation_transfer::dsl::*;
    let start_time = Instant::now();

    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result = liquidation_transfer
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load::<DbLiquidationTransfer>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_liquidation_transfers success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_liquidation_transfers success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_liquidation_transfers fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_liquidation_transfers_with_time(
    from_block: i64,
    to_block: i64,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbLiquidationTransfer>> {
    use crate::schema::liquidation_transfer::dsl::*;
    let start_time = Instant::now();

    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result = liquidation_transfer
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .filter(block_time.ge(BigDecimal::from(from_time)))
        .filter(block_time.le(BigDecimal::from(to_time)))
        .load::<DbLiquidationTransfer>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_liquidation_transfers_with_time success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_liquidation_transfers_with_time success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_liquidation_transfers_with_time fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_account_liquidation_transfers_by_time(
    from_time: i64,
    to_time: i64,
    account_id: String,
    limit: Option<u32>,
    _offset_block_time: Option<i64>,
    offset_block_number: Option<i64>,
    offset_transaction_index: Option<i32>,
    offset_log_index: Option<i32>,
) -> Result<(Vec<DbLiquidationTransfer>, Option<AccoutTradingCursor>)> {
    // if offset.unwrap_or_default() != 0 {
    //     return Ok(vec![]);
    // }
    use crate::schema::liquidation_transfer::dsl::*;
    use diesel::sql_types::Bool;
    let start_time = Instant::now();
    let mut cursor: Option<AccoutTradingCursor> = None;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result = if let Some(offset_block_number) = offset_block_number {
        let tuple_condition = sql::<Bool>(&format!(
            "(block_number, transaction_index, log_index) > ({}, {}, {})",
            offset_block_number,
            offset_transaction_index.unwrap_or_default(),
            offset_log_index.unwrap_or_default()
        ));
        liquidation_transfer
            .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
            .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
            .filter(liquidator_account_id.eq(account_id))
            .filter(tuple_condition)
            .order_by((block_number, transaction_index, log_index))
            .limit(limit.unwrap_or_default() as i64)
            .load::<DbLiquidationTransfer>(&mut conn)
            .await
    } else {
        liquidation_transfer
            .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
            .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
            .filter(liquidator_account_id.eq(account_id))
            .order_by((block_number, transaction_index, log_index))
            .limit(limit.unwrap_or_default() as i64)
            .load::<DbLiquidationTransfer>(&mut conn)
            .await
    };
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(mut events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_account_liquidation_transfers_by_time success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            if events.len() as u32 == limit.unwrap_or_default() {
                // deprecated last block_number, transaction_index, liquidation_result_log_idx to avoid a liquidation result to be split into 2
                if let Some(last) = events.pop() {
                    loop {
                        if let Some(elem) = events.pop() {
                            let same_liquidation_result = last.block_number == elem.block_number
                                && last.transaction_index == elem.transaction_index
                                && last.liquidation_result_log_idx
                                    == elem.liquidation_result_log_idx;
                            if !same_liquidation_result {
                                events.push(elem);
                                cursor = Some(AccoutTradingCursor {
                                    block_time: last
                                        .block_time
                                        .clone()
                                        .map(|f| f.to_i64().unwrap_or_default())
                                        .unwrap_or_default(),
                                    block_number: last.block_number,
                                    transaction_index: last.transaction_index,
                                    log_index: last.log_index,
                                });
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_account_liquidation_transfers_by_time success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_account_liquidation_transfers_by_time fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok((events, cursor))
}

pub async fn query_account_liquidation_transfers_count_by_time(
    from_time: i64,
    to_time: i64,
    account_id: String,
) -> Result<i64> {
    use crate::schema::liquidation_transfer::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let count = liquidation_transfer
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .filter(liquidator_account_id.eq(account_id))
        .count()
        .get_result::<i64>(&mut conn)
        .await?;
    let dur_ms = (Instant::now() - start_time).as_millis();

    tracing::info!(
        target: DB_CONTEXT,
        "query_account_liquidation_transfers_count_by_time success. count:{}, used time:{} ms",
        count,
        dur_ms
    );

    Ok(count)
}

pub async fn query_account_liquidation_transfers_by_time_and_result_keys(
    from_time: i64,
    to_time: i64,
    liquidation_result_keys: BTreeSet<(i64, i32)>,
) -> Result<Vec<DbLiquidationTransfer>> {
    let start_time = Instant::now();
    if liquidation_result_keys.is_empty() {
        return Ok(vec![]);
    }
    let conditions = liquidation_result_keys
        .iter()
        .map(|(block, idx)| format!("({}, {})", block, idx))
        .collect::<Vec<_>>()
        .join(",");
    let query = format!(
        "SELECT * FROM liquidation_transfer 
         WHERE block_time >= $1 
         AND block_time <= $2 
         AND (block_number, liquidation_result_log_idx) in ({})",
        conditions
    );
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = sql_query(&query)
        .bind::<Numeric, _>(BigDecimal::from(from_time))
        .bind::<Numeric, _>(BigDecimal::from(to_time))
        .load::<DbLiquidationTransfer>(&mut conn)
        .await;

    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_account_liquidation_transfers_by_time_and_result_keys success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_account_liquidation_transfers_by_time_and_result_keys success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_account_liquidation_transfers_by_time_and_result_keys fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}
