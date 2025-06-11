use crate::db::{DB_CONN_ERR_MSG, DB_CONTEXT, POOL};
use crate::schema::transaction_events;
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use diesel_async::RunQueryDsl;
use std::time::Instant;

#[derive(Debug, Copy, Clone)]
pub enum DbTransactionSide {
    Deposit = 1,
    Withdraw = 2,
    WithdrawApprove = 3,
}

impl DbTransactionSide {
    pub fn value(&self) -> i16 {
        *self as i16
    }
}

impl TryFrom<i16> for DbTransactionSide {
    type Error = anyhow::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Deposit),
            2 => Ok(Self::Withdraw),
            3 => Ok(Self::WithdrawApprove),
            _ => Err(anyhow::anyhow!(
                "cannot convert integer:{} to DbTransactionSide",
                value
            )),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DbTransactionStatus {
    Succeed = 1,
    #[allow(dead_code)]
    Failed = 2,
}

impl DbTransactionStatus {
    pub fn value(&self) -> i16 {
        *self as i16
    }
}

#[derive(Insertable, Queryable, Debug)]
#[diesel(table_name = transaction_events)]
pub struct DbTransactionEvent {
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
    pub effective_gas_price: Option<BigDecimal>,
    pub gas_used: Option<BigDecimal>,
    pub l1_fee: Option<BigDecimal>,
    pub l1_fee_scalar: Option<String>,
    pub l1_gas_price: Option<BigDecimal>,
    pub l1_gas_used: Option<BigDecimal>,
}

pub async fn create_balance_transaction_executions(
    balance_transactions: Vec<DbTransactionEvent>,
) -> Result<usize> {
    use crate::schema::transaction_events::dsl::*;
    if balance_transactions.is_empty() {
        return Ok(0);
    }
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let num_rows = diesel::insert_into(transaction_events)
        .values(balance_transactions)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}

pub async fn query_balance_transaction_executions(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<DbTransactionEvent>> {
    use crate::schema::transaction_events::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result = transaction_events
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load::<DbTransactionEvent>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_balance_transaction_executions success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_balance_transaction_executions success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_balance_transaction_executions fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_balance_transaction_executions_with_time(
    from_block: i64,
    to_block: i64,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbTransactionEvent>> {
    use crate::schema::transaction_events::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result = transaction_events
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .load::<DbTransactionEvent>(&mut conn)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_balance_transaction_executions success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_balance_transaction_executions success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_balance_transaction_executions fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

pub async fn query_account_balance_transaction_executions(
    account: String,
    from_time: i64,
    to_time: i64,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<DbTransactionEvent>> {
    use crate::schema::transaction_events::dsl::*;
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let query = transaction_events
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .filter(account_id.eq(account));
    let result = if let Some(offset) = offset {
        query
            .order_by((block_time, log_index))
            .offset(offset as i64)
            .limit(limit.unwrap_or_default() as i64)
            .load::<DbTransactionEvent>(&mut conn)
            .await
    } else {
        query.load::<DbTransactionEvent>(&mut conn).await
    };

    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_account_balance_transaction_executions success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_account_balance_transaction_executions success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_account_balance_transaction_executions fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}
