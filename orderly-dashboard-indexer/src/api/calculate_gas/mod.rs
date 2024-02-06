pub mod pull_gas_consumptions;
use crate::db::{serial_batches::DbSerialBatches, transaction_events::DbTransactionEvent};
use bigdecimal::ToPrimitive;
use ethers::prelude::TransactionReceipt;
use std::str::FromStr;

pub struct FeeStruct {
    pub l2_gas_used: u128,
    pub l1_gas_used: u128,
    pub l2_fee: u128,
    pub l1_fee: u128,
}

pub trait GasReceiptCalculator {
    fn cal_gas_used_and_wei_used(&self) -> FeeStruct;
}

impl GasReceiptCalculator for TransactionReceipt {
    fn cal_gas_used_and_wei_used(&self) -> FeeStruct {
        let l2_gas_used = self.gas_used.unwrap_or_default().as_u128();
        let l1_gas_used = self.l1_gas_used.unwrap_or_default().as_u128();
        let l2_fee = l2_gas_used * self.effective_gas_price.unwrap_or_default().as_u128();
        let l1_fee = (self.l1_gas_price.unwrap_or_default().as_u128() as f64
            * l1_gas_used as f64
            * self.l1_fee_scalar.unwrap_or_default().as_u128() as f64
            / 10_f64.powf(18.0) as f64) as u128;
        FeeStruct {
            l2_gas_used,
            l1_gas_used,
            l2_fee,
            l1_fee,
        }
    }
}

impl GasReceiptCalculator for DbSerialBatches {
    fn cal_gas_used_and_wei_used(&self) -> FeeStruct {
        let l2_gas_used = u128::from_str(&self.gas_used.clone().unwrap_or_default().to_string())
            .unwrap_or_default();
        let l1_gas_used = u128::from_str(&self.l1_gas_used.clone().unwrap_or_default().to_string())
            .unwrap_or_default();
        let l2_fee = l2_gas_used
            * u128::from_str(
                &self
                    .effective_gas_price
                    .clone()
                    .unwrap_or_default()
                    .to_string(),
            )
            .unwrap_or_default();
        let l1_fee = (self
            .l1_gas_price
            .clone()
            .unwrap_or_default()
            .to_f64()
            .unwrap_or_default()
            * l1_gas_used as f64
            * self
                .l1_fee_scalar
                .clone()
                .unwrap_or_default()
                .to_f64()
                .unwrap_or_default()
            / 10_f64.powf(18.0)) as u128;
        FeeStruct {
            l2_gas_used,
            l1_gas_used,
            l2_fee,
            l1_fee,
        }
    }
}

impl GasReceiptCalculator for DbTransactionEvent {
    fn cal_gas_used_and_wei_used(&self) -> FeeStruct {
        let l2_gas_used = u128::from_str(&self.gas_used.clone().unwrap_or_default().to_string())
            .unwrap_or_default();
        let l1_gas_used = u128::from_str(&self.l1_gas_used.clone().unwrap_or_default().to_string())
            .unwrap_or_default();
        let l2_fee = l2_gas_used
            * u128::from_str(
                &self
                    .effective_gas_price
                    .clone()
                    .unwrap_or_default()
                    .to_string(),
            )
            .unwrap_or_default();
        let l1_fee = (self
            .l1_gas_price
            .clone()
            .unwrap_or_default()
            .to_f64()
            .unwrap_or_default()
            * l1_gas_used as f64
            * self
                .l1_fee_scalar
                .clone()
                .unwrap_or_default()
                .to_f64()
                .unwrap_or_default()
            / 10_f64.powf(18.0)) as u128;
        FeeStruct {
            l2_gas_used,
            l1_gas_used,
            l2_fee,
            l1_fee,
        }
    }
}
