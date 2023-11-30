mod tx_event_handler;

use crate::config::COMMON_CONFIGS;
use crate::contract::tx_event_handler::consume_logs_from_tx_receipts;
use crate::eth_rpc::{get_block_receipts, get_block_with_txs};
use ethers::prelude::{Address, Block, Transaction, TransactionReceipt, H160};
use once_cell::sync::OnceCell;
use std::collections::BTreeMap;
use std::str::FromStr;
pub(crate) const HANDLE_LOG: &str = "handle_log";
pub(crate) const OPERATOR_MANAGER_SC: &str = "operator_manager_sc";
pub(crate) const LEDGER_SC: &str = "ledger_sc";
pub(crate) const VAULT_SC: &str = "vault_sc";

pub(crate) static ADDR_MAP: OnceCell<BTreeMap<Address, &str>> = OnceCell::new();

pub(crate) fn init_addr_set() -> anyhow::Result<()> {
    let mut addr_map = BTreeMap::new();
    let config = &unsafe { COMMON_CONFIGS.get_unchecked() }.l2_config;
    addr_map.insert(
        H160::from_str(&config.operator_manager_address)?,
        OPERATOR_MANAGER_SC,
    );
    addr_map.insert(H160::from_str(&config.ledger_address)?, LEDGER_SC);
    addr_map.insert(H160::from_str(&config.vault_manager_address)?, VAULT_SC);
    if ADDR_MAP.set(addr_map).is_err() {
        tracing::warn!(target: crate::ORDERLY_DASHBOARD_INDEXER, "ADDR_SET already inited");
    }
    Ok(())
}
pub(crate) async fn consume_data_on_block(block_height: u64) -> anyhow::Result<()> {
    tracing::info!(
        target: HANDLE_LOG,
        "consume_data_on_block block_height: {}",
        block_height
    );
    let (block, tx_receipt_vec) = query_and_filter_block_data_info(block_height).await?;
    consume_logs_from_tx_receipts(block.clone(), &tx_receipt_vec).await?;

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
