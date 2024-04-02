-- Your SQL goes here
CREATE TABLE IF NOT EXISTS hourly_gas_fee
(
    block_hour          timestamp  not null,

    gas_fee         numeric(32,18) not null,
    event_type      text not null,

    batch_count       bigint  not null,
    pulled_block_height bigint  not null,
    pulled_block_time   timestamp  not null,
    constraint hourly_block_type primary key (event_type, block_hour)
 );

create index orderly_gas_fee_index on hourly_gas_fee (block_hour);
-- Your SQL goes here
