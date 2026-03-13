# create mock data and consider to check how to add index for partitioned_executed_trades
## Write a test function to insert data into partitioned_executed_trades
Insert approximately 100 million to 120 million rows into partitioned_executed_trades, ensuring that each partition from executed_trades_before_y2024 through executed_trades_y2026q01 has over 10 million rows; no need to insert data into executed_trades_y2026q02 and executed_trades_y2026q03.
Table structure:
```
                                           Partitioned table "public.partitioned_executed_trades"
        Column         |            Type             | Collation | Nullable | Default | Storage  | Compression | Stats target | Description 
-----------------------+-----------------------------+-----------+----------+---------+----------+-------------+--------------+-------------
 block_number          | bigint                      |           | not null |         | plain    |             |              | 
 transaction_index     | integer                     |           | not null |         | plain    |             |              | 
 log_index             | integer                     |           | not null |         | plain    |             |              | 
 typ                   | smallint                    |           | not null |         | plain    |             |              | 
 account_id            | text                        |           | not null |         | extended |             |              | 
 symbol_hash           | text                        |           | not null |         | extended |             |              | 
 fee_asset_hash        | text                        |           | not null |         | extended |             |              | 
 trade_qty             | numeric                     |           | not null |         | main     |             |              | 
 notional              | numeric                     |           | not null |         | main     |             |              | 
 executed_price        | numeric                     |           | not null |         | main     |             |              | 
 fee                   | numeric                     |           | not null |         | main     |             |              | 
 sum_unitary_fundings  | numeric                     |           | not null |         | main     |             |              | 
 trade_id              | numeric                     |           | not null |         | main     |             |              | 
 match_id              | numeric                     |           | not null |         | main     |             |              | 
 timestamp             | numeric                     |           | not null |         | main     |             |              | 
 side                  | boolean                     |           | not null |         | plain    |             |              | 
 block_time            | timestamp without time zone |           | not null |         | plain    |             |              | 
 broker_hash           | text                        |           |          |         | extended |             |              | 
 transaction_id        | text                        |           |          |         | extended |             |              | 
 margin_mode           | smallint                    |           |          |         | plain    |             |              | 
 iso_margin_asset_hash | text                        |           |          |         | extended |             |              | 
 margin_from_cross     | numeric                     |           |          |         | main     |             |              | 
 address               | text                        |           |          |         | extended |             |              | 
Partition key: RANGE (block_time)
Indexes:
    "partitioned_executed_trades_uq" PRIMARY KEY, btree (block_number, transaction_index, log_index, block_time)
    "partitioned_executed_trades_account_id_index" btree (account_id, block_time)
    "partitioned_executed_trades_broker_index" btree (broker_hash, block_time)
Partitions: executed_trades_before_y2024 FOR VALUES FROM ('2023-01-01 00:00:00') TO ('2024-01-01 00:00:00'),
            executed_trades_y2024q01 FOR VALUES FROM ('2024-01-01 00:00:00') TO ('2024-04-01 00:00:00'),
            executed_trades_y2024q02 FOR VALUES FROM ('2024-04-01 00:00:00') TO ('2024-07-01 00:00:00'),
            executed_trades_y2024q03 FOR VALUES FROM ('2024-07-01 00:00:00') TO ('2024-10-01 00:00:00'),
            executed_trades_y2024q04 FOR VALUES FROM ('2024-10-01 00:00:00') TO ('2025-01-01 00:00:00'),
            executed_trades_y2025q01 FOR VALUES FROM ('2025-01-01 00:00:00') TO ('2025-04-01 00:00:00'),
            executed_trades_y2025q02 FOR VALUES FROM ('2025-04-01 00:00:00') TO ('2025-07-01 00:00:00'),
            executed_trades_y2025q03 FOR VALUES FROM ('2025-07-01 00:00:00') TO ('2025-10-01 00:00:00'),
            executed_trades_y2025q04 FOR VALUES FROM ('2025-10-01 00:00:00') TO ('2026-01-01 00:00:00'),
            executed_trades_y2026q01 FOR VALUES FROM ('2026-01-01 00:00:00') TO ('2026-04-01 00:00:00'),
            executed_trades_y2026q02 FOR VALUES FROM ('2026-04-01 00:00:00') TO ('2026-07-01 00:00:00'),
            executed_trades_y2026q03 FOR VALUES FROM ('2026-07-01 00:00:00') TO ('2026-10-01 00:00:00')
```

