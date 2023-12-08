-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_perp_summary
(
    account_id               text    not null,
    symbol                   text    not null,

    holding                  numeric not null,
    opening_cost             numeric not null,
    cost_position            numeric not null,

    total_trading_volume     numeric not null,
    total_trading_count      bigint  not null,
    total_trading_fee        numeric not null,

    total_realized_pnl       numeric not null,
    total_un_realized_pnl    numeric not null,

    total_liquidation_amount numeric not null,
    total_liquidation_count  bigint  not null,

    pulled_block_height      bigint  not null,
    pulled_block_time        bigint  not null,
    constraint user_perp_summary_uq primary key (account_id, symbol)
);

create index user_perp_account_id_index on user_perp_summary (account_id);
