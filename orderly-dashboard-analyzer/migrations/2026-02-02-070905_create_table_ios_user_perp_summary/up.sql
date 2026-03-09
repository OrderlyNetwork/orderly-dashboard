-- Your SQL goes here
CREATE TABLE IF NOT EXISTS iso_user_perp_summary
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
    margin_token             text    not null,
    margin_qty               numeric not null,

    pulled_block_height      bigint  not null,
    pulled_block_time        timestamp  not null,
    sum_unitary_fundings     numeric not null,
    constraint iso_user_perp_summary_uq primary key (account_id, symbol)
);

create index if not EXISTS iso_user_perp_account_symbol_index on iso_user_perp_summary (symbol);
alter table orderly_perp_summary add column iso_open_interest numeric;