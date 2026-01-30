-- Your SQL goes here
alter table settlement_result add column IF NOT EXISTS version smallint;

alter table settlement_execution add column IF NOT EXISTS version smallint;
alter table settlement_execution add column IF NOT EXISTS margin_mode smallint;
alter table settlement_execution add column IF NOT EXISTS iso_margin_asset_hash text;