use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::user_volume_statistics;
use diesel::pg::upsert::{excluded, on_constraint};

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = user_volume_statistics)]
pub struct DBNewUserVolumeStatistics {
    pub account_id: String,
    pub broker_id: String,
    pub perp_volume_ytd: BigDecimal,
    pub perp_volume_ltd: BigDecimal,
    pub perp_volume_last_1_day: BigDecimal,
    pub perp_volume_last_7_days: BigDecimal,
    pub perp_volume_last_30_days: BigDecimal,
    pub update_time: NaiveDateTime,
    pub address: String,
}

#[allow(dead_code)]
#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = user_volume_statistics)]
pub struct DBUserVolumeStatistics {
    pub id: i64,
    pub account_id: String,
    pub broker_id: String,
    pub perp_volume_ytd: BigDecimal,
    pub perp_volume_ltd: BigDecimal,
    pub perp_volume_last_1_day: BigDecimal,
    pub perp_volume_last_7_days: BigDecimal,
    pub perp_volume_last_30_days: BigDecimal,
    pub update_time: NaiveDateTime,
    pub address: String,
}

impl DBNewUserVolumeStatistics {
    pub fn new(
        account_id: String,
        broker_id: String,
        perp_volume_ytd: BigDecimal,
        perp_volume_ltd: BigDecimal,
        perp_volume_last_1_day: BigDecimal,
        perp_volume_last_7_days: BigDecimal,
        perp_volume_last_30_days: BigDecimal,
        address: String,
    ) -> DBNewUserVolumeStatistics {
        DBNewUserVolumeStatistics {
            account_id,
            broker_id,
            perp_volume_ytd,
            perp_volume_ltd,
            perp_volume_last_1_day,
            perp_volume_last_7_days,
            perp_volume_last_30_days,
            update_time: NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0)
                .unwrap_or_default(),
            address,
        }
    }
}

#[allow(dead_code)]
pub async fn create_or_user_volume_statistics(
    user_volumes: Vec<DBNewUserVolumeStatistics>,
) -> anyhow::Result<()> {
    use crate::schema::user_volume_statistics::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    diesel::insert_into(user_volume_statistics)
        .values(user_volumes)
        .on_conflict(on_constraint("user_volume_statistics_uq"))
        .do_update()
        .set((
            perp_volume_ytd.eq(excluded(perp_volume_ytd)),
            perp_volume_ltd.eq(excluded(perp_volume_ltd)),
            perp_volume_last_1_day.eq(excluded(perp_volume_last_1_day)),
            perp_volume_last_7_days.eq(excluded(perp_volume_last_7_days)),
            perp_volume_last_30_days.eq(excluded(perp_volume_last_30_days)),
            update_time.eq(excluded(update_time)),
        ))
        .execute(&mut conn)
        .await?;

    Ok(())
}
