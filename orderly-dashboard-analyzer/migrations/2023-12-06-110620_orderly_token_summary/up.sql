-- Your SQL goes here
CREATE TABLE IF NOT EXISTS orderly_token_summary
(
    token                 text    not null,
    chain_id              text    not null,
    balance               numeric not null,

    total_withdraw_amount numeric not null,
    total_withdraw_count  bigint  not null,

    total_deposit_amount  numeric not null,
    total_deposit_count   bigint  not null,

    pulled_block_height   bigint  not null,
    pulled_block_time     timestamp  not null,

    constraint orderly_token_summary_uq primary key (token, chain_id)
);