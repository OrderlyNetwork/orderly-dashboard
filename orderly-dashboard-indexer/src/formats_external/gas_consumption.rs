use crate::api::calculate_gas::{FeeStruct, GasReceiptCalculator};
use crate::db::serial_batches::{DbSerialBatches, SerialBatchType};
use crate::db::transaction_events::{DbTransactionEvent, DbTransactionSide};
use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct GasConsumptionResponse {
    pub transactions: Vec<TransactionGasCost>,
    pub last_block: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransactionGasCost {
    pub block_number: u64,
    pub transaction_id: String,
    pub block_timestamp: u64,
    pub fee_data: FeeStructExt,
    pub transaction_gas_data: TransactionGasCostData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FeeStructExt {
    pub l2_gas_used: String,
    pub l1_gas_used: String,
    pub l2_fee: String,
    pub l1_fee: String,
}

impl From<FeeStruct> for FeeStructExt {
    fn from(value: FeeStruct) -> Self {
        FeeStructExt {
            l2_gas_used: value.l2_gas_used.to_string(),
            l1_gas_used: value.l1_gas_used.to_string(),
            l2_fee: value.l2_fee.to_string(),
            l1_fee: value.l1_fee.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TransactionGasCostData {
    Deposit { chain_id: String },
    PerpTradesUpload { batch_id: u64 },
    EventUpload { batch_id: u64 },
}

impl Eq for TransactionGasCost {}

impl PartialEq<Self> for TransactionGasCost {
    fn eq(&self, other: &Self) -> bool {
        self.block_number == other.block_number
    }
}

impl PartialOrd<Self> for TransactionGasCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.block_number.cmp(&other.block_number))
    }
}

impl Ord for TransactionGasCost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.block_number.cmp(&other.block_number)
    }
}

impl TransactionGasCost {
    pub fn from_serial_batch(value: DbSerialBatches) -> TransactionGasCost {
        let fee_data = value.cal_gas_used_and_wei_used();
        let gas_cost_tx = if value.event_type == SerialBatchType::PerpTrade.value() {
            TransactionGasCostData::PerpTradesUpload {
                batch_id: value.batch_id as u64,
            }
        } else {
            TransactionGasCostData::EventUpload {
                batch_id: value.batch_id as u64,
            }
        };

        TransactionGasCost {
            block_number: value.block_number as u64,
            transaction_id: value.transaction_id,
            block_timestamp: value.block_time.to_u64().unwrap_or_default(),
            fee_data: fee_data.into(),
            transaction_gas_data: gas_cost_tx,
        }
    }

    pub fn try_from_balance_transaction(value: DbTransactionEvent) -> Option<TransactionGasCost> {
        if value.side != DbTransactionSide::Deposit.value() {
            return None;
        }
        let fee_data = value.cal_gas_used_and_wei_used();
        Some(TransactionGasCost {
            block_number: value.block_number as u64,
            transaction_id: value.transaction_id,
            block_timestamp: value.block_time.to_u64().unwrap_or_default(),
            fee_data: fee_data.into(),
            transaction_gas_data: TransactionGasCostData::Deposit {
                chain_id: value.chain_id.to_string(),
            },
        })
    }
}
