-- This file should undo anything in `up.sql`
drop index if exists transaction_events_account_id_index;

drop index if exists transaction_events_block_time_index;

drop index if exists executed_trades_time_index;

drop index if exists settlement_result_block_time_index;

drop index if exists liquidation_result_block_time_index;

drop index if exists adl_result_block_time_index;