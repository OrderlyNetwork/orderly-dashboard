mod tx_event_handler;

use crate::config::COMMON_CONFIGS;
use crate::contract::tx_event_handler::consume_logs_from_tx_receipts;
use crate::eth_rpc::{get_block_logs, get_block_receipts, get_block_with_txs};
use ethers::prelude::{Address, Block, Transaction, TransactionReceipt, H160, H256};
use ethers::types::Log;
use once_cell::sync::OnceCell;
use std::collections::BTreeMap;
use std::str::FromStr;
use tx_event_handler::consume_tx_and_logs;

use crate::consume_data_task::ORDERLY_DASHBOARD_INDEXER;

pub(crate) const HANDLE_LOG: &str = "handle_log";
pub(crate) const OPERATOR_MANAGER_SC: &str = "operator_manager_sc";
pub(crate) const LEDGER_SC: &str = "ledger_sc";
pub(crate) const VAULT_SC: &str = "vault_sc";

pub(crate) static ADDR_MAP: OnceCell<BTreeMap<Address, &str>> = OnceCell::new();
pub(crate) static TARGET_ADDRS: OnceCell<Vec<Address>> = OnceCell::new();

pub(crate) fn init_addr_set() -> anyhow::Result<()> {
    let mut addr_map = BTreeMap::new();
    let mut addrs = Vec::with_capacity(3);
    let config = &unsafe { COMMON_CONFIGS.get_unchecked() }.l2_config;
    let operator_manager_address = H160::from_str(&config.operator_manager_address)?;
    addr_map.insert(operator_manager_address, OPERATOR_MANAGER_SC);
    addrs.push(operator_manager_address);

    let ledger_address = H160::from_str(&config.ledger_address)?;
    addr_map.insert(ledger_address, LEDGER_SC);
    addrs.push(ledger_address);

    let vault_manager_address = H160::from_str(&config.vault_manager_address)?;
    addr_map.insert(vault_manager_address, VAULT_SC);
    addrs.push(vault_manager_address);

    if ADDR_MAP.set(addr_map).is_err() {
        tracing::warn!(target: ORDERLY_DASHBOARD_INDEXER, "ADDR_SET already inited");
    }
    if TARGET_ADDRS.set(addrs).is_err() {
        tracing::warn!(target: ORDERLY_DASHBOARD_INDEXER, "TARGET_ADDRS already inited");
    }

    Ok(())
}
pub(crate) async fn consume_data_on_block(block_height: u64) -> anyhow::Result<()> {
    tracing::info!(
        target: HANDLE_LOG,
        "consume_data_on_block block_height: {}",
        block_height
    );
    // todo: configurate it
    let consume_logs = true;
    if consume_logs {
        let (block, tx_logs_vec) = query_and_filter_block_data_logs(block_height).await?;
        consume_tx_and_logs(block, &tx_logs_vec).await?;
    } else {
        let (block, tx_receipt_vec) = query_and_filter_block_data_info(block_height).await?;
        consume_logs_from_tx_receipts(block, &tx_receipt_vec).await?;
    }

    Ok(())
}

pub async fn query_and_filter_block_data_info(
    block_height: u64,
) -> anyhow::Result<(Block<Transaction>, Vec<(Transaction, TransactionReceipt)>)> {
    let block = get_block_with_txs(block_height).await?;
    let target_txs = block
        .transactions
        .iter()
        .map(|tx| {
            // if let Some(to) = &tx.to {
            //     if receivers.contains(to) {
            //         return Some(tx.clone());
            //     }
            // }
            // None
            tx.clone()
        })
        .collect::<Vec<_>>();
    if target_txs.is_empty() {
        return Ok((block, vec![]));
    }
    let mut receipt_map = get_block_receipts(block_height)
        .await?
        .into_iter()
        .map(|receipt| (receipt.transaction_hash, receipt))
        .collect::<BTreeMap<_, _>>();
    let len = target_txs.len();
    let mut tx_receipt_vec: Vec<(Transaction, TransactionReceipt)> = Vec::with_capacity(len);
    block.transactions.iter().for_each(|tx| {
        if let Some(receipt) = receipt_map.remove(&tx.hash) {
            tx_receipt_vec.push((tx.clone(), receipt));
        } else {
            tracing::info!(
                target: HANDLE_LOG,
                "can not find receipt for tx: {:?}",
                tx.hash,
            );
        }
    });
    tracing::info!(
        target: HANDLE_LOG,
        "block_height: {}, block tx length {}, receipt length: {},block hash: {:?}",
        block_height,
        block.transactions.len(),
        tx_receipt_vec.len(),
        block.hash.unwrap_or_default(),
    );

    Ok((block, tx_receipt_vec))
}

pub async fn query_and_filter_block_data_logs(
    block_height: u64,
) -> anyhow::Result<(Block<Transaction>, Vec<(Transaction, Vec<Log>)>)> {
    let block = get_block_with_txs(block_height).await?;
    let target_txs = block
        .transactions
        .iter()
        .map(|tx| tx.clone())
        .collect::<Vec<_>>();
    if target_txs.is_empty() {
        return Ok((block, vec![]));
    }
    let len = target_txs.len();
    let mut tx_logs_map: BTreeMap<H256, Vec<Log>> = BTreeMap::new();
    for log in get_block_logs(block_height).await? {
        if let Some(logs) = tx_logs_map.get_mut(&log.transaction_hash.unwrap_or_default()) {
            logs.push(log);
        } else {
            let mut logs = Vec::new();
            let hash = log.transaction_hash.unwrap_or_default();
            logs.push(log);
            tx_logs_map.insert(hash, logs);
        }
    }
    let mut tx_log_vec: Vec<(Transaction, Vec<Log>)> = Vec::with_capacity(len);
    block.transactions.iter().for_each(|tx| {
        if let Some(logs) = tx_logs_map.remove(&tx.hash) {
            tx_log_vec.push((tx.clone(), logs));
        } else {
            tx_log_vec.push((tx.clone(), vec![]));
        }
    });
    tracing::info!(
        target: HANDLE_LOG,
        "block_height: {}, block tx length {}, tx_log_vec length: {},block hash: {:?}",
        block_height,
        block.transactions.len(),
        tx_log_vec.len(),
        block.hash.unwrap_or_default(),
    );

    Ok((block, tx_log_vec))
}

#[cfg(test)]
mod tests {
    use ethers::prelude::BlockNumber;
    use ethers::providers::{Http, Middleware, Provider};

    #[ignore]
    #[tokio::test]
    async fn test_fetch_block_receipts() {
        let provider = Provider::<Http>::try_from(
            "https://l2-orderly-l2-4460-sepolia-8tc3sd7dvy.t.conduit.xyz",
        )
        .unwrap();
        let receipts = provider
            .get_block_receipts(BlockNumber::Number(4777281.into()))
            .await
            .unwrap();
        println!("receipts: {:?}", receipts);
    }
}
