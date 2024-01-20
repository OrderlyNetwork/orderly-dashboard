use crate::db::{DB_CONTEXT, POOL};
use crate::schema::settlement_result;
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
#[table_name = "settlement_result"]
pub struct DbSettlementResult {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub block_time: BigDecimal,
    pub account_id: String,
    pub settled_amount: BigDecimal,
    pub settled_asset_hash: String,
    pub insurance_account_id: String,
    pub insurance_transfer_amount: BigDecimal,
}

impl DbSettlementResult {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.log_index)
    }
}

pub async fn create_settlement_results(settlement_reses: Vec<DbSettlementResult>) -> Result<usize> {
    use crate::schema::settlement_result::dsl::*;

    let num_rows = diesel::insert_into(settlement_result)
        .values(settlement_reses)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}

pub async fn query_settlement_results(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<DbSettlementResult>> {
    use crate::schema::settlement_result::dsl::*;
    let start_time = Instant::now();

    let result = settlement_result
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load_async::<DbSettlementResult>(&POOL)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_settlement_results success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_settlement_results success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_settlement_results fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}
