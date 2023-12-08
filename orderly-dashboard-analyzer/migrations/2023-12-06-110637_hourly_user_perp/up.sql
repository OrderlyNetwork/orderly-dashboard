-- Your SQL goes here
CREATE TABLE IF NOT EXISTS hourly_user_perp
(
    id                         SERIAL PRIMARY KEY,
    symbol                     text    not null,
    chain_hour                 bigint  not null,

    trading_fee                numeric not null,
    trading_volume             numeric not null,
    trading_count              numeric not null,

    realized_pnl               numeric not null,
    un_realized_pnl            numeric not null,
    latest_sum_unitary_funding numeric not null,

    liquidation_amount         numeric not null,
    liquidation_count          numeric not null,

    pulled_block_height        bigint  not null,
    pulled_block_timestamp     bigint  not null,
    created_time               numeric not null,
    updated_time               numeric not null
);

create index user_perp_chain_hour_index on hourly_user_perp (chain_hour);