Additional requirements:
- Data inserted by each test run must be identical; do not use random data
- block_number ranges from 1 to 3831371, transaction_index from 0 to 10, log_index from 0 to 1000
- 10 brokers; broker_hash is derived from broker mapping; 20 addresses; 100 account_ids; account_id is derived from (broker_hash, address) mapping
- Insert an extra 2 million rows for the period '2024-07-22 00:00:00' ~ '2024-07-22 23:59:59'
- Keep other fields (except block_number, transaction_index, log_index, account_id, broker_hash, address) as small as possible to reduce storage

## Test function implementation location
Implement in `orderly-dashboard-indexer/src/db/partitioned_executed_trades.rs`

## Index optimization suggestions by query condition (no code tests or implementation required; provide rationale)
Known: query limit is 400 rows.
Known: results must be ordered by block_time, block_number, transaction_index, log_index to support pagination.

1. Query with time range only
2. Query with broker_hash and time range
3. Query with account_id and time range
4. Query with address and time range
5. Query with symbol_hash and time range

How to optimize indexes for the above 5 query conditions to improve pagination query speed

---

## Index optimization suggestions (pagination limit 400, sort: block_time, block_number, transaction_index, log_index)

### 1. Query with time range only

**Query shape**: `WHERE block_time BETWEEN ? AND ? ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400` (and cursor-based pagination).

**Recommendation**:  
- **No new index required**, or add a "sort" index on **each partition** only when needed.  
- **Rationale**:  
  - Partition key is already `block_time` (RANGE); time-range queries do **partition pruning** and only scan relevant partitions.  
  - Primary key is `(block_number, transaction_index, log_index, block_time)`; within a partition, ordering by `(block_time, block_number, transaction_index, log_index)` does not fully match the primary key order, but time range usually benefits from primary key prefix or sequential I/O.  
  - With time range only and ~10M rows per partition, `ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400` within a partition is acceptable; if still slow, consider a btree index `(block_time, block_number, transaction_index, log_index)` on each partition to sort by time then by primary key components and apply limit, reducing sort cost.

Test SQL 1
```sql
select block_number,transaction_index,log_index,block_time,account_id,broker_hash,address from partitioned_executed_trades where block_time >= '2024-06-28 19:57:39' and block_time < '2024-06-29 19:57:39' order by block_time,block_number,transaction_index,log_index limit 400;
```

Execution plan before adding index
```
explain select block_number,transaction_index,log_index,block_time,account_id,broker_hash,address from partitioned_executed_trades where block_time >= '2024-06-28 19:57:39' and block_time < '2024-06-29 19:57:39' order by block_time,block_number,transaction_index,log_index limit 100;
                                                                                        QUERY PLAN                                                                                        
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
 Limit  (cost=245796.09..245796.09 rows=1 width=50)
   ->  Sort  (cost=245796.09..245796.09 rows=1 width=50)
         Sort Key: partitioned_executed_trades.block_time, partitioned_executed_trades.block_number, partitioned_executed_trades.transaction_index, partitioned_executed_trades.log_index
         ->  Gather  (cost=1000.00..245796.08 rows=1 width=50)
               Workers Planned: 2
               ->  Parallel Seq Scan on executed_trades_y2024q02 partitioned_executed_trades  (cost=0.00..244795.98 rows=1 width=50)
                     Filter: ((block_time >= '2024-06-28 19:57:39'::timestamp without time zone) AND (block_time < '2024-06-29 19:57:39'::timestamp without time zone))
(7 rows)
```

Test SQL 2
```sql
select block_number,transaction_index,log_index,block_time,account_id,broker_hash,address from partitioned_executed_trades where block_time >= '2024-06-28 19:57:39' and block_time < '2024-06-29 19:57:39'  order by block_time,block_number,transaction_index,log_index limit 400;
```


