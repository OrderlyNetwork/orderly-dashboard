use std::time::Duration;

use cached::{Cached, SizedCache};
use tokio::sync::mpsc::{channel, Receiver};

use crate::{
    client::cefi_get_account_info,
    db::user_info::{create_user_info, get_user_info, UserInfo},
    sync_broker::cal_broker_hash,
};

pub async fn sync_account_handler(
    mut account_receiver: Receiver<String>,
    base_url: String,
) -> anyhow::Result<()> {
    let mut sized_cache: SizedCache<String, ()> = SizedCache::with_size(10_000);
    let (tx, rx) = channel::<String>(1000);
    orderly_dashboard_indexer::runtime::spawn_future(sync_account_action(rx, base_url));
    loop {
        match account_receiver.recv().await {
            Some(account_id) => {
                if sized_cache.cache_set(account_id.clone(), ()).is_none() {
                    tx.try_send(account_id).ok();
                    continue;
                }
            }
            None => {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
    }
}

async fn sync_account_action(mut rx: Receiver<String>, base_url: String) -> anyhow::Result<()> {
    loop {
        match rx.recv().await {
            Some(account_id) => match get_user_info(account_id.clone()).await {
                Ok(Some(_)) => {}
                Ok(None) => {
                    tracing::info!("sync account from backend for accound_id: {}", account_id);
                    match cefi_get_account_info(&base_url, &account_id).await {
                        Ok(res) => {
                            if res.success {
                                let data = res.data.unwrap_or_default();
                                create_user_info(&UserInfo {
                                    account_id: account_id,
                                    broker_id: data.broker_id.clone(),
                                    broker_hash: cal_broker_hash(&data.broker_id),
                                    address: data.address,
                                })
                                .await
                                .ok();
                            }
                        }
                        Err(err) => {
                            tracing::warn!(
                                "get_user_info account_id: {} failed with err: {}",
                                account_id,
                                err
                            );
                        }
                    }
                    tokio::time::sleep(Duration::from_millis(200)).await;
                }
                Err(_) => {}
            },
            None => {}
        }
    }
}
