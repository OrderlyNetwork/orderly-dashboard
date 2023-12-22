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
    block_summary (id) {
        id -> Int4,
        latest_block_height -> Int8,
        pulled_block_height -> Int8,
        pulled_block_time -> Timestamp,
        pulled_event_id -> Int8,
        pulled_perp_trade_id -> Int8,
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
    hourly_orderly_perp (symbol, block_hour) {
        symbol -> Text,
        block_hour -> Timestamp,
        trading_fee -> Numeric,
        trading_volume -> Numeric,
        trading_count -> Int8,
        trading_user_count -> Int8,
        opening_count -> Int8,
        liquidation_amount -> Numeric,
        liquidation_count -> Int8,
        pulled_block_height -> Int8,
        pulled_block_time -> Timestamp,
    }
}

diesel::table! {
    hourly_orderly_token (token, block_hour) {
        token -> Text,
        block_hour -> Timestamp,
        chain_id -> Text,
        withdraw_amount -> Numeric,
        withdraw_count -> Int8,
        deposit_amount -> Numeric,
        deposit_count -> Int8,
        pulled_block_height -> Int8,
        pulled_block_time -> Timestamp,
    }
}

diesel::table! {
    hourly_user_perp (account_id, symbol, block_hour) {
        account_id -> Text,
        symbol -> Text,
        block_hour -> Timestamp,
        trading_fee -> Numeric,
        trading_volume -> Numeric,
        trading_count -> Int8,
        realized_pnl -> Numeric,
        un_realized_pnl -> Numeric,
        latest_sum_unitary_funding -> Numeric,
        liquidation_amount -> Numeric,
        liquidation_count -> Int8,
        pulled_block_height -> Int8,
        pulled_block_time -> Timestamp,
    }
}

diesel::table! {
    hourly_user_token (account_id, block_hour, token, chain_id) {
        account_id -> Text,
        token -> Text,
        block_hour -> Timestamp,
        chain_id -> Text,
        withdraw_amount -> Numeric,
        withdraw_count -> Int8,
        deposit_amount -> Numeric,
        deposit_count -> Int8,
        pulled_block_height -> Int8,
        pulled_block_time -> Timestamp,
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
    orderly_perp_summary (symbol) {
        symbol -> Text,
        open_interest -> Numeric,
        total_trading_volume -> Numeric,
        total_trading_fee -> Numeric,
        total_trading_count -> Int8,
        total_trading_user_count -> Int8,
        total_liquidation_amount -> Numeric,
        total_liquidation_count -> Int8,
        pulled_block_height -> Int8,
        pulled_block_time -> Timestamp,
        buy_amount -> Numeric,
        sell_amount -> Numeric,
    }
}

diesel::table! {
    orderly_token_summary (token, chain_id) {
        token -> Text,
        chain_id -> Text,
        balance -> Numeric,
        total_withdraw_amount -> Numeric,
        total_withdraw_count -> Int8,
        total_deposit_amount -> Numeric,
        total_deposit_count -> Int8,
        pulled_block_height -> Int8,
        pulled_block_time -> Timestamp,
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
    symbols (symbol) {
        symbol -> Varchar,
        symbol_hash -> Varchar,
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
    }
}

diesel::table! {
    user_perp_summary (account_id, symbol) {
        account_id -> Text,
        symbol -> Text,
        holding -> Numeric,
        opening_cost -> Numeric,
        cost_position -> Numeric,
        total_trading_volume -> Numeric,
        total_trading_count -> Int8,
        total_trading_fee -> Numeric,
        total_realized_pnl -> Numeric,
        total_un_realized_pnl -> Numeric,
        total_liquidation_amount -> Numeric,
        total_liquidation_count -> Int8,
        pulled_block_height -> Int8,
        pulled_block_time -> Timestamp,
    }
}

diesel::table! {
    user_token_summary (account_id, token, chain_id) {
        account_id -> Text,
        token -> Text,
        chain_id -> Text,
        balance -> Numeric,
        total_withdraw_amount -> Numeric,
        total_deposit_amount -> Numeric,
        total_withdraw_count -> Int8,
        total_deposit_count -> Int8,
        pulled_block_height -> Int8,
        pulled_block_time -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    adl_result,
    block_summary,
    executed_trades,
    hourly_orderly_perp,
    hourly_orderly_token,
    hourly_user_perp,
    hourly_user_token,
    liquidation_result,
    liquidation_transfer,
    orderly_perp_summary,
    orderly_token_summary,
    serial_batches,
    settings,
    settlement_execution,
    settlement_result,
    symbols,
    transaction_events,
    user_perp_summary,
    user_token_summary,
);
