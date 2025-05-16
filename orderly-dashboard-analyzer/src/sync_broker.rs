use crate::db::broker_info::create_or_update_broker_info;
use crate::db::broker_info::BrokerInfo;
use ethers_core::abi::Address;
use orderly_dashboard_indexer::sdk::solana::pubkey::Pubkey;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;

use tiny_keccak::{Hasher, Keccak};

#[allow(dead_code)]
pub fn start_sync_brokers(url: String) {
    tokio::spawn(async move {
        let brokers = list_brokers(url).await;
        let mut broker_vec: Vec<BrokerInfo> = vec![];
        for broker in brokers {
            broker_vec.push(BrokerInfo {
                broker_id: broker.broker_id.clone(),
                broker_hash: cal_broker_hash(&broker.broker_id),
            });
        }
        tracing::info!("got broker_vec: {:?}", broker_vec);
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

#[allow(dead_code)]
async fn list_brokers(url: String) -> Vec<Broker> {
    let res = get_brokers(url).await;
    match res {
        Ok(res_str) => {
            let response_data: Result<ResponseData, serde_json::Error> =
                serde_json::from_str(&res_str);
            match response_data {
                Ok(response_data) => {
                    if response_data.success {
                        let brokers: Brokers = response_data.data;
                        return brokers.rows;
                    }
                    return vec![];
                }
                Err(err) => {
                    tracing::warn!("parse brokers failed with err: {}", err);
                    return vec![];
                }
            }
        }
        Err(err) => {
            tracing::warn!("get broker failed with err: {}", err);
            return vec![];
        }
    }
}

async fn get_brokers(url: String) -> anyhow::Result<String> {
    let response = reqwest::get(url).await;
    match response {
        Ok(res) => Ok(res.text().await?),
        Err(err) => Err(anyhow::anyhow!(
            "reqwest get_brokers faield with err: {:?}",
            err
        )),
    }
}

pub fn cal_symbol_hash(symbol: &str) -> String {
    cal_string_hash(symbol)
}

pub fn cal_broker_hash(broker_id: &str) -> String {
    cal_string_hash(broker_id)
}

pub fn cal_string_hash(broker_id: &str) -> String {
    let mut hasher = Keccak::v256();
    hasher.update(broker_id.as_bytes());
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    let hex_output: String = output.iter().map(|byte| format!("{:02x}", byte)).collect();
    format!("0x{}", hex_output)
}

pub fn cal_account_id(broker_id: &str, address: &str) -> anyhow::Result<String> {
    use ethers_core::abi::Token;
    use std::str::FromStr;
    let mut hasher = Keccak::v256();
    hasher.update(broker_id.as_bytes());
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    let account_id = ethers_core::abi::encode(&[
        Token::Address(Address::from_str(address)?),
        Token::FixedBytes(output.to_vec()),
    ]);
    let mut hasher = Keccak::v256();
    hasher.update(&account_id);
    hasher.finalize(&mut output);
    Ok(format!("0x{}", hex::encode(output)))
}

pub fn get_sol_account_id(user_account: &Pubkey, broker_id: &str) -> anyhow::Result<String> {
    use ethers_core::abi::Token;
    let decoded_user_account = user_account.to_bytes();
    let mut hasher = Keccak::v256();
    hasher.update(broker_id.as_bytes());
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    let bytes = ethers_core::abi::encode(&[
        Token::FixedBytes(decoded_user_account.to_vec()),
        Token::FixedBytes(output.to_vec()),
    ]);
    let mut h = Keccak::v256();
    h.update(&bytes);
    let mut output = [0u8; 32];
    h.finalize(&mut output);
    Ok(format!("0x{}", hex::encode(output)))
}

#[cfg(test)]
mod tests {
    use super::{cal_account_id, cal_broker_hash, get_sol_account_id};
    use orderly_dashboard_indexer::sdk::solana::pubkey::Pubkey;
    use std::str::FromStr;

    #[test]
    fn test_broker_hash() {
        let hash = cal_broker_hash("ape_terminal");
        println!("broker hash is: {}", hash);
        assert_eq!(
            hash,
            "0x7aab5f0962136383bce11533727a379fb73b71d861f00adb3e3b020a1c7302fe"
        );
    }

    #[test]
    fn test_cal_account_id() {
        let account_id =
            cal_account_id("ape_terminal", "0x8975E0746f0842f015A5D08639E4bb1C6203952c").unwrap();
        println!("account_id is: {}", account_id);
        assert_eq!(
            account_id,
            "0xed612d1901647232514a778494285fef073fd06a79cd12212026f22f2bd886ee"
        );
    }

    #[test]
    fn test_get_sol_cal_account_id() {
        let account_id = get_sol_account_id(
            &Pubkey::from_str("Bm7g7u9bEVynrr69T7nepNAW6dW499eCER87g4ydrHaR").unwrap(),
            "raydium",
        )
        .unwrap();
        println!("account_id is: {}", account_id);
        assert_eq!(
            account_id,
            "0xec32f797363b2b3638089f13f8254aa09d530cb937d917732e17adfa89c67488"
        );
    }
}
