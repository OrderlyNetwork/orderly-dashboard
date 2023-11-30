use crate::bindings::operator_manager;
use crate::bindings::operator_manager::{operator_managerCalls, operator_managerEvents};
use crate::bindings::user_ledger::{user_ledgerEvents, Liquidation, Settlement};
use crate::contract::{ADDR_MAP, HANDLE_LOG, LEDGER_SC, OPERATOR_MANAGER_SC};
use std::str::FromStr;

use crate::db::executed_trades::{create_executed_trades, DbExecutedTrades, TradeType};

use crate::db::liquidation_transfer::{create_liquidation_transfers, DbLiquidationTransfer};

use crate::db::serial_batches::{create_serial_batches, DbSerialBatches, SerialBatchType};
use crate::db::{
    adl_result::{create_adl_results, DbAdlResult},
    liquidation_result::{create_liquidation_results, DbLiquidationResult},
    settlement_execution::{create_settlement_executions, DbSettlementExecution},
    settlement_result::{create_settlement_results, DbSettlementResult},
    transaction_events::{
        create_balance_transaction_executions, DbTransactionEvent, DbTransactionSide,
        DbTransactionStatus,
    },
};
use crate::utils::{convert_amount, format_hash, format_hash_160, to_hex_format};
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use ethers::abi::RawLog;
use ethers::core::abi::AbiDecode;
use ethers::prelude::{Block, EthLogDecode, Log, Transaction, TransactionReceipt};

pub(crate) async fn consume_logs_from_tx_receipts(
    block: Block<Transaction>,
    tx_receipts: &Vec<(Transaction, TransactionReceipt)>,
) -> Result<()> {
    for tx_receipt in tx_receipts.iter() {
        let (tx, receipt) = &tx_receipt;
        // 1 (success)
        if receipt.status.unwrap_or_default().as_u64() == 1 {
            for log in &receipt.logs {
                if let Err(err) = handle_log(log.clone(), Some(block.timestamp.as_u64())).await {
                    tracing::warn!(target: HANDLE_LOG, "handle_log meet err:{:?}", err);
                }
            }
            if let Err(err) = handle_tx_params(tx, Some(block.timestamp.as_u64())).await {
                tracing::warn!(target: HANDLE_LOG, "handle_tx_params meet err:{:?}", err);
            }
        }
    }
    Ok(())
}

