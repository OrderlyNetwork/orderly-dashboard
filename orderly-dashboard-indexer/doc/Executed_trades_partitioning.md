```text
                  Table "public.executed_trades"
        Column        |   Type   | Collation | Nullable | Default 
----------------------+----------+-----------+----------+---------
 block_number         | bigint   |           | not null | 
 transaction_index    | integer  |           | not null | 
 log_index            | integer  |           | not null | 
 typ                  | smallint |           | not null | 
 account_id           | text     |           | not null | 
 symbol_hash          | text     |           | not null | 
 fee_asset_hash       | text     |           | not null | 
 trade_qty            | numeric  |           | not null | 
 notional             | numeric  |           | not null | 
 executed_price       | numeric  |           | not null | 
 fee                  | numeric  |           | not null | 
 sum_unitary_fundings | numeric  |           | not null | 
 trade_id             | numeric  |           | not null | 
 match_id             | numeric  |           | not null | 
 timestamp            | numeric  |           | not null | 
 side                 | boolean  |           | not null | 
 block_time           | bigint   |           | not null | 0
Indexes:
    "executed_trades_uq" PRIMARY KEY, btree (block_number, transaction_index, log_index)
    "executed_trades_account_block_time_index" btree (account_id, block_time)
    "executed_trades_block_time_index" btree (block_time)
    "executed_trades_time_index" btree ("timestamp")
    "trade_id_channel" UNIQUE, btree (trade_id, typ)
    "trades_account_id_index" btree (account_id)
```
change to partitioning by block_time:
```
               Partitioned table "public.partitioned_executed_trades"
        Column        |            Type             | Collation | Nullable | Default 
----------------------+-----------------------------+-----------+----------+---------
 block_number         | bigint                      |           | not null | 
 transaction_index    | integer                     |           | not null | 
 log_index            | integer                     |           | not null | 
 typ                  | smallint                    |           | not null | 
 account_id           | text                        |           | not null | 
 symbol_hash          | text                        |           | not null | 
 fee_asset_hash       | text                        |           | not null | 
 trade_qty            | numeric                     |           | not null | 
 notional             | numeric                     |           | not null | 
 executed_price       | numeric                     |           | not null | 
 fee                  | numeric                     |           | not null | 
 sum_unitary_fundings | numeric                     |           | not null | 
 trade_id             | numeric                     |           | not null | 
 match_id             | numeric                     |           | not null | 
 timestamp            | numeric                     |           | not null | 
 side                 | boolean                     |           | not null | 
 block_time           | timestamp without time zone |           | not null | 
Partition key: RANGE (block_time)
Indexes:
    "partitioned_executed_trades_uq" PRIMARY KEY, btree (block_number, transaction_index, log_index, block_time)
    "partitioned_executed_trades_account_id_index" btree (account_id, block_time)
Number of partitions: 7 (Use \d+ to list them.)
```
