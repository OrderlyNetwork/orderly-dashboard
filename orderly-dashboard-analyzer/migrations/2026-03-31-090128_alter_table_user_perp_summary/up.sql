-- Your SQL goes here
create index IF NOT EXISTS user_perp_summary_total_realized_pnl_index on user_perp_summary (total_realized_pnl);