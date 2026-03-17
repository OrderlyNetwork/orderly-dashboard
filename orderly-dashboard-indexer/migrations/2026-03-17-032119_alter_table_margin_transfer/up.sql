-- Your SQL goes here
create index if not exists margin_transfer_account_time_index
    on margin_transfer (account_id, block_time);

alter table margin_transfer drop column IF EXISTS timestamp;