// AUTO-GENERATED by typescript-type-def

export default types;
export namespace types {
    export type F64 = number;
    export type DailyVolumeExtern = {
        "daytime": (string)[];
        "volume": (types.F64)[];
    };
    export type DailyTradingFeeExtern = {
        "daytime": (string)[];
        "fee_amount": (types.F64)[];
    };
    export type TradingVolumeRanking = {
        "account_ids": (string)[];
        "volume": (types.F64)[];
    };
    export type TradingPnlRanking = {
        "account_ids": (string)[];
        "volume": (types.F64)[];
    };
    export type TokenAmountRanking = {
        "account_ids": (string)[];
        "volume": (types.F64)[];
    };
    export type OrderlyPerpDaily = {
        "trading_volume": types.F64;
        "trading_fee": types.F64;
        "trading_count": types.F64;
        "trading_user_count": types.F64;
        "liquidation_amount": types.F64;
        "liquidation_count": types.F64;
        "opening_count": types.F64;
    };
    export type DailyData<T> = {
        "daytime": (string)[];
        "data": (T)[];
    };
    export type OrderlyGasFee = {
        "avg_gas_fee": types.F64;
    };
    export type OrderlyTokenDaily = {
        "withdraw_amount": types.F64;
        "withdraw_count": types.F64;
        "deposit_amount": types.F64;
        "deposit_count": types.F64;
    };
    export type UserPerpHoldingRanking = {
        "account_ids": (string)[];
        "holding": (types.F64)[];
    };
    export type CountAverageExtern = {
        "latest_day_metric": types.F64;
        "latest_week_metric": types.F64;
        "latest_month_metric": types.F64;
    };
    export type TransactionSide = ("deposit" | "withdraw");
    export type TransactionStatus = ("succeed" | "failed");
    export type PurchaseSide = ("BUY" | "SELL");
    export type U64 = number;
    export type Trade = {
        "account_id": string;
        "symbol_hash": string;
        "fee_asset_hash": string;
        "trade_qty": string;
        "notional": string;
        "executed_price": string;
        "fee": string;
        "sum_unitary_fundings": string;
        "trade_id": types.U64;
        "match_id": types.U64;
        "timestamp": types.U64;
        "side": types.PurchaseSide;
    };
    export type SettlementExecution = {
        "symbol_hash": string;
        "mark_price": string;
        "sum_unitary_fundings": string;
        "settled_amount": string;
    };
    export type LiquidationTransfer = {
        "liquidation_transfer_id": string;
        "liquidator_account_id": string;
        "symbol_hash": string;
        "position_qty_transfer": string;
        "cost_position_transfer": string;
        "liquidator_fee": string;
        "insurance_fee": string;
        "liquidation_fee": string;
        "mark_price": string;
        "sum_unitary_fundings": string;
    };
    export type I64 = number;
    export type I16 = number;
    export type TradingEventInnerData = ({
        "Transaction": {
            "account_id": string;
            "sender": (string | null);
            "receiver": string;
            "token_hash": string;
            "broker_hash": string;
            "chain_id": string;
            "side": types.TransactionSide;
            "token_amount": string;
            "withdraw_nonce": (types.I64 | null);
            "status": types.TransactionStatus;
            "fail_reason": (types.I16 | null);
            "fee": string;
        };
    } | {
        "ProcessedTrades": {
            "batch_id": types.U64;
            "trades": (types.Trade)[];
        };
    } | {
        "SettlementResult": {
            "account_id": string;
            "settled_amount": string;
            "settled_asset_hash": string;
            "insurance_account_id": string;
            "insurance_transfer_amount": string;
            "settlement_executions": (types.SettlementExecution)[];
        };
    } | {
        "LiquidationResult": {
            "liquidated_account_id": string;
            "insurance_account_id": string;
            "liquidated_asset_hash": string;
            "insurance_transfer_amount": string;
            "liquidation_transfers": (types.LiquidationTransfer)[];
        };
    } | {
        "AdlResult": {
            "account_id": string;
            "insurance_account_id": string;
            "symbol_hash": string;
            "position_qty_transfer": string;
            "cost_position_transfer": string;
            "adl_price": string;
            "sum_unitary_fundings": string;
        };
    });
    export type U32 = number;
    export type TradingEvent = {
        "block_number": types.U64;
        "transaction_index": types.U32;
        "log_index": types.U32;
        "transaction_id": string;
        "block_timestamp": types.U64;
        "data": types.TradingEventInnerData;
    };
    export type AccountTradingEventsResponse = {
        "events": (types.TradingEvent)[];
    };
}
