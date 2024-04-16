-- Your SQL goes here
create index if not exists transaction_events_account_index
    on transaction_events (account_id);

alter table executed_trades
    add block_time bigint not null default 0;

create index if not exists executed_trades_block_time_index
    on executed_trades (block_time);

create index if not exists serial_batches_block_time_index
    on serial_batches (block_time);