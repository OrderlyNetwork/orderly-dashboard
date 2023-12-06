-- Your SQL goes here
CREATE TABLE IF NOT EXISTS hourly_orderly_perp
(
    id                     SERIAL PRIMARY KEY,
    symbol                 text                     not null,
    chain_hour             bigint                   not null,
    trading_fee            numeric                  not null,
    trading_volume         numeric                  not null,
    trading_count          numeric                  not null,
    opening_count          numeric                  not null,
    liquidation_amount     numeric                  not null,
    liquidation_count      numeric                  not null,
    pulled_block_height    bigint                   not null,
    pulled_block_timestamp bigint                   not null,
    created_time           timestamp with time zone not null,
    updated_time           timestamp with time zone not null
);

create index orderly_perp_chain_hour_index on hourly_orderly_perp (chain_hour);
