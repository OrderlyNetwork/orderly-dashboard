-- Your SQL goes here
create table margin_transfer
(
    block_number         bigint   not null,
    transaction_index    integer  not null,
    transaction_id       text     not null,
    log_index            integer  not null,
    block_time           numeric  not null,
    account_id           text     not null,
    transfer_amount      numeric  not null,
    transfer_asset_hash  text     not null,
    iso_symbol_hash      text     not null,
    timestamp            bigint   not null,
    constraint margin_transfer_uq
        primary key(block_number, transaction_index, log_index)
);