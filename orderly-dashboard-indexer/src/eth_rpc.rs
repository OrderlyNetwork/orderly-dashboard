use crate::config::COMMON_CONFIGS;
use anyhow::Result;
use ethers::prelude::{Block, BlockId, BlockNumber, Transaction, TransactionReceipt, H256};
use ethers::providers::{Http, Middleware, Provider};
use once_cell::sync::OnceCell;
use std::time::Duration;
use tokio::time::timeout;

use crate::consume_data_task::ORDERLY_DASHBOARD_INDEXER;
pub(crate) static PROVIDER: OnceCell<Provider<Http>> = OnceCell::new();

pub(crate) fn init_provider() -> Result<()> {
    let rpc = if let Ok(orderly_rpc) = std::env::var("ORDERLY_RPC") {
        orderly_rpc
    } else {
        unsafe { &COMMON_CONFIGS.get_unchecked().l2_config.rpc_url }.clone()
    };
    tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "rpc wrapped: {}", rpc[0..8].to_string() + "***" + &rpc[rpc.len() - 5..]);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let http_client = Http::new_with_client(url::Url::parse(&rpc)?, client);
    let provider = Provider::new(http_client);
    PROVIDER.set(provider).ok();
    Ok(())
}

pub(crate) fn clone_provider() -> Provider<Http> {
    unsafe { PROVIDER.get_unchecked() }.clone()
}

pub async fn get_latest_block_num() -> Result<u64> {
    Ok(get_blocknumber_with_timeout().await?)
}

pub async fn get_blocknumber_with_timeout() -> Result<u64> {
    let provider = unsafe { PROVIDER.get_unchecked() };

    let result = timeout(Duration::from_secs(3), provider.get_block_number()).await;
    match result {
        Err(_) => {
            return Err(anyhow::anyhow!("request elapsed"));
        }
        Ok(Ok(number)) => {
            return Ok(number.as_u64());
        }
        Ok(Err(err)) => {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "get_blocknumber query err: {}",
                err
            );
            return Err(anyhow::anyhow!("query err"));
        }
    }
}

pub async fn get_block_with_txs(block_num: u64) -> Result<Block<Transaction>> {
    let provider = unsafe { PROVIDER.get_unchecked() };

    let result = timeout(
        Duration::from_secs(8),
        provider.get_block_with_txs(BlockId::Number(BlockNumber::Number(block_num.into()))),
    )
    .await;
    match result {
        Err(_) => {
            return Err(anyhow::anyhow!("get_block_with_txs request elapsed for 8s"));
        }
        Ok(Ok(block)) => {
            return Ok(
                block.ok_or_else(|| anyhow::anyhow!("block not found in calling block api"))?
            );
        }
        Ok(Err(err)) => {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "get_block_with_txs query err: {}",
                err
            );
            return Err(anyhow::anyhow!("query err"));
        }
    }
}

#[allow(dead_code)]
pub async fn get_tx_receipt(tx_hash: H256) -> Result<TransactionReceipt> {
    let provider = unsafe { PROVIDER.get_unchecked() };

    let result = timeout(
        Duration::from_secs(3),
        provider.get_transaction_receipt(tx_hash),
    )
    .await;
    match result {
        Err(_) => {
            return Err(anyhow::anyhow!("get_tx_receipt request elapsed"));
        }
        Ok(Ok(receipt)) => {
            return Ok(
                receipt.ok_or_else(|| anyhow::anyhow!("receipt not found in calling block api"))?
            );
        }
        Ok(Err(err)) => {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "get_tx_receipt query err: {}",
                err
            );
            return Err(anyhow::anyhow!("get_tx_receipt query err:{}", err));
        }
    }
}

pub async fn get_block_receipts(block_num: u64) -> Result<Vec<TransactionReceipt>> {
    let provider = unsafe { PROVIDER.get_unchecked() };

    let result = timeout(
        Duration::from_secs(8),
        provider.get_block_receipts(BlockNumber::Number(block_num.into())),
    )
    .await;
    match result {
        Err(_) => {
            return Err(anyhow::anyhow!("get_block_receipts request elapsed"));
        }
        Ok(Ok(receipts)) => {
            return Ok(receipts);
        }
        Ok(Err(err)) => {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "get_block_receipts query err: {}",
                err
            );
            return Err(anyhow::anyhow!("get_block_receipts query err:{}", err));
        }
    }
}
