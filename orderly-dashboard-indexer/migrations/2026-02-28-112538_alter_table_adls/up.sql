-- Your SQL goes here
alter table adl_result add column IF NOT EXISTS margin_mode smallint;
alter table adl_result add column IF NOT EXISTS iso_margin_asset_hash text;
alter table adl_result add column IF NOT EXISTS margin_to_cross numeric;