use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{Duration, Local};
use diesel::sql_types::*;
use diesel::QueryableByName;

#[allow(unused_imports)]
use orderly_dashboard_analyzer::{
    db::{hourly_orderly_perp::HourlyOrderlyPerp, POOL},
    schema::hourly_orderly_perp,
    schema::hourly_orderly_perp::dsl::*,
};

use crate::format_extern::trading_metrics::CountAverageExtern;

#[derive(Debug, Clone, QueryableByName)]
pub struct CountWrapper {
    #[sql_type = "Numeric"]
    count: BigDecimal,
}

pub async fn get_average(field: &str) -> CountAverageExtern {
    let now = Local::now().naive_utc();
    let current_day_start = now.clone() - Duration::days(1);
    let current_week_start = now.clone() - Duration::days(7);
    let current_month_start = now.clone() - Duration::days(30);

    let current_day = diesel::sql_query(format!(
        "select sum({})/1 as count from hourly_orderly_perp where block_hour>=$1",
        field
    ))
    .bind::<Timestamp, _>(current_day_start)
    .get_result_async::<CountWrapper>(&POOL)
    .await
    .unwrap();

    let current_week = diesel::sql_query(format!(
        "select sum({})/7 as count from hourly_orderly_perp where block_hour>=$1",
        field
    ))
    .bind::<Timestamp, _>(current_week_start)
    .get_result_async::<CountWrapper>(&POOL)
    .await
    .unwrap();

    let current_month = diesel::sql_query(format!(
        "select sum({})/30 as count from hourly_orderly_perp where block_hour>=$1",
        field
    ))
    .bind::<Timestamp, _>(current_month_start)
    .get_result_async::<CountWrapper>(&POOL)
    .await
    .unwrap();

    CountAverageExtern {
        latest_day_metric: current_day.count.with_scale(2).to_f64().unwrap(),
        latest_week_metric: current_week.count.with_scale(2).to_f64().unwrap(),
        latest_month_metric: current_month.count.with_scale(2).to_f64().unwrap(),
    }
}
