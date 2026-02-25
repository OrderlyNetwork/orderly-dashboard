-- Your SQL goes here
create index if not exists partitioned_executed_trades_broker_index on partitioned_executed_trades (broker_hash, block_time);