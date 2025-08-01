use crate::db::DB_CONN_ERR_MSG;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{Duration, Local};
use diesel::sql_types::*;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use std::time::Instant;

use crate::format_extern::trading_metrics::{
    TokenAmountRanking, TradingPnlRanking, TradingVolumeRanking, UserPerpHoldingRanking,
};

#[derive(Debug, QueryableByName, Clone)]
pub struct AccountVolume {
    #[diesel(sql_type = Text)]
    account_id: String,
    #[diesel(sql_type = Numeric)]
    volume: BigDecimal,
}

#[derive(Debug, QueryableByName, Clone)]
pub struct AccountDepositWithdrawView {
    #[diesel(sql_type = Text)]
    account_id: String,
    #[diesel(sql_type = Text)]
    token: String,
    #[diesel(sql_type = Numeric)]
    amount: BigDecimal,
}

#[derive(Debug, QueryableByName, Clone)]
pub struct DbTradingPnlRankingView {
    #[diesel(sql_type = Text)]
    pub account_id: String,
    #[diesel(sql_type = Text)]
    pub symbol: String,
    #[diesel(sql_type = Numeric)]
    pub realized_pnl: BigDecimal,
}

#[derive(Debug, QueryableByName, Clone)]
pub struct AccountPerpHolding {
    #[diesel(sql_type = Text)]
    account_id: String,
    #[diesel(sql_type = Numeric)]
    holding: BigDecimal,
}

pub async fn get_token_ranking(
    hour: i64,
    account_size: i64,
    withdraw: bool,
    token_hash: String,
) -> Vec<TokenAmountRanking> {
    #[allow(unused_imports)]
    use orderly_dashboard_analyzer::{
        db::{hourly_user_token::HourlyUserToken, POOL},
        schema::hourly_user_token,
        schema::hourly_user_token::dsl::*,
    };

    let now = Local::now().naive_utc();
    let start_time = now - Duration::hours(hour);

    let mut sql = "select account_id, token, sum(withdraw_amount) as amount from hourly_user_token \
    where block_hour>=$1 and token_hash=$2 group by account_id, token order by volume desc limit $3";

    if !withdraw {
        sql = "select account_id, token, sum(deposit_amount) as amount from hourly_user_token \
    where block_hour>=$1 and token_hash=$2 group by account_id, token order by volume desc limit $3"
    }

    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result = diesel::sql_query(sql)
        .bind::<Timestamp, _>(start_time)
        .bind::<Text, _>(token_hash)
        .bind::<BigInt, _>(account_size)
        .get_results::<AccountDepositWithdrawView>(&mut conn)
        .await;

    let mut deposit_withdraw_vec: Vec<TokenAmountRanking> = Vec::new();
    match select_result {
        Ok(select_data) => {
            for account_volume in select_data {
                deposit_withdraw_vec.push(TokenAmountRanking {
                    account_id: account_volume.account_id,
                    token_hash: account_volume.token,
                    amount: account_volume.amount.to_string(),
                });
            }
        }
        Err(_) => {}
    }
    deposit_withdraw_vec
}

