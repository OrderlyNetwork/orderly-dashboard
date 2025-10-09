-- Your SQL goes here
alter table user_volume_statistics add column IF NOT EXISTS perp_volume_last_90_days numeric not null default 0;