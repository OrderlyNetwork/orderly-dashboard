create index if not exists transaction_events_account_id_index
    on transaction_events (account_id);

create index if not exists transaction_events_block_time_index
    on transaction_events (block_time);

create index if not exists executed_trades_time_index
    on executed_trades (timestamp);

create index if not exists settlement_result_block_time_index
    on settlement_result (block_time);

create index if not exists liquidation_result_block_time_index
    on liquidation_result (block_time);

create index if not exists adl_result_block_time_index
    on adl_result (block_time);
