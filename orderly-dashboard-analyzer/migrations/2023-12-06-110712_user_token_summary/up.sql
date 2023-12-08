-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_token_summary
(
    account_id            text    not null,
    token                 text    not null,

    chain_id              text    not null,
    balance               numeric not null,

    total_withdraw_amount numeric not null,
    total_deposit_amount  numeric not null,

    total_withdraw_count  bigint  not null,
    total_deposit_count   bigint  not null,

    pulled_block_height   bigint  not null,
    pulled_block_time     bigint  not null,

    constraint user_token_summary_uq primary key (account_id, token, chain_id)
);

create index user_token_account_id_index on user_token_summary (account_id);

