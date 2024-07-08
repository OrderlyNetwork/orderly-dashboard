-- Your SQL goes here
create index if not exists settlement_result_account_block_time_index
    on settlement_result (account_id, block_time);

create index if not exists settlement_result_block_number_log_index_index
    on settlement_result (block_number, log_index);

create index if not exists settlement_execution_block_number_settlement_result_log_idx
    on settlement_execution (block_number, settlement_result_log_idx);