-- Your SQL goes here
alter table settlement_execution
    add block_time numeric default null;

create index if not exists settlement_execution_block_time_index
    on settlement_execution (block_time);