-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_token_summary
(
    id                     SERIAL PRIMARY KEY,
    account_id             text    not null,
    token                  text    not null,
    chain_id               text    not null,
    token_address          text    not null,
    balance                numeric not null,

    total_withdraw_amount  numeric not null,
    total_deposit_amount   numeric not null,

    total_withdraw_count   numeric not null,
    total_deposit_count    numeric not null,

    pulled_block_height    numeric not null,
    pulled_block_timestamp numeric not null,
    created_time           numeric not null,
    updated_time           numeric not null
);

create index user_token_account_id_index on user_token_summary (account_id);

