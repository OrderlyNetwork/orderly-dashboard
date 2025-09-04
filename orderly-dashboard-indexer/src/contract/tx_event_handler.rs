use crate::bindings::operator_manager;
use crate::bindings::operator_manager::{operator_managerCalls, operator_managerEvents};
use crate::bindings::user_ledger::{
    user_ledgerEvents, AccountDepositSolFilter, AccountWithdrawSolApproveFilter,
    LiquidationTransfer, SettlementExecution, SwapResultUploadedFilter,
};
use crate::bindings::vault_manager::{
    self, vault_managerEvents, RebalanceBurnFilter, RebalanceBurnResultFilter, RebalanceMintFilter,
    RebalanceMintResultFilter,
};
use crate::config::{get_common_cfg, COMMON_CONFIGS};
use crate::contract::{ADDR_MAP, HANDLE_LOG, LEDGER_SC, OPERATOR_MANAGER_SC, VAULT_MANAGER_SC};
use crate::db::balance_transfer::{create_balance_transfer_events, DbBalanceTransferEvent};
use crate::db::fee_distribution::{create_fee_distributions, DbFeeDistribution};
use crate::db::rebalance_events::{
    create_rebalance, update_rebalance_burn_status, update_rebalance_mint,
    update_rebalance_mint_status, DBRebalanceEvent,
};
use crate::db::swap_result_uploaded::{
    create_swap_result_uploaded_events, DbSwapResultUploadedEvent,
};
use crate::eth_rpc::PROVIDER;
use std::collections::BTreeSet;
use std::future::Future;
use std::{collections::VecDeque, str::FromStr};

use crate::db::executed_trades::{create_executed_trades, DbExecutedTrades, TradeType};

use crate::db::liquidation_transfer::{
    create_liquidation_transfers, DbLiquidationTransfer, LiquidationTransferVersion,
};

use crate::api::calculate_gas::GasReceiptCalculator;
use crate::db::serial_batches::{create_serial_batches, DbSerialBatches, SerialBatchType};
use crate::db::{
    adl_result::{create_adl_results, AdlVersion, DbAdlResult},
    liquidation_result::{
        create_liquidation_results, DbLiquidationResult, LiquidationResultVersion,
    },
    partitioned_executed_trades::{
        create_partitioned_executed_trades, DbPartitionedExecutedTrades,
    },
    settlement_execution::{create_settlement_executions, DbSettlementExecution},
    settlement_result::{create_settlement_results, DbSettlementResult},
    transaction_events::{
        create_balance_transaction_executions, DbTransactionEvent, DbTransactionSide,
        DbTransactionStatus,
    },
};
use crate::utils::{convert_amount, format_hash, format_hash_160, to_hex_format, u256_to_i128};
use anyhow::Result;
use base58::ToBase58;
use bigdecimal::{BigDecimal, FromPrimitive};
use ethers::abi::{ParamType, RawLog};
use ethers::contract::EthEvent;
use ethers::core::abi::{self, AbiDecode};
use ethers::prelude::{Block, EthLogDecode, Log, Transaction, TransactionReceipt};
use ethers::providers::Middleware;
use ethers::types::{BlockNumber, Filter, H160, U64};

pub(crate) async fn consume_logs_from_tx_receipts(
    block: Block<Transaction>,
    tx_receipts: &Vec<(Transaction, TransactionReceipt)>,
) -> Result<()> {
    for tx_receipt in tx_receipts.iter() {
        let (tx, receipt) = &tx_receipt;
        // cache
        let mut liquidation_result_log_index_queue: VecDeque<i32> = VecDeque::new();
        let mut settlement_result_log_index_queue: VecDeque<i32> = VecDeque::new();
        let mut liquidation_trasfers_cache: Vec<DbLiquidationTransfer> = Vec::new();
        let mut settlement_exectutions_cahce: Vec<DbSettlementExecution> = Vec::new();
        let mut executed_trades_cache: Vec<DbExecutedTrades> = Vec::new();
        let mut executed_partitioned_trades_cache: Vec<DbPartitionedExecutedTrades> = Vec::new();
        // 1 (success)
        if receipt.status.unwrap_or_default().as_u64() == 1 {
            for log in &receipt.logs {
                if let Err(err) = handle_log(
                    &mut liquidation_result_log_index_queue,
                    &mut settlement_result_log_index_queue,
                    &mut liquidation_trasfers_cache,
                    &mut settlement_exectutions_cahce,
                    &mut executed_trades_cache,
                    &mut executed_partitioned_trades_cache,
                    log.clone(),
                    Some(receipt),
                    Some(block.timestamp.as_u64()),
                )
                .await
                {
                    tracing::warn!(target: HANDLE_LOG, "handle_log meet err:{:?}", err);
                }
            }
            if block.number.unwrap_or_default().as_u64()
                < unsafe { COMMON_CONFIGS.get_unchecked().l2_config.upgrade_height }
            {
                if let Err(err) = handle_tx_params(
                    &mut liquidation_result_log_index_queue,
                    &mut settlement_result_log_index_queue,
                    tx,
                    Some(block.timestamp.as_u64()),
                )
                .await
                {
                    tracing::warn!(target: HANDLE_LOG, "handle_tx_params meet err:{:?}, block: {}", err, block.number.unwrap_or_default().as_u64());
                }
            }
        }
        // excute
        // clean cache
        liquidation_result_log_index_queue.clear();

        if !executed_trades_cache.is_empty() {
            create_executed_trades(executed_trades_cache).await?;
        }

        if !executed_partitioned_trades_cache.is_empty() {
            create_partitioned_executed_trades(executed_partitioned_trades_cache).await?;
        }
    }
    Ok(())
}

