// @generated automatically by Diesel CLI.

diesel::table! {
    block_summary (id) {
        id -> Int8,
        latest_block_height -> Int8,
        pulled_block_height -> Int8,
        pulled_block_timestamp -> Int8,
        pulled_block_hash -> Text,
        pulled_event_id -> Int8,
        pulled_spot_trade_id -> Int8,
        pulled_perp_trade_id -> Int8,
        created_time -> Numeric,
        updated_time -> Numeric,
    }
}

diesel::table! {
    hourly_order_token (id) {
        id -> Int4,
        token -> Text,
        chain_hour -> Int8,
        chain_id -> Text,
        token_address -> Text,
        withdraw_amount -> Numeric,
        withdraw_count -> Numeric,
        deposit_amount -> Numeric,
        deposit_count -> Numeric,
        pulled_block_height -> Numeric,
        pulled_block_timestamp -> Numeric,
        created_time -> Numeric,
        updated_time -> Numeric,
    }
}

diesel::table! {
    hourly_orderly_perp (id) {
        id -> Int4,
        symbol -> Text,
        chain_hour -> Int8,
        trading_fee -> Numeric,
        trading_volume -> Numeric,
        trading_count -> Numeric,
        trading_user_count -> Numeric,
        opening_count -> Numeric,
        liquidation_amount -> Numeric,
        liquidation_count -> Numeric,
        pulled_block_height -> Int8,
        pulled_block_timestamp -> Int8,
        created_time -> Numeric,
        updated_time -> Numeric,
    }
}

diesel::table! {
    hourly_user_perp (id) {
        id -> Int4,
        symbol -> Text,
        chain_hour -> Int8,
        trading_fee -> Numeric,
        trading_volume -> Numeric,
        trading_count -> Numeric,
        realized_pnl -> Numeric,
        un_realized_pnl -> Numeric,
        latest_sum_unitary_funding -> Numeric,
        liquidation_amount -> Numeric,
        liquidation_count -> Numeric,
        pulled_block_height -> Int8,
        pulled_block_timestamp -> Int8,
        created_time -> Numeric,
        updated_time -> Numeric,
    }
}

diesel::table! {
    hourly_user_token (id) {
        id -> Int4,
        account_id -> Text,
        token -> Text,
        chain_hour -> Int8,
        chain_id -> Text,
        token_address -> Text,
        withdraw_amount -> Numeric,
        withdraw_count -> Numeric,
        deposit_amount -> Numeric,
        deposit_count -> Numeric,
        pulled_block_height -> Numeric,
        pulled_block_timestamp -> Numeric,
        created_time -> Numeric,
        updated_time -> Numeric,
    }
}

diesel::table! {
    orderly_perp_summary (id) {
        id -> Int4,
        symbol -> Text,
        open_interest -> Numeric,
        total_trading_volume -> Numeric,
        total_trading_count -> Numeric,
        total_trading_user_count -> Numeric,
        total_liquidation_amount -> Numeric,
        total_liquidation_count -> Numeric,
        pulled_block_height -> Int8,
        pulled_block_timestamp -> Int8,
        created_timestamp -> Numeric,
        updated_timestamp -> Numeric,
    }
}

diesel::table! {
    orderly_token_summary (id) {
        id -> Int4,
        token -> Text,
        chain_id -> Text,
        token_address -> Text,
        balance -> Numeric,
        total_withdraw_amount -> Numeric,
        total_deposit_amount -> Numeric,
        total_withdraw_count -> Numeric,
        total_deposit_count -> Numeric,
        pulled_block_height -> Numeric,
        pulled_block_timestamp -> Numeric,
        created_time -> Numeric,
        updated_time -> Numeric,
    }
}

diesel::table! {
    user_perp_summary (id) {
        id -> Int4,
        account_id -> Text,
        symbol -> Text,
        holding -> Numeric,
        opening_cost -> Numeric,
        cost_position -> Numeric,
        total_trading_volume -> Numeric,
        total_trading_count -> Numeric,
        total_liquidation_amount -> Numeric,
        total_liquidation_count -> Numeric,
        pulled_block_height -> Int8,
        pulled_block_timestamp -> Int8,
        created_timestamp -> Numeric,
        updated_timestamp -> Numeric,
    }
}

diesel::table! {
    user_token_summary (id) {
        id -> Int4,
        account_id -> Text,
        token -> Text,
        chain_id -> Text,
        token_address -> Text,
        balance -> Numeric,
        total_withdraw_amount -> Numeric,
        total_deposit_amount -> Numeric,
        total_withdraw_count -> Numeric,
        total_deposit_count -> Numeric,
        pulled_block_height -> Numeric,
        pulled_block_timestamp -> Numeric,
        created_time -> Numeric,
        updated_time -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    block_summary,
    hourly_order_token,
    hourly_orderly_perp,
    hourly_user_perp,
    hourly_user_token,
    orderly_perp_summary,
    orderly_token_summary,
    user_perp_summary,
    user_token_summary,
);
