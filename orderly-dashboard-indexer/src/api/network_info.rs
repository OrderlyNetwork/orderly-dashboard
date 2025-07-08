use std::collections::HashMap;

use crate::db::settings::get_db_network_info;
use crate::eth_rpc::get_blockheader_by_number;
use crate::formats_external::{BlockTimeResponse, FailureResponse, NetworkInfo};
use crate::formats_external::{IndexerQueryResponse, SuccessResponse};
use anyhow::{Context, Result};

pub async fn get_network_info() -> Result<IndexerQueryResponse<NetworkInfo>> {
    let info = get_db_network_info().await?;
    if let Some(height) = info.finalized_height {
        return Ok(IndexerQueryResponse::Success(SuccessResponse::new(
            NetworkInfo {
                finalized_height: Some(height),
            },
        )));
    }
    Ok(IndexerQueryResponse::Success(SuccessResponse::new(
        NetworkInfo::default(),
    )))
}

pub async fn pull_block_time(
    params: &HashMap<String, String>,
) -> Result<IndexerQueryResponse<BlockTimeResponse>> {
    let block_number = params
        .get("block_number")
        .context("param block_number not found")?
        .parse::<i64>()?;

    match get_blockheader_by_number(block_number as u64).await {
        Ok(header) => Ok(IndexerQueryResponse::Success(SuccessResponse::new(
            BlockTimeResponse {
                block_timestamp: header.timestamp.as_u128() as i64,
            },
        ))),
        Err(err) => Ok(IndexerQueryResponse::Failure(FailureResponse::new(
            -1201,
            format!("get_blockheader_by_number failed with err: {}", err),
        ))),
    }
}