pub(crate) async fn consume_tx_and_logs(
    block: Block<Transaction>,
    tx_logs: &Vec<(Transaction, Vec<Log>)>,
) -> Result<()> {
    for tx_receipt in tx_logs.iter() {
        let (tx, logs) = &tx_receipt;
        // cache
        let mut liquidation_result_log_index_queue: VecDeque<i32> = VecDeque::new();
        let mut settlement_result_log_index_queue: VecDeque<i32> = VecDeque::new();
        let mut liquidation_trasfers_cache: Vec<DbLiquidationTransfer> = Vec::new();
        let mut settlement_exectutions_cahce: Vec<DbSettlementExecution> = Vec::new();
        let mut executed_trades_cache: Vec<DbExecutedTrades> = Vec::new();
        let mut executed_partitioned_trades_cache: Vec<DbPartitionedExecutedTrades> = Vec::new();
        // 1 (success)

        for log in logs {
            if let Err(err) = handle_log(
                &mut liquidation_result_log_index_queue,
                &mut settlement_result_log_index_queue,
                &mut liquidation_trasfers_cache,
                &mut settlement_exectutions_cahce,
                &mut executed_trades_cache,
                &mut executed_partitioned_trades_cache,
                log.clone(),
                None,
                Some(block.timestamp.as_u64()),
            )
            .await
            {
                tracing::warn!(target: HANDLE_LOG, "handle_log meet err:{:?}", err);
            }
        }
        if block.number.unwrap_or_default().as_u64()
            < unsafe { COMMON_CONFIGS.get_unchecked().l2_config.upgrade_height }
        {
            if let Err(err) = handle_tx_params(
                &mut liquidation_result_log_index_queue,
                &mut settlement_result_log_index_queue,
                tx,
                Some(block.timestamp.as_u64()),
            )
            .await
            {
                tracing::warn!(target: HANDLE_LOG, "handle_tx_params meet err:{:?}, block: {}", err, block.number.unwrap_or_default().as_u64());
            }
        }
        // excute
        // clean cache
        liquidation_result_log_index_queue.clear();

        if !executed_trades_cache.is_empty() {
            create_executed_trades(executed_trades_cache).await?;
        }

        if !executed_partitioned_trades_cache.is_empty() {
            create_partitioned_executed_trades(executed_partitioned_trades_cache).await?;
        }
    }
    Ok(())
}

