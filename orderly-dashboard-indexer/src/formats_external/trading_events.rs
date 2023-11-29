use crate::db::adl_result::DbAdlResult;
use crate::db::executed_trades::DbExecutedTrades;
use crate::db::liquidation_result::DbLiquidationResult;
use crate::db::liquidation_transfer::DbLiquidationTransfer;
use crate::db::serial_batches::DbSerialBatches;
use crate::db::settlement_execution::DbSettlementExecution;
use crate::db::settlement_result::DbSettlementResult;
use crate::db::transaction_events::DbTransactionEvent;
use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct TradingEventsResponse {
    pub events: Vec<TradingEvent>,
    pub last_block: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TradingEvent {
    pub block_number: u64,
    pub transaction_index: u32,
    pub log_index: u32,
    pub transaction_id: String,
    pub block_timestamp: u64,
    pub data: TradingEventInnerData,
}

impl Eq for TradingEvent {}

impl PartialEq<Self> for TradingEvent {
    fn eq(&self, other: &Self) -> bool {
        self.block_number == other.block_number
            && self.transaction_index == other.transaction_index
            && self.log_index == other.log_index
    }
}

impl PartialOrd<Self> for TradingEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let block_order = self.block_number.cmp(&other.block_number);
        if block_order != Ordering::Equal {
            return Some(block_order);
        }
        let tx_order = self.transaction_index.cmp(&other.transaction_index);
        if tx_order != Ordering::Equal {
            return Some(tx_order);
        }
        Some(self.log_index.cmp(&other.log_index))
    }
}

impl Ord for TradingEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        let block_order = self.block_number.cmp(&other.block_number);
        if block_order != Ordering::Equal {
            return block_order;
        }
        let tx_order = self.transaction_index.cmp(&other.transaction_index);
        if tx_order != Ordering::Equal {
            return tx_order;
        }
        self.log_index.cmp(&other.log_index)
    }
}

impl TradingEvent {
    pub fn from_serial_batch_and_trades(
        value: DbSerialBatches,
        trades: Vec<DbExecutedTrades>,
    ) -> TradingEvent {
        let trades: Vec<Trade> = trades.into_iter().map(Into::into).collect::<Vec<_>>();
        TradingEvent {
            block_number: value.block_number as u64,
            transaction_index: value.transaction_index as u32,
            log_index: value.log_index as u32,
            transaction_id: value.transaction_id.clone(),
            block_timestamp: value.block_time.to_u64().unwrap_or_default(),
            data: TradingEventInnerData::ProcessedTrades {
                batch_id: value.batch_id as u64,
                trades,
            },
        }
    }

    pub fn from_balance_transaction(value: DbTransactionEvent) -> TradingEvent {
        TradingEvent {
            block_number: value.block_number as u64,
            transaction_index: value.transaction_index as u32,
            log_index: value.log_index as u32,
            transaction_id: value.transaction_id,
            block_timestamp: value.block_time.to_u64().unwrap_or_default(),
            data: TradingEventInnerData::Transaction {
                account_id: value.account_id,
                sender: value.sender,
                receiver: value.receiver,
                token_hash: value.token_hash,
                broker_hash: value.broker_hash,
                chain_id: value.chain_id.to_string(),
                side: TransactionSide::try_from(value.side).unwrap_or(TransactionSide::Deposit),
                token_amount: value.amount.to_string(),
                withdraw_nonce: value.withdraw_nonce,
                status: TransactionStatus::try_from(value.status)
                    .unwrap_or(TransactionStatus::Succeed),
                fail_reason: value.fail_reason,
                fee: value.fee.to_string(),
            },
        }
    }

    pub fn from_settlement(
        settlement_res: DbSettlementResult,
        executions: Vec<DbSettlementExecution>,
    ) -> TradingEvent {
        let settlement_executions = executions
            .into_iter()
            .map(SettlementExecution::from)
            .collect::<Vec<_>>();
        TradingEvent {
            block_number: settlement_res.block_number as u64,
            transaction_index: settlement_res.transaction_index as u32,
            log_index: settlement_res.log_index as u32,
            transaction_id: settlement_res.transaction_id,
            block_timestamp: settlement_res.block_time.to_u64().unwrap(),
            data: TradingEventInnerData::SettlementResult {
                account_id: settlement_res.account_id,
                settled_amount: settlement_res.settled_amount.to_string(),
                settled_asset_hash: settlement_res.settled_asset_hash,
                insurance_account_id: settlement_res.insurance_account_id,
                insurance_transfer_amount: settlement_res.insurance_transfer_amount.to_string(),
                settlement_executions,
            },
        }
    }

