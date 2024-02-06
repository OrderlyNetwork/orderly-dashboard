-- This file should undo anything in `up.sql`
alter table transaction_events drop column effective_gas_price;
alter table transaction_events drop column l1_fee;
alter table transaction_events drop column l1_fee_scalar;
alter table transaction_events drop column l1_gas_price;
alter table transaction_events drop column l1_gas_used;