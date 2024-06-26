-- Your SQL goes here
create table settlement_result
(
    block_number                  bigint   not null,
    transaction_index             integer  not null,
    log_index                     integer  not null,
    transaction_id                text     not null,
    block_time                    numeric  not null,
    account_id                    text     not null,
    settled_amount                numeric  not null,
    settled_asset_hash            text     not null,
    insurance_account_id          text     not null,
    insurance_transfer_amount     numeric  not null,
    constraint settlement_result_uq
        primary key(block_number, transaction_index, log_index)
);

create index settlement_account_id_index
    on settlement_result (account_id);