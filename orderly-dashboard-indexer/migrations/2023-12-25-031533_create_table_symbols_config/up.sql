-- Your SQL goes here
create table if not exists symbols_config
(
    symbol                  text            not null,
    symbol_hash             text            not null,
    base_maintenance_margin integer,
    base_initial_margin     integer,
    mark_price              numeric,
    index_price_orderly     numeric,
    sum_unitary_fundings    numeric,
    last_mark_price_updated numeric,
    last_funding_updated    numeric,
    constraint symbol_pk
    primary key(symbol)
);