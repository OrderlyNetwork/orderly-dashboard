use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::sql_types::{Date, Numeric, Timestamp, Varchar};
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;

#[allow(unused_imports)]
use orderly_dashboard_analyzer::{
    db::{hourly_orderly_perp::HourlyOrderlyPerp, POOL},
    schema::hourly_orderly_perp,
    // schema::hourly_orderly_perp::dsl::*,
};

use crate::db::{DB_CONN_ERR_MSG, DB_CONTEXT};
use crate::format_extern::trading_metrics::{DailyData, OrderlyGasFee, OrderlyPerpDaily};

#[derive(Debug, Clone, QueryableByName)]
struct OrderlyDailyData {
    #[diesel(sql_type = Date)]
    pub trading_day: NaiveDate,
    #[diesel(sql_type = Numeric)]
    pub trading_volume: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub trading_fee: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub trading_count: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub trading_user_count: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub liquidation_amount: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub liquidation_count: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub opening_count: BigDecimal,
}

#[derive(Debug, Clone, QueryableByName)]
struct OrderlyDailyGas {
    #[diesel(sql_type = Date)]
    pub trading_day: NaiveDate,
    #[diesel(sql_type = Numeric)]
    pub avg_gas_fee: BigDecimal,
}

pub async fn daily_orderly_perp(
    from_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> DailyData<OrderlyPerpDaily> {
    let sql_query = diesel::sql_query(
        "select \
      date(block_hour) as trading_day,\
      sum(trading_volume) as trading_volume, \
      sum(trading_fee) as trading_fee, \
      sum(trading_count) as trading_count,\
      sum(trading_user_count) as trading_user_count,\
      sum(liquidation_amount) as liquidation_amount,\
      sum(liquidation_count) as liquidation_count,\
      sum(opening_count) as opening_count \
      from hourly_orderly_perp where block_hour>=$1 and block_hour<=$2 \
      group by trading_day order by trading_day asc;",
    );
    tracing::debug!(target:DB_CONTEXT,"daily_orderly_perp query string: {:?}; from_time: {}, end_time: {}", sql_query, from_time, end_time);
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result: Result<Vec<OrderlyDailyData>, _> = sql_query
        .bind::<Timestamp, _>(from_time)
        .bind::<Timestamp, _>(end_time)
        .get_results::<OrderlyDailyData>(&mut conn)
        .await;

    let mut daytime_vec: Vec<String> = Vec::new();
    let mut orderly_perp_vec: Vec<OrderlyPerpDaily> = Vec::new();
    match select_result {
        Ok(select_data) => {
            let daily_format = "%Y-%m-%d";
            for daily_data in select_data {
                daytime_vec.push(daily_data.trading_day.format(daily_format).to_string());
                orderly_perp_vec.push(OrderlyPerpDaily {
                    trading_volume: daily_data.trading_volume.to_f64().unwrap(),
                    trading_fee: daily_data.trading_fee.to_f64().unwrap(),
                    trading_count: daily_data.trading_count.to_f64().unwrap(),
                    trading_user_count: daily_data.trading_user_count.to_f64().unwrap(),
                    liquidation_amount: daily_data.liquidation_amount.to_f64().unwrap(),
                    liquidation_count: daily_data.liquidation_count.to_f64().unwrap(),
                    opening_count: daily_data.opening_count.to_f64().unwrap(),
                });
            }
        }
        Err(error) => {
            tracing::error!(target:DB_CONTEXT,"{}",error);
        }
    };
    DailyData {
        daytime: daytime_vec,
        data: orderly_perp_vec,
    }
}

pub async fn daily_gas_fee(
    from_time: NaiveDateTime,
    end_time: NaiveDateTime,
    p_event_type: String,
) -> DailyData<OrderlyGasFee> {
    let sql_query = diesel::sql_query(
        "select \
      date(block_hour) as trading_day,\
      sum(gas_fee)/sum(batch_count) as avg_gas_fee \
      from hourly_gas_fee where block_hour>=$1 and block_hour<=$2 \
      and event_type = $3
      group by trading_day order by trading_day asc;",
    );

    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result: Result<Vec<OrderlyDailyGas>, _> = sql_query
        .bind::<Timestamp, _>(from_time)
        .bind::<Timestamp, _>(end_time)
        .bind::<Varchar, _>(p_event_type)
        .get_results::<OrderlyDailyGas>(&mut conn)
        .await;

    let mut daytime_vec: Vec<String> = Vec::new();
    let mut orderly_perp_vec: Vec<OrderlyGasFee> = Vec::new();
    match select_result {
        Ok(select_data) => {
            let daily_format = "%Y-%m-%d";
            for daily_data in select_data {
                daytime_vec.push(daily_data.trading_day.format(daily_format).to_string());
                orderly_perp_vec.push(OrderlyGasFee {
                    avg_gas_fee: daily_data.avg_gas_fee.to_f64().unwrap(),
                });
            }
        }
        Err(error) => {
            tracing::error!(target:DB_CONTEXT,"{}",error);
        }
    };
    DailyData {
        daytime: daytime_vec,
        data: orderly_perp_vec,
    }
}
