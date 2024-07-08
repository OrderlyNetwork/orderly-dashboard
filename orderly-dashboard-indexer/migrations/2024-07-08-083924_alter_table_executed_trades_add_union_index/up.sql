-- Your SQL goes here
create index if not exists executed_trades_account_block_time_index
    on executed_trades (account_id, block_time);