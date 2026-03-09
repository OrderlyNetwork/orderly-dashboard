-- Your SQL goes here
alter table adl_result add column margin_mode smallint;
alter table adl_result add column iso_margin_asset_hash text;
alter table adl_result add column margin_to_cross numeric;
alter table adl_result add column is_insurance_account boolean;