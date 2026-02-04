-- Your SQL goes here
alter table partitioned_executed_trades add column margin_mode smallint;
alter table partitioned_executed_trades add column iso_margin_asset_hash text;
alter table partitioned_executed_trades add column margin_from_cross numeric;