use crate::constants::ALERT_CONTEXT;
use crate::db::DB_CONTEXT;
use crate::db::POOL;
use crate::schema::executed_trades;
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
#[table_name = "executed_trades"]
pub struct DbExecutedTrades {
    pub block_number: i64,
    pub transaction_index: i32,
    pub log_index: i32,
    pub typ: i16,
    pub account_id: String,
    pub symbol_hash: String,
    pub fee_asset_hash: String,
    pub trade_qty: BigDecimal,
    pub notional: BigDecimal,
    pub executed_price: BigDecimal,
    pub fee: BigDecimal,
    pub sum_unitary_fundings: BigDecimal,
    pub trade_id: BigDecimal,
    pub match_id: BigDecimal,
    pub timestamp: BigDecimal,
    pub side: bool,
}

impl DbExecutedTrades {
    pub fn get_batch_key(&self) -> (i64, i32) {
        (self.block_number, self.transaction_index)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TradeType {
    PerpTrade = 1,
    SpotTrade = 2,
}

impl TradeType {
    pub fn value(self) -> i16 {
        self as i16
    }
}

impl TryFrom<i16> for TradeType {
    type Error = anyhow::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::PerpTrade),
            2 => Ok(Self::SpotTrade),
            _ => Err(anyhow::anyhow!("cannot convert integer to TradeType")),
        }
    }
}

pub async fn create_executed_trades(trades: Vec<DbExecutedTrades>) -> Result<usize> {
    use crate::schema::executed_trades::dsl::*;

    let num_rows = diesel::insert_into(executed_trades)
        .values(trades)
        .on_conflict_do_nothing()
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}

pub async fn query_executed_trades(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<DbExecutedTrades>> {
    use crate::schema::executed_trades::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_executed_trades start",
    );
    let start_time = Instant::now();

    let result = executed_trades
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load_async::<DbExecutedTrades>(&POOL)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            if dur_ms >= 100 {
                tracing::warn!(
                    target: ALERT_CONTEXT,
                    "query_executed_trades slow query. from:{}, to:{} length:{}, used time:{} ms",
                        from_block,
                        to_block,
                    events.len(),
                    dur_ms
                );
            }
            tracing::info!(
                target: DB_CONTEXT,
                "query_executed_trades success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_executed_trades success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_executed_trades fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}
