-- Your SQL goes here
create index if not exists liquidation_transfer_account_id_time_index
    on liquidation_transfer (liquidator_account_id, block_time);