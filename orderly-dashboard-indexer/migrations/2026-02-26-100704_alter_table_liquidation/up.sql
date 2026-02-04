-- Your SQL goes here
alter table liquidation_transfer add column margin_mode smallint;
alter table liquidation_transfer add column iso_margin_asset_hash text;
alter table liquidation_transfer add column margin_to_cross numeric;
alter table liquidation_result add column is_insurance_account boolean;