Recommended index to add

```sql
create index if not exists partitioned_executed_trades_block_time_cursor_idx on partitioned_executed_trades (block_time, block_number,transaction_index,log_index);
```

Execution plan after adding index
```
explain select block_number,transaction_index,log_index,block_time,account_id,broker_hash,address from partitioned_executed_trades where block_time >= '2024-06-28 19:57:39' and block_time < '2024-06-29 19:57:39' order by block_time,block_number,transaction_index,log_index limit 100;
                                                                                        QUERY PLAN                                                                                         
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
 Limit  (cost=0.56..169.49 rows=100 width=50)
   ->  Index Scan using executed_trades_y2024q02_block_time_block_number_transactio_idx on executed_trades_y2024q02 partitioned_executed_trades  (cost=0.56..41257.06 rows=24422 width=50)
         Index Cond: ((block_time >= '2024-06-28 19:57:39'::timestamp without time zone) AND (block_time < '2024-06-29 19:57:39'::timestamp without time zone))
(3 rows)
```



---

### 2. Query with broker_hash + time range

**Query shape**: `WHERE broker_hash = ? AND block_time BETWEEN ? AND ? ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400`.

**Recommendation**:  
- **Keep or standardize on existing** `partitioned_executed_trades_broker_index`: `btree (broker_hash, block_time)`.  
- **Optional optimization**: If pagination cursor is `(block_time, block_number, transaction_index, log_index)` and you want the index to fully cover sort and limit, add on **each partition**:  
  `(broker_hash, block_time, block_number, transaction_index, log_index)`.  
- **Rationale**:  
  - `(broker_hash, block_time)` efficiently filters by broker and uses partition pruning (time range).  
  - Sort columns are `block_time, block_number, transaction_index, log_index`; including `block_number, transaction_index, log_index` in the index enables **index-only or minimal heap fetches** for sorting and avoids large sorts; using the last row’s `(block_time, block_number, transaction_index, log_index)` as cursor for the next page also uses the same index.

---

Recommended index to add
```sql
create index if not exists partitioned_executed_trades_broker_cursor_idx on partitioned_executed_trades (broker_hash,block_time, block_number,transaction_index,log_index);
```

### 3. Query with account_id + time range

**Query shape**: `WHERE account_id = ? AND block_time BETWEEN ? AND ? ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400`.

**Recommendation**:  
- **Keep existing** `partitioned_executed_trades_account_id_index`: `btree (account_id, block_time)`.  
- **Optional optimization**: If there is still heavy sort or heap fetches, add on **each partition**:  
  `(account_id, block_time, block_number, transaction_index, log_index)`.  
- **Rationale**:  
  - `(account_id, block_time)` already quickly narrows to "user + time range" and works with partition key for partition pruning.  
  - Adding `block_number, transaction_index, log_index` lets sort and limit 400 be done on the index; cursor pagination is similar to case 2, for stable latency.


Recommended index to add
```sql
create index if not exists partitioned_executed_trades_account_cursor_idx on partitioned_executed_trades (account_id,block_time, block_number,transaction_index,log_index);
```

---

### 4. Query with address + time range

**Query shape**: `WHERE address = ? AND block_time BETWEEN ? AND ? ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400`.

**Recommendation**:  
- **Add index**: Create a btree index `(address, block_time)` on the partitioned table (if not already present).  
- **Optional optimization**: Same as 2 and 3; for more stable pagination performance, add on each partition:  
  `(address, block_time, block_number, transaction_index, log_index)`.  
- **Rationale**:  
  - Current indexes in the doc are only primary key, `account_id`, and `broker_hash`; there is no `address` index. Filtering by address becomes a full partition scan, which is expensive at ~10M rows per partition.  
  - With `(address, block_time)` you can narrow by address and time first, then sort and limit on the index (or an extended index including sort columns), avoiding large sorts and random heap fetches.

