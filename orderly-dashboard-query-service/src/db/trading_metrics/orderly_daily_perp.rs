use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::QueryableByName;
use diesel::sql_types::{Date, Numeric, Timestamp};

#[allow(unused_imports)]
use orderly_dashboard_analyzer::{
    db::{hourly_orderly_perp::HourlyOrderlyPerp, POOL},
    schema::hourly_orderly_perp,
    schema::hourly_orderly_perp::dsl::*,
};

use crate::db::DB_CONTEXT;
use crate::format_extern::trading_metrics::{DailyData, OrderlyPerpDaily};

#[derive(Debug, Clone, QueryableByName)]
struct OrderlyDailyData {
    #[sql_type = "Date"]
    pub trading_day: NaiveDate,
    #[sql_type = "Numeric"]
    pub trading_volume: BigDecimal,
    #[sql_type = "Numeric"]
    pub trading_fee: BigDecimal,
    #[sql_type = "Numeric"]
    pub trading_count: BigDecimal,
    #[sql_type = "Numeric"]
    pub trading_user_count: BigDecimal,
    #[sql_type = "Numeric"]
    pub liquidation_amount: BigDecimal,
    #[sql_type = "Numeric"]
    pub liquidation_count: BigDecimal,
    #[sql_type = "Numeric"]
    pub opening_count: BigDecimal,
}

pub async fn daily_orderly_perp(from_time: NaiveDateTime, end_time: NaiveDateTime) -> DailyData<OrderlyPerpDaily> {
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
      group by trading_day order by trading_day desc;",
    );

    let select_result: Result<Vec<OrderlyDailyData>, _> = sql_query
        .bind::<Timestamp, _>(from_time)
        .bind::<Timestamp, _>(end_time)
        .get_results_async::<OrderlyDailyData>(&POOL)
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
    DailyData { daytime: daytime_vec, data: orderly_perp_vec }
}