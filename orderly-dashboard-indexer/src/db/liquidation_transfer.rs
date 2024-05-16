use crate::db::{DB_CONTEXT, POOL};
use crate::schema::liquidation_transfer;
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
#[table_name = "liquidation_transfer"]
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
    let num_rows = diesel::insert_into(liquidation_transfer)
        .values(liquidation_transfers)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}

pub async fn query_liquidation_transfers(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<DbLiquidationTransfer>> {
    use crate::schema::liquidation_transfer::dsl::*;
    let start_time = Instant::now();

    let result = liquidation_transfer
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load_async::<DbLiquidationTransfer>(&POOL)
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
            AsyncError::Execute(Error::NotFound) => {
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

pub async fn query_account_liquidation_transfers_by_time(
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbLiquidationTransfer>> {
    use crate::schema::liquidation_transfer::dsl::*;
    let start_time = Instant::now();

    let result = liquidation_transfer
        .filter(block_time.ge(BigDecimal::from_i64(from_time).unwrap_or_default()))
        .filter(block_time.le(BigDecimal::from_i64(to_time).unwrap_or_default()))
        .load_async::<DbLiquidationTransfer>(&POOL)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_account_liquidation_transfers_by_time success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
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

    Ok(events)
}
