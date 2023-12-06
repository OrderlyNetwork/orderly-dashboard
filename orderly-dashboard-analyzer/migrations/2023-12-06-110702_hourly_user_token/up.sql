-- Your SQL goes here
CREATE TABLE IF NOT EXISTS hourly_user_token
(
    id                     SERIAL PRIMARY KEY,
    account_id             text    not null,
    token                  text    not null,
    chain_hour             bigint  not null,
    chain_id               text    not null,
    token_address          text    not null,
    withdraw_amount        numeric not null,
    deposit_amount         numeric not null,
    pulled_block_height    numeric not null,
    pulled_block_timestamp numeric not null,
    created_time           timestamp with time zone,
    updated_time           timestamp with time zone
);

create index user_token_chain_hour_index on hourly_user_token (chain_hour);
