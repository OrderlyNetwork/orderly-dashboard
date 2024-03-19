use crate::db::DB_CONTEXT;
use crate::db::POOL;
use crate::schema::adl_result;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use std::time::Instant;

#[derive(Insertable, Queryable, Debug)]
#[table_name = "adl_result"]
pub struct DbAdlResult {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    pub account_id: String,
    pub insurance_account_id: String,
    pub symbol_hash: String,
    pub position_qty_transfer: BigDecimal,
    pub cost_position_transfer: BigDecimal,
    pub adl_price: BigDecimal,
    pub sum_unitary_fundings: BigDecimal,
}

pub async fn create_adl_results(adls: Vec<DbAdlResult>) -> Result<usize> {
    use crate::schema::adl_result::dsl::*;

    let num_rows = diesel::insert_into(adl_result)
        .values(adls)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}

pub async fn query_adl_results(from_block: i64, to_block: i64) -> Result<Vec<DbAdlResult>> {
    use crate::schema::adl_result::dsl::*;
    let start_time = Instant::now();

    let result = adl_result
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load_async::<DbAdlResult>(&POOL)
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
            AsyncError::Execute(Error::NotFound) => {
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

pub async fn query_account_adl_results(
    account: String,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbAdlResult>> {
    use crate::schema::adl_result::dsl::*;
    let start_time = Instant::now();

    let result = adl_result
        .filter(account_id.eq(account))
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .load_async::<DbAdlResult>(&POOL)
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
            AsyncError::Execute(Error::NotFound) => {
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
