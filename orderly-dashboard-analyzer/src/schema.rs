// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "timetz", schema = "pg_catalog"))]
    pub struct Timetz;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Timetz;

    block_summary (id) {
        id -> Int8,
        pulled_block_height -> Int8,
        pulled_block_timestamp -> Int8,
        pulled_block_hash -> Text,
        pulled_event_id -> Int8,
        pulled_spot_trade_id -> Int8,
        pulled_perp_trade_id -> Int8,
        created_time -> Timetz,
        updated_time -> Timetz,
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
        deposit_amount -> Numeric,
        pulled_block_height -> Numeric,
        pulled_block_timestamp -> Numeric,
        created_time -> Timestamptz,
        updated_time -> Timestamptz,
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
        liquidation_amount -> Numeric,
        liquidation_count -> Numeric,
        pulled_block_height -> Int8,
        pulled_block_timestamp -> Int8,
        created_time -> Nullable<Timestamptz>,
        updated_time -> Nullable<Timestamptz>,
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
        deposit_amount -> Numeric,
        pulled_block_height -> Numeric,
        pulled_block_timestamp -> Numeric,
        created_time -> Nullable<Timestamptz>,
        updated_time -> Nullable<Timestamptz>,
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
        pulled_block_height -> Numeric,
        pulled_block_timestamp -> Timestamptz,
        created_time -> Timestamptz,
        updated_time -> Timestamptz,
    }
}

diesel::table! {
    user_perp_summary (id) {
        id -> Int4,
        symbol -> Text,
        open_interest -> Numeric,
        total_trading_volume -> Numeric,
        total_trading_count -> Numeric,
        total_liquidation_amount -> Numeric,
        total_liquidation_count -> Numeric,
        pulled_block_height -> Int8,
        pulled_block_timestamp -> Int8,
        created_timestamp -> Nullable<Timestamptz>,
        updated_timestamp -> Nullable<Timestamptz>,
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
        pulled_block_height -> Numeric,
        pulled_block_timestamp -> Timestamptz,
        created_time -> Timestamptz,
        updated_time -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    block_summary,
    hourly_order_token,
    hourly_user_perp,
    hourly_user_token,
    orderly_token_summary,
    user_perp_summary,
    user_token_summary,
);
