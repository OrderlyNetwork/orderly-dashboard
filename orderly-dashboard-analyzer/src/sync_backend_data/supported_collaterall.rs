// https://orderly.network/docs/build-on-omnichain/evm-api/restful-api/public/get-supported-collateral-info
use std::{str::FromStr, time::Duration};

use crate::client::list_collecteral_infos;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

use crate::db::collateral_info::{create_or_update_collateral_infos, DBCollateralInfo};

pub fn update_collecteral_infos_task(base_url: String) {
    orderly_dashboard_indexer::runtime::spawn_future(async move {
        loop {
            if let Err(err) = upsert_collecteral_infos(&base_url).await {
                tracing::warn!("upsert_collecteral_infos failed with err: {}", err);
                tokio::time::sleep(Duration::from_secs(30)).await;
            } else {
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        }
        #[allow(unreachable_code)]
        Ok(())
    });
}

async fn upsert_collecteral_infos(base_url: &str) -> anyhow::Result<()> {
    let data = list_collecteral_infos(base_url).await?;
    tracing::info!("collecterall lens: {:?}", data.data.rows.len());
    let nsecs = data.timestamp % 1000 * 1_000_000;
    let update_time =
        NaiveDateTime::from_timestamp_opt(data.timestamp / 1000, nsecs as u32).unwrap();
    let collecterall_infos = data
        .data
        .rows
        .into_iter()
        .map(|v| {
            DBCollateralInfo::new(
                v.token,
                v.token_hash,
                v.decimals as i16,
                BigDecimal::from_str(&v.minimum_withdraw_amount.to_string()).unwrap(),
                BigDecimal::from_str(&v.base_weight.to_string()).unwrap(),
                v.discount_factor.map(|discount_factor| {
                    BigDecimal::from_str(&discount_factor.to_string()).unwrap()
                }),
                BigDecimal::from_str(&v.haircut.to_string()).unwrap(),
                BigDecimal::from_str(&v.user_max_qty.to_string()).unwrap(),
                v.is_collateral,
                update_time,
            )
        })
        .collect::<Vec<_>>();
    let market_data_len = collecterall_infos.len();
    create_or_update_collateral_infos(collecterall_infos).await?;
    tracing::info!(
        "create_or_update_collateral_infos len: {:?}",
        market_data_len
    );
    Ok(())
}
