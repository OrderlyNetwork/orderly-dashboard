use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::QueryableByName;
use diesel::sql_types::{Date, Numeric, Timestamp};

use crate::db::DB_CONTEXT;

#[allow(unused_imports)]
use orderly_dashboard_analyzer::{
    db::{hourly_orderly_token::HourlyOrderlyToken, POOL},
    schema::hourly_orderly_token,
    schema::hourly_orderly_token::dsl::*,
};

use crate::format_extern::trading_metrics::{DailyData, OrderlyTokenDaily};

#[derive(Debug, Clone, QueryableByName)]
pub struct OrderlyDailyData {
    #[sql_type = "Date"]
    pub trading_day: NaiveDate,
    #[sql_type = "Numeric"]
    pub withdraw_amount: BigDecimal,
    #[sql_type = "Numeric"]
    pub withdraw_count: BigDecimal,
    #[sql_type = "Numeric"]
    pub deposit_amount: BigDecimal,
    #[sql_type = "Numeric"]
    pub deposit_count: BigDecimal,
}


pub async fn get_daily_token(
    from_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> DailyData<OrderlyTokenDaily> {
    let sql_query = diesel::sql_query(
        "select \
      date(block_hour) as trading_day,\
      sum(withdraw_amount) as withdraw_amount, \
      sum(withdraw_count) as withdraw_count, \
      sum(deposit_amount) as deposit_amount, \
      sum(deposit_count) as deposit_count \
      from hourly_orderly_token where block_hour>=$1 and block_hour<=$2 \
      group by trading_day order by trading_day desc;",
    );

    let result: Result<Vec<OrderlyDailyData>, _> = sql_query
        .bind::<Timestamp, _>(from_time)
        .bind::<Timestamp, _>(end_time)
        .get_results_async::<OrderlyDailyData>(&POOL)
        .await;

    let mut daytime_vec: Vec<String> = Vec::new();
    let mut orderly_token_vec: Vec<OrderlyTokenDaily> = Vec::new();
    match result {
        Ok(daily_data_vec) => {
            let daily_format = "%Y-%m-%d";
            for daily_data in daily_data_vec {
                daytime_vec.push(daily_data.trading_day.format(daily_format).to_string());
                orderly_token_vec.push(OrderlyTokenDaily {
                    withdraw_amount: daily_data.withdraw_amount.to_f64().unwrap(),
                    withdraw_count: daily_data.withdraw_count.to_f64().unwrap(),
                    deposit_amount: daily_data.deposit_amount.to_f64().unwrap(),
                    deposit_count: daily_data.deposit_count.to_f64().unwrap(),
                });
            }
        }
        Err(error) => {
            tracing::error!(target:DB_CONTEXT,"{}",error);
        }
    };
    DailyData { daytime: daytime_vec, data: orderly_token_vec }
}