pub async fn get_pnl_ranking(
    hour: i64,
    account_size: i64,
    symbol_hash: Option<String>,
) -> Vec<TradingPnlRanking> {
    #[allow(unused_imports)]
    use orderly_dashboard_analyzer::{
        db::{hourly_user_perp::HourlyUserPerp, POOL},
        schema::hourly_user_perp,
        schema::hourly_user_perp::dsl::*,
    };

    let now = Local::now().naive_utc();
    let start_time = now - Duration::hours(hour);
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    // let select_result = diesel::sql_query(
    //     "select account_id, symbol, sum(realized_pnl) as realized_pnl  from hourly_user_perp \
    // where block_hour>=$1 group by account_id order by volume desc limit $2",
    // )
    // .bind::<Timestamp, _>(start_time)
    // .bind::<BigInt, _>(account_size)
    // .get_results::<DbTradingPnlRankingView>(&mut conn)
    // .await;

    let select_result = if let Some(symbol_hash) = symbol_hash {
        diesel::sql_query(
            "select account_id, symbol, sum(realized_pnl) as realized_pnl  from hourly_user_perp \
        where block_hour>=$1 and symbol_hash=$2 order by realized_pnl desc limit $3",
        )
        .bind::<Timestamp, _>(start_time)
        .bind::<Text, _>(symbol_hash)
        .bind::<BigInt, _>(account_size)
        .get_results::<DbTradingPnlRankingView>(&mut conn)
        .await
    } else {
        diesel::sql_query(
            "select account_id, symbol, sum(realized_pnl) as realized_pnl  from hourly_user_perp \
        where block_hour>=$1 order by realized_pnl desc limit $2",
        )
        .bind::<Timestamp, _>(start_time)
        .bind::<BigInt, _>(account_size)
        .get_results::<DbTradingPnlRankingView>(&mut conn)
        .await
    };

    let mut trading_pnl_ranking_v: Vec<_> = Vec::new();
    match select_result {
        Ok(select_data) => {
            for trading_pnl_ranking in select_data {
                trading_pnl_ranking_v.push(TradingPnlRanking {
                    account_id: trading_pnl_ranking.account_id,
                    symbol: trading_pnl_ranking.symbol,
                    realized_pnl: trading_pnl_ranking.realized_pnl.to_string(),
                });
            }
            trading_pnl_ranking_v
        }
        Err(_) => trading_pnl_ranking_v,
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
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let select_result = diesel::sql_query(
        "select account_id,sum(trading_volume) as volume from hourly_user_perp \
    where block_hour>=$1 group by account_id order by volume desc limit $2",
    )
    .bind::<Timestamp, _>(start_time)
    .bind::<BigInt, _>(account_size)
    .get_results::<AccountVolume>(&mut conn)
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

    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result = diesel::sql_query("select account_id,holding from user_perp_summary where symbol = $1 order by holding desc limit $2")
        .bind::<Text, _>(p_symbol)
        .bind::<BigInt, _>(account_size)
        .get_results::<AccountPerpHolding>(&mut conn)
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
pub struct UserSymbolSummaryRank {
    #[diesel(sql_type = Text)]
    pub account_id: String,
    #[diesel(sql_type = Text)]
    pub address: String,
    #[diesel(sql_type = Text)]
    pub broker_id: String,
    #[diesel(sql_type = Text)]
    pub symbol_hash: String,
    #[diesel(sql_type = Numeric)]
    pub holding: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub total_realized_pnl: BigDecimal,
    #[diesel(sql_type = Text)]
    pub symbol: String,
    #[diesel(sql_type = Numeric)]
    pub index_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub mark_price: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub holding_value: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub opening_cost: BigDecimal,
}

// slow query, should not be used frequently
pub async fn query_user_perp_max_symbol_holding(
    offset: i32,
    limit: i32,
    account_id: Option<String>,
    symbol_hash: Option<String>,
) -> anyhow::Result<Vec<UserSymbolSummaryRank>> {
    use orderly_dashboard_analyzer::db::POOL;

    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result = if account_id.is_none() && symbol_hash.is_none() {
        let sql_query = diesel::sql_query(
            "
SELECT
  u.account_id,
  us.address,
  us.broker_id,
  u.symbol as symbol_hash,
  u.holding,
  u.total_realized_pnl,
  m.symbol,
  m.index_price,
  m.mark_price,
  ABS(u.holding * m.index_price) AS holding_value,
  u.opening_cost
FROM
  user_perp_summary u
  JOIN market_info m ON u.symbol = m.symbol_hash
  JOIN user_info us ON u.account_id = us.account_id
ORDER BY holding_value DESC
OFFSET $1
LIMIT $2;
            ",
        )
        .bind::<Integer, _>(offset)
        .bind::<Integer, _>(limit);
        tracing::info!("query_user_perp_max_symbol_holding start");
        let inst = Instant::now();

        let select_result: Vec<UserSymbolSummaryRank> = sql_query
            .get_results::<UserSymbolSummaryRank>(&mut conn)
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
        u.total_realized_pnl,
        m.symbol,
        m.index_price,
        m.mark_price,
        ABS(u.holding * m.index_price) AS holding_value,
        u.opening_cost
    FROM user_perp_summary u
    JOIN market_info m ON u.symbol = m.symbol_hash
    JOIN user_info us ON u.account_id = us.account_id
    WHERE u.symbol = $1
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

        let select_result: Vec<UserSymbolSummaryRank> = sql_query
            .get_results::<UserSymbolSummaryRank>(&mut conn)
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
        us.address,
        us.broker_id,
        u.symbol as symbol_hash,
        u.holding,
        u.total_realized_pnl,
        m.symbol,
        m.index_price,
        m.mark_price,
        ABS(u.holding * m.index_price) AS holding_value,
        u.opening_cost
    FROM user_perp_summary u
    JOIN market_info m ON u.symbol = m.symbol_hash
    JOIN user_info us ON u.account_id = us.account_id
    WHERE u.account_id = $1
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

        let select_result: Vec<UserSymbolSummaryRank> = sql_query
            .get_results::<UserSymbolSummaryRank>(&mut conn)
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
        us.address,
        us.broker_id,
        u.symbol as symbol_hash,
        u.holding,
        u.total_realized_pnl,
        m.symbol,
        m.index_price,
        m.mark_price,
        ABS(u.holding * m.index_price) AS holding_value,
        u.opening_cost
    FROM user_perp_summary u
    JOIN market_info m ON u.symbol = m.symbol_hash
    JOIN user_info us ON u.account_id = us.account_id
    WHERE u.account_id = $1 AND u.symbol = $2
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

        let select_result: Vec<UserSymbolSummaryRank> = sql_query
            .get_results::<UserSymbolSummaryRank>(&mut conn)
            .await?;
        tracing::info!(
            "query_user_perp_max_symbol_holding end, elapse_ms: {}",
            inst.elapsed().as_millis()
        );

        select_result
    };

    Ok(select_result)
}

// slow query, should not be used frequently
pub async fn query_user_perp_max_symbol_realized_pnl(
    offset: i32,
    limit: i32,
    account_id: Option<String>,
    symbol_hash: Option<String>,
    order_by: String,
) -> anyhow::Result<Vec<UserSymbolSummaryRank>> {
    use orderly_dashboard_analyzer::db::POOL;

    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result = if account_id.is_none() && symbol_hash.is_none() {
        let sql_query = diesel::sql_query(format!(
            "
    SELECT
        u.account_id,
        us.address,
        us.broker_id,
        u.symbol as symbol_hash,
        u.holding,
        u.total_realized_pnl,
        m.symbol,
        m.index_price,
        m.mark_price,
        ABS(u.holding * m.index_price) AS holding_value,
        u.opening_cost
    FROM user_perp_summary u
    JOIN market_info m ON u.symbol = m.symbol_hash
    JOIN user_info us ON u.account_id = us.account_id
    ORDER BY total_realized_pnl {}
    OFFSET $1
    LIMIT $2;
            ",
            order_by
        ))
        .bind::<Integer, _>(offset)
        .bind::<Integer, _>(limit);
        tracing::info!("query_user_perp_max_symbol_realized_pnl start",);
        let inst = Instant::now();

        let select_result: Vec<UserSymbolSummaryRank> = sql_query
            .get_results::<UserSymbolSummaryRank>(&mut conn)
            .await?;
        tracing::info!(
            "query_user_perp_max_symbol_realized_pnl end, elapse_ms: {}",
            inst.elapsed().as_millis()
        );

        select_result
    } else if account_id.is_none() && symbol_hash.is_some() {
        let symbol_hash = symbol_hash.unwrap_or_default();
        let sql_query = diesel::sql_query(format!(
            "
    SELECT
        u.account_id,
        us.address,
        us.broker_id,
        u.symbol as symbol_hash,
        u.holding,
        u.total_realized_pnl,
        m.symbol,
        m.index_price,
        m.mark_price,
        ABS(u.holding * m.index_price) AS holding_value,
        u.opening_cost
    FROM user_perp_summary u
    JOIN market_info m ON u.symbol = m.symbol_hash
    JOIN user_info us ON u.account_id = us.account_id
    WHERE u.symbol = $1
    ORDER BY total_realized_pnl {}
    OFFSET $2
    LIMIT $3;
            ",
            order_by
        ))
        .bind::<Text, _>(symbol_hash.clone())
        .bind::<Integer, _>(offset)
        .bind::<Integer, _>(limit);

        tracing::info!(
            "query_user_perp_max_symbol_realized_pnl with symbol: {}, offset: {}, limit: {}",
            symbol_hash,
            offset,
            limit
        );
        let inst = Instant::now();

        let select_result: Vec<UserSymbolSummaryRank> = sql_query
            .get_results::<UserSymbolSummaryRank>(&mut conn)
            .await?;
        tracing::info!(
            "query_user_perp_max_symbol_realized_pnl end, elapse_ms: {}",
            inst.elapsed().as_millis()
        );

        select_result
    } else if account_id.is_some() && symbol_hash.is_none() {
        let account_id = account_id.unwrap_or_default();
        let sql_query = diesel::sql_query(format!(
            "
    SELECT
        u.account_id,
        us.address,
        us.broker_id,
        u.symbol as symbol_hash,
        u.holding,
        u.total_realized_pnl,
        m.symbol,
        m.index_price,
        m.mark_price,
        ABS(u.holding * m.index_price) AS holding_value,
        u.opening_cost
    FROM user_perp_summary u
    JOIN market_info m ON u.symbol = m.symbol_hash
    JOIN user_info us ON u.account_id = us.account_id
    WHERE u.account_id = $1
    ORDER BY total_realized_pnl ${}
    OFFSET $2
    LIMIT $3;
            ",
            order_by
        ))
        .bind::<Text, _>(account_id.clone())
        .bind::<Integer, _>(offset)
        .bind::<Integer, _>(limit);
        tracing::info!(
            "query_user_perp_max_symbol_realized_pnl with account_id: {}, offset: {}, limit: {}",
            account_id,
            offset,
            limit
        );
        let inst = Instant::now();

        let select_result: Vec<UserSymbolSummaryRank> = sql_query
            .get_results::<UserSymbolSummaryRank>(&mut conn)
            .await?;
        tracing::info!(
            "query_user_perp_max_symbol_holding end, elapse_ms: {}",
            inst.elapsed().as_millis()
        );

        select_result
    } else {
        let account_id = account_id.unwrap_or_default();
        let symbol_hash = symbol_hash.unwrap_or_default();
        let sql_query = diesel::sql_query(format!(
            "
    SELECT
        u.account_id,
        us.address,
        us.broker_id,
        u.symbol as symbol_hash,
        u.holding,
        u.total_realized_pnl,
        m.symbol,
        m.index_price,
        m.mark_price,
        ABS(u.holding * m.index_price) AS holding_value,
        u.opening_cost
    FROM user_perp_summary u
    JOIN market_info m ON u.symbol = m.symbol_hash
    JOIN user_info us ON u.account_id = us.account_id
    WHERE u.account_id = $1 AND u.symbol = $2
    ORDER BY total_realized_pnl {}
    OFFSET $3
    LIMIT $4;
            ",
            order_by
        ))
        .bind::<Text, _>(account_id.clone())
        .bind::<Text, _>(symbol_hash.clone())
        .bind::<Integer, _>(offset)
        .bind::<Integer, _>(limit);
        tracing::info!("query_user_perp_max_symbol_holding with account_id: {}, symbol_hash: {}, offset: {}, limit: {}", account_id, symbol_hash, offset, limit);
        let inst = Instant::now();

        let select_result: Vec<UserSymbolSummaryRank> = sql_query
            .get_results::<UserSymbolSummaryRank>(&mut conn)
            .await?;
        tracing::info!(
            "query_user_perp_max_symbol_holding end, elapse_ms: {}",
            inst.elapsed().as_millis()
        );

        select_result
    };

    Ok(select_result)
}
