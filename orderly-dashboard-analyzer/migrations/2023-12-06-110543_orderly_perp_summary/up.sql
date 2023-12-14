-- Your SQL goes here
CREATE TABLE IF NOT EXISTS orderly_perp_summary
(
    symbol                   text    not null primary key,

    open_interest            numeric not null,
    total_trading_volume     numeric not null,
    total_trading_fee        numeric not null,

    total_trading_count      bigint  not null,
    total_trading_user_count bigint  not null,
    total_liquidation_amount numeric not null,
    total_liquidation_count  bigint  not null,

    pulled_block_height      bigint  not null,
    pulled_block_time        timestamp  not null
);