Recommended index to add
```sql
create index if not exists partitioned_executed_trades_address_cursor_idx on partitioned_executed_trades (address,block_time, block_number,transaction_index,log_index);
```

---

### 5. Query with symbol_hash + time range

**Query shape**: `WHERE symbol_hash = ? AND block_time BETWEEN ? AND ? ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400`.

**Recommendation**:  
- **Add index**: Create a btree index `(symbol_hash, block_time)` on the partitioned table (currently only primary key, `account_id`, and `broker_hash` exist; no `symbol_hash` index).  
- **Optional optimization**: For more stable pagination performance, add on each partition:  
  `(symbol_hash, block_time, block_number, transaction_index, log_index)`.  
- **Rationale**:  
  - Filtering by symbol_hash without a matching index causes a full partition scan, which is expensive at ~10M rows per partition.  
  - `(symbol_hash, block_time)` narrows by symbol and time, and works with the partition key for partition pruning.  
  - Including `block_number, transaction_index, log_index` in the index allows sort, limit 400, and cursor pagination to be done on the index, avoiding large sorts and random heap fetches.

Recommended index to add
```sql
create index if not exists partitioned_executed_trades_symbol_hash_cursor_idx on partitioned_executed_trades (symbol_hash, block_time, block_number, transaction_index, log_index);
```

---

### Summary

| Query condition | Recommendation |
|-----------------|----------------|
| Time range only | Rely on partition pruning + primary key; optionally add per-partition `(block_time, block_number, transaction_index, log_index)` |
| broker_hash + time | Use existing `(broker_hash, block_time)`; optionally per-partition `(broker_hash, block_time, block_number, transaction_index, log_index)` |
| account_id + time | Use existing `(account_id, block_time)`; optionally per-partition `(account_id, block_time, block_number, transaction_index, log_index)` |
| address + time | Add `(address, block_time)`; optionally per-partition `(address, block_time, block_number, transaction_index, log_index)` |
| symbol_hash + time | Add `(symbol_hash, block_time)`; optionally per-partition `(symbol_hash, block_time, block_number, transaction_index, log_index)` |

**General rationale**: Pagination needs stable, reproducible ordering (doc requires `block_time, block_number, transaction_index, log_index`). Including these sort columns in the index avoids filesort on large result sets and lets `LIMIT 400` and cursor-based next-page queries scan only a small number of index pages, improving pagination speed.

**Composite vs redundant indexes**: For conditions on broker_hash, account_id, address, symbol_hash, if you already have a long index "filter column + block_time + cursor columns" (e.g. `(symbol_hash, block_time, block_number, transaction_index, log_index)`), you do **not** need a separate short index "filter column + block_time only" (e.g. `(symbol_hash, block_time)`). PostgreSQL B-tree composite indexes follow left-prefix rules; the long index already covers filter and sort on the short prefix. A separate short index only adds storage and write cost with no query benefit.

# Query statement construction
If the index has been created with the following SQL:
 ```sql
create index if not exists partitioned_executed_trades_block_time_cursor_idx on partitioned_executed_trades (block_time, block_number,transaction_index,log_index);
```

The first-page pagination query is:
```sql
select block_number,transaction_index,log_index,block_time,account_id,broker_hash,address from partitioned_executed_trades where block_time >= '2024-06-28 19:57:39' and block_time < '2024-06-29 19:57:39'  order by block_time,block_number,transaction_index,log_index limit 400;
```
The last row of the first page is used as the filter for the next page; i.e. the next page should return rows starting after that last row. How to write the query so the SQL is efficient?

### Approach (cursor pagination / keyset pagination)

- Do not use `OFFSET`: `OFFSET 400`, `OFFSET 800`, etc. on large tables force the database to scan and discard all preceding rows; later pages get slower.
- Use the sort key of the "last row of the previous page" as the cursor: form the cursor from the four columns `(block_time, block_number, transaction_index, log_index)`; the next page condition is "sort key strictly greater than cursor", so the scan can start from after the cursor on the index and stop after 400 rows.
- The index `(block_time, block_number, transaction_index, log_index)` supports both time-range filtering and "order by this quadruple" and "greater than this quadruple" for the cursor; the next-page query can use the same index and avoid large sorts and random heap fetches.

