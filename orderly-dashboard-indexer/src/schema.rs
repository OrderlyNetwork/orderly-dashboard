// @generated automatically by Diesel CLI.

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
    settings (id) {
        id -> Int4,
        value -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(executed_trades, settings,);
