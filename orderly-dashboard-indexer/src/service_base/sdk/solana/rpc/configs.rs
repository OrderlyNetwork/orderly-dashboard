use serde::{Deserialize, Serialize};

use crate::service_base::sdk::solana::{
    rpc::CommitmentConfig, transaction_status::UiTransactionEncoding,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionConfig {
    pub encoding: Option<UiTransactionEncoding>,
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
    pub max_supported_transaction_version: Option<u8>,
}

impl RpcTransactionConfig {
    pub fn new_json_confirmed_v0() -> RpcTransactionConfig {
        RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::Json),
            commitment: Some(CommitmentConfig::confirmed()),
            max_supported_transaction_version: Some(0),
        }
    }
}
