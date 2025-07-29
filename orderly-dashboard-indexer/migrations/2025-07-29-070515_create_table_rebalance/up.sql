-- Your SQL goes here
create table if not exists rebalance_events
(
    rebalance_id            bigint                not null,
    token_hash              text                  not null,
    amount                  numeric               not null,
    src_chain_id            numeric               not null,
    dst_chain_id            numeric               not null,
    burn_tx_id              text                  not null,
    burn_result_tx_id       text,
    mint_tx_id              text,
    mint_result_tx_id       text,
    burn_result_block_time  bigint,
    mint_result_block_time  bigint,
    burn_result_block_number       bigint,
    mint_result_block_number       bigint,
    burn_success            boolean,
    mint_success            boolean,
    constraint rebalance_events_pk
    primary key(rebalance_id)
);