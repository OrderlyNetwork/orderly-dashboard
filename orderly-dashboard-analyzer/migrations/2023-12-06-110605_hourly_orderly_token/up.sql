-- Your SQL goes here
CREATE TABLE IF NOT EXISTS hourly_order_token
(
    id                     SERIAL PRIMARY KEY,
    token                  text  not null,
    chain_hour             bigint not null,
    chain_id               text not null,
    token_address          text not null,
    withdraw_amount        numeric not null,
    deposit_amount         numeric not null,
    pulled_block_height    numeric not null,
    pulled_block_timestamp numeric not null,
    created_time           timestamp with time zone not null,
    updated_time           timestamp with time zone not null
);

create index orderly_token_chain_hour_index on hourly_order_token (chain_hour);
