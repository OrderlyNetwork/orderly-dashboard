use crate::db::{DB_CONTEXT, POOL};
use crate::schema::settlement_execution;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use std::time::Instant;

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name = "settlement_execution"]
pub struct DbSettlementExecution {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub settlement_result_log_idx: i32,
    pub transaction_id: String,
    pub symbol_hash: String,
    pub sum_unitary_fundings: BigDecimal,
    pub mark_price: BigDecimal,
    pub settled_amount: BigDecimal,
    pub block_time: Option<BigDecimal>,
}

impl DbSettlementExecution {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.settlement_result_log_idx)
    }

    pub fn is_result_log_set(&self) -> bool {
        self.settlement_result_log_idx >= 0
    }
}

pub async fn create_settlement_executions(
    settlement_execs: Vec<DbSettlementExecution>,
) -> Result<usize> {
    use crate::schema::settlement_execution::dsl::*;
    if settlement_execs.is_empty() {
        return Ok(0);
    }
    let num_rows = diesel::insert_into(settlement_execution)
        .values(settlement_execs)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}

pub async fn query_settlement_executions(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<DbSettlementExecution>> {
    use crate::schema::settlement_execution::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_settlement_executions start",
    );
    let start_time = Instant::now();

    let result = settlement_execution
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load_async::<DbSettlementExecution>(&POOL)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_settlement_executions success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_settlement_executions success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_settlement_executions fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_account_settlement_executions(
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbSettlementExecution>> {
    use crate::schema::settlement_execution::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_account_settlement_executions start",
    );
    let start_time = Instant::now();

    let result = settlement_execution
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .load_async::<DbSettlementExecution>(&POOL)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_account_settlement_executions success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_account_settlement_executions success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_account_settlement_executions fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}
