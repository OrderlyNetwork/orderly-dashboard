-- Your SQL goes here
create table if not exists fee_distribution
(
    block_number                  bigint   not null,
    transaction_index             integer  not null,
    log_index                     integer  not null,
    transaction_id                text     not null,
    block_time                    numeric  not null,
    event_id                      numeric  not null,
    from_account_id               text     not null,
    to_account_id                 text     not null,
    amount                        numeric  not null,
    token_hash                    text     not null,
    constraint fee_distribution_uq
        primary key(block_number, transaction_index, log_index)
);