## APIs
Response format
* success: `true` | `false`
* err_code: `0` -> ok or others
* err_msg: `null` | `string`
* data: `null` | json format data
```json
{
    "data": {
    },
    "err_code": 0,
    "err_msg": null,
    "success": true
}
```

Insert test data
```shell
psql -U {your databse account} -d {your databse} -f ../../scripts/init_data.sql
```

### Pull trading events on EVM(will be disabled recently)
```shell
GET /events?broker_id=woofi_pro&address=0x9e15a53b9dfa30e6b220d0e3c93253bea7191769&from_time=1711123200&to_time=1713801600
```

### Pull trading events on Solana
```shell
GET /sol_events?broker_id=raydium&address=3nrWg4GJijT1NnxrmupFD7CJjshZnHyRAAYEmLizMqBc
```

### Pull trading events api v2 on EVM
```shell
GET /events_v2?address=0x9cccf6a1c43552bceddbf81155ef54699fe4f946&broker_id=orderly&from_time=1736152774&to_time=1737362374&event_type=PERPTRADE&offset=0
```
Parameters:

| Name | Type | Required | Description |
|-----|--------|-------------|------------|
| `address` | String | Y | user wallet address |
| `broker_id` | String | Y | broker name |
| `from_time` | Integer | N | from time, default two weeks ago |
| `to_time` | Integer | N | to time, default now |
| `event_type` | String | N | there are `PERPTRADE`, `SETTLEMENT`, `LIQUIDATION`, `ADL`, `TRANSACTION` for selecting events, if not set, all kinds of events will be returned |
| `offset` | Integer | N | this api is paginated, offset is the offset of the page, default is 0 |

Response:

| Name | Type | Required | Description |
|-----|--------|-------------|------------|
| `success` | Bool | Y | success status |
| `data` | object{events: array[object], next_offset: Optional(integer)} | Y | return events and next offset, if there is no more events, next_offset will be null, and you need to set offset to the value of next_offset to get the next page |

For example:
request: https://dev-orderly-dashboard-query-service.orderly.network/events_v2?address=0x9cccf6a1c43552bceddbf81155ef54699fe4f946&broker_id=orderly&from_time=1736152774&to_time=1737362374&from=01/06/2025&to=01/20/2025&offset=0

response:
```json
{
    "success": true,
    "data": {
        "events": [...],
        "next_offset": 1000
    }
}
```
The single event object is like this:
```json
{
    "block_number": 123456,
    "transaction_index": 1,
    "log_index": 1,
    "transaction_id": "0x1234567890",
    "block_timestamp": 1736152774,
    "data": {...}
}
```
The single event's data struture in rust is like this [TradingEvent](https://github.com/OrderlyNetwork/orderly-dashboard/blob/main/orderly-dashboard-indexer/src/formats_external/trading_events.rs#L47):
```rust
#[derive(Debug, Deserialize, Serialize, Clone, TypeDef)]
pub struct TradingEvent {
    pub block_number: u64,
    pub transaction_index: u32,
    pub log_index: u32,
    pub transaction_id: String,
    pub block_timestamp: u64,
    pub data: TradingEventInnerData,
}

#[derive(Debug, Deserialize, Serialize, Clone, TypeDef)]
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
    LiquidationResultV2 {
        account_id: String,
        liquidated_asset_hash: String,
        insurance_transfer_amount: String,
        liquidation_transfers: Vec<LiquidationTransferV2>,
    },
    AdlResultV2 {
        account_id: String,
        symbol_hash: String,
        position_qty_transfer: String,
        cost_position_transfer: String,
        adl_price: String,
        sum_unitary_fundings: String,
    },
}
```