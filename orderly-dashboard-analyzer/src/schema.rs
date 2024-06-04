// @generated automatically by Diesel CLI.

diesel::table! {
    block_summary (id) {
        id -> Int4,
        latest_block_height -> Int8,
        pulled_block_height -> Int8,
        pulled_block_time -> Timestamp,
        pulled_event_id -> Int8,
        pulled_perp_trade_id -> Int8,
        metrics_type -> Text,
    }
}

diesel::table! {
    broker_info (broker_id) {
        broker_id -> Text,
        broker_hash -> Text,
    }
}

diesel::table! {
    hourly_gas_fee (event_type, block_hour) {
        block_hour -> Timestamp,
        gas_fee -> Numeric,
        event_type -> Text,
        batch_count -> Int8,
        pulled_block_height -> Int8,
        pulled_block_time -> Timestamp,
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
    symbols (symbol) {
        symbol -> Text,
        symbol_hash -> Text,
    }
}

diesel::table! {
    user_info (account_id) {
        account_id -> Text,
        broker_id -> Text,
        broker_hash -> Text,
        address -> Text,
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
        sum_unitary_fundings -> Numeric,
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
    block_summary,
    broker_info,
    hourly_gas_fee,
    hourly_orderly_perp,
    hourly_orderly_token,
    hourly_user_perp,
    hourly_user_token,
    orderly_perp_summary,
    orderly_token_summary,
    symbols,
    user_info,
    user_perp_summary,
    user_token_summary,
);
