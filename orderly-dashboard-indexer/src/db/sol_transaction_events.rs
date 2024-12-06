use crate::db::{DB_CONTEXT, POOL};
use crate::schema::sol_transaction_events;
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
#[table_name = "sol_transaction_events"]
pub struct DbSolTransactionEvent {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    pub account_id: String,
    pub sender: Option<String>,
    pub receiver: String,
    pub token_hash: String,
    pub broker_hash: String,
    pub chain_id: BigDecimal,
    pub side: i16,
    pub amount: BigDecimal,
    pub fee: BigDecimal,
    pub status: i16,
    pub withdraw_nonce: Option<i64>,
    pub fail_reason: Option<i16>,
}

pub async fn create_sol_balance_transaction_executions(
    balance_transactions: Vec<DbSolTransactionEvent>,
) -> Result<usize> {
    use crate::schema::sol_transaction_events::dsl::*;
    if balance_transactions.is_empty() {
        return Ok(0);
    }
    let num_rows = diesel::insert_into(sol_transaction_events)
        .values(balance_transactions)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}

pub async fn query_sol_balance_transaction_executions(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<DbSolTransactionEvent>> {
    use crate::schema::sol_transaction_events::dsl::*;
    let start_time = Instant::now();

    let result = sol_transaction_events
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load_async::<DbSolTransactionEvent>(&POOL)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_sol_balance_transaction_executions success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_sol_balance_transaction_executions success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_sol_balance_transaction_executions fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_account_sol_balance_transaction_executions(
    account: String,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbSolTransactionEvent>> {
    use crate::schema::sol_transaction_events::dsl::*;
    let start_time = Instant::now();

    let result = sol_transaction_events
        .filter(account_id.eq(account))
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .load_async::<DbSolTransactionEvent>(&POOL)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_account_sol_balance_transaction_executions success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_account_sol_balance_transaction_executions success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_account_sol_balance_transaction_executions fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}
