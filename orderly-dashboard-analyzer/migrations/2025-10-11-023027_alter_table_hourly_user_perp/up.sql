-- Your SQL goes here
create index IF NOT EXISTS user_perp_account_hour_index on hourly_user_perp (account_id, block_hour);