### First page (unchanged)

```sql
SELECT block_number, transaction_index, log_index, block_time, account_id, broker_hash, address
FROM partitioned_executed_trades
WHERE block_time >= '2024-06-28 19:57:39'
  AND block_time < '2024-06-29 19:57:39'
ORDER BY block_time, block_number, transaction_index, log_index
LIMIT 400;
```

### Next page (cursor pagination)

Let the four fields of the last row of the previous page be: `last_block_time`, `last_block_number`, `last_transaction_index`, `last_log_index`.

**Option 1 (row comparison, recommended, index-friendly):**

```sql
SELECT block_number, transaction_index, log_index, block_time, account_id, broker_hash, address
FROM partitioned_executed_trades
WHERE block_time >= '2024-06-28 19:57:39'
  AND block_time < '2024-06-29 19:57:39'
  AND (block_time, block_number, transaction_index, log_index) > (
    :last_block_time, :last_block_number, :last_transaction_index, :last_log_index
  )
ORDER BY block_time, block_number, transaction_index, log_index
LIMIT 400;
```

(Replace `:last_*` with actual parameter placeholders, e.g. `$1,$2,$3,$4` or `?`.)

Example:
```sql
SELECT block_number, transaction_index, log_index, block_time, account_id, broker_hash, address
FROM partitioned_executed_trades
WHERE block_time >= '2024-06-28 19:57:39'
  AND block_time < '2024-06-29 19:57:39'
  AND (block_time, block_number, transaction_index, log_index) > (
    '2024-06-28 20:02:46', 2758, 0, 32
  )
ORDER BY block_time, block_number, transaction_index, log_index
LIMIT 400;
```

explain:
```sql
explain SELECT block_number, transaction_index, log_index, block_time, account_id, broker_hash, address
FROM partitioned_executed_trades
WHERE block_time >= '2024-06-28 19:57:39'
  AND block_time < '2024-06-29 19:57:39'
  AND (block_time, block_number, transaction_index, log_index) > (
    '2024-06-28 20:02:46', 2758, 0, 32
  )
ORDER BY block_time, block_number, transaction_index, log_index
LIMIT 400;
```

explain result:
```
                                                                                                                                               QUERY PLAN                                                                                                                                                
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
 Limit  (cost=0.56..233.04 rows=127 width=50)
   ->  Index Scan using executed_trades_y2024q02_block_time_block_number_transactio_idx on executed_trades_y2024q02 partitioned_executed_trades  (cost=0.56..233.04 rows=127 width=50)
         Index Cond: ((block_time >= '2024-06-28 19:57:39'::timestamp without time zone) AND (block_time < '2024-06-29 19:57:39'::timestamp without time zone) AND (ROW(block_time, block_number, transaction_index, log_index) > ROW('2024-06-28 20:02:46'::timestamp without time zone, 2758, 0, 32)))
(3 rows)
```

**Option 2 (equivalent expansion, for clients that do not support row comparison):**

```sql
SELECT block_number, transaction_index, log_index, block_time, account_id, broker_hash, address
FROM partitioned_executed_trades
WHERE block_time >= '2024-06-28 19:57:39'
  AND block_time < '2024-06-29 19:57:39'
  AND (
    block_time > :last_block_time
    OR (block_time = :last_block_time AND block_number > :last_block_number)
    OR (block_time = :last_block_time AND block_number = :last_block_number AND transaction_index > :last_transaction_index)
    OR (block_time = :last_block_time AND block_number = :last_block_number AND transaction_index = :last_transaction_index AND log_index > :last_log_index)
  )
ORDER BY block_time, block_number, transaction_index, log_index
LIMIT 400;
```

### Why this is more efficient

- The condition `(block_time, block_number, transaction_index, log_index) > (last_...)` matches the `ORDER BY`; the index can start scanning from "after the cursor" and return the next 400 rows without a large sort and without relying on `OFFSET`, so pagination latency is stable.

