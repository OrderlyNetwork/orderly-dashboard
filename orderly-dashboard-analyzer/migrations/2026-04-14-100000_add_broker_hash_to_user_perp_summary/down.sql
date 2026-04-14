drop index if exists idx_ups_nonzero_symbol_account_cover;

alter table user_perp_summary
drop column if exists broker_hash;
