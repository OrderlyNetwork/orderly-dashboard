-- Your SQL goes here
create table liquidation_transfer
(
    block_number                  bigint   not null,
    transaction_index             integer  not null,
    log_index                     integer  not null,
    transaction_id                text     not null,
    liquidation_transfer_id       numeric  not null,
    liquidator_account_id         text     not null,
    symbol_hash                   text     not null,
    position_qty_transfer         numeric  not null,
    cost_position_transfer        numeric  not null,
    liquidator_fee                numeric  not null,
    insurance_fee                 numeric  not null,
    mark_price                    numeric  not null,
    sum_unitary_fundings          numeric  not null,
    liquidation_fee               numeric  not null,
    constraint liquidation_transfer_uq
        primary key(block_number, transaction_index, log_index)
);