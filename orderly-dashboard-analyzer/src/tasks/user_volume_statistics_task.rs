use std::{
    collections::{BTreeMap, BTreeSet},
    time::Instant,
};

use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use orderly_dashboard_analyzer::db::{
    user_info::get_user_infos,
    user_volume_statistics::{create_or_user_volume_statistics, DBNewUserVolumeStatistics},
};

use crate::{
    client::cefi_get_account_info,
    db::{
        hourly_user_perp::get_user_trading_volume_in_time_range,
        user_info::{create_user_info, UserInfo},
        user_perp_summary::get_user_ltd_trading_volume,
    },
    sync_broker::cal_broker_hash,
};

pub async fn cal_user_volume_statistics_task(base_url: String) -> anyhow::Result<()> {
    loop {
        if let Err(err) = cal_user_volume_statistics(&base_url).await {
            tracing::warn!("cal_user_volume_statistics err: {}", err);
        }
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}

pub async fn cal_user_volume_statistics(base_url: &str) -> anyhow::Result<()> {
    let limit: usize = 1000;
    let mut offset_account_id = None;
    let (ytd_from, ytd_to) = get_ytd_time_range();
    let (d90_from, d90_to) = get_90d_time_range();
    let (d30_from, d30_to) = get_30d_time_range();
    let (d7_from, d7_to) = get_7d_time_range();
    let (d1_from, d1_to) = get_1d_time_range();

    let inst = std::time::Instant::now();
    loop {
        let inst1 = Instant::now();
        let ltd_res1 = get_user_ltd_trading_volume(offset_account_id.clone(), limit as i64).await?;
        tracing::info!(
            "get_user_trading_volume_in_time_range ltd time cost: {} ms, offset_account_id: {:?}",
            inst1.elapsed().as_millis(),
            offset_account_id
        );

        let mut account_set = BTreeSet::new();
        let mut account_ids = Vec::with_capacity(limit);
        for res in ltd_res1.iter() {
            account_ids.push(res.account_id.clone());
            account_set.insert(res.account_id.clone());
        }
        let account_brokers = get_user_infos(account_ids.clone()).await?;
        let mut account_broker_addr_map = account_brokers
            .iter()
            .map(|v| {
                (
                    v.account_id.clone(),
                    (v.broker_id.clone(), v.address.clone()),
                )
            })
            .collect::<BTreeMap<_, _>>();
        if account_brokers.len() != ltd_res1.len() {
            let diff_len = ltd_res1.len() - account_brokers.len();
            tracing::info!("missed account length: {}", diff_len);
            for res in ltd_res1.iter() {
                if !account_broker_addr_map.contains_key(&res.account_id) {
                    loop {
                        match cefi_get_account_info(base_url, &res.account_id).await {
                            Ok(account_info) => {
                                if account_info.success {
                                    if let Some(data) = account_info.data {
                                        account_broker_addr_map.insert(
                                            res.account_id.clone(),
                                            (data.broker_id.clone(), data.address.clone()),
                                        );
                                        create_user_info(&UserInfo {
                                            account_id: res.account_id.clone(),
                                            broker_id: data.broker_id.clone(),
                                            broker_hash: cal_broker_hash(&data.broker_id),
                                            address: data.address,
                                        })
                                        .await
                                        .ok();
                                    }
                                } else {
                                    tracing::warn!(
                                        "cefi_get_account_info account_id: {} not success with code: {:?}, message: {:?}",
                                        res.account_id, account_info.code, account_info.message,
                                    );
                                }
                                break;
                            }
                            Err(err) => {
                                tracing::warn!(
                                    "cefi_get_account_info account_id: {} failed with err: {}",
                                    res.account_id,
                                    err
                                );
                                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                            }
                        }
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                }
            }
        }

        // ytd
        let ytd_res1 =
            seperate_get_user_trading_volume_in_time_range(&account_ids, ytd_from, ytd_to, "ytd")
                .await?;

        // 90d
        let account_ids = ytd_res1.keys().cloned().collect();
        let d90_res1 =
            seperate_get_user_trading_volume_in_time_range(&account_ids, d90_from, d90_to, "d90")
                .await?;

        // 30d
        let account_ids = d90_res1.keys().cloned().collect();
        let d30_res1 =
            seperate_get_user_trading_volume_in_time_range(&account_ids, d30_from, d30_to, "d30")
                .await?;

        // 7d
        let account_ids = d30_res1.keys().cloned().collect();
        let d7_res1 =
            seperate_get_user_trading_volume_in_time_range(&account_ids, d7_from, d7_to, "d7")
                .await?;

        // 1d
        let account_ids = d7_res1.keys().cloned().collect();
        let d1_res1 =
            seperate_get_user_trading_volume_in_time_range(&account_ids, d1_from, d1_to, "d1")
                .await?;

        let ltd_res1_len = ltd_res1.len();
        let last_ltd = ltd_res1.last().cloned();

        let mut user_volume_statistics: Vec<DBNewUserVolumeStatistics> =
            Vec::with_capacity(ltd_res1_len);
        for account_v in ltd_res1 {
            let (broker_id, address) = account_broker_addr_map
                .get(&account_v.account_id)
                .cloned()
                .unwrap_or_default();
            user_volume_statistics.push(DBNewUserVolumeStatistics::new(
                account_v.account_id.clone(),
                broker_id,
                if let Some(volume) = ytd_res1.get(&account_v.account_id) {
                    volume.clone()
                } else {
                    BigDecimal::from(0)
                },
                account_v.volume.clone(),
                d1_res1
                    .get(&account_v.account_id)
                    .cloned()
                    .unwrap_or(BigDecimal::from(0)),
                d7_res1
                    .get(&account_v.account_id)
                    .cloned()
                    .unwrap_or(BigDecimal::from(0)),
                d30_res1
                    .get(&account_v.account_id)
                    .cloned()
                    .unwrap_or(BigDecimal::from(0)),
                address,
                d90_res1
                    .get(&account_v.account_id)
                    .cloned()
                    .unwrap_or(BigDecimal::from(0)),
            ));
        }
        create_or_user_volume_statistics(user_volume_statistics).await?;

        if ltd_res1_len < limit {
            break;
        } else {
            if let Some(account) = last_ltd {
                offset_account_id = Some(account.account_id);
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }

    let time_cost_s = inst.elapsed().as_secs();
    tracing::info!("cal_user_volume_statistics task cost: {} s", time_cost_s);
    Ok(())
}

fn get_ytd_time_range() -> (i64, i64) {
    // yesterday, yesterday - 365d
    let now = Utc::now();
    let to = (now - Duration::days(1))
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap_or_default();
    let from = (now - Duration::days(366))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap_or_default();
    (from.timestamp(), to.timestamp())
}

fn get_30d_time_range() -> (i64, i64) {
    // yesterday, yesterday - 365d
    let now = Utc::now();
    let to = (now - Duration::days(1))
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap_or_default();
    let from = (now - Duration::days(31))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap_or_default();
    (from.timestamp(), to.timestamp())
}

fn get_90d_time_range() -> (i64, i64) {
    // yesterday, yesterday - 365d
    let now = Utc::now();
    let to = (now - Duration::days(1))
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap_or_default();
    let from = (now - Duration::days(91))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap_or_default();
    (from.timestamp(), to.timestamp())
}

fn get_7d_time_range() -> (i64, i64) {
    // yesterday, yesterday - 7d
    let now = Utc::now();
    let to = (now - Duration::days(1))
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap_or_default();
    let from = (now - Duration::days(8))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap_or_default();
    (from.timestamp(), to.timestamp())
}

fn get_1d_time_range() -> (i64, i64) {
    // yesterday, yesterday - 7d
    let now = Utc::now();
    let to = (now - Duration::days(1))
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap_or_default();
    let from = (now - Duration::days(1))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap_or_default();
    (from.timestamp(), to.timestamp())
}

async fn seperate_get_user_trading_volume_in_time_range(
    account_ids: &Vec<String>,
    from_time: i64,
    to_time: i64,
    context: &str,
) -> anyhow::Result<BTreeMap<String, BigDecimal>> {
    let inst1 = Instant::now();
    let mut cache: BTreeMap<String, BigDecimal> = BTreeMap::new();
    for chunk in account_ids.chunks(10) {
        get_user_trading_volume_in_time_range(chunk.to_vec(), from_time, to_time)
            .await?
            .into_iter()
            .for_each(|v| {
                cache.insert(v.account_id, v.volume);
            });
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }

    tracing::info!(
        "get_user_trading_volume_in_time_range {} account_ids length: {}, result len: {}, time cost: {} ms",
        context,
        account_ids.len(),
        cache.len(),
        inst1.elapsed().as_millis()
    );
    Ok(cache)
}
