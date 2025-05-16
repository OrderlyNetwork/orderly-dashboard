use std::time::Instant;

use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{Duration, Local};
use diesel::sql_types::*;
use diesel::QueryableByName;

use crate::format_extern::trading_metrics::{
    TokenAmountRanking, TradingPnlRanking, TradingVolumeRanking, UserPerpHoldingRanking,
};

#[derive(Debug, QueryableByName, Clone)]
pub struct AccountVolume {
    #[sql_type = "Text"]
    account_id: String,
    #[sql_type = "Numeric"]
    volume: BigDecimal,
}

#[derive(Debug, QueryableByName, Clone)]
pub struct AccountPerpHolding {
    #[sql_type = "Text"]
    account_id: String,
    #[sql_type = "Numeric"]
    holding: BigDecimal,
}

pub async fn get_token_ranking(hour: i64, account_size: i64, withdraw: bool) -> TokenAmountRanking {
    #[allow(unused_imports)]
    use orderly_dashboard_analyzer::{
        db::{hourly_user_token::HourlyUserToken, POOL},
        schema::hourly_user_token,
        schema::hourly_user_token::dsl::*,
    };

    let now = Local::now().naive_utc();
    let start_time = now - Duration::hours(hour);

    let mut sql = "select account_id,sum(withdraw_amount) as volume from hourly_user_token \
    where block_hour>=$1 group by account_id order by volume desc limit $2";

    if !withdraw {
        sql = "select account_id,sum(deposit_amount) as volume from hourly_user_token \
    where block_hour>=$1 group by account_id order by volume desc limit $2"
    }

    let select_result = diesel::sql_query(sql)
        .bind::<Timestamp, _>(start_time)
        .bind::<BigInt, _>(account_size)
        .get_results_async::<AccountVolume>(&POOL)
        .await;

    match select_result {
        Ok(select_data) => {
            let mut account_vec: Vec<String> = Vec::new();
            let mut volume_vec: Vec<f64> = Vec::new();
            for account_volume in select_data {
                account_vec.push(account_volume.account_id);
                volume_vec.push(account_volume.volume.with_scale(2).to_f64().unwrap());
            }
            TokenAmountRanking {
                account_ids: account_vec,
                volume: volume_vec,
            }
        }
        Err(_) => TokenAmountRanking {
            account_ids: vec![],
            volume: vec![],
        },
    }
}

pub async fn get_pnl_ranking(hour: i64, account_size: i64) -> TradingPnlRanking {
    #[allow(unused_imports)]
    use orderly_dashboard_analyzer::{
        db::{hourly_user_perp::HourlyUserPerp, POOL},
        schema::hourly_user_perp,
        schema::hourly_user_perp::dsl::*,
    };

    let now = Local::now().naive_utc();
    let start_time = now - Duration::hours(hour);

    let select_result = diesel::sql_query(
        "select account_id,sum(realized_pnl) as volume from hourly_user_perp \
    where block_hour>=$1 group by account_id order by volume desc limit $2",
    )
    .bind::<Timestamp, _>(start_time)
    .bind::<BigInt, _>(account_size)
    .get_results_async::<AccountVolume>(&POOL)
    .await;

    match select_result {
        Ok(select_data) => {
            let mut account_vec: Vec<String> = Vec::new();
            let mut volume_vec: Vec<f64> = Vec::new();
            for account_volume in select_data {
                account_vec.push(account_volume.account_id);
                volume_vec.push(account_volume.volume.with_scale(2).to_f64().unwrap());
            }
            TradingPnlRanking {
                account_ids: account_vec,
                volume: volume_vec,
            }
        }
        Err(_) => TradingPnlRanking {
            account_ids: vec![],
            volume: vec![],
        },
    }
}

