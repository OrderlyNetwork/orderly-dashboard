-- Your SQL goes here
create table IF NOT EXISTS partitioned_executed_trades
(
    block_number         bigint    not null,
    transaction_index    integer   not null,
    log_index            integer   not null,
    -- 1: perp trade
    typ                  smallint  not null,
    account_id           text      not null,
    symbol_hash          text      not null,
    fee_asset_hash       text      not null,
    trade_qty            numeric   not null,
    notional             numeric   not null,
    executed_price       numeric   not null,
    fee                  numeric   not null,
    sum_unitary_fundings numeric   not null,
    trade_id             numeric   not null,
    match_id             numeric   not null,
    timestamp            numeric   not null,
    -- 1: Buy; 0 Sell
    side                 boolean   not null,
    block_time           TIMESTAMP not null,
    constraint partitioned_executed_trades_uq
        primary key(block_number, transaction_index, log_index, block_time)
) PARTITION BY RANGE (block_time);

create index IF NOT EXISTS partitioned_executed_trades_account_id_index
    on partitioned_executed_trades (account_id, block_time);

CREATE TABLE IF NOT EXISTS executed_trades_before_y2024 PARTITION OF partitioned_executed_trades
    FOR VALUES FROM ('2023-01-01 00:00:00') TO ('2024-01-01 00:00:00');

CREATE TABLE IF NOT EXISTS executed_trades_y2024q01 PARTITION OF partitioned_executed_trades
    FOR VALUES FROM ('2024-01-01 00:00:00') TO ('2024-04-01 00:00:00');

CREATE TABLE IF NOT EXISTS executed_trades_y2024q02 PARTITION OF partitioned_executed_trades
    FOR VALUES FROM ('2024-04-01 00:00:00') TO ('2024-07-01 00:00:00');

CREATE TABLE IF NOT EXISTS executed_trades_y2024q03 PARTITION OF partitioned_executed_trades
    FOR VALUES FROM ('2024-07-01 00:00:00') TO ('2024-10-01 00:00:00');

CREATE TABLE IF NOT EXISTS executed_trades_y2024q04 PARTITION OF partitioned_executed_trades
    FOR VALUES FROM ('2024-10-01 00:00:00') TO ('2025-01-01 00:00:00');

CREATE TABLE IF NOT EXISTS executed_trades_y2025q01 PARTITION OF partitioned_executed_trades
    FOR VALUES FROM ('2025-01-01 00:00:00') TO ('2025-04-01 00:00:00');

CREATE TABLE IF NOT EXISTS executed_trades_y2025q02 PARTITION OF partitioned_executed_trades
    FOR VALUES FROM ('2025-04-01 00:00:00') TO ('2025-07-01 00:00:00');