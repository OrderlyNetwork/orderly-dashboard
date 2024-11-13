use std::fmt;

use serde_json::{json, Value};

// https://docs.rs/solana-client/2.0.9/solana_client/rpc_request/enum.RpcRequest.html
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RpcRequest {
    GetBlockHeight,
    GetBlockTime,
    GetSignaturesForAddress,
    GetTransaction,
    GetTokenAccountBalance,
}

impl RpcRequest {
    pub fn build_request_json(self, id: u64, params: Value) -> Value {
        let jsonrpc = "2.0";
        json!({
           "jsonrpc": jsonrpc,
           "id": id,
           "method": format!("{self}"),
           "params": params,
        })
    }
}

impl fmt::Display for RpcRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method = match self {
            RpcRequest::GetBlockHeight => "getBlockHeight",
            RpcRequest::GetBlockTime => "getBlockTime",
            RpcRequest::GetSignaturesForAddress => "getSignaturesForAddress",
            RpcRequest::GetTransaction => "getTransaction",
            RpcRequest::GetTokenAccountBalance => "getTokenAccountBalance",
        };
        write!(f, "{method}")
    }
}
