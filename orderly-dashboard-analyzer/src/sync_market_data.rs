use std::time::Duration;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use num_traits::FromPrimitive;

use crate::{
    client::list_market_infos,
    db::market_info::{create_or_update_market_infos, DBMarketInfo},
};

pub fn update_market_infos_task(base_url: String) {
    orderly_dashboard_indexer::runtime::spawn_future(async move {
        loop {
            if let Err(err) = upsert_market_infos(&base_url).await {
                tracing::warn!("upsert_market_infos failed with err: {}", err);
            } else {
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
        #[allow(unreachable_code)]
        Ok(())
    });
}

async fn upsert_market_infos(base_url: &str) -> anyhow::Result<()> {
    let data = list_market_infos(base_url).await.unwrap();
    tracing::info!("market infos lens: {:?}", data.data.rows.len());
    let nsecs = data.timestamp % 1000 * 1_000_000;
    let update_time =
        NaiveDateTime::from_timestamp_opt(data.timestamp / 1000, nsecs as u32).unwrap();
    let market_infos = data
        .data
        .rows
        .into_iter()
        .map(|v| {
            DBMarketInfo::new(
                v.symbol,
                BigDecimal::from_f64(v.index_price).unwrap(),
                BigDecimal::from_f64(v.mark_price).unwrap(),
                BigDecimal::from_f64(v.sum_unitary_funding).unwrap(),
                BigDecimal::from_f64(v.open_interest).unwrap(),
                update_time,
            )
        })
        .collect::<Vec<_>>();
    let market_data_len = market_infos.len();
    create_or_update_market_infos(market_infos).await?;
    tracing::info!("create_or_update_market_infos len: {:?}", market_data_len);
    Ok(())
}
