-- Your SQL goes here
alter table partitioned_executed_trades add column IF NOT EXISTS margin_mode smallint;
alter table partitioned_executed_trades add column IF NOT EXISTS iso_margin_asset_hash text;
alter table partitioned_executed_trades add column IF NOT EXISTS margin_from_cross numeric;