    pub fn from_liquidation(
        liquidation_res: DbLiquidationResult,
        transfers: Vec<DbLiquidationTransfer>,
    ) -> TradingEvent {
        let liquidation_transfers = transfers
            .into_iter()
            .map(LiquidationTransfer::from)
            .collect::<Vec<_>>();
        TradingEvent {
            block_number: liquidation_res.block_number as u64,
            transaction_index: liquidation_res.transaction_index as u32,
            log_index: liquidation_res.log_index as u32,
            transaction_id: liquidation_res.transaction_id,
            block_timestamp: liquidation_res.block_time.to_u64().unwrap(),
            data: TradingEventInnerData::LiquidationResult {
                liquidated_account_id: "".to_string(),
                insurance_account_id: "".to_string(),
                liquidated_asset_hash: "".to_string(),
                insurance_transfer_amount: "".to_string(),
                liquidation_transfers,
            },
        }
    }

    pub fn from_adl_result(adl: DbAdlResult) -> TradingEvent {
        TradingEvent {
            block_number: adl.block_number as u64,
            transaction_index: adl.transaction_index as u32,
            log_index: adl.log_index as u32,
            transaction_id: adl.transaction_id,
            block_timestamp: adl.block_time.to_u64().unwrap(),
            data: TradingEventInnerData::AdlResult {
                account_id: adl.account_id,
                insurance_account_id: adl.insurance_account_id,
                symbol_hash: adl.symbol_hash,
                position_qty_transfer: adl.position_qty_transfer.to_string(),
                cost_position_transfer: adl.cost_position_transfer.to_string(),
                adl_price: adl.adl_price.to_string(),
                sum_unitary_fundings: adl.sum_unitary_fundings.to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TransactionSide {
    Deposit,
    Withdraw,
}

impl TryFrom<i16> for TransactionSide {
    type Error = anyhow::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Deposit),
            2 => Ok(Self::Withdraw),
            _ => Err(anyhow::anyhow!("cannot convert integer to TransactionSide")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Succeed,
    Failed,
}

impl TryFrom<i16> for TransactionStatus {
    type Error = anyhow::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Succeed),
            2 => Ok(Self::Failed),
            _ => Err(anyhow::anyhow!("cannot convert integer to TransactionSide")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum PurchaseSide {
    Buy,
    Sell,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Trade {
    pub account_id: String,
    pub symbol_hash: String,
    pub fee_asset_hash: String,
    pub trade_qty: String,
    pub notional: String,
    pub executed_price: String,
    pub fee: String,
    pub sum_unitary_fundings: String,
    pub trade_id: u64,
    pub match_id: u64,
    pub timestamp: u64,
    // buy (false) or sell (true)
    pub side: PurchaseSide,
}

impl From<DbExecutedTrades> for Trade {
    fn from(value: DbExecutedTrades) -> Self {
        Trade {
            account_id: value.account_id,
            symbol_hash: value.symbol_hash,
            fee_asset_hash: value.fee_asset_hash,
            trade_qty: value.trade_qty.to_string(),
            notional: value.notional.to_string(),
            executed_price: value.executed_price.to_string(),
            fee: value.fee.to_string(),
            sum_unitary_fundings: value.sum_unitary_fundings.to_string(),
            trade_id: value.trade_id.to_u64().unwrap_or_default(),
            match_id: value.match_id.to_u64().unwrap_or_default(),
            timestamp: value.timestamp.to_u64().unwrap_or_default(),
            side: if value.side {
                PurchaseSide::Sell
            } else {
                PurchaseSide::Buy
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SettlementExecution {
    pub symbol_hash: String,
    pub mark_price: String,
    pub sum_unitary_fundings: String,
    pub settled_amount: String,
}

impl From<DbSettlementExecution> for SettlementExecution {
    fn from(value: DbSettlementExecution) -> Self {
        SettlementExecution {
            symbol_hash: value.symbol_hash,
            mark_price: value.mark_price.to_string(),
            sum_unitary_fundings: value.sum_unitary_fundings.to_string(),
            settled_amount: value.settled_amount.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LiquidationTransfer {
    pub liquidation_transfer_id: String,
    pub liquidator_account_id: String,
    pub symbol_hash: String,
    pub position_qty_transfer: String,
    pub cost_position_transfer: String,
    pub liquidator_fee: String,
    pub insurance_fee: String,
    pub liquidation_fee: String,
    pub mark_price: String,
    pub sum_unitary_fundings: String,
}

impl From<DbLiquidationTransfer> for LiquidationTransfer {
    fn from(value: DbLiquidationTransfer) -> Self {
        LiquidationTransfer {
            liquidation_transfer_id: value.liquidation_transfer_id.to_string(),
            liquidator_account_id: value.liquidator_account_id,
            symbol_hash: value.symbol_hash,
            position_qty_transfer: value.position_qty_transfer.to_string(),
            cost_position_transfer: value.cost_position_transfer.to_string(),
            liquidator_fee: value.liquidator_fee.to_string(),
            insurance_fee: value.insurance_fee.to_string(),
            liquidation_fee: value.liquidation_fee.to_string(),
            mark_price: value.mark_price.to_string(),
            sum_unitary_fundings: value.sum_unitary_fundings.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TradingEventInnerData {
    Transaction {
        account_id: String,
        sender: Option<String>,
        receiver: String, // receiver address
        token_hash: String,
        broker_hash: String,
        chain_id: String,
        side: TransactionSide, // “deposit｜withdraw"
        token_amount: String,
        withdraw_nonce: Option<i64>, // optional
        status: TransactionStatus,   // "succeed|failed"
        fail_reason: Option<i16>,    // optional
        fee: String,                 // zero fee for deposit
    },
    ProcessedTrades {
        batch_id: u64,
        trades: Vec<Trade>,
    },
    SettlementResult {
        account_id: String,
        settled_amount: String,
        settled_asset_hash: String,
        insurance_account_id: String,
        insurance_transfer_amount: String,
        settlement_executions: Vec<SettlementExecution>,
    },
    LiquidationResult {
        liquidated_account_id: String,
        insurance_account_id: String,
        liquidated_asset_hash: String,
        insurance_transfer_amount: String,
        liquidation_transfers: Vec<LiquidationTransfer>,
    },
    AdlResult {
        account_id: String,
        insurance_account_id: String,
        symbol_hash: String,
        position_qty_transfer: String,
        cost_position_transfer: String,
        adl_price: String,
        sum_unitary_fundings: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats_external::{Response, SuccessResponse};
    #[test]
    fn test_serialize_trading_event_response() {
        let mut response_data = TradingEventsResponse::default();
        response_data.last_block = 10_000_003;
        response_data.events.push(TradingEvent {
            block_number: 10_000_000,
            transaction_index: 0,
            log_index: 0,
            transaction_id: "0x19c5983e4c79802fc8ab706ae68b0877bde76d0f8f0b54ee094b204bf27599a5"
                .to_string(),
            block_timestamp: 1700622090,
            data: TradingEventInnerData::Transaction {
                account_id: "0x32ff7ea4f2eaa3d60da0d6985505e7bb40af02f1e0ca9c926c60643c6fc21d23"
                    .to_string(),
                sender: Some("0x6cbe925762348413fc2cfdd7bc9a8d04cb8e249e".to_string()),
                receiver: "0x6bf34299774aa438d257682a0a6f6da0456de6dc".to_string(),
                token_hash: "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa"
                    .to_string(),
                broker_hash: "0x6ca2f644ef7bd6d75953318c7f2580014941e753b3c6d54da56b3bf75dd14dfc"
                    .to_string(),
                chain_id: "421613".to_string(),
                side: TransactionSide::Withdraw,
                token_amount: "10000000".to_string(),
                withdraw_nonce: Some(23),
                status: TransactionStatus::Succeed,
                fail_reason: None,
                fee: "2000000".to_string(),
            },
        });
        response_data.events.push(TradingEvent {
            block_number: 10_000_000,
            transaction_index: 1,
            log_index: 0,
            transaction_id: "0x19c5983e4c79802fc8ab706ae68b0877bde76d0f8f0b54ee094b204bf27599a5"
                .to_string(),
            block_timestamp: 1700622092,
            data: TradingEventInnerData::ProcessedTrades {
                batch_id: 10_000,
                trades: vec![
                    Trade {
                        account_id:
                            "0x93280b058c1f4966f1f27cf12331926319b4ac2872486c07e8fa2df824649dd8"
                                .to_string(),
                        symbol_hash:
                            "0x7e83089239db756ee233fa8972addfea16ae653db0f692e4851aed546b21caeb"
                                .to_string(),
                        fee_asset_hash:
                            "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa"
                                .to_string(),
                        trade_qty: "100000000".to_string(),
                        notional: "1986500000".to_string(),
                        executed_price: "198650000000".to_string(),
                        fee: "715140".to_string(),
                        sum_unitary_fundings: "192860000000000000".to_string(),
                        trade_id: 20613,
                        match_id: 1700628604744716970,
                        timestamp: 1700628604744,
                        side: PurchaseSide::Buy,
                    },
                    Trade {
                        account_id:
                            "0x32ff7ea4f2eaa3d60da0d6985505e7bb40af02f1e0ca9c926c60643c6fc21d23"
                                .to_string(),
                        symbol_hash:
                            "0x7e83089239db756ee233fa8972addfea16ae653db0f692e4851aed546b21caeb"
                                .to_string(),
                        fee_asset_hash:
                            "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa"
                                .to_string(),
                        trade_qty: "-100000000".to_string(),
                        notional: "-1986500000".to_string(),
                        executed_price: "198650000000".to_string(),
                        fee: "476760".to_string(),
                        sum_unitary_fundings: "192860000000000000".to_string(),
                        trade_id: 20614,
                        match_id: 1700628604744716970,
                        timestamp: 1700628604744,
                        side: PurchaseSide::Sell,
                    },
                ],
            },
        });
        response_data.events.push(TradingEvent {
            block_number: 10_000_001,
            transaction_index: 0,
            log_index: 2,
            transaction_id: "0xd982aafa29dc80c1cc236f8228981e5a2315ddfe33d248096979c2251a91dd71"
                .to_string(),
            block_timestamp: 1700622094,
            data: TradingEventInnerData::SettlementResult {
                account_id: "0x42e06e99e9f1dd203e77eaa4d4fc1ae2042aeaf5679e81c0b2396a8ca271fa4c".to_string(),
                settled_amount: "2758834259".to_string(),
                settled_asset_hash: "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa".to_string(),
                insurance_account_id: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                insurance_transfer_amount: "0".to_string(),
                settlement_executions: vec![
                    SettlementExecution {
                        symbol_hash: "0xaaaaaaaaaaa".to_string(),
                        mark_price: "10000000000000000000".to_string(),
                        sum_unitary_fundings: "10000000000".to_string(),
                        settled_amount: "1379417129".to_string(),
                    },
                    SettlementExecution {
                        symbol_hash: "0xbbbbbbbb".to_string(),
                        mark_price: "350000000".to_string(),
                        sum_unitary_fundings: "9899998888".to_string(),
                        settled_amount: "1379417130".to_string(),
                    },
                ],
            },
        });
        response_data.events.push(TradingEvent {
            block_number: 10_000_001,
            transaction_index: 0,
            log_index: 1,
            transaction_id: "0xd982aafa29dc80c1cc236f8228981e5a2315ddfe33d248096979c2251a91dd71"
                .to_string(),
            block_timestamp: 1700622096,
            data: TradingEventInnerData::LiquidationResult {
                liquidated_account_id: "0xc4032e963bacb7411229d2018a7293dab86f24c202557d78d505afd6b49122cd".to_string(),
                insurance_account_id: "0x1fdb733d3f6e5b3c9b48caa1f59f9977bd38e8c0eb2f38c94ecd8a37ec91fc53".to_string(),
                liquidated_asset_hash: "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa".to_string(),
                insurance_transfer_amount: "0".to_string(),
                liquidation_transfers: vec![LiquidationTransfer {
                    liquidation_transfer_id: "100".to_string(),
                    liquidator_account_id: "0x9e4337fd086c18f6ce84c0da27101b26754a9ea7be95ec96645b987f04bcc2b2".to_string(),
                    symbol_hash: "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa".to_string(),
                    position_qty_transfer: "120".to_string(),
                    cost_position_transfer: "120".to_string(),
                    liquidator_fee: "12".to_string(),
                    insurance_fee: "10".to_string(),
                    liquidation_fee: "10".to_string(),
                    mark_price: "100000".to_string(),
                    sum_unitary_fundings: "1000000000".to_string(),
                }, LiquidationTransfer {
                    liquidation_transfer_id: "101".to_string(),
                    liquidator_account_id: "0x32ff7ea4f2eaa3d60da0d6985505e7bb40af02f1e0ca9c926c60643c6fc21d23".to_string(),
                    symbol_hash: "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa".to_string(),
                    position_qty_transfer: "109".to_string(),
                    cost_position_transfer: "10".to_string(),
                    liquidator_fee: "50".to_string(),
                    insurance_fee: "20".to_string(),
                    liquidation_fee: "15".to_string(),
                    mark_price: "12000000".to_string(),
                    sum_unitary_fundings: "11000000000".to_string(),
                },],
            }
        });
        response_data.events.push(
            TradingEvent{
                block_number: 0,
                transaction_index: 0,
                log_index: 0,
                transaction_id: "".to_string(),
                block_timestamp: 0,
                data: TradingEventInnerData::AdlResult {
                    account_id: "0x32ff7ea4f2eaa3d60da0d6985505e7bb40af02f1e0ca9c926c60643c6fc21d23".to_string(),
                    insurance_account_id: "0xee8f820585e359e969b2cd517edf326b55b89deb886aa2b51dbe9061b85bbe2d".to_string(),
                    symbol_hash: "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa".to_string(),
                    position_qty_transfer: "100".to_string(),
                    cost_position_transfer: "10000".to_string(),
                    adl_price: "120000000".to_string(),
                    sum_unitary_fundings: "1000000000".to_string(),
                },
            }
        );
        let response = Response::Success(SuccessResponse::new(response_data));
        let serde_str = serde_json::to_string(&response).unwrap();
        println!("{}", serde_str);
    }
}
