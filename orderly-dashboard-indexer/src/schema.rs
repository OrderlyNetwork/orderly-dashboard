// @generated automatically by Diesel CLI.

diesel::table! {
    adl_result (block_number, transaction_index, log_index) {
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        transaction_id -> Text,
        block_time -> Numeric,
        account_id -> Text,
        insurance_account_id -> Text,
        symbol_hash -> Text,
        position_qty_transfer -> Numeric,
        cost_position_transfer -> Numeric,
        adl_price -> Numeric,
        sum_unitary_fundings -> Numeric,
    }
}

diesel::table! {
    executed_trades (block_number, transaction_index, log_index) {
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        batch_id -> Numeric,
        typ -> Int2,
        account_id -> Text,
        symbol_hash -> Text,
        fee_asset_hash -> Text,
        trade_qty -> Numeric,
        notional -> Numeric,
        executed_price -> Numeric,
        fee -> Numeric,
        sum_unitary_fundings -> Numeric,
        trade_id -> Numeric,
        match_id -> Numeric,
        timestamp -> Numeric,
        side -> Bool,
    }
}

diesel::table! {
    liquidation_result (block_number, transaction_index, log_index) {
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        transaction_id -> Text,
        block_time -> Numeric,
        liquidated_account_id -> Text,
        insurance_account_id -> Text,
        liquidated_asset_hash -> Text,
        insurance_transfer_amount -> Numeric,
    }
}

diesel::table! {
    liquidation_transfer (block_number, transaction_index, log_index) {
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        transaction_id -> Text,
        liquidation_transfer_id -> Numeric,
        liquidator_account_id -> Text,
        symbol_hash -> Text,
        position_qty_transfer -> Numeric,
        cost_position_transfer -> Numeric,
        liquidator_fee -> Numeric,
        insurance_fee -> Numeric,
        mark_price -> Numeric,
        sum_unitary_fundings -> Numeric,
        liquidation_fee -> Numeric,
    }
}

diesel::table! {
    serial_batches (batch_id, event_type) {
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        transaction_id -> Text,
        block_time -> Numeric,
        batch_id -> Int8,
        event_type -> Int2,
    }
}

diesel::table! {
    settings (id) {
        id -> Int4,
        value -> Text,
    }
}

diesel::table! {
    settlement_execution (block_number, transaction_index, log_index) {
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        transaction_id -> Text,
        symbol_hash -> Text,
        sum_unitary_fundings -> Numeric,
        mark_price -> Numeric,
        settled_amount -> Numeric,
    }
}

diesel::table! {
    settlement_result (block_number, transaction_index, log_index) {
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        transaction_id -> Text,
        block_time -> Numeric,
        account_id -> Text,
        settled_amount -> Numeric,
        settled_asset_hash -> Text,
        insurance_account_id -> Text,
        insurance_transfer_amount -> Numeric,
        settlement_executions_count -> Numeric,
    }
}

diesel::table! {
    transaction_events (event_id) {
        event_id -> Int8,
        account_id -> Text,
        sender -> Nullable<Text>,
        receiver -> Text,
        transaction_id -> Text,
        token_hash -> Text,
        broker_hash -> Text,
        chain_id -> Numeric,
        side -> Int2,
        amount -> Numeric,
        fee -> Numeric,
        status -> Int2,
        block_time -> Numeric,
        withdraw_nonce -> Nullable<Int8>,
        fail_reason -> Nullable<Int2>,
        created_time -> Timestamp,
        updated_time -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    adl_result,
    executed_trades,
    liquidation_result,
    liquidation_transfer,
    serial_batches,
    settings,
    settlement_execution,
    settlement_result,
    transaction_events,
);
