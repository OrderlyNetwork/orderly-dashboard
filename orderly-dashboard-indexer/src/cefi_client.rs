use chrono::Utc;
use reqwest::Client;
use std::{
    sync::atomic::{AtomicI64, Ordering},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use tokio::time::timeout;

use crate::consume_data_task::ORDERLY_DASHBOARD_INDEXER;

pub struct CefiClient {
    inner_client: Client,
    server_addr: String,
    last_send_timstamp_ms: AtomicI64,
}

impl CefiClient {
    pub fn new(server_addr: String) -> CefiClient {
        CefiClient {
            inner_client: Client::new(),
            server_addr,
            last_send_timstamp_ms: AtomicI64::new(0),
        }
    }
}

impl CefiClient {
    pub async fn cefi_get_account_info_with_retry(&self, account_id: &str) -> CefiAccountInfo {
        loop {
            match self.cefi_get_account_info(account_id).await {
                Ok(res) => {
                    if res.success {
                        if let Some(account) = res.data {
                            return account;
                        }
                    }
                    tracing::warn!(
                        "cefi_get_account_info decode failed with response: {:?}",
                        res
                    );
                }
                Err(err) => {
                    tracing::warn!("cefi_get_account_info req failed with msg err: {}", err);
                }
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    pub async fn cefi_get_account_info(
        &self,
        account_id: &str,
    ) -> anyhow::Result<ResponseData<CefiAccountInfo>> {
        loop {
            let timestamp_ms = self.last_send_timstamp_ms.load(Ordering::Relaxed);
            let now_ms = Utc::now().timestamp_millis();
            if now_ms - timestamp_ms > 200 {
                self.last_send_timstamp_ms.store(now_ms, Ordering::Relaxed);
                break;
            }
        }
        let result = timeout(Duration::from_secs(3), self._get_account_info(account_id)).await;
        match result {
            Ok(res) => {
                let res = res?;
                let response_data: ResponseData<CefiAccountInfo> = serde_json::from_str(&res)?;
                Ok(response_data)
            }
            Err(err) => Err(anyhow::anyhow!(
                "cefi_get_account_info request account_id: {} timeout: {}",
                account_id,
                err
            )),
        }
    }

    async fn _get_account_info(&self, account_id: &str) -> anyhow::Result<String> {
        let uri = format!(
            "{}/v1/public/account?account_id={}",
            self.server_addr, account_id
        );
        tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "cefi_get_account_info account_id: {}", account_id);
        let response = self.inner_client.get(uri).send().await;
        match response {
            Ok(res) => Ok(res.text().await?),
            Err(err) => Err(anyhow::anyhow!(
                "reqwest _get_account_info faield with err: {:?} for account_id: {}",
                err,
                account_id
            )),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ResponseData<T> {
    pub success: bool,
    pub code: Option<i32>,
    pub message: Option<String>,
    pub data: Option<T>,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CefiAccountInfo {
    pub address: String,
    pub broker_id: String,
}
