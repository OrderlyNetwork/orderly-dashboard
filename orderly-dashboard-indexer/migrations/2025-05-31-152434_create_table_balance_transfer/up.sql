-- Your SQL goes here
create table if not exists balance_transfer
(
    block_number                  bigint   not null,
    transaction_index             integer  not null,
    log_index                     integer  not null,
    transaction_id                text     not null,
    block_time                    numeric  not null,
    account_id                    text     not null,
    amount                        numeric  not null,
    token_hash                    text     not null,
    is_from_account_id            boolean  not null,
    transfer_type                 smallint not null,
    constraint balance_transfer_id primary key (block_number,transaction_index,log_index)
);