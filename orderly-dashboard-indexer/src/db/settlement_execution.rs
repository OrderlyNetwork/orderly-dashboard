use crate::db::POOL;
use crate::schema::settlement_execution;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
#[derive(Insertable, Queryable, Debug)]
#[table_name = "settlement_execution"]
pub struct DbSettlementExecution {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub transaction_id: String,
    pub symbol_hash: String,
    pub sum_unitary_fundings: BigDecimal,
    pub mark_price: BigDecimal,
    pub settled_amount: BigDecimal,
}

pub(crate) async fn create_settlement_executions(
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
