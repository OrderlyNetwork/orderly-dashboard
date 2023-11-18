mod tx_event_handler;

use crate::client::HttpClient;
use crate::config::COMMON_CONFIGS;
use crate::contract::tx_event_handler::consume_logs_from_tx_receipts;
use crate::eth_rpc::{get_block_with_txs, get_tx_receipt};
use ethers::prelude::{Address, Block, Transaction, TransactionReceipt, H160};
use futures::future::try_join_all;
use once_cell::sync::OnceCell;
use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;
pub(crate) const HANDLE_LOG: &str = "handle_log";
pub(crate) const OPERATOR_MANAGER_SC: &str = "operator_manager_sc";
pub(crate) const LEDGER_SC: &str = "ledger_sc";

pub(crate) static ADDR_MAP: OnceCell<BTreeMap<Address, &str>> = OnceCell::new();

pub(crate) fn init_addr_set() -> anyhow::Result<()> {
    let mut addr_map = BTreeMap::new();
    let config = &unsafe { COMMON_CONFIGS.get_unchecked() }.l2_config;
    addr_map.insert(
        H160::from_str(&config.operator_manager_address)?,
        OPERATOR_MANAGER_SC,
    );
    addr_map.insert(H160::from_str(&config.ledger_address)?, LEDGER_SC);
    if ADDR_MAP.set(addr_map).is_err() {
        tracing::warn!(target: crate::ORDERLY_DASHBOARD_INDEXER, "ADDR_SET already inited");
    }
    Ok(())
}
pub(crate) async fn consume_data_on_block(
    block_height: u64,
    http_client: HttpClient,
) -> anyhow::Result<()> {
    tracing::info!(
        target: HANDLE_LOG,
        "consume_data_on_block block_height: {}",
        block_height
    );
    let (block, tx_receipt_vec) = query_and_filter_block_data_info(block_height).await?;
    consume_logs_from_tx_receipts(block.clone(), &tx_receipt_vec, http_client).await?;

    Ok(())
}

pub async fn query_and_filter_block_data_info(
    block_height: u64,
) -> anyhow::Result<(Block<Transaction>, Vec<(Transaction, TransactionReceipt)>)> {
    let block = get_block_with_txs(block_height).await?;
    tracing::info!(
        target: HANDLE_LOG,
        "block tx length: {:?},block hash: {:?}",
        block.transactions.len(),
        block.hash.unwrap_or_default(),
    );
    // todo: global variable
    // let receivers = unsafe { ADDR_SET.get_unchecked() };
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
    let mut futs = Vec::with_capacity(target_txs.len());
    for tx in &target_txs {
        futs.push(get_tx_receipt(tx.hash));
    }
    let receipts = try_join_all(futs).await?;
    let len = target_txs.len();
    let mut tx_receipt_vec: Vec<(Transaction, TransactionReceipt)> = Vec::with_capacity(len);
    (0..len).for_each(|i| {
        tx_receipt_vec.push((target_txs[i].clone(), receipts[i].clone()));
    });

    Ok((block, tx_receipt_vec))
}
