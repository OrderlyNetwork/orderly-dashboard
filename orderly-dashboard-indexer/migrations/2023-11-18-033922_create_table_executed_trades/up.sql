-- Your SQL goes here
create table executed_trades
(
    block_number         bigint   not null,
    transaction_index    integer  not null,
    log_index            integer  not null,
    batch_id             numeric  not null,
    -- 1: perp trade
    typ                  smallint not null,
    account_id           text     not null,
    symbol_hash          text     not null,
    fee_asset_hash       text     not null,
    trade_qty            numeric  not null,
    notional             numeric  not null,
    executed_price       numeric  not null,
    fee                  numeric  not null,
    sum_unitary_fundings numeric  not null,
    trade_id             numeric  not null,
    match_id             numeric  not null,
    timestamp            numeric  not null,
    -- 1: Buy; 0 Sell
    side                 boolean  not null,
    constraint executed_trades_uq
        primary key(block_number, transaction_index, log_index)
);

create index trades_account_id_index
    on executed_trades (account_id);
