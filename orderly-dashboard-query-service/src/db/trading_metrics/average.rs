use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::sql_types::*;
use diesel::QueryableByName;

use crate::db::DB_CONTEXT;
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
    let now = Utc::now().naive_utc();
    let current_day_start = now.clone() - Duration::days(1);
    let current_week_start = now.clone() - Duration::days(7);
    let current_month_start = now.clone() - Duration::days(30);

    let current_day = load_average_metric(field, 1, current_day_start).await;
    let current_week = load_average_metric(field, 7, current_week_start).await;
    let current_month = load_average_metric(field, 30, current_month_start).await;
    CountAverageExtern {
        latest_day_metric: current_day.count.with_scale(2).to_f64().unwrap(),
        latest_week_metric: current_week.count.with_scale(2).to_f64().unwrap(),
        latest_month_metric: current_month.count.with_scale(2).to_f64().unwrap(),
    }
}

async fn load_average_metric(field: &str, days: i32, start_time: NaiveDateTime) -> CountWrapper {
    let query = format!(
        "select sum({})/{} as count from hourly_orderly_perp where block_hour>=$1",
        field, days
    );
    tracing::debug!(target: DB_CONTEXT, "{}; block_hour:{}", query, start_time);
    let select_result = diesel::sql_query(query)
        .bind::<Timestamp, _>(start_time)
        .get_result_async::<CountWrapper>(&POOL)
        .await;

    match select_result {
        Ok(select_data) => select_data,
        Err(_) => CountWrapper {
            count: BigDecimal::from(0),
        },
    }
}
