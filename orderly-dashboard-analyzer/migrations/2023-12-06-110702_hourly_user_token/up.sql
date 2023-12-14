-- Your SQL goes here
CREATE TABLE IF NOT EXISTS hourly_user_token
(
    account_id          text    not null,
    token               text    not null,
    block_hour          timestamp  not null,
    chain_id            text    not null,

    withdraw_amount     numeric not null,
    withdraw_count      bigint  not null,
    deposit_amount      numeric not null,
    deposit_count       bigint  not null,

    pulled_block_height bigint  not null,
    pulled_block_time   timestamp  not null,

    constraint hourly_user_token_uq primary key (account_id, block_hour, token, chain_id)
);

create index user_token_chain_hour_index on hourly_user_token (block_hour);