pub(crate) async fn handle_tx_params(
    tx: &Transaction,
    #[allow(unused_variables)] block_t: Option<u64>,
) -> Result<()> {
    let addr_set = unsafe { ADDR_MAP.get_unchecked() };
    if addr_set.get(&tx.to.clone().unwrap_or_default()).is_none() {
        return Ok(());
    }

    // block_number, transaction_index, log index
    let call_data = operator_manager::operator_managerCalls::decode(&tx.input)?;
    let offset = 1000_000;
    match call_data {
        operator_managerCalls::FuturesTradeUpload(futures_upload) => {
            let _batch_id = futures_upload.data.batch_id;
            let trades = futures_upload.data.trades;
            let db_trades = trades
                .iter()
                .enumerate()
                .map(|(index, trade)| DbExecutedTrades {
                    block_number: tx.block_number.unwrap_or_default().as_u64() as i64,
                    transaction_index: tx.transaction_index.unwrap_or_default().as_u64() as i32,
                    log_index: offset + (index as i32),
                    typ: TradeType::PerpTrade.value(),
                    account_id: to_hex_format(&trade.account_id),
                    symbol_hash: to_hex_format(&trade.symbol_hash),
                    fee_asset_hash: to_hex_format(&trade.fee_asset_hash),
                    trade_qty: convert_amount(trade.trade_qty).unwrap_or_default(),
                    notional: convert_amount(trade.notional).unwrap_or_default(),
                    executed_price: convert_amount(trade.executed_price as i128)
                        .unwrap_or_default(),
                    fee: convert_amount(trade.fee as i128).unwrap_or_default(),
                    sum_unitary_fundings: convert_amount(trade.sum_unitary_fundings)
                        .unwrap_or_default(),
                    trade_id: BigDecimal::from_u64(trade.trade_id).unwrap_or_default(),
                    match_id: BigDecimal::from_u64(trade.match_id).unwrap_or_default(),
                    timestamp: BigDecimal::from_u64(trade.timestamp).unwrap_or_default(),
                    side: trade.side,
                })
                .collect::<Vec<_>>();
            create_executed_trades(db_trades).await?;
        }
        operator_managerCalls::EventUpload(event_upload) => {
            let mut settlement_execs: Vec<DbSettlementExecution> =
                Vec::with_capacity(event_upload.data.events.len());
            let mut liquidation_transfers: Vec<DbLiquidationTransfer> =
                Vec::with_capacity(event_upload.data.events.len());
            for event in event_upload.data.events {
                if event.biz_type == 2 {
                    // settlement
                    if let Ok(settlement) = Settlement::decode(event.data) {
                        for (index, settlement_exec) in
                            settlement.settlement_executions.iter().enumerate()
                        {
                            settlement_execs.push(DbSettlementExecution {
                                block_number: tx.block_number.unwrap_or_default().as_u64() as i64,
                                transaction_index: tx.transaction_index.unwrap_or_default().as_u64()
                                    as i32,
                                log_index: index as i32,
                                transaction_id: format_hash(tx.hash),
                                symbol_hash: to_hex_format(&settlement_exec.symbol_hash),
                                sum_unitary_fundings: convert_amount(
                                    settlement_exec.sum_unitary_fundings,
                                )
                                .unwrap_or_default(),
                                mark_price: convert_amount(settlement_exec.mark_price as i128)?,
                                settled_amount: convert_amount(settlement_exec.settled_amount)?,
                            });
                        }
                    } else {
                    }
                } else if event.biz_type == 4 {
                    // liquidation
                    let liquidation = Liquidation::decode(event.data)?;
                    for (index, liquidation_transfer) in
                        liquidation.liquidation_transfers.iter().enumerate()
                    {
                        liquidation_transfers.push(DbLiquidationTransfer {
                            block_number: tx.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: tx.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: index as i32,
                            transaction_id: format_hash(tx.hash),
                            liquidation_transfer_id: BigDecimal::from_u64(
                                liquidation_transfer.liquidation_transfer_id,
                            )
                            .unwrap_or_default(),
                            liquidator_account_id: to_hex_format(
                                &liquidation_transfer.liquidator_account_id,
                            ),
                            symbol_hash: to_hex_format(&liquidation_transfer.symbol_hash),
                            position_qty_transfer: convert_amount(
                                liquidation_transfer.position_qty_transfer,
                            )?,
                            cost_position_transfer: convert_amount(
                                liquidation_transfer.cost_position_transfer,
                            )?,
                            liquidator_fee: convert_amount(liquidation_transfer.liquidator_fee)?,
                            insurance_fee: convert_amount(liquidation_transfer.insurance_fee)?,
                            mark_price: convert_amount(liquidation_transfer.mark_price as i128)?,
                            sum_unitary_fundings: convert_amount(
                                liquidation_transfer.sum_unitary_fundings,
                            )?,
                            liquidation_fee: convert_amount(liquidation_transfer.liquidation_fee)?,
                        });
                    }
                }
            }
            if !settlement_execs.is_empty() {
                create_settlement_executions(settlement_execs).await?;
            }
            if !liquidation_transfers.is_empty() {
                create_liquidation_transfers(liquidation_transfers).await?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub(crate) async fn handle_log(log: Log, block_t: Option<u64>) -> Result<()> {
    let addr_map = unsafe { ADDR_MAP.get_unchecked() };
    if let Some(sc_name) = addr_map.get(&log.address) {
        match *sc_name {
            OPERATOR_MANAGER_SC => {
                let event = operator_manager::operator_managerEvents::decode_log(&RawLog::from(
                    log.clone(),
                ))?;
                match event {
                    operator_managerEvents::EventUpload1Filter(event_upload) => {
                        create_serial_batches(vec![DbSerialBatches {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            batch_id: event_upload.batch_id as i64,
                            event_type: SerialBatchType::EventUpload.value(),
                        }])
                        .await?;
                    }
                    operator_managerEvents::EventUpload2Filter(event_upload) => {
                        create_serial_batches(vec![DbSerialBatches {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            batch_id: event_upload.batch_id as i64,
                            event_type: SerialBatchType::EventUpload.value(),
                        }])
                        .await?;
                    }
                    operator_managerEvents::FuturesTradeUpload1Filter(futures_upload) => {
                        create_serial_batches(vec![DbSerialBatches {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            batch_id: futures_upload.batch_id as i64,
                            event_type: SerialBatchType::PerpTrade.value(),
                        }])
                        .await?;
                    }
                    operator_managerEvents::FuturesTradeUpload2Filter(futures_upload) => {
                        create_serial_batches(vec![DbSerialBatches {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default()).into(),
                            batch_id: futures_upload.batch_id as i64,
                            event_type: SerialBatchType::PerpTrade.value(),
                        }])
                        .await?;
                    }
                    _ => {}
                }
            }
            LEDGER_SC => {
                let event = user_ledgerEvents::decode_log(&RawLog::from(log.clone()))?;
                match event {
                    user_ledgerEvents::AccountDeposit1Filter(deposit_event) => {
                        create_balance_transaction_executions(vec![DbTransactionEvent {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            account_id: to_hex_format(&deposit_event.account_id),
                            sender: None,
                            receiver: format_hash_160(deposit_event.user_address),
                            token_hash: to_hex_format(&deposit_event.token_hash),
                            broker_hash: to_hex_format(&deposit_event.broker_hash),
                            chain_id: convert_amount(deposit_event.src_chain_id.as_u128() as i128)?,
                            side: DbTransactionSide::Deposit.value(),
                            amount: convert_amount(deposit_event.token_amount as i128)?,
                            fee: BigDecimal::from_str("0")?,
                            status: DbTransactionStatus::Succeed.value(),
                            withdraw_nonce: None,
                            fail_reason: None,
                        }])
                        .await?;
                    }
                    user_ledgerEvents::AccountDeposit2Filter(deposit_event) => {
                        create_balance_transaction_executions(vec![DbTransactionEvent {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            account_id: to_hex_format(&deposit_event.account_id),
                            sender: None,
                            receiver: format_hash_160(deposit_event.user_address),
                            token_hash: to_hex_format(&deposit_event.token_hash),
                            broker_hash: to_hex_format(&deposit_event.broker_hash),
                            chain_id: convert_amount(deposit_event.src_chain_id.as_u128() as i128)?,
                            side: DbTransactionSide::Deposit.value(),
                            amount: convert_amount(deposit_event.token_amount as i128)?,
                            fee: BigDecimal::from_str("0")?,
                            status: DbTransactionStatus::Succeed.value(),
                            withdraw_nonce: None,
                            fail_reason: None,
                        }])
                        .await?;
                    }
                    user_ledgerEvents::AccountWithdrawFinish1Filter(withdraw_event) => {
                        create_balance_transaction_executions(vec![DbTransactionEvent {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            account_id: to_hex_format(&withdraw_event.account_id),
                            sender: Some(format_hash_160(withdraw_event.sender)),
                            receiver: format_hash_160(withdraw_event.receiver),
                            token_hash: to_hex_format(&withdraw_event.token_hash),
                            broker_hash: to_hex_format(&withdraw_event.broker_hash),
                            chain_id: convert_amount(withdraw_event.chain_id.as_u128() as i128)?,
                            side: DbTransactionSide::Withdraw.value(),
                            amount: convert_amount(withdraw_event.token_amount as i128)?,
                            fee: convert_amount(withdraw_event.fee as i128)?,
                            status: DbTransactionStatus::Succeed.value(),
                            withdraw_nonce: Some(withdraw_event.withdraw_nonce as i64),
                            fail_reason: None,
                        }])
                        .await?;
                    }
                    user_ledgerEvents::AccountWithdrawFinish2Filter(withdraw_event) => {
                        create_balance_transaction_executions(vec![DbTransactionEvent {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            account_id: to_hex_format(&withdraw_event.account_id),
                            sender: Some(format_hash_160(withdraw_event.sender)),
                            receiver: format_hash_160(withdraw_event.receiver),
                            token_hash: to_hex_format(&withdraw_event.token_hash),
                            broker_hash: to_hex_format(&withdraw_event.broker_hash),
                            chain_id: convert_amount(withdraw_event.chain_id.as_u128() as i128)?,
                            side: DbTransactionSide::Withdraw.value(),
                            amount: convert_amount(withdraw_event.token_amount as i128)?,
                            fee: convert_amount(withdraw_event.fee as i128)?,
                            status: DbTransactionStatus::Succeed.value(),
                            withdraw_nonce: Some(withdraw_event.withdraw_nonce as i64),
                            fail_reason: None,
                        }])
                        .await?;
                    }
                    user_ledgerEvents::AdlResultFilter(adl_result_event) => {
                        create_adl_results(vec![DbAdlResult {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            account_id: to_hex_format(&adl_result_event.account_id),
                            insurance_account_id: to_hex_format(
                                &adl_result_event.insurance_account_id,
                            ),
                            symbol_hash: to_hex_format(&adl_result_event.symbol_hash),
                            position_qty_transfer: convert_amount(
                                adl_result_event.position_qty_transfer,
                            )?,
                            cost_position_transfer: convert_amount(
                                adl_result_event.cost_position_transfer,
                            )?,
                            adl_price: convert_amount(adl_result_event.adl_price as i128)?,
                            sum_unitary_fundings: convert_amount(
                                adl_result_event.sum_unitary_fundings,
                            )?,
                        }])
                        .await?;
                    }
                    user_ledgerEvents::LiquidationResultFilter(liquidation_result_event) => {
                        create_liquidation_results(vec![DbLiquidationResult {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            liquidated_account_id: to_hex_format(
                                &liquidation_result_event.liquidated_account_id,
                            ),
                            insurance_account_id: to_hex_format(
                                &liquidation_result_event.insurance_account_id,
                            ),
                            liquidated_asset_hash: to_hex_format(
                                &liquidation_result_event.liquidated_account_id,
                            ),
                            insurance_transfer_amount: convert_amount(
                                liquidation_result_event.insurance_transfer_amount as i128,
                            )?,
                        }])
                        .await?;
                    }
                    user_ledgerEvents::ProcessValidatedFuturesFilter(trade) => {
                        let db_trade = DbExecutedTrades {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            typ: TradeType::PerpTrade.value(),
                            account_id: to_hex_format(&trade.account_id),
                            symbol_hash: to_hex_format(&trade.symbol_hash),
                            fee_asset_hash: to_hex_format(&trade.fee_asset_hash),
                            trade_qty: convert_amount(trade.trade_qty).unwrap_or_default(),
                            notional: convert_amount(trade.notional).unwrap_or_default(),
                            executed_price: convert_amount(trade.executed_price as i128)
                                .unwrap_or_default(),
                            fee: convert_amount(trade.fee as i128).unwrap_or_default(),
                            sum_unitary_fundings: convert_amount(trade.sum_unitary_fundings)
                                .unwrap_or_default(),
                            trade_id: BigDecimal::from_u64(trade.trade_id).unwrap_or_default(),
                            match_id: BigDecimal::from_u64(trade.match_id).unwrap_or_default(),
                            timestamp: BigDecimal::from_u64(trade.timestamp).unwrap_or_default(),
                            side: trade.side,
                        };
                        create_executed_trades(vec![db_trade]).await?;
                    }
                    user_ledgerEvents::SettlementResultFilter(settlement_res_event) => {
                        create_settlement_results(vec![DbSettlementResult {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            account_id: to_hex_format(&settlement_res_event.account_id),
                            settled_amount: convert_amount(settlement_res_event.settled_amount)?,
                            settled_asset_hash: to_hex_format(
                                &settlement_res_event.settled_asset_hash,
                            ),
                            insurance_account_id: to_hex_format(
                                &settlement_res_event.insurance_account_id,
                            ),
                            insurance_transfer_amount: convert_amount(
                                settlement_res_event.settled_amount,
                            )?,
                        }])
                        .await?;
                    }
                    user_ledgerEvents::SettlementExecutionFilter(settlement_exec) => {
                        create_settlement_executions(vec![DbSettlementExecution {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.transaction_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            symbol_hash: to_hex_format(&settlement_exec.symbol_hash),
                            sum_unitary_fundings: convert_amount(
                                settlement_exec.sum_unitary_fundings,
                            )
                            .unwrap_or_default(),
                            mark_price: convert_amount(settlement_exec.mark_price as i128)?,
                            settled_amount: convert_amount(settlement_exec.settled_amount)?,
                        }])
                        .await?;
                    }
                    user_ledgerEvents::LiquidationTransferFilter(liquidation_transfer) => {
                        create_liquidation_transfers(vec![DbLiquidationTransfer {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.transaction_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            liquidation_transfer_id: BigDecimal::from_u64(
                                liquidation_transfer.liquidation_transfer_id,
                            )
                            .unwrap_or_default(),
                            liquidator_account_id: to_hex_format(
                                &liquidation_transfer.liquidator_account_id,
                            ),
                            symbol_hash: to_hex_format(&liquidation_transfer.symbol_hash),
                            position_qty_transfer: convert_amount(
                                liquidation_transfer.position_qty_transfer,
                            )?,
                            cost_position_transfer: convert_amount(
                                liquidation_transfer.cost_position_transfer,
                            )?,
                            liquidator_fee: convert_amount(liquidation_transfer.liquidator_fee)?,
                            insurance_fee: convert_amount(liquidation_transfer.insurance_fee)?,
                            mark_price: convert_amount(liquidation_transfer.mark_price as i128)?,
                            sum_unitary_fundings: convert_amount(
                                liquidation_transfer.sum_unitary_fundings,
                            )?,
                            liquidation_fee: convert_amount(liquidation_transfer.liquidation_fee)?,
                        }])
                        .await?;
                    }
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
