-- Your SQL goes here
alter table serial_batches
    add effective_gas_price numeric default null;
alter table serial_batches
    add gas_used numeric default null;
alter table serial_batches
    add l1_fee numeric default null;
alter table serial_batches
    add l1_fee_scalar text default null;
alter table serial_batches
    add l1_gas_price numeric default null;
alter table serial_batches
    add l1_gas_used numeric default null;