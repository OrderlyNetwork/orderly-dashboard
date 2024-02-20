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
    fee_distribution (block_number, transaction_index, log_index) {
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        transaction_id -> Text,
        block_time -> Numeric,
        event_id -> Numeric,
        from_account_id -> Text,
        to_account_id -> Text,
        amount -> Numeric,
        token_hash -> Text,
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
        liquidation_result_log_idx -> Int4,
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
        effective_gas_price -> Nullable<Numeric>,
        l1_fee -> Nullable<Numeric>,
        l1_gas_price -> Nullable<Numeric>,
        l1_gas_used -> Nullable<Numeric>,
        gas_used -> Nullable<Numeric>,
        l1_fee_scalar -> Nullable<Text>,
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
        settlement_result_log_idx -> Int4,
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
    }
}

diesel::table! {
    symbols_config (symbol) {
        symbol -> Text,
        symbol_hash -> Text,
        base_maintenance_margin -> Nullable<Int4>,
        base_initial_margin -> Nullable<Int4>,
        mark_price -> Nullable<Numeric>,
        index_price_orderly -> Nullable<Numeric>,
        sum_unitary_fundings -> Nullable<Numeric>,
        last_mark_price_updated -> Nullable<Numeric>,
        last_funding_updated -> Nullable<Numeric>,
    }
}

diesel::table! {
    transaction_events (block_number, transaction_index, log_index) {
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        transaction_id -> Text,
        block_time -> Numeric,
        account_id -> Text,
        sender -> Nullable<Text>,
        receiver -> Text,
        token_hash -> Text,
        broker_hash -> Text,
        chain_id -> Numeric,
        side -> Int2,
        amount -> Numeric,
        fee -> Numeric,
        status -> Int2,
        withdraw_nonce -> Nullable<Int8>,
        fail_reason -> Nullable<Int2>,
        effective_gas_price -> Nullable<Numeric>,
        l1_fee -> Nullable<Numeric>,
        l1_gas_price -> Nullable<Numeric>,
        l1_gas_used -> Nullable<Numeric>,
        gas_used -> Nullable<Numeric>,
        l1_fee_scalar -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    adl_result,
    executed_trades,
    fee_distribution,
    liquidation_result,
    liquidation_transfer,
    serial_batches,
    settings,
    settlement_execution,
    settlement_result,
    symbols_config,
    transaction_events,
);