pub async fn get_daily_trading_volume_ranking(
    hour: i64,
    account_size: i64,
) -> TradingVolumeRanking {
    #[allow(unused_imports)]
    use orderly_dashboard_analyzer::{
        db::{hourly_user_perp::HourlyUserPerp, POOL},
        schema::hourly_user_perp,
        schema::hourly_user_perp::dsl::*,
    };

    let now = Local::now().naive_utc();
    let start_time = now - Duration::hours(hour);

    let select_result = diesel::sql_query(
        "select account_id,sum(trading_volume) as volume from hourly_user_perp \
    where block_hour>=$1 group by account_id order by volume desc limit $2",
    )
    .bind::<Timestamp, _>(start_time)
    .bind::<BigInt, _>(account_size)
    .get_results_async::<AccountVolume>(&POOL)
    .await;

    match select_result {
        Ok(select_data) => {
            let mut account_vec: Vec<String> = Vec::new();
            let mut volume_vec: Vec<f64> = Vec::new();
            for account_volume in select_data {
                account_vec.push(account_volume.account_id);
                volume_vec.push(account_volume.volume.with_scale(2).to_f64().unwrap());
            }
            TradingVolumeRanking {
                account_ids: account_vec,
                volume: volume_vec,
            }
        }
        Err(_) => TradingVolumeRanking {
            account_ids: vec![],
            volume: vec![],
        },
    }
}

pub async fn get_user_perp_holding_ranking(
    p_symbol: String,
    account_size: i64,
) -> UserPerpHoldingRanking {
    #[allow(unused_imports)]
    use orderly_dashboard_analyzer::{
        db::{user_perp_summary::UserPerpSummary, POOL},
        schema::user_perp_summary,
        schema::user_perp_summary::dsl::*,
    };

    let select_result = diesel::sql_query("select account_id,holding from user_perp_summary where symbol = $1 order by holding desc limit $2")
        .bind::<Text, _>(p_symbol)
        .bind::<BigInt, _>(account_size)
        .get_results_async::<AccountPerpHolding>(&POOL)
        .await;

    match select_result {
        Ok(select_data) => {
            let mut account_vec: Vec<String> = Vec::new();
            let mut holding_vec: Vec<f64> = Vec::new();

            for perp_holding in select_data {
                account_vec.push(perp_holding.account_id);
                holding_vec.push(perp_holding.holding.with_scale(2).to_f64().unwrap());
            }
            UserPerpHoldingRanking {
                account_ids: account_vec,
                holding: holding_vec,
            }
        }
        Err(_) => UserPerpHoldingRanking {
            account_ids: vec![],
            holding: vec![],
        },
    }
}

#[derive(Debug, Clone, QueryableByName)]
pub struct UserSymbolHoldingRank {
    #[sql_type = "Text"]
    pub account_id: String,
    #[sql_type = "Text"]
    pub symbol_hash: String,
    #[sql_type = "Numeric"]
    pub holding: BigDecimal,
    #[sql_type = "Text"]
    pub symbol: String,
    #[sql_type = "Numeric"]
    pub index_price: BigDecimal,
    #[sql_type = "Numeric"]
    pub holding_value: BigDecimal,
}

