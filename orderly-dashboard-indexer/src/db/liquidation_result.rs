use crate::db::DB_CONTEXT;
use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::liquidation_result;
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::prelude::*;
use diesel::sql_types::Numeric;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use diesel_async::RunQueryDsl;
use std::collections::BTreeSet;
use std::time::Instant;

#[derive(Insertable, Queryable, QueryableByName, Debug)]
#[diesel(table_name = liquidation_result)]
pub struct DbLiquidationResult {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    // reuse liquidatedAccountId as accountId in v2
    pub liquidated_account_id: String,
    pub insurance_account_id: String,
    pub liquidated_asset_hash: String,
    pub insurance_transfer_amount: BigDecimal,
    pub version: Option<i16>,
}

#[derive(Debug, Copy, Clone)]
pub enum LiquidationResultVersion {
    V1 = 1,
    V2 = 2,
}

impl LiquidationResultVersion {
    pub fn value(&self) -> i16 {
        *self as i16
    }
}

impl TryFrom<i16> for LiquidationResultVersion {
    type Error = anyhow::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 | 1 => Ok(Self::V1),
            2 => Ok(Self::V2),
            _ => Err(anyhow::anyhow!(
                "cannot convert integer:{} to LiquidationResultVersion",
                value
            )),
        }
    }
}

impl DbLiquidationResult {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.log_index)
    }
}

pub async fn create_liquidation_results(liquidations: Vec<DbLiquidationResult>) -> Result<usize> {
    use crate::schema::liquidation_result::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let num_rows = diesel::insert_into(liquidation_result)
        .values(liquidations)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}

pub async fn query_liquidation_results(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<DbLiquidationResult>> {
    use crate::schema::liquidation_result::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = liquidation_result
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load::<DbLiquidationResult>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_liquidation_results success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_liquidation_results success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_liquidation_results fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

#[allow(dead_code)]
pub async fn query_liquidation_results_with_time(
    from_block: i64,
    to_block: i64,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbLiquidationResult>> {
    use crate::schema::liquidation_result::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = liquidation_result
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .filter(block_time.ge(BigDecimal::from(from_time)))
        .filter(block_time.le(BigDecimal::from(to_time)))
        .load::<DbLiquidationResult>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_liquidation_results_with_time success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_liquidation_results_with_time success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_liquidation_results_with_time fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

#[allow(dead_code)]
pub async fn query_liquidation_results_by_time(
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbLiquidationResult>> {
    use crate::schema::liquidation_result::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = liquidation_result
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .load::<DbLiquidationResult>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_liquidation_results_by_time success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_liquidation_results success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_liquidation_results_by_time fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_liquidation_results_by_time_and_account(
    from_time: i64,
    to_time: i64,
    account_id: String,
) -> Result<Vec<DbLiquidationResult>> {
    use crate::schema::liquidation_result::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = liquidation_result
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .filter(liquidated_account_id.eq(account_id))
        .load::<DbLiquidationResult>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_liquidation_results_by_time success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_liquidation_results success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_liquidation_results_by_time fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_liquidation_results_by_time_and_keys(
    from_time: i64,
    to_time: i64,
    keys: BTreeSet<(i64, i32)>,
) -> Result<Vec<DbLiquidationResult>> {
    use diesel::sql_query;
    let start_time = Instant::now();
    if keys.is_empty() {
        return Ok(vec![]);
    }

    let conditions = keys
        .iter()
        .map(|(block, idx)| format!("({}, {})", block, idx))
        .collect::<Vec<_>>()
        .join(",");

    let query = format!(
        "SELECT * FROM liquidation_result 
         WHERE block_time >= $1 
         AND block_time <= $2 
         AND (block_number, log_index) in ({})",
        conditions
    );
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = sql_query(&query)
        .bind::<Numeric, _>(BigDecimal::from(from_time))
        .bind::<Numeric, _>(BigDecimal::from(to_time))
        .load::<DbLiquidationResult>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_liquidation_results_by_time_and_keys success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_liquidation_results_by_time_and_keys success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_liquidation_results_by_time_and_keys fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}
