use crate::bindings::operator_manager;
use crate::bindings::operator_manager::{operator_managerCalls, operator_managerEvents};
use crate::bindings::user_ledger::{self, user_ledgerEvents, Liquidation, Settlement};
use crate::client::HttpClient;
use crate::contract::{ADDR_MAP, HANDLE_LOG, LEDGER_SC, OPERATOR_MANAGER_SC};
use crate::db::executed_trades::{create_executed_trades, DbexecutedTrades, TradeType};
use crate::utils::{convert_token, to_hex_format};
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use ethers::abi;
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
            let db_trades = trades
                .iter()
                .map(|trade| DbexecutedTrades {
                    block_number: tx.block_number.unwrap_or_default().as_u64() as i64,
                    transaction_index: tx.transaction_index.unwrap_or_default().as_u64() as i32,
                    log_index: 0,
                    batch_id: BigDecimal::from_u64(batch_id).unwrap_or_default(),
                    typ: TradeType::PerpTrade.value(),
                    account_id: to_hex_format(&trade.account_id),
                    symbol_hash: to_hex_format(&trade.symbol_hash),
                    fee_asset_hash: to_hex_format(&trade.fee_asset_hash),
                    trade_qty: convert_token(trade.trade_qty as u128).unwrap_or_default(),
                    notional: convert_token(trade.notional as u128).unwrap_or_default(),
                    executed_price: convert_token(trade.executed_price).unwrap_or_default(),
                    fee: convert_token(trade.fee).unwrap_or_default(),
                    sum_unitary_fundings: convert_token(trade.sum_unitary_fundings as u128)
                        .unwrap_or_default(),
                    trade_id: BigDecimal::from_u64(trade.trade_id).unwrap_or_default(),
                    match_id: BigDecimal::from_u64(trade.match_id).unwrap_or_default(),
                    timestamp: BigDecimal::from_u64(trade.timestamp).unwrap_or_default(),
                })
                .collect::<Vec<_>>();
            create_executed_trades(db_trades).await?;
        }
        operator_managerCalls::EventUpload(event_upload) => {
            for event in event_upload.data.events {
                if event.biz_type == 2 {
                    // settlement
                    let settlement = Settlement::decode(event.data)?;
                } else if event.biz_type == 4 {
                    // liquidation
                    let liquidation = Liquidation::decode(event.data)?;
                }
            }
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
                    user_ledgerEvents::AccountDeposit1Filter(deposit_event) => {}
                    user_ledgerEvents::AccountDeposit2Filter(_) => {}
                    user_ledgerEvents::AccountWithdrawFinish1Filter(_) => {}
                    user_ledgerEvents::AccountWithdrawFinish2Filter(_) => {}
                    user_ledgerEvents::AdlResultFilter(_) => {}
                    user_ledgerEvents::LiquidationResultFilter(_) => {}
                    user_ledgerEvents::ProcessValidatedFuturesFilter(_) => {}
                    user_ledgerEvents::SettlementResultFilter(_) => {}
                    user_ledgerEvents::SettlementExecutionFilter(_) => {}
                    user_ledgerEvents::LiquidationTransferFilter(_) => {}
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
