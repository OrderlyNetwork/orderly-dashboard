-- Your SQL goes here
alter table liquidation_transfer add column IF NOT EXISTS margin_mode smallint;
alter table liquidation_transfer add column IF NOT EXISTS iso_margin_asset_hash text;
alter table liquidation_transfer add column IF NOT EXISTS margin_to_cross numeric;
alter table liquidation_result add column IF NOT EXISTS is_insurance_account boolean;