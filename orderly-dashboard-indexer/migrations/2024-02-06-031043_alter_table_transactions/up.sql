-- Your SQL goes here
alter table transaction_events
    add effective_gas_price numeric default null;
alter table transaction_events
    add gas_used numeric default null;
alter table transaction_events
    add l1_fee numeric default null;
alter table transaction_events
    add l1_fee_scalar numeric default null;
alter table transaction_events
    add l1_gas_price numeric default null;
alter table transaction_events
    add l1_gas_used numeric default null;