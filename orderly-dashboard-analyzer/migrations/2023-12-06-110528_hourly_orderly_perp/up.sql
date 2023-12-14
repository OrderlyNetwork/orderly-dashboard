-- Your SQL goes here
CREATE TABLE IF NOT EXISTS hourly_orderly_perp
(
    symbol              text    not null,
    block_hour          timestamp  not null,

    trading_fee         numeric not null,
    trading_volume      numeric not null,

    trading_count       bigint  not null,
    trading_user_count  bigint  not null,
    opening_count       bigint  not null,

    liquidation_amount  numeric not null,
    liquidation_count   bigint  not null,

    pulled_block_height bigint  not null,
    pulled_block_time   timestamp  not null,
    constraint hourly_orderly_perp_uq primary key (symbol, block_hour)
);

create index orderly_perp_chain_hour_index on hourly_orderly_perp (block_hour);
