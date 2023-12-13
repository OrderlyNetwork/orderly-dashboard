-- Your SQL goes here
alter table orderly_perp_summary add column buy_amount numeric not null;
alter table orderly_perp_summary add column sell_amount numeric not null;