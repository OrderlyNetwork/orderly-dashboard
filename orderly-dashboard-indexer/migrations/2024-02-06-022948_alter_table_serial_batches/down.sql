-- This file should undo anything in `up.sql`
alter table serial_batches drop column effective_gas_price;
alter table serial_batches drop column l1_fee;
alter table serial_batches drop column l1_fee_scalar;
alter table serial_batches drop column l1_gas_price;
alter table serial_batches drop column l1_gas_used;
