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
    pub block_time: i64,
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

// time of executed trades need to be convert from secs to millisecond
pub async fn query_account_executed_trades(
    account: String,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbExecutedTrades>> {
    use crate::schema::executed_trades::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_account_executed_trades start",
    );
    let start_time = Instant::now();

    let result = executed_trades
        .filter(account_id.eq(account))
        .filter(block_time.ge(from_time))
        .filter(block_time.le(to_time))
        .load_async::<DbExecutedTrades>(&POOL)
        .await;
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            if dur_ms >= 100 {
                tracing::warn!(
                    target: ALERT_CONTEXT,
                    "query_account_executed_trades slow query. from_time:{}, to_time:{} length:{}, used time:{} ms",
                        from_time,
                        to_time,
                    events.len(),
                    dur_ms
                );
            }
            tracing::info!(
                target: DB_CONTEXT,
                "query_account_executed_trades success. length:{}, used time:{} ms",
                events.len(),
                dur_ms
            );
            events
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_account_executed_trades success. length:0, used time:{} ms",
                    dur_ms
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_account_executed_trades fail. err:{:?}, used time:{} ms",
                    error,
                    dur_ms
                );
                Err(error)?
            }
        },
    };

    Ok(events)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_create_executed_trades() {
        dotenv::dotenv().ok();
        let system = actix::System::new();
        system.block_on(async move {
            create_executed_trades(vec![DbExecutedTrades {
                block_number: 1,
                transaction_index: 1,
                log_index: 1,
                typ: 1,
                account_id: "0x2c9fb1ecc28408943f182b644b4d07ae68e8cc87b41b8cf15f89c0065cb45631".to_string(),
                symbol_hash: "0x4db998531f1de048a3a9e0793099604c3fa2aa7626f913b3242e6006afe1007e".to_string(),
                fee_asset_hash: "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa".to_string(),
                trade_qty: 1.into(),
                notional: 1_000_0000_000_u64.into(),
                executed_price: 100_0000_000_000_u64.into(),
                fee: 100_0000_000_u64.into(),
                sum_unitary_fundings: 100_0000_000_000_u64.into(),
                trade_id: 1.into(),
                match_id: 100.into(),
                timestamp: 1722785206000_u64.into(),
                side: true,
                block_time: 1722785206,
            }])
            .await
            .unwrap();

            let inst = Instant::now();
            create_executed_trades(vec![DbExecutedTrades {
                block_number: 2,
                transaction_index: 1,
                log_index: 1,
                typ: 1,
                account_id: "0x2c9fb1ecc28408943f182b644b4d07ae68e8cc87b41b8cf15f89c0065cb45631".to_string(),
                symbol_hash: "0x4db998531f1de048a3a9e0793099604c3fa2aa7626f913b3242e6006afe1007e".to_string(),
                fee_asset_hash: "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa".to_string(),
                trade_qty: 1.into(),
                notional: 1_000_0000_000_u64.into(),
                executed_price: 100_0000_000_000_u64.into(),
                fee: 100_0000_000_u64.into(),
                sum_unitary_fundings: 100_0000_000_000_u64.into(),
                trade_id: 2.into(),
                match_id: 100.into(),
                timestamp: 1722785206000_u64.into(),
                side: true,
                block_time: 1722785206,
            }])
            .await
            .unwrap();

            let elapse_1 = inst.elapsed().as_micros();
            println!("insert one key spend: {} micros", elapse_1);

            let inst = Instant::now();
            let mut batches: Vec<DbExecutedTrades> = vec![];
            (0..2000).into_iter().for_each(|i| {
                batches.push(DbExecutedTrades {
                    block_number: 3,
                    transaction_index: 1,
                    log_index: i,
                    typ: 1,
                    account_id: "0x2c9fb1ecc28408943f182b644b4d07ae68e8cc87b41b8cf15f89c0065cb45631".to_string(),
                    symbol_hash: "0x4db998531f1de048a3a9e0793099604c3fa2aa7626f913b3242e6006afe1007e".to_string(),
                    fee_asset_hash: "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa".to_string(),
                    trade_qty: 1.into(),
                    notional: 1_000_0000_000_u64.into(),
                    executed_price: 100_0000_000_000_u64.into(),
                    fee: 100_0000_000_u64.into(),
                    sum_unitary_fundings: 100_0000_000_000_u64.into(),
                    trade_id: (1_000_000 + i).into(),
                    match_id: 100.into(),
                    timestamp: 1722785206000_u64.into(),
                    side: true,
                    block_time: 1722785206,
                });
            });
            let len = batches.len();
            create_executed_trades(batches).await.unwrap();

            let elapse_2 = inst.elapsed().as_micros();
            let avg_elapse_rate = elapse_2 as f64 / len as f64 / elapse_1 as f64;
            println!(
                "insert batches len: {} spent: {} micros, avg_elapse_rate: {}",
                len,
                elapse_2, 
                avg_elapse_rate,
            );

            actix::System::current().stop();
        });

        system.run().unwrap();
    }
}
