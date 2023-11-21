-- Your SQL goes here
create table liquidation_result
(
    block_number                  bigint   not null,
    transaction_index             integer  not null,
    log_index                     integer  not null,
    transaction_id                text     not null,
    liquidated_account_id         text     not null,
    insurance_account_id          numeric  not null,
    liquidated_asset_hash            text     not null,
    insurance_transfer_amount     numeric  not null,
    last_engine_event_id          numeric  not null,
    constraint liquidation_result_uq
        primary key(block_number, transaction_index, log_index)
);

create index liquidation_account_id_index
    on liquidation_result (liquidated_account_id);