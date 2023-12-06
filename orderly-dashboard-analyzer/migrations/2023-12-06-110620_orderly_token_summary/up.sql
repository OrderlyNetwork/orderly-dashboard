-- Your SQL goes here
CREATE TABLE IF NOT EXISTS orderly_token_summary
(
    id                     SERIAL PRIMARY KEY,
    token                  text not null,
    chain_id               text not null,
    token_address          text not null,
    balance                numeric not null,
    total_withdraw_amount  numeric not null,
    total_deposit_amount   numeric not null,
    pulled_block_height    numeric not null,
    pulled_block_timestamp timestamp with time zone not null,
    created_time           timestamp with time zone not null,
    updated_time           timestamp with time zone not null
);