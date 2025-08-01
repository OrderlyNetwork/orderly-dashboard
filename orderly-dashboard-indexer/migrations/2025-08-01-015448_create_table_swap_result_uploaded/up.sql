-- Your SQL goes here
create table if not exists swap_result_uploaded
(
    block_number                  bigint   not null,
    transaction_index             integer  not null,
    log_index                     integer  not null,
    transaction_id                text     not null,
    block_time                    numeric  not null,
    account_id                    text     not null,
    buy_token_hash                text     not null,
    sell_token_hash               text     not null,
    buy_quantity                  numeric  not null,
    sell_quantity                 numeric  not null,
    chain_id                      numeric  not null,
    swap_status                   smallint not null,
    constraint swap_result_uploaded_id primary key (block_number,transaction_index,log_index)
);

create index IF NOT EXISTS swap_result_uploaded_account_id_index
    on swap_result_uploaded (account_id, block_time);
