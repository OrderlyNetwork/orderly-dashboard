-- Your SQL goes here
alter table partitioned_executed_trades add column broker_hash text;
alter table partitioned_executed_trades add column transaction_id text;