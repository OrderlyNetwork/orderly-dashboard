mod bindings;
use anyhow::Result;
use bindings::operator_manager::{operator_managerCalls, operator_managerEvents};
use ethers::core::abi::{AbiDecode, RawLog};
use ethers::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::H256;
use std::convert::TryFrom;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Http>::try_from("https://l2-orderly-l2-4460-sepolia-8tc3sd7dvy.t.conduit.xyz")
            .expect("could not instantiate HTTP Provider");
    let hash = H256::from_str("0x515864ece6ad21577f7b559eb78bb939b4b8d24f57ea01126335a956706f5076")
        .unwrap();
    let tx = provider.get_transaction(hash).await.unwrap();
    if let Some(tx) = tx {
        let operator_call = operator_managerCalls::decode(tx.input).unwrap();
        // let futures_upload_data: FuturesTradeUploadData = FuturesTradeUploadData::decode(tx.input).unwrap();
        println!("operator_call:{:?}", operator_call);
    } else {
        println!("tx not found");
    }
    let receipt = provider.get_transaction_receipt(hash).await.unwrap();
    if let Some(receipt) = receipt {
        for log in receipt.logs {
            let event = operator_managerEvents::decode_log(&RawLog::from(log)).unwrap();
            println!("operator_managerEvents:{:?}", event);
        }
    }
    Ok(())
}
