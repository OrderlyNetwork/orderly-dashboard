-- Your SQL goes here
create index if not exists partitioned_executed_trades_block_time_cursor_idx on partitioned_executed_trades (block_time, block_number, transaction_index, log_index);
create index if not exists partitioned_executed_trades_broker_cursor_idx on partitioned_executed_trades (broker_hash, block_time, block_number, transaction_index, log_index);
create index if not exists partitioned_executed_trades_account_cursor_idx on partitioned_executed_trades (account_id, block_time, block_number, transaction_index, log_index);
create index if not exists partitioned_executed_trades_address_cursor_idx on partitioned_executed_trades (address, block_time, block_number, transaction_index, log_index);
create index if not exists partitioned_executed_trades_symbol_hash_cursor_idx on partitioned_executed_trades (symbol_hash, block_time, block_number, transaction_index, log_index);

DROP INDEX IF EXISTS partitioned_executed_trades_account_id_index;
DROP INDEX IF EXISTS partitioned_executed_trades_broker_index;