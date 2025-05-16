-- Your SQL goes here
create table market_info
(
    symbol               text     not null,
    symbol_hash          text     not null,
    index_price          numeric  not null,
    mark_price           numeric  not null,
    sum_unitary_funding  numeric  not null,
    open_interest        numeric  not null,
    update_time          timestamp  not null,
    constraint market_info_uq
        primary key(symbol_hash)
);
