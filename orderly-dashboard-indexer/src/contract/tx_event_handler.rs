use crate::bindings::operator_manager;
use crate::bindings::operator_manager::{operator_managerCalls, operator_managerEvents};
use crate::bindings::user_ledger::{self, user_ledgerEvents};
use crate::client::HttpClient;
use crate::contract::{ADDR_MAP, HANDLE_LOG, LEDGER_SC, OPERATOR_MANAGER_SC};
use anyhow::Result;
use ethers::abi::RawLog;
use ethers::core::abi::AbiDecode;
use ethers::prelude::{Block, EthLogDecode, Log, Transaction, TransactionReceipt};

pub(crate) async fn consume_logs_from_tx_receipts(
    block: Block<Transaction>,
    tx_receipts: &Vec<(Transaction, TransactionReceipt)>,
    http_client: HttpClient,
) -> Result<()> {
    for tx_receipt in tx_receipts.iter() {
        let (tx, receipt) = &tx_receipt;
        // 1 (success)
        if receipt.status.unwrap_or_default().as_u64() == 1 {
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
            if let Err(err) =
                handle_tx_params(tx, http_client.clone(), Some(block.timestamp.as_u64())).await
            {
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
    let addr_set = unsafe { ADDR_MAP.get_unchecked() };
    if addr_set.get(&tx.to.clone().unwrap_or_default()).is_none() {
        return Ok(());
    }

    // block_number, transaction_index, log index
    let call_data = operator_manager::operator_managerCalls::decode(&tx.input)?;
    match call_data {
        operator_managerCalls::FuturesTradeUpload(futures_upload) => {
            let batch_id = futures_upload.data.batch_id;
            let trades = futures_upload.data.trades;
        }
        _ => {}
    }
    Ok(())
}

pub(crate) async fn handle_log(
    log: Log,
    http_client: HttpClient,
    block_t: Option<u64>,
) -> Result<()> {
    let addr_map = unsafe { ADDR_MAP.get_unchecked() };
    if let Some(sc_name) = addr_map.get(&log.address) {
        match *sc_name {
            OPERATOR_MANAGER_SC => {
                let event =
                    operator_manager::operator_managerEvents::decode_log(&RawLog::from(log))?;
                match event {
                    operator_managerEvents::EventUpload1Filter(event_upload) => {}
                    operator_managerEvents::EventUpload2Filter(event_upload) => {}
                    operator_managerEvents::FuturesTradeUpload1Filter(event_upload) => {}
                    operator_managerEvents::FuturesTradeUpload2Filter(event_upload) => {}
                    _ => {}
                }
            }
            LEDGER_SC => {
                let event = user_ledgerEvents::decode_log(&RawLog::from(log))?;
                match event {
                    user_ledgerEvents::AccountDeposit1Filter(_) => {}
                    user_ledgerEvents::AccountDeposit2Filter(_) => {}
                    user_ledgerEvents::AccountWithdrawApprove1Filter(_) => {}
                    user_ledgerEvents::AccountWithdrawApprove2Filter(_) => {}
                    user_ledgerEvents::AccountWithdrawFail1Filter(_) => {}
                    user_ledgerEvents::AccountWithdrawFail2Filter(_) => {}
                    user_ledgerEvents::AccountWithdrawFinish1Filter(_) => {}
                    user_ledgerEvents::AccountWithdrawFinish2Filter(_) => {}
                    user_ledgerEvents::AdlResultFilter(_) => {}
                    user_ledgerEvents::LiquidationResultFilter(_) => {}
                    user_ledgerEvents::ProcessValidatedFuturesFilter(_) => {}
                    user_ledgerEvents::SettlementResultFilter(_) => {}
                    _ => {}
                }
            }
            _ => {}
        }
    } else {
        return Ok(());
    }
    Ok(())
}
