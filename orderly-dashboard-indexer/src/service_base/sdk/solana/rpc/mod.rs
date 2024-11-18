pub mod configs;
pub mod rpc_request;

use std::{
    fmt,
    str::FromStr,
    sync::atomic::{AtomicU64, Ordering},
};

use base58::ToBase58;
use configs::RpcTransactionConfig;
use generic_array::{typenum::U64, GenericArray};
use reqwest::{header::CONTENT_TYPE, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;

use super::{response::RpcResult, transaction_status::UnixTimestamp};
use crate::service_base::sdk::solana::{
    commitment_config::CommitmentConfig,
    parse_token::UiTokenAmount,
    pubkey::Pubkey,
    response::RpcConfirmedTransactionStatusWithSignature,
    rpc::rpc_request::RpcRequest,
    transaction_status::{EncodedConfirmedTransactionWithStatusMeta, Slot, UiTransactionEncoding},
};

#[derive(Debug, Default)]
pub struct GetConfirmedSignaturesForAddress2Config {
    pub before: Option<Signature>,
    pub until: Option<Signature>,
    pub limit: Option<usize>,
    pub commitment: Option<CommitmentConfig>,
}

pub const SIGNATURE_BYTES: usize = 64;
/// Maximum string length of a base58 encoded signature
const MAX_BASE58_SIGNATURE_LEN: usize = 88;
const SOL_SDK: &str = "sol_sdk";

pub struct Signature(GenericArray<u8, U64>);

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseSignatureError {
    #[error("string decoded to wrong size for signature")]
    WrongSize,
    #[error("failed to decode string to signature")]
    Invalid,
}

impl FromStr for Signature {
    type Err = ParseSignatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > MAX_BASE58_SIGNATURE_LEN {
            return Err(ParseSignatureError::WrongSize);
        }
        let bytes = bs58::decode(s)
            .into_vec()
            .map_err(|_| ParseSignatureError::Invalid)?;
        Signature::try_from(bytes).map_err(|_| ParseSignatureError::WrongSize)
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl From<Signature> for [u8; 64] {
    fn from(signature: Signature) -> Self {
        signature.0.into()
    }
}

impl From<[u8; SIGNATURE_BYTES]> for Signature {
    #[inline]
    fn from(signature: [u8; SIGNATURE_BYTES]) -> Self {
        Self(GenericArray::from(signature))
    }
}

impl<'a> TryFrom<&'a [u8]> for Signature {
    type Error = <[u8; SIGNATURE_BYTES] as TryFrom<&'a [u8]>>::Error;

    #[inline]
    fn try_from(signature: &'a [u8]) -> Result<Self, Self::Error> {
        <[u8; SIGNATURE_BYTES]>::try_from(signature).map(Self::from)
    }
}

impl TryFrom<Vec<u8>> for Signature {
    type Error = <[u8; SIGNATURE_BYTES] as TryFrom<Vec<u8>>>::Error;

    #[inline]
    fn try_from(signature: Vec<u8>) -> Result<Self, Self::Error> {
        <[u8; SIGNATURE_BYTES]>::try_from(signature).map(Self::from)
    }
}

#[derive(Error, Debug)]
pub enum ParseCommitmentLevelError {
    #[error("invalid variant")]
    Invalid,
}
pub struct SolRpcClient {
    client: Client,
    url: String,
    request_id: AtomicU64,
    commit_config: CommitmentConfig,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcSignaturesForAddressConfig {
    pub before: Option<String>, // Signature as base-58 string
    pub until: Option<String>,  // Signature as base-58 string
    pub limit: Option<usize>,
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
    pub min_context_slot: Option<Slot>,
}

impl SolRpcClient {
    pub fn new(url: &str, commit_config: CommitmentConfig) -> SolRpcClient {
        SolRpcClient {
            client: Client::new(),
            url: url.to_string(),
            request_id: AtomicU64::new(0),
            commit_config,
        }
    }

    pub fn commitment(&self) -> CommitmentConfig {
        self.commit_config
    }

    async fn send(
        &self,
        request: RpcRequest,
        params: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        let request_id = self.request_id.fetch_add(1, Ordering::Relaxed);
        let request_json = request.build_request_json(request_id, params).to_string();
        let response = {
            let client = self.client.clone();
            let request_json = request_json.clone();
            client
                .post(&self.url)
                .header(CONTENT_TYPE, "application/json")
                .body(request_json)
                .send()
                .await
        }?;
        let mut json = response.json::<serde_json::Value>().await?;
        if json["error"].is_object() {
            return Err(anyhow::anyhow!("error info: {:?}", json["error"]));
        }
        Ok(json["result"].take())
    }

    pub async fn get_signatures_for_address_with_config(
        &self,
        address: &Pubkey,
        config: GetConfirmedSignaturesForAddress2Config,
    ) -> anyhow::Result<Vec<RpcConfirmedTransactionStatusWithSignature>> {
        tracing::info!(target: SOL_SDK, "start to get signatures_for_address_with_config of: {:?}", config);
        let config = RpcSignaturesForAddressConfig {
            before: config.before.map(|signature| signature.to_string()),
            until: config.until.map(|signature| signature.to_string()),
            limit: config.limit,
            commitment: config.commitment,
            min_context_slot: None,
        };

        let result = self
            .send(
                RpcRequest::GetSignaturesForAddress,
                json!([address.to_string(), config]),
            )
            .await?;

        tracing::info!(target: SOL_SDK, "get signatures_for_address_with_config finish with result: {}", result);
        Ok(serde_json::from_value(result)?)
    }

    pub async fn get_transaction(
        &self,
        signature: &Signature,
        encoding: UiTransactionEncoding,
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        tracing::info!(target: SOL_SDK, "start to get transaction of: {}", signature.0.to_base58());
        let result = self
            .send(
                RpcRequest::GetTransaction,
                json!([signature.to_string(), encoding]),
            )
            .await?;
        tracing::info!(target: SOL_SDK, "GetTransaction value={}", result);

        Ok(serde_json::from_value(result)?)
    }

    pub async fn get_transaction_with_config(
        &self,
        signature: &Signature,
        config: RpcTransactionConfig,
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        tracing::info!(target: SOL_SDK, "start to get transaction of: {}", signature.0.to_base58());
        let result = self
            .send(
                RpcRequest::GetTransaction,
                json!([signature.to_string(), config]),
            )
            .await?;
        tracing::info!(target: SOL_SDK, "GetTransaction value={}", result);

        Ok(serde_json::from_value(result)?)
    }

    pub async fn get_block_height(&self) -> anyhow::Result<u64> {
        tracing::trace!(target: SOL_SDK, "start to get block height");
        self.get_block_height_with_commitment(self.commitment())
            .await
    }

    pub async fn get_block_height_with_commitment(
        &self,
        commitment_config: CommitmentConfig,
    ) -> anyhow::Result<u64> {
        let response = self
            .send(RpcRequest::GetBlockHeight, json!([commitment_config]))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    // https://docs.rs/solana-client/2.0.9/solana_client/nonblocking/rpc_client/struct.RpcClient.html#method.get_block_time
    pub async fn get_block_time(&self, slot: Slot) -> anyhow::Result<UnixTimestamp> {
        tracing::trace!(target: SOL_SDK, "start to get block time for slot: {}", slot);
        let request = RpcRequest::GetBlockTime;
        let response = self.send(request, json!([slot])).await;

        response
            .map(|result_json: Value| {
                if result_json.is_null() {
                    return Err(anyhow::anyhow!("Block Not Found: slot={slot}"));
                }
                let result = serde_json::from_value(result_json)
                .map_err(|err| anyhow::anyhow!("Failed to parse block timestamp: {:?}, request: {:?}", err, request))?;
                tracing::trace!(target: SOL_SDK, "Response block timestamp {:?} {:?}", slot, result);
                Ok(result)
            })
            .map_err(|err| anyhow::anyhow!("Failed to get block time: {:?}, request: {:?}", err, request))?
    }

    pub async fn get_token_account_balance(
        &self,
        pubkey: &Pubkey,
    ) -> anyhow::Result<UiTokenAmount> {
        Ok(self
            .get_token_account_balance_with_commitment(pubkey, CommitmentConfig::confirmed())
            .await?
            .value)
    }

    pub async fn get_token_account_balance_with_commitment(
        &self,
        pubkey: &Pubkey,
        commitment_config: CommitmentConfig,
    ) -> RpcResult<UiTokenAmount> {
        let value = self
            .send(
                RpcRequest::GetTokenAccountBalance,
                json!([pubkey.to_string(), commitment_config]),
            )
            .await?;
        Ok(serde_json::from_value(value)?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Signature, SolRpcClient};
    use crate::service_base::sdk::solana::{
        commitment_config::CommitmentConfig, pubkey::Pubkey,
        rpc::GetConfirmedSignaturesForAddress2Config, transaction_status::UiTransactionEncoding,
    };

    #[ignore]
    #[tokio::test]
    async fn test_get_signatures_for_address_with_config() {
        let client = SolRpcClient::new(
            "https://api.devnet.solana.com",
            CommitmentConfig::confirmed(),
        );
        let signature = Signature::from_str("3CJi8sDTr1jBpfYwuufDd7wAqiSgRPrqSaNDDuxiTh53UbyVQ5QRoQUyDb49CQoWbK2DtfafPr9ufm9puHZvR9kW").unwrap();
        let config = GetConfirmedSignaturesForAddress2Config {
            before: Some(signature),
            until: None,
            limit: Some(1000),
            commitment: None,
        };
        let address = Pubkey::from_str("2vauk9Xi84cehajW8HKdWw7nAtcjBTuQCjPPToxb5UwM").unwrap();
        let txs = client
            .get_signatures_for_address_with_config(&address, config)
            .await
            .unwrap();
        println!("txs={:?}", txs);
        assert_eq!(txs.len(), 4);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_transaction() {
        let url = "https://api.devnet.solana.com";
        let signature = Signature::from_str("3njCSYscVEoJiBmm9TtaYoxi29oHFKZa5ggyCek6JvFaRAiEBLMb3F5x8XZnsBB3zGuPirFaZacRue5Mo7rjq37a").unwrap();
        let client = SolRpcClient::new(url, CommitmentConfig::confirmed());
        let tx = client
            .get_transaction(&signature, UiTransactionEncoding::Json)
            .await
            .unwrap();
        println!("tx={:?}", tx);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_err_transaction() {
        let url = "https://api.devnet.solana.com";
        let signature = Signature::from_str("GMwEgrp1ZRSBMka2HZkx5hUcjjKZknBRcu1A7y3LDHCweK7h8X3GpbPAeykN3FXfb3Vpi5SrT866t35z5eSDCG2").unwrap();
        let client = SolRpcClient::new(url, CommitmentConfig::confirmed());
        let tx = client
            .get_transaction(&signature, UiTransactionEncoding::Json)
            .await
            .unwrap();
        println!("err tx={:?}", tx);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_block_height() {
        let url = "https://api.devnet.solana.com";
        let client = SolRpcClient::new(url, CommitmentConfig::confirmed());
        let block_height = client.get_block_height().await.unwrap();
        println!("block_height: {:?}", block_height);
    }
}
