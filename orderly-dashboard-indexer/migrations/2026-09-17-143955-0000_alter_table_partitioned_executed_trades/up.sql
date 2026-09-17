-- Your SQL goes here
alter table partitioned_executed_trades add column IF NOT EXISTS is_insurance_account boolean;
