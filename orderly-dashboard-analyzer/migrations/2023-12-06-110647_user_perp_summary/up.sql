-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_perp_summary
(
    id                       SERIAL PRIMARY KEY,
    account_id               text    not null,
    symbol                   text    not null,
    holding                  numeric not null,
    total_trading_volume     numeric not null,
    total_trading_count      numeric not null,
    total_liquidation_amount numeric not null,
    total_liquidation_count  numeric not null,
    pulled_block_height      bigint  not null,
    pulled_block_timestamp   bigint  not null,
    created_timestamp        numeric not null ,
    updated_timestamp        numeric not null
);

create index user_perp_account_id_index on user_perp_summary (account_id);
