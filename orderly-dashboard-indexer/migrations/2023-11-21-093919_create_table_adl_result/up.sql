-- Your SQL goes here
create table adl_result
(
    block_number                  bigint   not null,
    transaction_index             integer  not null,
    log_index                     integer  not null,
    account_id                    text     not null,
    insurance_account_id          numeric  not null,
    symbol_hash                   text     not null,
    position_qty_transfer         numeric  not null,
    cost_position_transfer        numeric  not null,
    adl_price                     numeric  not null,
    sum_unitary_fundings          numeric  not null,
    constraint adl_result_uq
        primary key(block_number, transaction_index, log_index)
);

create index adl_account_id_index
    on adl_result (account_id);