pub(crate) async fn handle_tx_params(
    liquidation_result_log_index_queue: &mut VecDeque<i32>,
    settlement_result_log_index_queue: &mut VecDeque<i32>,
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
                    block_time: block_t.unwrap_or_default() as i64,
                })
                .collect::<Vec<_>>();
            let partitioned_trades = db_trades
                .iter()
                .map(|item| item.clone().into())
                .collect::<Vec<DbPartitionedExecutedTrades>>();
            create_executed_trades(db_trades).await?;
            create_partitioned_executed_trades(partitioned_trades).await?;
        }
        operator_managerCalls::EventUpload(event_upload) => {
            let mut settlement_execs: Vec<DbSettlementExecution> =
                Vec::with_capacity(event_upload.data.events.len());
            let mut liquidation_trans: Vec<DbLiquidationTransfer> =
                Vec::with_capacity(event_upload.data.events.len());
            for (event_idx, event) in event_upload.data.events.into_iter().enumerate() {
                if event.biz_type == 2 {
                    // settlement
                    let settlement_result_log_idx = settlement_result_log_index_queue
                        .pop_front()
                        .unwrap_or_default();
                    let calldata_types = vec![ParamType::Tuple(vec![
                        ParamType::FixedBytes(32),
                        ParamType::FixedBytes(32),
                        ParamType::FixedBytes(32),
                        ParamType::Int(128),
                        ParamType::Uint(128),
                        ParamType::Uint(64),
                        ParamType::Array(Box::new(ParamType::Tuple(vec![
                            ParamType::FixedBytes(32),
                            ParamType::Uint(128),
                            ParamType::Int(128),
                            ParamType::Int(128),
                        ]))),
                    ])];
                    let decoded = abi::decode(&calldata_types, &event.data)?;
                    let decoded = decoded[0].clone().into_tuple().unwrap();
                    let mut settlement_executions = Vec::new();
                    for e in decoded[6].clone().into_array().unwrap() {
                        let symbol_hash: [u8; 32] = e.clone().into_tuple().unwrap()[0]
                            .clone()
                            .into_fixed_bytes()
                            .unwrap()
                            .try_into()
                            .unwrap();
                        let mark_price = e.clone().into_tuple().unwrap()[1]
                            .clone()
                            .into_uint()
                            .unwrap()
                            .as_u128();
                        let sum_unitary_fundings = u256_to_i128(
                            e.clone().into_tuple().unwrap()[2]
                                .clone()
                                .into_int()
                                .unwrap(),
                        );
                        let settled_amount = u256_to_i128(
                            e.clone().into_tuple().unwrap()[3]
                                .clone()
                                .into_int()
                                .unwrap(),
                        );
                        settlement_executions.push(SettlementExecution {
                            symbol_hash,
                            mark_price,
                            sum_unitary_fundings,
                            settled_amount,
                        });
                    }

                    for (index, settlement_exec) in settlement_executions.iter().enumerate() {
                        settlement_execs.push(DbSettlementExecution {
                            block_number: tx.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: tx.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: (event_idx * 10000 + index) as i32,
                            settlement_result_log_idx,
                            transaction_id: format_hash(tx.hash),
                            symbol_hash: to_hex_format(&settlement_exec.symbol_hash),
                            sum_unitary_fundings: convert_amount(
                                settlement_exec.sum_unitary_fundings,
                            )
                            .unwrap_or_default(),
                            mark_price: convert_amount(settlement_exec.mark_price as i128)?,
                            settled_amount: convert_amount(settlement_exec.settled_amount)?,
                            block_time: Some((block_t.unwrap_or_default() as i64).into()),
                        });
                    }
                } else if event.biz_type == 4 {
                    let liquidation_result_log_idx = liquidation_result_log_index_queue
                        .pop_front()
                        .unwrap_or_default();
                    // liquidation
                    // struct Liquidation {
                    //     bytes32 liquidatedAccountId;
                    //     bytes32 insuranceAccountId;
                    //     bytes32 liquidatedAssetHash;
                    //     uint128 insuranceTransferAmount;
                    //     uint64 timestamp;
                    //     LiquidationTransfer[] liquidationTransfers;
                    // }

                    // struct LiquidationTransfer {
                    //     bytes32 liquidatorAccountId;
                    //     bytes32 symbolHash;
                    //     int128 positionQtyTransfer;
                    //     int128 costPositionTransfer;
                    //     int128 liquidatorFee;
                    //     int128 insuranceFee;
                    //     int128 liquidationFee;
                    //     uint128 markPrice;
                    //     int128 sumUnitaryFundings;
                    //     uint64 liquidationTransferId;
                    // }
                    let calldata_types = vec![ParamType::Tuple(vec![
                        ParamType::FixedBytes(32),
                        ParamType::FixedBytes(32),
                        ParamType::FixedBytes(32),
                        ParamType::Uint(128),
                        ParamType::Uint(64),
                        ParamType::Array(Box::new(ParamType::Tuple(vec![
                            ParamType::FixedBytes(32),
                            ParamType::FixedBytes(32),
                            ParamType::Int(128),
                            ParamType::Int(128),
                            ParamType::Int(128),
                            ParamType::Int(128),
                            ParamType::Int(128),
                            ParamType::Uint(128),
                            ParamType::Int(128),
                            ParamType::Uint(64),
                        ]))),
                    ])];
                    let decoded = abi::decode(&calldata_types, &event.data)?;
                    let decoded = decoded[0].clone().into_tuple().unwrap();
                    let mut liquidation_transfers = Vec::new();
                    // liquidation_transfers
                    for l in decoded[5].clone().into_array().unwrap() {
                        let liquidator_account_id: [u8; 32] = l.clone().into_tuple().unwrap()[0]
                            .clone()
                            .into_fixed_bytes()
                            .unwrap()
                            .try_into()
                            .unwrap();
                        let symbol_hash = l.clone().into_tuple().unwrap()[1]
                            .clone()
                            .into_fixed_bytes()
                            .unwrap()
                            .try_into()
                            .unwrap();
                        let position_qty_transfer = u256_to_i128(
                            l.clone().into_tuple().unwrap()[2]
                                .clone()
                                .into_int()
                                .unwrap(),
                        );
                        let cost_position_transfer = u256_to_i128(
                            l.clone().into_tuple().unwrap()[3]
                                .clone()
                                .into_int()
                                .unwrap(),
                        );
                        let liquidator_fee = u256_to_i128(
                            l.clone().into_tuple().unwrap()[4]
                                .clone()
                                .into_int()
                                .unwrap(),
                        );
                        let insurance_fee = u256_to_i128(
                            l.clone().into_tuple().unwrap()[5]
                                .clone()
                                .into_int()
                                .unwrap(),
                        );
                        let liquidation_fee = u256_to_i128(
                            l.clone().into_tuple().unwrap()[6]
                                .clone()
                                .into_int()
                                .unwrap(),
                        );
                        let mark_price = l.clone().into_tuple().unwrap()[7]
                            .clone()
                            .into_uint()
                            .unwrap()
                            .as_u128();
                        let sum_unitary_fundings = u256_to_i128(
                            l.clone().into_tuple().unwrap()[8]
                                .clone()
                                .into_int()
                                .unwrap(),
                        );
                        let liquidation_transfer_id = l.clone().into_tuple().unwrap()[9]
                            .clone()
                            .into_uint()
                            .unwrap()
                            .as_u64();
                        liquidation_transfers.push(LiquidationTransfer {
                            liquidation_transfer_id,
                            liquidator_account_id,
                            symbol_hash,
                            position_qty_transfer,
                            cost_position_transfer,
                            liquidator_fee,
                            insurance_fee,
                            liquidation_fee,
                            mark_price,
                            sum_unitary_fundings,
                        });
                    }

                    for (index, liquidation_transfer) in liquidation_transfers.iter().enumerate() {
                        liquidation_trans.push(DbLiquidationTransfer {
                            block_number: tx.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: tx.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: (event_idx * 10000 + index) as i32,
                            liquidation_result_log_idx,
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
                            block_time: Some((block_t.unwrap_or_default() as i64).into()),
                            version: Some(LiquidationTransferVersion::V1.value()),
                        });
                    }
                }
            }
            if !settlement_execs.is_empty() {
                tracing::info!(target: HANDLE_LOG, "insert settlement_execs with length: {}", settlement_execs.len());
                create_settlement_executions(settlement_execs).await?;
            }
            if !liquidation_trans.is_empty() {
                tracing::info!(target: HANDLE_LOG, "insert liquidation_transfers with length: {:?}", liquidation_trans.len());
                create_liquidation_transfers(liquidation_trans).await?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub(crate) async fn handle_log(
    liquidation_result_log_index_queue: &mut VecDeque<i32>,
    settlement_result_log_index_queue: &mut VecDeque<i32>,
    liquidation_trasfers: &mut Vec<DbLiquidationTransfer>,
    settlement_exectutions: &mut Vec<DbSettlementExecution>,
    executed_trades_cache: &mut Vec<DbExecutedTrades>,
    executed_partitioned_trades_cache: &mut Vec<DbPartitionedExecutedTrades>,
    log: Log,
    receipt: Option<&TransactionReceipt>,
    block_t: Option<u64>,
) -> Result<()> {
    let addr_map = unsafe { ADDR_MAP.get_unchecked() };
    if let Some(sc_name) = addr_map.get(&log.address) {
        match *sc_name {
            OPERATOR_MANAGER_SC => {
                let event = operator_manager::operator_managerEvents::decode_log(&RawLog::from(
                    log.clone(),
                ))?;
                match event {
                    operator_managerEvents::EventUpload1Filter(event_upload) => {
                        let fee_st = receipt
                            .map(|r| r.cal_gas_used_and_wei_used())
                            .unwrap_or_default();
                        create_serial_batches(vec![DbSerialBatches {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            batch_id: event_upload.batch_id as i64,
                            event_type: SerialBatchType::EventUpload.value(),
                            effective_gas_price: if let Some(receipt) = receipt {
                                receipt.effective_gas_price.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            gas_used: if let Some(receipt) = receipt {
                                receipt.gas_used.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            l1_fee: Some(convert_amount(fee_st.l1_fee as i128).unwrap_or_default()),
                            l1_fee_scalar: Some(fee_st.l1_fee_scalar),
                            l1_gas_price: Some(
                                convert_amount(fee_st.l1_gas_price as i128).unwrap_or_default(),
                            ),
                            l1_gas_used: Some(
                                convert_amount(fee_st.l1_gas_used as i128).unwrap_or_default(),
                            ),
                        }])
                        .await?;
                    }
                    operator_managerEvents::EventUpload2Filter(event_upload) => {
                        let fee_st = receipt
                            .map(|r| r.cal_gas_used_and_wei_used())
                            .unwrap_or_default();
                        create_serial_batches(vec![DbSerialBatches {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            batch_id: event_upload.batch_id as i64,
                            event_type: SerialBatchType::EventUpload.value(),
                            effective_gas_price: if let Some(receipt) = receipt {
                                receipt.effective_gas_price.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            gas_used: if let Some(receipt) = receipt {
                                receipt.gas_used.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            l1_fee: Some(convert_amount(fee_st.l1_fee as i128).unwrap_or_default()),
                            l1_fee_scalar: Some(fee_st.l1_fee_scalar),
                            l1_gas_price: Some(
                                convert_amount(fee_st.l1_gas_price as i128).unwrap_or_default(),
                            ),
                            l1_gas_used: Some(
                                convert_amount(fee_st.l1_gas_used as i128).unwrap_or_default(),
                            ),
                        }])
                        .await?;
                    }
                    operator_managerEvents::FuturesTradeUpload1Filter(futures_upload) => {
                        let fee_st = receipt
                            .map(|r| r.cal_gas_used_and_wei_used())
                            .unwrap_or_default();
                        create_serial_batches(vec![DbSerialBatches {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            batch_id: futures_upload.batch_id as i64,
                            event_type: SerialBatchType::PerpTrade.value(),
                            effective_gas_price: if let Some(receipt) = receipt {
                                receipt.effective_gas_price.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            gas_used: if let Some(receipt) = receipt {
                                receipt.gas_used.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            l1_fee: Some(convert_amount(fee_st.l1_fee as i128).unwrap_or_default()),
                            l1_fee_scalar: Some(fee_st.l1_fee_scalar),
                            l1_gas_price: Some(
                                convert_amount(fee_st.l1_gas_price as i128).unwrap_or_default(),
                            ),
                            l1_gas_used: Some(
                                convert_amount(fee_st.l1_gas_used as i128).unwrap_or_default(),
                            ),
                        }])
                        .await?;
                    }
                    operator_managerEvents::FuturesTradeUpload2Filter(futures_upload) => {
                        let fee_st = receipt
                            .map(|r| r.cal_gas_used_and_wei_used())
                            .unwrap_or_default();
                        create_serial_batches(vec![DbSerialBatches {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default()).into(),
                            batch_id: futures_upload.batch_id as i64,
                            event_type: SerialBatchType::PerpTrade.value(),
                            effective_gas_price: if let Some(receipt) = receipt {
                                receipt.effective_gas_price.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            gas_used: if let Some(receipt) = receipt {
                                receipt.gas_used.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            l1_fee: Some(convert_amount(fee_st.l1_fee as i128).unwrap_or_default()),
                            l1_fee_scalar: Some(fee_st.l1_fee_scalar),
                            l1_gas_price: Some(
                                convert_amount(fee_st.l1_gas_price as i128).unwrap_or_default(),
                            ),
                            l1_gas_used: Some(
                                convert_amount(fee_st.l1_gas_used as i128).unwrap_or_default(),
                            ),
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
                        let fee_st = receipt
                            .map(|r| r.cal_gas_used_and_wei_used())
                            .unwrap_or_default();
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
                            effective_gas_price: if let Some(receipt) = receipt {
                                receipt.effective_gas_price.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            gas_used: if let Some(receipt) = receipt {
                                receipt.gas_used.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            l1_fee: Some(convert_amount(fee_st.l1_fee as i128).unwrap_or_default()),
                            l1_fee_scalar: Some(fee_st.l1_fee_scalar),
                            l1_gas_price: Some(
                                convert_amount(fee_st.l1_gas_price as i128).unwrap_or_default(),
                            ),
                            l1_gas_used: Some(
                                convert_amount(fee_st.l1_gas_used as i128).unwrap_or_default(),
                            ),
                        }])
                        .await?;
                    }
                    user_ledgerEvents::AccountDeposit2Filter(deposit_event) => {
                        let fee_st = receipt
                            .map(|r| r.cal_gas_used_and_wei_used())
                            .unwrap_or_default();
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
                            effective_gas_price: if let Some(receipt) = receipt {
                                receipt.effective_gas_price.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            gas_used: if let Some(receipt) = receipt {
                                receipt.gas_used.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            l1_fee: Some(convert_amount(fee_st.l1_fee as i128).unwrap_or_default()),
                            l1_fee_scalar: Some(fee_st.l1_fee_scalar),
                            l1_gas_price: Some(
                                convert_amount(fee_st.l1_gas_price as i128).unwrap_or_default(),
                            ),
                            l1_gas_used: Some(
                                convert_amount(fee_st.l1_gas_used as i128).unwrap_or_default(),
                            ),
                        }])
                        .await?;
                    }
                    user_ledgerEvents::AccountDepositSolFilter(deposit_event) => {
                        let fee_st = receipt
                            .map(|r| r.cal_gas_used_and_wei_used())
                            .unwrap_or_default();
                        create_balance_transaction_executions(vec![DbTransactionEvent {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            account_id: to_hex_format(&deposit_event.account_id),
                            sender: None,
                            receiver: deposit_event.pubkey.to_base58(),
                            token_hash: to_hex_format(&deposit_event.token_hash),
                            broker_hash: to_hex_format(&deposit_event.broker_hash),
                            chain_id: convert_amount(deposit_event.src_chain_id.as_u128() as i128)?,
                            side: DbTransactionSide::Deposit.value(),
                            amount: convert_amount(deposit_event.token_amount as i128)?,
                            fee: BigDecimal::from_str("0")?,
                            status: DbTransactionStatus::Succeed.value(),
                            withdraw_nonce: None,
                            fail_reason: None,
                            effective_gas_price: if let Some(receipt) = receipt {
                                receipt.effective_gas_price.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            gas_used: if let Some(receipt) = receipt {
                                receipt.gas_used.map(|amount| {
                                    convert_amount(amount.as_u128() as i128).unwrap_or_default()
                                })
                            } else {
                                None
                            },
                            l1_fee: Some(convert_amount(fee_st.l1_fee as i128).unwrap_or_default()),
                            l1_fee_scalar: Some(fee_st.l1_fee_scalar),
                            l1_gas_price: Some(
                                convert_amount(fee_st.l1_gas_price as i128).unwrap_or_default(),
                            ),
                            l1_gas_used: Some(
                                convert_amount(fee_st.l1_gas_used as i128).unwrap_or_default(),
                            ),
                        }])
                        .await?;
                    }
                    user_ledgerEvents::AccountWithdrawSolApproveFilter(withdraw_event) => {
                        create_balance_transaction_executions(vec![DbTransactionEvent {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            account_id: to_hex_format(&withdraw_event.account_id),
                            sender: Some(withdraw_event.sender.to_base58()),
                            receiver: withdraw_event.receiver.to_base58(),
                            token_hash: to_hex_format(&withdraw_event.token_hash),
                            broker_hash: to_hex_format(&withdraw_event.broker_hash),
                            chain_id: convert_amount(withdraw_event.chain_id.as_u128() as i128)?,
                            side: DbTransactionSide::SolWithdrawApprove.value(),
                            amount: convert_amount(withdraw_event.token_amount as i128)?,
                            fee: convert_amount(withdraw_event.fee as i128)?,
                            status: DbTransactionStatus::Succeed.value(),
                            withdraw_nonce: Some(withdraw_event.withdraw_nonce as i64),
                            fail_reason: None,
                            effective_gas_price: None,
                            gas_used: None,
                            l1_fee: None,
                            l1_fee_scalar: None,
                            l1_gas_price: None,
                            l1_gas_used: None,
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
                            effective_gas_price: None,
                            gas_used: None,
                            l1_fee: None,
                            l1_fee_scalar: None,
                            l1_gas_price: None,
                            l1_gas_used: None,
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
                            effective_gas_price: None,
                            gas_used: None,
                            l1_fee: None,
                            l1_fee_scalar: None,
                            l1_gas_price: None,
                            l1_gas_used: None,
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
                            version: Some(AdlVersion::V1.value()),
                        }])
                        .await?;
                    }
                    user_ledgerEvents::AdlResultV2Filter(adl_result_event) => {
                        create_adl_results(vec![DbAdlResult {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            account_id: to_hex_format(&adl_result_event.account_id),
                            // useless field in AdlResultV2
                            insurance_account_id: "".to_string(),
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
                            version: Some(AdlVersion::V2.value()),
                        }])
                        .await?;
                    }
                    user_ledgerEvents::LiquidationResultFilter(liquidation_result_event) => {
                        let log_index = log.log_index.unwrap_or_default().as_u64() as i32;
                        liquidation_result_log_index_queue.push_back(log_index);
                        create_liquidation_results(vec![DbLiquidationResult {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            liquidated_account_id: to_hex_format(
                                &liquidation_result_event.liquidated_account_id,
                            ),
                            insurance_account_id: to_hex_format(
                                &liquidation_result_event.insurance_account_id,
                            ),
                            liquidated_asset_hash: to_hex_format(
                                &liquidation_result_event.liquidated_asset_hash,
                            ),
                            insurance_transfer_amount: convert_amount(
                                liquidation_result_event.insurance_transfer_amount as i128,
                            )?,
                            version: Some(LiquidationResultVersion::V1.value()),
                        }])
                        .await?;
                        if !liquidation_trasfers.is_empty() {
                            liquidation_trasfers
                                .iter_mut()
                                .for_each(|v| v.liquidation_result_log_idx = log_index);
                            create_liquidation_transfers(liquidation_trasfers.clone()).await?;
                            liquidation_trasfers.clear();
                        }
                    }
                    user_ledgerEvents::LiquidationResultV2Filter(liquidation_result_event) => {
                        let log_index = log.log_index.unwrap_or_default().as_u64() as i32;
                        liquidation_result_log_index_queue.push_back(log_index);
                        create_liquidation_results(vec![DbLiquidationResult {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            // reuse for account_id
                            liquidated_account_id: to_hex_format(
                                &liquidation_result_event.account_id,
                            ),
                            // useless in LiquidationResultV2
                            insurance_account_id: "".to_string(),
                            liquidated_asset_hash: to_hex_format(
                                &liquidation_result_event.liquidated_asset_hash,
                            ),
                            insurance_transfer_amount: convert_amount(
                                liquidation_result_event.insurance_transfer_amount as i128,
                            )?,
                            version: Some(LiquidationResultVersion::V2.value()),
                        }])
                        .await?;
                        if !liquidation_trasfers.is_empty() {
                            liquidation_trasfers
                                .iter_mut()
                                .for_each(|v| v.liquidation_result_log_idx = log_index);
                            create_liquidation_transfers(liquidation_trasfers.clone()).await?;
                            liquidation_trasfers.clear();
                        }
                    }
                    user_ledgerEvents::ProcessValidatedFutures1Filter(trade) => {
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
                            fee: convert_amount(trade.fee).unwrap_or_default(),
                            sum_unitary_fundings: convert_amount(trade.sum_unitary_fundings)
                                .unwrap_or_default(),
                            trade_id: BigDecimal::from_u64(trade.trade_id).unwrap_or_default(),
                            match_id: BigDecimal::from_u64(trade.match_id).unwrap_or_default(),
                            timestamp: BigDecimal::from_u64(trade.timestamp).unwrap_or_default(),
                            side: trade.side,
                            block_time: block_t.unwrap_or_default() as i64,
                        };
                        executed_trades_cache.push(db_trade.clone());
                        executed_partitioned_trades_cache.push(db_trade.into());
                    }
                    user_ledgerEvents::ProcessValidatedFutures2Filter(trade) => {
                        // todo: update to write in batches
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
                            block_time: block_t.unwrap_or_default() as i64,
                        };
                        executed_trades_cache.push(db_trade.clone());
                        executed_partitioned_trades_cache.push(db_trade.into());
                        // create_executed_trades(vec![db_trade]).await?;
                    }
                    user_ledgerEvents::SettlementResultFilter(settlement_res_event) => {
                        // https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getlogs
                        // logIndex: QUANTITY - integer of the log index position in the block.
                        let log_index = log.log_index.unwrap_or_default().as_u64() as i32;
                        settlement_result_log_index_queue.push_back(log_index);
                        create_settlement_results(vec![DbSettlementResult {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index,
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
                        // attention please: update settlement_execution's settlement_result_log_idx
                        if !settlement_exectutions.is_empty() {
                            settlement_exectutions
                                .iter_mut()
                                .for_each(|v| v.settlement_result_log_idx = log_index);
                            create_settlement_executions(settlement_exectutions.clone()).await?;
                            settlement_exectutions.clear();
                        }
                    }
                    user_ledgerEvents::SettlementExecutionFilter(settlement_exec) => {
                        settlement_exectutions.push(DbSettlementExecution {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            // here is an error
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            // will be update in handle settlement result flow
                            settlement_result_log_idx: -1,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            symbol_hash: to_hex_format(&settlement_exec.symbol_hash),
                            sum_unitary_fundings: convert_amount(
                                settlement_exec.sum_unitary_fundings,
                            )
                            .unwrap_or_default(),
                            mark_price: convert_amount(settlement_exec.mark_price as i128)?,
                            settled_amount: convert_amount(settlement_exec.settled_amount)?,
                            block_time: Some((block_t.unwrap_or_default() as i64).into()),
                        });
                    }
                    user_ledgerEvents::LiquidationTransferFilter(liquidation_transfer) => {
                        liquidation_trasfers.push(DbLiquidationTransfer {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            // will be update in handle settlement result flow
                            liquidation_result_log_idx: -1,
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
                            block_time: Some((block_t.unwrap_or_default() as i64).into()),
                            version: Some(LiquidationTransferVersion::V1.value()),
                        });
                    }
                    user_ledgerEvents::LiquidationTransferV2Filter(liquidation_transfer) => {
                        liquidation_trasfers.push(DbLiquidationTransfer {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            // will be update in handle settlement result flow
                            liquidation_result_log_idx: -1,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            // removed in LiquidationTransferV2
                            liquidation_transfer_id: BigDecimal::from(0),
                            // reused as accountId in LiquidationTransferV2
                            liquidator_account_id: to_hex_format(&liquidation_transfer.account_id),
                            symbol_hash: to_hex_format(&liquidation_transfer.symbol_hash),
                            position_qty_transfer: convert_amount(
                                liquidation_transfer.position_qty_transfer,
                            )?,
                            cost_position_transfer: convert_amount(
                                liquidation_transfer.cost_position_transfer,
                            )?,
                            liquidator_fee: convert_amount(0)?,
                            insurance_fee: convert_amount(0)?,
                            mark_price: convert_amount(liquidation_transfer.mark_price as i128)?,
                            sum_unitary_fundings: convert_amount(
                                liquidation_transfer.sum_unitary_fundings,
                            )?,
                            liquidation_fee: convert_amount(liquidation_transfer.fee)?,
                            block_time: Some((block_t.unwrap_or_default() as i64).into()),
                            version: Some(LiquidationTransferVersion::V2.value()),
                        });
                    }
                    user_ledgerEvents::FeeDistributionFilter(event) => {
                        create_fee_distributions(vec![DbFeeDistribution {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            event_id: convert_amount(event.event_id as i128)?,
                            from_account_id: to_hex_format(&event.from_account_id),
                            to_account_id: to_hex_format(&event.to_account_id),
                            amount: convert_amount(event.amount as i128)?,
                            token_hash: to_hex_format(&event.token_hash),
                        }])
                        .await?;
                    }
                    user_ledgerEvents::BalanceTransferFilter(event) => {
                        create_balance_transfer_events(vec![DbBalanceTransferEvent {
                            block_number: log.block_number.unwrap_or_default().as_u64() as i64,
                            transaction_index: log.transaction_index.unwrap_or_default().as_u64()
                                as i32,
                            log_index: log.log_index.unwrap_or_default().as_u64() as i32,
                            transaction_id: format_hash(log.transaction_hash.unwrap_or_default()),
                            block_time: (block_t.unwrap_or_default() as i64).into(),
                            from_account_id: to_hex_format(&event.from_account_id),
                            to_account_id: to_hex_format(&event.to_account_id),
                            amount: convert_amount(event.amount as i128)?,
                            token_hash: to_hex_format(&event.token_hash),
                            is_from_account_id: event.is_from_account_id,
                            transfer_type: event.transfer_type as i16,
                            transfer_id: convert_amount(event.amount as i128)?,
                        }])
                        .await?;
                    }
                    user_ledgerEvents::SwapResultUploadedFilter(event) => {
                        create_swap_result_uploaded_events(vec![DbSwapResultUploadedEvent::new(
                            log.block_number.unwrap_or_default().as_u64() as i64,
                            log.transaction_index.unwrap_or_default().as_u64() as i32,
                            log.log_index.unwrap_or_default().as_u64() as i32,
                            format_hash(log.transaction_hash.unwrap_or_default()),
                            (block_t.unwrap_or_default() as i64).into(),
                            to_hex_format(&event.account_id),
                            to_hex_format(&event.buy_token_hash),
                            to_hex_format(&event.sell_token_hash),
                            convert_amount(event.buy_quantity as i128)?,
                            convert_amount(event.sell_quantity as i128)?,
                            convert_amount(event.chain_id.as_u128() as i128)?,
                            event.swap_status as i16,
                        )])
                        .await?;
                    }
                    _ => {}
                }
            }
            VAULT_MANAGER_SC => {
                let event =
                    vault_manager::vault_managerEvents::decode_log(&RawLog::from(log.clone()))?;
                match event {
                    vault_managerEvents::RebalanceBurnFilter(filter) => {
                        create_rebalance(DBRebalanceEvent::new(
                            filter.rebalance_id as i64,
                            to_hex_format(&filter.token_hash),
                            BigDecimal::from_str(&filter.amount.to_string())?,
                            BigDecimal::from_str(&filter.src_chain_id.to_string())?,
                            BigDecimal::from_str(&filter.dst_chain_id.to_string())?,
                            format_hash(log.transaction_hash.unwrap_or_default()),
                        ))
                        .await?;
                    }
                    vault_managerEvents::RebalanceBurnResultFilter(filter) => {
                        update_rebalance_burn_status(
                            filter.rebalance_id as i64,
                            format_hash(log.transaction_hash.unwrap_or_default()),
                            (block_t.unwrap_or_default() as i64).into(),
                            log.block_number.unwrap_or_default().as_u64() as i64,
                            filter.success,
                        )
                        .await?;
                    }
                    vault_managerEvents::RebalanceMintFilter(filter) => {
                        update_rebalance_mint(
                            filter.rebalance_id as i64,
                            format_hash(log.transaction_hash.unwrap_or_default()),
                        )
                        .await?;
                    }
                    vault_managerEvents::RebalanceMintResultFilter(filter) => {
                        update_rebalance_mint_status(
                            filter.rebalance_id as i64,
                            format_hash(log.transaction_hash.unwrap_or_default()),
                            (block_t.unwrap_or_default() as i64).into(),
                            log.block_number.unwrap_or_default().as_u64() as i64,
                            filter.success,
                        )
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

pub async fn simple_recover_deposit_sol_logs(start_block: u64, end_block: u64) -> Result<()> {
    if end_block < start_block {
        tracing::info!("end block less than start block");
        return Ok(());
    }
    let mut current_pull_block = start_block;
    let page_size = 10_000;
    if end_block < current_pull_block {
        tracing::info!("do not has new block, skip pull task");
        return Ok(());
    }

    let provider = unsafe { PROVIDER.get_unchecked() };
    loop {
        let to_block = std::cmp::min(end_block, current_pull_block + page_size);
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(3),
            get_contract_deposit_sol_logs(current_pull_block, to_block),
        )
        .await;
        if result.is_err() {
            tracing::warn!(
                "get_contract_logs, current_pull_block={}, end_block={}, timeout 3s",
                current_pull_block,
                to_block,
            );
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            continue;
        }
        let logs_result = result.unwrap();
        if let Err(err) = &logs_result {
            tracing::info!(
                "get_contract_logs, current_pull_block={}, end_block={}, failed with err:{}",
                current_pull_block,
                to_block,
                err
            );
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            continue;
        }
        let logs = logs_result?;
        let removed_logs = logs
            .iter()
            .filter_map(|log| {
                if log.removed.unwrap_or_default() {
                    Some(log.transaction_hash.unwrap_or_default())
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();
        tracing::info!(
            "from:{},to:{}, logs length:{}, removed_logs len: {}",
            current_pull_block,
            to_block,
            logs.len(),
            removed_logs.len(),
        );

        let mut block_num_mp_time = (0_u64, 0_u64);
        for log in logs {
            if log.removed.unwrap_or_default()
                || removed_logs.contains(&log.transaction_hash.unwrap_or_default())
            {
                tracing::warn!("removed_log info: {:?}", log);
                continue;
            }
            let (mut q1, mut q2) = (VecDeque::new(), VecDeque::new());
            let (mut v1, mut v2, mut v3, mut v4) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
            let block_number = log.block_number.unwrap_or_default().as_u64();
            if block_number != block_num_mp_time.0 {
                block_num_mp_time.0 = block_number;
                loop {
                    let result = tokio::time::timeout(
                        std::time::Duration::from_secs(3),
                        provider.get_block(BlockNumber::Number(U64::from(block_number))),
                    )
                    .await;
                    if result.is_err() {
                        tracing::warn!("get_block, with number {},timeout in 3s", block_number,);
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        continue;
                    }
                    let blk_result = result.unwrap();
                    if let Err(err) = &blk_result {
                        tracing::warn!(
                            "get_block, with number {},failed with err: {}",
                            start_block,
                            err,
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        continue;
                    }
                    let blk = blk_result.unwrap().unwrap();
                    block_num_mp_time.1 = blk.timestamp.as_u64();
                    break;
                }
            }
            if let Err(err) = handle_log(
                &mut q1,
                &mut q2,
                &mut v1,
                &mut v2,
                &mut v3,
                &mut v4,
                log,
                None,
                Some(block_num_mp_time.1),
            )
            .await
            {
                tracing::warn!("handle_log err: {:?}", err);
            }
        }
        if to_block >= end_block {
            break;
        }
        current_pull_block = to_block + 1;
    }

    Ok(())
}

pub async fn get_contract_deposit_sol_logs(start_block: u64, to_block: u64) -> Result<Vec<Log>> {
    let subnet_config = &get_common_cfg().l2_config;
    let topics = vec![AccountDepositSolFilter::signature()];
    let filter = Filter::new()
        .from_block(BlockNumber::Number(U64::from(start_block)))
        .to_block(BlockNumber::Number(U64::from(to_block)))
        .address(vec![H160::from_str(&subnet_config.ledger_address)?])
        .topic0(topics);

    let provider = unsafe { PROVIDER.get_unchecked() };
    let logs = provider.get_logs(&filter).await?;
    Ok(logs)
}

pub async fn simple_recover_sol_deposit_withdraw_approve_and_rebalance_logs(
    start_block: u64,
    end_block: u64,
) -> Result<()> {
    let page_size = 20_000;
    simple_recover_logs(
        start_block,
        end_block,
        page_size,
        get_contract_sol_deposit_withdraw_approve_and_rebalance_logs,
    )
    .await?;
    Ok(())
}

pub async fn simple_recover_swap_result_uploded_logs(
    start_block: u64,
    end_block: u64,
) -> Result<()> {
    let page_size = 1_000;
    simple_recover_logs(
        start_block,
        end_block,
        page_size,
        get_contract_swap_uploaded_logs,
    )
    .await?;
    Ok(())
}

pub async fn get_contract_swap_uploaded_logs(start_block: u64, to_block: u64) -> Result<Vec<Log>> {
    let subnet_config = &get_common_cfg().l2_config;
    let topics = vec![SwapResultUploadedFilter::signature()];
    let filter = Filter::new()
        .from_block(BlockNumber::Number(U64::from(start_block)))
        .to_block(BlockNumber::Number(U64::from(to_block)))
        .address(vec![H160::from_str(&subnet_config.ledger_address)?])
        .topic0(topics);

    let provider = unsafe { PROVIDER.get_unchecked() };
    let logs = provider.get_logs(&filter).await?;
    Ok(logs)
}

pub async fn get_contract_sol_deposit_withdraw_approve_and_rebalance_logs(
    start_block: u64,
    to_block: u64,
) -> Result<Vec<Log>> {
    let subnet_config = &get_common_cfg().l2_config;
    let topics = vec![
        AccountDepositSolFilter::signature(),
        AccountWithdrawSolApproveFilter::signature(),
        RebalanceBurnFilter::signature(),
        RebalanceBurnResultFilter::signature(),
        RebalanceMintFilter::signature(),
        RebalanceMintResultFilter::signature(),
    ];
    let filter = Filter::new()
        .from_block(BlockNumber::Number(U64::from(start_block)))
        .to_block(BlockNumber::Number(U64::from(to_block)))
        .address(vec![
            H160::from_str(&subnet_config.ledger_address)?,
            H160::from_str(&subnet_config.vault_manager_address)?,
        ])
        .topic0(topics);

    let provider = unsafe { PROVIDER.get_unchecked() };
    let logs = provider.get_logs(&filter).await?;
    Ok(logs)
}

pub async fn simple_recover_logs<
    K: Fn(u64, u64) -> FutK + Send + Clone + 'static,
    FutK: Future<Output = anyhow::Result<Vec<Log>>> + Send + 'static,
>(
    start_block: u64,
    end_block: u64,
    page_size: u64,
    get_sc_logs: K,
) -> Result<()> {
    if end_block < start_block {
        tracing::info!("end block less than start block");
        return Ok(());
    }
    let mut current_pull_block = start_block;
    if end_block < current_pull_block {
        tracing::info!("do not has new block, skip pull task");
        return Ok(());
    }

    let provider = unsafe { PROVIDER.get_unchecked() };
    loop {
        let to_block = std::cmp::min(end_block, current_pull_block + page_size);
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(6),
            get_sc_logs(current_pull_block, to_block),
        )
        .await;
        if result.is_err() {
            tracing::warn!(
                "simple_recover_logs get_contract_logs, current_pull_block={}, end_block={}, timeout 6s",
                current_pull_block,
                to_block,
            );
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            continue;
        }
        let logs_result = result.unwrap();
        if let Err(err) = &logs_result {
            tracing::info!(
                "simple_recover_logs get_contract_logs, current_pull_block={}, end_block={}, failed with err:{}",
                current_pull_block,
                to_block,
                err
            );
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            continue;
        }
        let logs = logs_result?;
        let removed_logs = logs
            .iter()
            .filter_map(|log| {
                if log.removed.unwrap_or_default() {
                    Some(log.transaction_hash.unwrap_or_default())
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();
        tracing::info!(
            "from:{},to:{}, logs length:{}, removed_logs len: {}",
            current_pull_block,
            to_block,
            logs.len(),
            removed_logs.len(),
        );

        let mut block_num_mp_time = (0_u64, 0_u64);
        for log in logs {
            if log.removed.unwrap_or_default()
                || removed_logs.contains(&log.transaction_hash.unwrap_or_default())
            {
                tracing::warn!("removed_log info: {:?}", log);
                continue;
            }
            let (mut q1, mut q2) = (VecDeque::new(), VecDeque::new());
            let (mut v1, mut v2, mut v3, mut v4) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
            let block_number = log.block_number.unwrap_or_default().as_u64();
            if block_number != block_num_mp_time.0 {
                block_num_mp_time.0 = block_number;
                loop {
                    let result = tokio::time::timeout(
                        std::time::Duration::from_secs(3),
                        provider.get_block(BlockNumber::Number(U64::from(block_number))),
                    )
                    .await;
                    if result.is_err() {
                        tracing::warn!("get_block, with number {},timeout in 3s", start_block,);
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        continue;
                    }
                    let blk_result = result.unwrap();
                    if let Err(err) = &blk_result {
                        tracing::warn!(
                            "get_block, with number {},failed with err: {}",
                            start_block,
                            err,
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        continue;
                    }
                    let blk = blk_result.unwrap().unwrap();
                    block_num_mp_time.1 = blk.timestamp.as_u64();
                    break;
                }
            }
            if let Err(err) = handle_log(
                &mut q1,
                &mut q2,
                &mut v1,
                &mut v2,
                &mut v3,
                &mut v4,
                log,
                None,
                Some(block_num_mp_time.1),
            )
            .await
            {
                tracing::warn!("handle_log err: {:?}", err);
            }
        }
        if to_block >= end_block {
            break;
        }
        current_pull_block = to_block + 1;
    }

    Ok(())
}
