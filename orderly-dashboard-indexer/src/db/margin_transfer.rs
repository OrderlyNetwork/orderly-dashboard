use crate::db::DB_CONTEXT;
use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::margin_transfer;
use diesel_async::RunQueryDsl;

use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use std::time::Instant;

#[derive(Insertable, Queryable, Debug)]
#[diesel(table_name = margin_transfer)]
pub struct DbMarginTransfer {
    pub block_number: i64,
    pub transaction_index: i32,
    pub transaction_id: String,
    pub log_index: i32,
    pub block_time: BigDecimal,
    pub account_id: String,
    pub transfer_amount: BigDecimal,
    pub transfer_asset_hash: String,
    pub iso_symbol_hash: String,
    pub timestamp: i64,
}

pub async fn create_margin_transfers(adls: Vec<DbMarginTransfer>) -> Result<usize> {
    use crate::schema::margin_transfer::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let num_rows = diesel::insert_into(margin_transfer)
        .values(adls)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}

pub async fn query_margin_transfers(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<DbMarginTransfer>> {
    use crate::schema::margin_transfer::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = margin_transfer
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load::<DbMarginTransfer>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_margin_transfers success. length:{}, used time:{} ms",
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
                    "query_margin_transfers success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_margin_transfers fail. err:{:?}, used time:{} ms",
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
pub async fn query_margin_transfers_with_time(
    from_block: i64,
    to_block: i64,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbMarginTransfer>> {
    use crate::schema::margin_transfer::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = margin_transfer
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .load::<DbMarginTransfer>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_margin_transfers_with_time success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_margin_transfers_with_time success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_margin_transfers_with_time fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_account_margin_transfers(
    account: String,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbMarginTransfer>> {
    use crate::schema::margin_transfer::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = margin_transfer
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .filter(account_id.eq(account))
        .load::<DbMarginTransfer>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_account_margin_transfers success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_account_margin_transfers success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_account_margin_transfers fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}
