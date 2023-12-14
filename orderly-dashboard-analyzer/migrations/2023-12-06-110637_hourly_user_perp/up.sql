-- Your SQL goes here
CREATE TABLE IF NOT EXISTS hourly_user_perp
(
    account_id                 text    not null,
    symbol                     text    not null,
    block_hour                 timestamp  not null,

    trading_fee                numeric not null,
    trading_volume             numeric not null,
    trading_count              bigint  not null,

    realized_pnl               numeric not null,
    un_realized_pnl            numeric not null,
    latest_sum_unitary_funding numeric not null,

    liquidation_amount         numeric not null,
    liquidation_count          bigint  not null,

    pulled_block_height        bigint  not null,
    pulled_block_time          timestamp  not null,

    constraint hourly_user_perp_uq primary key (account_id, symbol, block_hour)
);

create index user_perp_chain_hour_index on hourly_user_perp (block_hour);
