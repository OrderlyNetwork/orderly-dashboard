-- Your SQL goes here
alter table liquidation_transfer
    add block_time numeric default null;

create index if not exists liquidation_transfer_block_time_index
    on liquidation_transfer (block_time);