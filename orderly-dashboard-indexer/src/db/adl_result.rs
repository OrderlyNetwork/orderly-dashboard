use crate::db::DB_CONTEXT;
use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::adl_result;
use diesel_async::RunQueryDsl;

use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use std::time::Instant;

#[derive(Insertable, Queryable, Debug)]
#[diesel(table_name = adl_result)]
pub struct DbAdlResult {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    pub account_id: String,
    // adl v2 removed insuranceAccountId
    pub insurance_account_id: String,
    pub symbol_hash: String,
    pub position_qty_transfer: BigDecimal,
    pub cost_position_transfer: BigDecimal,
    pub adl_price: BigDecimal,
    pub sum_unitary_fundings: BigDecimal,
    pub version: Option<i16>,
}

#[derive(Debug, Copy, Clone)]
pub enum AdlVersion {
    V1 = 1,
    V2 = 2,
}

impl AdlVersion {
    pub fn value(&self) -> i16 {
        *self as i16
    }
}

impl TryFrom<i16> for AdlVersion {
    type Error = anyhow::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 | 1 => Ok(Self::V1),
            2 => Ok(Self::V2),
            _ => Err(anyhow::anyhow!(
                "cannot convert integer:{} to AdlVersion",
                value
            )),
        }
    }
}

pub async fn create_adl_results(adls: Vec<DbAdlResult>) -> Result<usize> {
    use crate::schema::adl_result::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let num_rows = diesel::insert_into(adl_result)
        .values(adls)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}

pub async fn query_adl_results(from_block: i64, to_block: i64) -> Result<Vec<DbAdlResult>> {
    use crate::schema::adl_result::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = adl_result
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load::<DbAdlResult>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_adl_results success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            // Err(diesel::NotFound)
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_adl_results success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_adl_results fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_adl_results_with_time(
    from_block: i64,
    to_block: i64,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbAdlResult>> {
    use crate::schema::adl_result::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = adl_result
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .load::<DbAdlResult>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_adl_results_with_time success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_adl_results_with_time success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_adl_results_with_time fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_account_adl_results(
    account: String,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbAdlResult>> {
    use crate::schema::adl_result::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = adl_result
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .filter(account_id.eq(account))
        .load::<DbAdlResult>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_account_adl_results success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_account_adl_results success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_account_adl_results fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}
