-- Your SQL goes here
create table if not exists sol_transaction_events
(
    block_number                  bigint   not null,
    transaction_index             integer  not null,
    log_index                     integer  not null,
    transaction_id                text     not null,
    block_time                    numeric  not null,
    account_id                    text      not null,
    sender                        text,
    receiver                      text      not null,
    token_hash                    text      not null,
    broker_hash                   text      not null,
    chain_id                      numeric   not null,
    side                          smallint  not null,
    amount                        numeric   not null,
    fee                           numeric   not null,
    status                        smallint  not null,
    withdraw_nonce                bigint,
    fail_reason                   smallint,
    constraint sol_transaction_events_id primary key (block_number,transaction_index,log_index)
);

create index if not exists sol_transaction_events_account_id_index
    on sol_transaction_events (account_id);

create index if not exists sol_transaction_events_block_time_index
    on sol_transaction_events (block_time);