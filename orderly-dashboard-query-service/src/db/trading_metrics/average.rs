use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::sql_types::*;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;

use crate::db::{DB_CONN_ERR_MSG, DB_CONTEXT};

#[allow(unused_imports)]
use orderly_dashboard_analyzer::{
    db::{hourly_orderly_perp::HourlyOrderlyPerp, POOL},
    schema::hourly_orderly_perp,
    schema::hourly_orderly_perp::dsl::*,
};

use crate::format_extern::trading_metrics::CountAverageExtern;

#[derive(Debug, Clone, QueryableByName)]
pub struct CountWrapper {
    #[diesel(sql_type = Numeric)]
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
        last_24hours_metric: current_day.count.with_scale(2).to_f64().unwrap(),
        last_7days_metric: current_week.count.with_scale(2).to_f64().unwrap(),
        last_30days_metric: current_month.count.with_scale(2).to_f64().unwrap(),
    }
}

async fn load_average_metric(field: &str, days: i32, start_time: NaiveDateTime) -> CountWrapper {
    let query = format!(
        "select sum({})/{} as count from hourly_orderly_perp where block_hour>=$1",
        field, days
    );
    tracing::debug!(target: DB_CONTEXT, "{}; block_hour:{}", query, start_time);
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result = diesel::sql_query(query)
        .bind::<Timestamp, _>(start_time)
        .get_result::<CountWrapper>(&mut conn)
        .await;

    match select_result {
        Ok(select_data) => select_data,
        Err(_) => CountWrapper {
            count: BigDecimal::from(0),
        },
    }
}
