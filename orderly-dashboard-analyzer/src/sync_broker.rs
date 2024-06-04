use crate::analyzer::analyzer_job::HTTPException::Timeout;
use crate::db::broker_info::create_or_update_broker_info;
use crate::{analyzer::analyzer_job::HTTPException, db::broker_info::BrokerInfo};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;

use tiny_keccak::{Hasher, Keccak};
pub fn start_sync_brokers(url: String) {
    tokio::spawn(async move {
            let brokers = list_brokers(url).await;
            let mut broker_vec: Vec<BrokerInfo> = vec![];
            for broker in brokers {
                let mut hasher = Keccak::v256();
                hasher.update(broker.broker_id.clone().as_bytes());
                let mut output = [0u8; 32];
                hasher.finalize(&mut output);
                let hex_output: String = output.iter().map(|byte| format!("{:02x}", byte)).collect();
                broker_vec.push(BrokerInfo {
                    broker_id: broker.broker_id.clone(),
                    broker_hash: format!("0x{}",hex_output),
                });
            }
            create_or_update_broker_info(broker_vec).await;
            time::sleep(Duration::from_secs(60)).await;
        
    });
}
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct Broker {
    broker_id: String,
    broker_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct ResponseData {
    success: bool,
    data: Brokers,
    timestamp: i128,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct Brokers {
    rows: Vec<Broker>,
}

async fn list_brokers(url: String) -> Vec<Broker> {
    let res = get_brokers(url).await;
    match res {
        Ok(res_str) => {
            let response_data: ResponseData = serde_json::from_str(&res_str).unwrap();
            if response_data.success {
                let brokers: Brokers = response_data.data;
                return brokers.rows;
            }
            return vec![];
        }
        Err(_) => {
            return vec![];
        }
    }
}

async fn get_brokers(url: String) -> Result<String, HTTPException> {
    let response = reqwest::get(url).await;
    match response {
        Ok(res) => Ok(res.text().await.unwrap()),
        Err(_) => Err(Timeout),
    }
}
