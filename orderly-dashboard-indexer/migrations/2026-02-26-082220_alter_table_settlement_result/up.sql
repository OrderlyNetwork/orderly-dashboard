-- Your SQL goes here
alter table settlement_result add column version smallint;

alter table settlement_execution add column version smallint;
alter table settlement_execution add column margin_mode smallint;
alter table settlement_execution add column iso_margin_asset_hash text;