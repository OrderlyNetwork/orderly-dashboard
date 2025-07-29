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
        version -> Nullable<Int2>,
    }
}

diesel::table! {
    balance_transfer (block_number, transaction_index, log_index) {
        block_number -> Int8,
        transaction_index -> Int4,
        log_index -> Int4,
        transaction_id -> Text,
        block_time -> Numeric,
        account_id -> Text,
        amount -> Numeric,
        token_hash -> Text,
        is_from_account_id -> Bool,
        transfer_type -> Int2,
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
        block_time -> Int8,
    }
}

diesel::table! {
    executed_trades_before_y2024 (block_number, transaction_index, log_index, block_time) {
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
        block_time -> Timestamp,
    }
}

diesel::table! {
    executed_trades_y2024q01 (block_number, transaction_index, log_index, block_time) {
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
        block_time -> Timestamp,
    }
}

diesel::table! {
    executed_trades_y2024q02 (block_number, transaction_index, log_index, block_time) {
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
        block_time -> Timestamp,
    }
}

diesel::table! {
    executed_trades_y2024q03 (block_number, transaction_index, log_index, block_time) {
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
        block_time -> Timestamp,
    }
}

diesel::table! {
    executed_trades_y2024q04 (block_number, transaction_index, log_index, block_time) {
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
        block_time -> Timestamp,
    }
}

diesel::table! {
    executed_trades_y2025q01 (block_number, transaction_index, log_index, block_time) {
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
        block_time -> Timestamp,
    }
}

diesel::table! {
    executed_trades_y2025q02 (block_number, transaction_index, log_index, block_time) {
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
        block_time -> Timestamp,
    }
}

diesel::table! {
    executed_trades_y2025q03 (block_number, transaction_index, log_index, block_time) {
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
        block_time -> Timestamp,
    }
}

diesel::table! {
    executed_trades_y2025q04 (block_number, transaction_index, log_index, block_time) {
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
        block_time -> Timestamp,
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
        version -> Nullable<Int2>,
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
        block_time -> Nullable<Numeric>,
        version -> Nullable<Int2>,
    }
}

diesel::table! {
    partitioned_executed_trades (block_number, transaction_index, log_index, block_time) {
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
        block_time -> Timestamp,
    }
}

diesel::table! {
    rebalance_events (rebalance_id) {
        rebalance_id -> Int8,
        token_hash -> Text,
        amount -> Numeric,
        src_chain_id -> Numeric,
        dst_chain_id -> Numeric,
        burn_tx_id -> Text,
        burn_result_tx_id -> Nullable<Text>,
        mint_tx_id -> Nullable<Text>,
        mint_result_tx_id -> Nullable<Text>,
        burn_result_block_time -> Nullable<Int8>,
        mint_result_block_time -> Nullable<Int8>,
        burn_result_block_number -> Nullable<Int8>,
        mint_result_block_number -> Nullable<Int8>,
        burn_success -> Nullable<Bool>,
        mint_success -> Nullable<Bool>,
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
        gas_used -> Nullable<Numeric>,
        l1_fee -> Nullable<Numeric>,
        l1_fee_scalar -> Nullable<Text>,
        l1_gas_price -> Nullable<Numeric>,
        l1_gas_used -> Nullable<Numeric>,
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
        block_time -> Nullable<Numeric>,
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
    sol_transaction_events (block_number, transaction_index, log_index) {
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
        gas_used -> Nullable<Numeric>,
        l1_fee -> Nullable<Numeric>,
        l1_fee_scalar -> Nullable<Text>,
        l1_gas_price -> Nullable<Numeric>,
        l1_gas_used -> Nullable<Numeric>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    adl_result,
    balance_transfer,
    executed_trades,
    executed_trades_before_y2024,
    executed_trades_y2024q01,
    executed_trades_y2024q02,
    executed_trades_y2024q03,
    executed_trades_y2024q04,
    executed_trades_y2025q01,
    executed_trades_y2025q02,
    executed_trades_y2025q03,
    executed_trades_y2025q04,
    fee_distribution,
    liquidation_result,
    liquidation_transfer,
    partitioned_executed_trades,
    rebalance_events,
    serial_batches,
    settings,
    settlement_execution,
    settlement_result,
    sol_transaction_events,
    symbols_config,
    transaction_events,
);
