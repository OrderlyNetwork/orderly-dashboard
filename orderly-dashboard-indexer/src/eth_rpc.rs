use crate::config::COMMON_CONFIGS;
use anyhow::Result;
use ethers::prelude::{Block, BlockId, BlockNumber, Transaction, TransactionReceipt, H256};
use ethers::providers::{Http, Middleware, Provider};
use once_cell::sync::OnceCell;
use std::time::Duration;
use tokio::time::timeout;

pub(crate) static PROVIDER: OnceCell<Provider<Http>> = OnceCell::new();

pub(crate) fn init_provider() -> Result<()> {
    let rpc = unsafe { COMMON_CONFIGS.get_unchecked().l2_config.rpc_url.clone() };
    PROVIDER.set(Provider::<Http>::try_from(rpc)?).ok();
    Ok(())
}

pub async fn get_latest_block_num() -> Result<u64> {
    Ok(get_block_with_id(BlockId::Number(BlockNumber::Latest))
        .await?
        .ok_or_else(|| anyhow::anyhow!("block not found in calling block api"))?
        .number
        .expect("block number should exist")
        .as_u64())
}

pub async fn get_block_with_id(block_id: BlockId) -> Result<Option<Block<H256>>> {
    let provider = unsafe { PROVIDER.get_unchecked() };

    // todo: replace to eth_getBlockByNumber
    let result = timeout(Duration::from_secs(8), provider.get_block(block_id)).await;
    match result {
        Err(_) => {
            return Err(anyhow::anyhow!("request elapsed"));
        }
        Ok(Ok(block)) => {
            return Ok(block);
        }
        Ok(Err(err)) => {
            tracing::warn!(
                target: crate::ORDERLY_DASHBOARD_INDEXER,
                "get_block_num query err: {}",
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
                target: crate::ORDERLY_DASHBOARD_INDEXER,
                "get_block_with_txs query err: {}",
                err
            );
            return Err(anyhow::anyhow!("query err"));
        }
    }
}

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
                target: crate::ORDERLY_DASHBOARD_INDEXER,
                "get_tx_receipt query err: {}",
                err
            );
            return Err(anyhow::anyhow!("get_tx_receipt query err:{}", err));
        }
    }
}
