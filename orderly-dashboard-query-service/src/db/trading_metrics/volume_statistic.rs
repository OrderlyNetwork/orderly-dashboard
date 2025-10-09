use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::sql_types::{Numeric, Text};
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
#[allow(unused_imports)]
use orderly_dashboard_analyzer::{
    db::{
        user_volume_statistics::{DBNewUserVolumeStatistics, DBUserVolumeStatistics},
        POOL,
    },
    schema::user_volume_statistics,
};

use crate::db::DB_CONN_ERR_MSG;

#[derive(Debug, Clone, QueryableByName)]
pub struct DbBrokerVolumeStatistic {
    #[diesel(sql_type = Numeric)]
    pub perp_volume_ytd: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub perp_volume_ltd: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub perp_volume_last_1_day: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub perp_volume_last_7_days: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub perp_volume_last_30_days: BigDecimal,
    #[diesel(sql_type = Numeric)]
    pub perp_volume_last_90_days: BigDecimal,
}

pub async fn get_user_volume_statistic(
    account_id_: String,
) -> anyhow::Result<Option<DBUserVolumeStatistics>> {
    use orderly_dashboard_analyzer::schema::user_volume_statistics::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result = user_volume_statistics
        .filter(account_id.eq(account_id_))
        .limit(1)
        .get_result::<DBUserVolumeStatistics>(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(Some(user)),
        Err(error) => match error {
            diesel::NotFound => return Ok(None),
            _ => return Err(anyhow::anyhow!(error)),
        },
    }
}

pub async fn db_get_broker_volume_statistic(
    broker_id_: String,
) -> anyhow::Result<Option<DbBrokerVolumeStatistic>> {
    let sql_query = diesel::sql_query(
        "
select
  sum(perp_volume_ytd) as perp_volume_ytd,
  sum(perp_volume_ltd) as perp_volume_ltd,
  sum(perp_volume_last_1_day) as perp_volume_last_1_day,
  sum(perp_volume_last_7_days) as perp_volume_last_7_days,
  sum(perp_volume_last_30_days) as perp_volume_last_30_days,
  sum(perp_volume_last_90_days) as perp_volume_last_90_days
from
  user_volume_statistics
where
  broker_id = $1;",
    );
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result: Result<DbBrokerVolumeStatistic, _> = sql_query
        .bind::<Text, _>(broker_id_)
        .get_result::<DbBrokerVolumeStatistic>(&mut conn)
        .await;

    match result {
        Ok(broker) => Ok(Some(broker)),
        Err(error) => match error {
            diesel::NotFound => return Ok(None),
            _ => return Err(anyhow::anyhow!(error)),
        },
    }
}
