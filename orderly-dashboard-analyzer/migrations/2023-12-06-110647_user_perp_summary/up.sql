-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_perp_summary
(
    id                       SERIAL PRIMARY KEY,
    symbol                   text    not null,
    open_interest            numeric not null,
    total_trading_volume     numeric not null,
    total_trading_count      numeric not null,
    total_liquidation_amount numeric not null,
    total_liquidation_count  numeric not null,
    pulled_block_height      bigint  not null,
    pulled_block_timestamp   bigint  not null,
    created_timestamp        timestamp with time zone,
    updated_timestamp        timestamp with time zone
);