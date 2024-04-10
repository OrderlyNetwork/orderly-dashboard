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
alter table block_summary add column metrics_type text not null default 'trade';

insert into block_summary(id,metrics_type,latest_block_height,pulled_block_height,
                          pulled_block_time,pulled_event_id,pulled_perp_trade_id)
values (2,'gas_fee',0,0,'1970-01-01 00:00:00.000000',0,0);
