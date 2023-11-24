use crate::db::POOL;
use crate::schema::transaction_events;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};

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

#[derive(Debug, Copy, Clone)]
pub enum DbTransactionStatus {
    Succeed = 1,
    Failed = 2,
}

impl DbTransactionStatus {
    pub fn value(&self) -> i16 {
        *self as i16
    }
}

#[derive(Insertable, Queryable, Debug)]
#[table_name = "transaction_events"]
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
}

pub(crate) async fn create_balance_transaction_executions(
    balance_transactions: Vec<DbTransactionEvent>,
) -> Result<usize> {
    use crate::schema::transaction_events::dsl::*;
    if balance_transactions.is_empty() {
        return Ok(0);
    }
    let num_rows = diesel::insert_into(transaction_events)
        .values(balance_transactions)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}