// slow query, should not be used frequently
pub async fn query_user_perp_max_symbol_holding(
    offset: i32,
    limit: i32,
    account_id: Option<String>,
    symbol_hash: Option<String>,
) -> anyhow::Result<Vec<UserSymbolHoldingRank>> {
    use orderly_dashboard_analyzer::db::POOL;

    let select_result = if account_id.is_none() && symbol_hash.is_none() {
        let sql_query = diesel::sql_query(
            "
    SELECT
        u.account_id,
        u.symbol as symbol_hash,
        u.holding,
        m.symbol,
        m.index_price,
        ABS(u.holding * m.index_price) AS holding_value
    FROM user_perp_summary u
    JOIN market_info m ON u.symbol = m.symbol_hash
    WHERE u.holding != 0
    ORDER BY holding_value DESC
    OFFSET $1
    LIMIT $2;
            ",
        )
        .bind::<Integer, _>(offset)
        .bind::<Integer, _>(limit);
        tracing::info!("query_user_perp_max_symbol_holding start");
        let inst = Instant::now();

        let select_result: Vec<UserSymbolHoldingRank> = sql_query
            .get_results_async::<UserSymbolHoldingRank>(&POOL)
            .await?;
        tracing::info!(
            "query_user_perp_max_symbol_holding end, elapse_ms: {}",
            inst.elapsed().as_millis()
        );

        select_result
    } else if account_id.is_none() && symbol_hash.is_some() {
        let symbol_hash = symbol_hash.unwrap_or_default();
        let sql_query = diesel::sql_query(
            "
    SELECT
        u.account_id,
        u.symbol as symbol_hash,
        u.holding,
        m.symbol,
        m.index_price,
        ABS(u.holding * m.index_price) AS holding_value
    FROM user_perp_summary u
    JOIN market_info m ON u.symbol = m.symbol_hash
    WHERE u.symbol = $1 AND u.holding != 0 
    ORDER BY holding_value DESC
    OFFSET $2
    LIMIT $3;
            ",
        )
        .bind::<Text, _>(symbol_hash.clone())
        .bind::<Integer, _>(offset)
        .bind::<Integer, _>(limit);

        tracing::info!(
            "query_user_perp_max_symbol_holding with symbol: {}, offset: {}, limit: {}",
            symbol_hash,
            offset,
            limit
        );
        let inst = Instant::now();

        let select_result: Vec<UserSymbolHoldingRank> = sql_query
            .get_results_async::<UserSymbolHoldingRank>(&POOL)
            .await?;
        tracing::info!(
            "query_user_perp_max_symbol_holding end, elapse_ms: {}",
            inst.elapsed().as_millis()
        );

        select_result
    } else if account_id.is_some() && symbol_hash.is_none() {
        let account_id = account_id.unwrap_or_default();
        let sql_query = diesel::sql_query(
            "
    SELECT
        u.account_id,
        u.symbol as symbol_hash,
        u.holding,
        m.symbol,
        m.index_price,
        ABS(u.holding * m.index_price) AS holding_value
    FROM user_perp_summary u
    JOIN market_info m ON u.symbol = m.symbol_hash
    WHERE u.account_id = $1 AND u.holding != 0 
    ORDER BY holding_value DESC
    OFFSET $2
    LIMIT $3;
            ",
        )
        .bind::<Text, _>(account_id.clone())
        .bind::<Integer, _>(offset)
        .bind::<Integer, _>(limit);
        tracing::info!(
            "query_user_perp_max_symbol_holding with account_id: {}, offset: {}, limit: {}",
            account_id,
            offset,
            limit
        );
        let inst = Instant::now();

        let select_result: Vec<UserSymbolHoldingRank> = sql_query
            .get_results_async::<UserSymbolHoldingRank>(&POOL)
            .await?;
        tracing::info!(
            "query_user_perp_max_symbol_holding end, elapse_ms: {}",
            inst.elapsed().as_millis()
        );

        select_result
    } else {
        let account_id = account_id.unwrap_or_default();
        let symbol_hash = symbol_hash.unwrap_or_default();
        let sql_query = diesel::sql_query(
            "
    SELECT
        u.account_id,
        u.symbol as symbol_hash,
        u.holding,
        m.symbol,
        m.index_price,
        ABS(u.holding * m.index_price) AS holding_value
    FROM user_perp_summary u
    JOIN market_info m ON u.symbol = m.symbol_hash
    WHERE u.account_id = $1 AND u.symbol = $2 AND u.holding != 0 
    ORDER BY holding_value DESC
    OFFSET $3
    LIMIT $4;
            ",
        )
        .bind::<Text, _>(account_id.clone())
        .bind::<Text, _>(symbol_hash.clone())
        .bind::<Integer, _>(offset)
        .bind::<Integer, _>(limit);
        tracing::info!("query_user_perp_max_symbol_holding with account_id: {}, symbol_hash: {}, offset: {}, limit: {}", account_id, symbol_hash, offset, limit);
        let inst = Instant::now();

        let select_result: Vec<UserSymbolHoldingRank> = sql_query
            .get_results_async::<UserSymbolHoldingRank>(&POOL)
            .await?;
        tracing::info!(
            "query_user_perp_max_symbol_holding end, elapse_ms: {}",
            inst.elapsed().as_millis()
        );

        select_result
    };

    Ok(select_result)
}