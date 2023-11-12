use crate::client::HttpClient;
use crate::contract::{ADDR_SET, HANDLE_LOG};
use anyhow::Result;
use ethers::prelude::{Block, Log, Transaction, TransactionReceipt};

pub(crate) async fn consume_logs_from_tx_receipts(
    block: Block<Transaction>,
    tx_receipts: &Vec<(Transaction, TransactionReceipt)>,
    http_client: HttpClient,
) -> Result<()> {
    for tx_receipt in tx_receipts.iter() {
        let (tx, receipt) = &tx_receipt;
        // 1 (success)
        if receipt.status.unwrap_or_default().as_u64() == 1 {
            if let Err(err) =
                handle_tx_params(tx, http_client.clone(), Some(block.timestamp.as_u64())).await
            {
            }
            for log in &receipt.logs {
                if let Err(err) = handle_log(
                    log.clone(),
                    http_client.clone(),
                    Some(block.timestamp.as_u64()),
                )
                .await
                {
                    tracing::warn!(target: HANDLE_LOG, "handle_log meet err:{:?}", err);
                }
            }
        }
    }
    Ok(())
}

pub(crate) async fn handle_tx_params(
    tx: &Transaction,
    http_client: HttpClient,
    block_t: Option<u64>,
) -> Result<()> {
    let addr_set = unsafe { ADDR_SET.get_unchecked() };
    if !addr_set.contains(&tx.to.clone().unwrap_or_default()) {
        return Ok(());
    }
    Ok(())
}

pub(crate) async fn handle_log(
    log: Log,
    http_client: HttpClient,
    block_t: Option<u64>,
) -> Result<()> {
    let addr_set = unsafe { ADDR_SET.get_unchecked() };
    if !addr_set.contains(&log.address) {
        return Ok(());
    }

    // provider.get_block_with_txs().await?;
    // let hash = H256::from_str("0x515864ece6ad21577f7b559eb78bb939b4b8d24f57ea01126335a956706f5076")
    //     .unwrap();
    // let tx = provider.get_transaction(hash).await.unwrap();
    // if let Some(tx) = tx {
    //     let operator_call = operator_managerCalls::decode(tx.input).unwrap();
    //     // let futures_upload_data: FuturesTradeUploadData = FuturesTradeUploadData::decode(tx.input).unwrap();
    //     println!("operator_call:{:?}", operator_call);
    // } else {
    //     println!("tx not found");
    // }
    // let receipt = provider.get_transaction_receipt(hash).await.unwrap();
    // if let Some(receipt) = receipt {
    //     for log in receipt.logs {
    //         let event = operator_managerEvents::decode_log(&RawLog::from(log)).unwrap();
    //         println!("operator_managerEvents:{:?}", event);
    //     }
    // }
    Ok(())
}
