-- Your SQL goes here
create table settlement_execution
(
    block_number                  bigint   not null,
    transaction_index             integer  not null,
    log_index                     integer  not null,
    settlement_result_log_idx     integer  not null,
    transaction_id                text     not null,
    symbol_hash                   text     not null,
    sum_unitary_fundings          numeric  not null,
    mark_price                    numeric  not null,
    settled_amount                numeric  not null,
    constraint settlement_execution_uq
        primary key(block_number, transaction_index, log_index)
);
