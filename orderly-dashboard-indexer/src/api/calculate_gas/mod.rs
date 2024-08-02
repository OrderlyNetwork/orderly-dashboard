pub mod pull_gas_consumptions;
use crate::db::{serial_batches::DbSerialBatches, transaction_events::DbTransactionEvent};
use bigdecimal::ToPrimitive;
use ethers::prelude::TransactionReceipt;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct FeeStruct {
    pub l2_gas_used: u128,
    pub l1_gas_used: u128,
    pub l2_fee: u128,
    pub l1_fee: u128,
    pub l1_fee_scalar: String,
    pub l1_gas_price: u128,
}

pub trait GasReceiptCalculator {
    fn cal_gas_used_and_wei_used(&self) -> FeeStruct;
}

impl GasReceiptCalculator for TransactionReceipt {
    fn cal_gas_used_and_wei_used(&self) -> FeeStruct {
        let fields = &self.other;
        let l1fee_s = fields.get_with("l1Fee", |value| serde_json::from_value::<String>(value));
        let mut l1fee: Option<u128> = None;
        if let Some(Ok(l1fee_s)) = l1fee_s {
            let l1fee_s = l1fee_s.strip_prefix("0x").unwrap_or("0");
            l1fee = u128::from_str_radix(&l1fee_s, 16).ok();
        }

        let l2_gas_used = self.gas_used.unwrap_or_default().as_u128();

        let l1_gas_used_s =
            fields.get_with("l1GasUsed", |value| serde_json::from_value::<String>(value));
        let mut l1_gas_used: Option<u128> = None;
        if let Some(Ok(l1_gas_used_s)) = l1_gas_used_s {
            let l1_gas_used_s = l1_gas_used_s.strip_prefix("0x").unwrap_or("0");
            l1_gas_used = u128::from_str_radix(&l1_gas_used_s, 16).ok();
        }

        let l2_fee = l2_gas_used * self.effective_gas_price.unwrap_or_default().as_u128();

        let l1_fee_scalar_s = fields.get_with("l1FeeScalar", |value| {
            serde_json::from_value::<String>(value)
        });
        let mut l1_fee_scalar = "".to_string();
        if let Some(Ok(l1_fee_scalar_s)) = l1_fee_scalar_s {
            l1_fee_scalar = l1_fee_scalar_s;
        }

        let l1_gas_price_s = fields.get_with("l1GasPrice", |value| {
            serde_json::from_value::<String>(value)
        });
        let mut l1_gas_price: Option<u128> = None;
        if let Some(Ok(l1_gas_price_s)) = l1_gas_price_s {
            let l1_gas_price_s = l1_gas_price_s.strip_prefix("0x").unwrap_or("0");
            l1_gas_price = u128::from_str_radix(&l1_gas_price_s, 16).ok();
        }
        FeeStruct {
            l2_gas_used,
            l1_gas_used: l1_gas_used.unwrap_or_default(),
            l2_fee,
            l1_fee: l1fee.unwrap_or_default(),
            l1_fee_scalar,
            l1_gas_price: l1_gas_price.unwrap_or_default(),
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
        let l1_fee = self
            .l1_fee
            .clone()
            .unwrap_or_default()
            .to_u128()
            .unwrap_or_default();
        FeeStruct {
            l2_gas_used,
            l1_gas_used,
            l2_fee,
            l1_fee,
            l1_fee_scalar: self
                .l1_fee_scalar
                .clone()
                .unwrap_or_else(|| "0.0".to_string()),
            l1_gas_price: self
                .l1_gas_price
                .clone()
                .unwrap_or_default()
                .to_u128()
                .unwrap(),
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
        let l1_fee = self
            .l1_fee
            .clone()
            .unwrap_or_default()
            .to_u128()
            .unwrap_or_default();
        FeeStruct {
            l2_gas_used,
            l1_gas_used,
            l2_fee,
            l1_fee,
            l1_fee_scalar: self
                .l1_fee_scalar
                .clone()
                .unwrap_or_else(|| "0.0".to_string()),
            l1_gas_price: self
                .l1_gas_price
                .clone()
                .unwrap_or_default()
                .to_u128()
                .unwrap_or_default(),
        }
    }
}
