use crate::db::{DB_CONN_ERR_MSG, DB_CONTEXT, POOL};
use crate::schema::settlement_execution;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Integer, Nullable, Numeric, Text};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use diesel_async::RunQueryDsl;
use std::time::Instant;

#[derive(Insertable, Queryable, Debug, Clone)]
#[diesel(table_name = settlement_execution)]
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

#[allow(dead_code)]
#[derive(Debug, Clone, QueryableByName)]
pub struct DbSettlementExecutionView {
    #[diesel(sql_type = Numeric)]
    pub result_settled_amount: BigDecimal,
    #[diesel(sql_type = Text)]
    pub insurance_account_id: String,
    #[diesel(sql_type = Numeric)]
    pub insurance_transfer_amount: BigDecimal,
    #[diesel(sql_type = BigInt)]
    pub block_number: i64,
    #[diesel(sql_type = Integer)]
    pub transaction_index: i32,
    #[diesel(sql_type = Integer)]
    pub log_index: i32,
    #[diesel(sql_type = Integer)]
    pub settlement_result_log_idx: i32,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Text)]
    pub symbol_hash: String,
    #[diesel(sql_type = Numeric)]
    pub sum_unitary_fundings: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub mark_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub settled_amount: BigDecimal,
    #[diesel(sql_type = Nullable<Numeric>)]
    pub block_time: Option<BigDecimal>,
}

impl DbSettlementExecutionView {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.settlement_result_log_idx)
    }
}

pub async fn create_settlement_executions(
    settlement_execs: Vec<DbSettlementExecution>,
) -> Result<usize> {
    use crate::schema::settlement_execution::dsl::*;
    if settlement_execs.is_empty() {
        return Ok(0);
    }
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let num_rows = diesel::insert_into(settlement_execution)
        .values(settlement_execs)
        .on_conflict_do_nothing()
        .execute(&mut conn)
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

    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result = settlement_execution
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load::<DbSettlementExecution>(&mut conn)
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
            diesel::NotFound => {
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

pub async fn query_settlement_executions_with_time(
    from_block: i64,
    to_block: i64,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbSettlementExecution>> {
    use crate::schema::settlement_execution::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_settlement_executions_with_time start",
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = settlement_execution
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .filter(block_time.ge(BigDecimal::from(from_time)))
        .filter(block_time.le(BigDecimal::from(to_time)))
        .load::<DbSettlementExecution>(&mut conn)
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
            diesel::NotFound => {
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
    account_id_: String,
    from_time: i64,
    to_time: i64,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<DbSettlementExecutionView>> {
    use diesel::sql_query;
    tracing::info!(
        target: DB_CONTEXT,
        "query_account_settlement_executions start",
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result = if let Some(offset) = offset {
        sql_query(
            "select
                    result.settled_amount as result_settled_amount,
                    result.insurance_account_id as insurance_account_id,
                    result.insurance_transfer_amount as insurance_transfer_amount,
                    executions.block_number as block_number,
                    executions.transaction_index as transaction_index,
                    executions.log_index as log_index,
                    executions.settlement_result_log_idx as settlement_result_log_idx,
                    executions.transaction_id as transaction_id,
                    executions.symbol_hash as symbol_hash,
                    executions.sum_unitary_fundings as sum_unitary_fundings,
                    executions.mark_price as mark_price,
                    executions.settled_amount as settled_amount,
                    executions.block_time as block_time
                  from
                    settlement_result result
                    left join settlement_execution executions on result.block_number = executions.block_number
                    and result.log_index = executions.settlement_result_log_idx
                  where
                    result.account_id = $3
                    and result.block_time >= $1
                    and result.block_time <= $2
                    and executions.block_time >= $1
                    and executions.block_time <= $2
                    order by block_number, transaction_index, log_index
                    offset $4 limit $5
                    ;",
        )
            .bind::<diesel::sql_types::Numeric, _>(BigDecimal::from(from_time))
            .bind::<diesel::sql_types::Numeric, _>(BigDecimal::from(to_time))
            .bind::<diesel::sql_types::Text, _>(account_id_)
            .bind::<diesel::sql_types::BigInt, _>(offset as i64)
            .bind::<diesel::sql_types::BigInt, _>(limit.unwrap_or_default() as i64)
            .get_results::<DbSettlementExecutionView>(&mut conn)
            .await
    } else {
        sql_query(
            "select
                    result.settled_amount as result_settled_amount,
                    result.insurance_account_id as insurance_account_id,
                    result.insurance_transfer_amount as insurance_transfer_amount,
                    executions.block_number as block_number,
                    executions.transaction_index as transaction_index,
                    executions.log_index as log_index,
                    executions.settlement_result_log_idx as settlement_result_log_idx,
                    executions.transaction_id as transaction_id,
                    executions.symbol_hash as symbol_hash,
                    executions.sum_unitary_fundings as sum_unitary_fundings,
                    executions.mark_price as mark_price,
                    executions.settled_amount as settled_amount,
                    executions.block_time as block_time
                  from
                    settlement_result result
                    left join settlement_execution executions on result.block_number = executions.block_number
                    and result.log_index = executions.settlement_result_log_idx
                  where
                    result.account_id = $3
                    and result.block_time >= $1
                    and result.block_time <= $2
                    and executions.block_time >= $1
                    and executions.block_time <= $2;",
        )
            .bind::<diesel::sql_types::Numeric, _>(BigDecimal::from(from_time))
            .bind::<diesel::sql_types::Numeric, _>(BigDecimal::from(to_time))
            .bind::<diesel::sql_types::Text, _>(account_id_)
            .get_results::<DbSettlementExecutionView>(&mut conn)
            .await
    };

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
            diesel::NotFound => {
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
