-- Your SQL goes here
create table if not exists transaction_events
(
    event_id       bigint    not null,
    account_id     text      not null,
    sender         text,
    receiver       text      not null,
    transaction_id text      not null,
    token_hash     text      not null,
    broker_hash    text      not null,
    chain_id       numeric   not null,
    side           smallint  not null,
    amount         numeric   not null,
    fee            numeric   not null,
    status         smallint  not null,
    block_time     numeric   not null,
    withdraw_nonce  bigint,
    fail_reason    smallint,
    created_time   timestamp not null default current_timestamp,
    updated_time   timestamp not null default current_timestamp,
    constraint transaction_events_id primary key (event_id)
    );