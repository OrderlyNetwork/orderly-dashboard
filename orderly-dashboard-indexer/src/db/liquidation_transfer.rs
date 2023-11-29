use crate::db::{DB_CONTEXT, POOL};
use crate::schema::liquidation_transfer;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use std::time::Instant;

#[derive(Insertable, Queryable, Debug)]
#[table_name = "liquidation_transfer"]
pub struct DbLiquidationTransfer {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub liquidation_transfer_id: BigDecimal,
    pub liquidator_account_id: String,
    pub symbol_hash: String,
    pub position_qty_transfer: BigDecimal,
    pub cost_position_transfer: BigDecimal,
    pub liquidator_fee: BigDecimal,
    pub insurance_fee: BigDecimal,
    pub mark_price: BigDecimal,
    pub sum_unitary_fundings: BigDecimal,
    pub liquidation_fee: BigDecimal,
}

impl DbLiquidationTransfer {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.log_index)
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
    tracing::info!(
        target: DB_CONTEXT,
        "query_liquidation_transfers start",
    );
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
