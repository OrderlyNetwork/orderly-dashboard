use crate::constants::ALERT_CONTEXT;
use crate::db::DB_CONTEXT;
use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::executed_trades;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use diesel_async::RunQueryDsl;
use std::time::Instant;

#[derive(Insertable, Queryable, Debug, Clone)]
#[diesel(table_name = executed_trades)]
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
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let num_rows = diesel::insert_into(executed_trades)
        .values(trades)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}

pub async fn delete_oldest_executed_trades(limit: i64) -> Result<usize> {
    use crate::schema::executed_trades::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let ids: Vec<i64> = executed_trades
        .select(block_number)
        .order(block_number.asc())
        .limit(limit)
        .load::<i64>(&mut conn)
        .await?;
    if ids.is_empty() {
        return Ok(0);
    }
    // ids.dedup();
    let min_val = if ids.is_empty() { 0 } else { ids[0] };
    let max_val = ids.last().cloned().unwrap_or_default();

    let deleted_num = diesel::delete(
        executed_trades
            .filter(block_number.ge(min_val))
            .filter(block_number.le(max_val)),
    )
    .execute(&mut conn)
    .await?;
    Ok(deleted_num)
}

pub async fn query_executed_trades(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<DbExecutedTrades>> {
    use crate::schema::executed_trades::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_executed_trades start with from: {} to: {}", from_block, to_block,
    );
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let start_time = Instant::now();

    let result = executed_trades
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .load::<DbExecutedTrades>(&mut conn)
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
            diesel::NotFound => {
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

pub async fn query_executed_trades_with_time(
    from_block: i64,
    to_block: i64,
    from_time: i64,
    to_time: i64,
) -> Result<Vec<DbExecutedTrades>> {
    use crate::schema::executed_trades::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_executed_trades start with from: {} to: {}", from_block, to_block,
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let result = executed_trades
        .filter(block_number.ge(from_block))
        .filter(block_number.le(to_block))
        .filter(block_time.ge(from_time))
        .filter(block_time.le(to_time))
        .load::<DbExecutedTrades>(&mut conn)
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
            diesel::NotFound => {
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
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<DbExecutedTrades>> {
    use crate::schema::executed_trades::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_account_executed_trades start with account: {}, from_time: {}, to_time: {}",
        account, from_time, to_time,
    );
    let start_time = Instant::now();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let query = executed_trades
        .filter(block_time.ge(from_time))
        .filter(block_time.le(to_time))
        .filter(account_id.eq(account.clone()));

    let result = if let Some(offset) = offset {
        query
            .order_by((block_time, log_index))
            .offset(offset as i64)
            .limit(limit.unwrap_or_default() as i64)
            .load::<DbExecutedTrades>(&mut conn)
            .await
    } else {
        query.load::<DbExecutedTrades>(&mut conn).await
    };
    let dur_ms = (Instant::now() - start_time).as_millis();

    let events = match result {
        Ok(events) => {
            if dur_ms >= 100 {
                tracing::warn!(
                    target: ALERT_CONTEXT,
                    "query_account_executed_trades slow query. account_id: {}, from_time:{}, to_time:{} length:{}, used time:{} ms",
                        account,
                        from_time,
                        to_time,
                    events.len(),
                    dur_ms
                );
            } else {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_account_executed_trades success. length:{}, used time:{} ms",
                    events.len(),
                    dur_ms
                );
            }
            events
        }
        Err(error) => match error {
            diesel::NotFound => {
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
    use chrono::Datelike;
    use chrono::Timelike;

    use crate::db::settings::update_last_rpc_processed_height;
    use crate::init::init_log;

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

    #[ignore]
    #[test]
    fn test_insert_trades_for_migration() {
        dotenv::dotenv().ok();
        init_log();
        let system = actix::System::new();
        system.block_on(async move {
            let data_time = chrono::Utc::now();
            for (year, q) in [
                (2023, 1),
                (2024, 1),
                (2024, 2),
                (2024, 3),
                (2024, 4),
                (2025, 1),
                (2025, 2),
                (2025, 3),
            ] {
                let data_time = data_time.with_year(year).unwrap();
                let data_time = data_time
                    .with_month(q * 3)
                    .unwrap()
                    .with_hour(10)
                    .unwrap()
                    .with_minute(30)
                    .unwrap()
                    .with_second(36)
                    .unwrap();
                let timestamp = data_time.timestamp();
                update_last_rpc_processed_height(1_999).await.unwrap();
                for blocknum in 0..2_000 {
                    let inst = Instant::now();
                    let mut trades = vec![];
                    for i in 0..100 {
                        trades.push(DbExecutedTrades {
                            block_number: blocknum,
                            transaction_index: 1,
                            log_index: i,
                            typ: 0,
                            account_id: "0x100000".to_string() + &i.to_string(),
                            symbol_hash: "0xaaaaaaaaaaa".to_string(),
                            fee_asset_hash: "0xbbbbbbbb".to_string(),
                            trade_qty: 100000.into(),
                            notional: 1000.into(),
                            executed_price: 1900000.into(),
                            fee: 1900.into(),
                            sum_unitary_fundings: 2000.into(),
                            trade_id: (year as i64 * 1000000
                                + q as i64 * 10000
                                + blocknum * 100
                                + i as i64)
                                .into(),
                            match_id: 1000.into(),
                            timestamp: timestamp.into(),
                            side: true,
                            block_time: timestamp,
                        });
                    }
                    create_executed_trades(trades).await.unwrap();

                    let elapsed_ms = inst.elapsed().as_millis();
                    tracing::info!(
                        "create_executed_trades elapsed_ms: {} for year: {}, q: {}, blocknum: {}",
                        elapsed_ms,
                        year,
                        q,
                        blocknum
                    );
                }
            }
        });
    }
}
