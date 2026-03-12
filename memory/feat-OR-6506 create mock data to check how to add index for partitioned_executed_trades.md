# create mock data and consider to check how to add index for partitioned_executed_trades
## 写一个测试函数往partitioned_executed_trades表中添加数据
往partitioned_executed_trades表中添加约1亿~1.2亿条数据，确保分区表executed_trades_before_y2024到executed_trades_y2026q01中的数据都有都超过1千万条，executed_trades_y2026q02和executed_trades_y2026q03表不需要插入数据
表结构为：
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

还有以下额外要求：
- 每次运行测试函数插入的数据必须相同，不能随机生成数据
- block_number的范围从1到3831371，transaction_index的范围0到10，log_index的范围0到1000
- broker有10个，broker_hash由broker映射生成，address有20个，account_id有100个，account_id是(broker_hash, address)映射生成生成的
- '2024-07-22 00:00:00' ~ '2024-07-22 23:59:59'这段时间额外插入200万条数据
- 除block_number、transaction_index、log_index、account_id、broker_hash、address外的其他字段尽量小一点，减少存储空间

## 测试函数实现位置  
写在`orderly-dashboard-indexer/src/db/partitioned_executed_trades.rs`中

## 后续根据查询条件分别提出索引优化建议，不需要代码测试和代码实现，要给出原因
已知查询返回的limit是400条
已知查询结果需要根据block_time、block_number、transaction_index、log_index排序确保能够分页查询

1. 只传入时间范围的查询 
2. 传入broker_hash、时间范围的查询  
3. 传入account_id、时间范围的查询 
4. 传入address、时间范围的查询 
5. 传入symbol_hash、时间范围的查询 

以上5种查询条件改怎么优化索引来提高分页查询的速度

---

## 索引优化建议（分页 limit 400，排序：block_time, block_number, transaction_index, log_index）

### 1. 只传入时间范围的查询

**查询形态**：`WHERE block_time BETWEEN ? AND ? ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400`（及基于游标的分页）。

**建议**：  
- **无需新增索引**，或仅在需要时在**每个分区**上建「排序用」索引。  
- **原因**：  
  - 分区键已是 `block_time`（RANGE），按时间范围查询会做 **partition pruning**，只扫相关分区。  
  - 主键为 `(block_number, transaction_index, log_index, block_time)`，在单分区内按 `(block_time, block_number, transaction_index, log_index)` 排序时，顺序与主键不完全一致，但时间范围通常能利用到主键前缀或顺序 I/O。  
  - 若只按时间范围且每分区数据量在千万级，在分区内做 `ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400` 成本尚可；若仍慢，再考虑在每个分区上建 `(block_time, block_number, transaction_index, log_index)` 的 btree 索引，便于「先按时间再按主键分量」排序并 limit，减少 sort。


测试sql1
```sql
select block_number,transaction_index,log_index,block_time,account_id,broker_hash,address from partitioned_executed_trades where block_time >= '2024-06-28 19:57:39' and block_time < '2024-06-29 19:57:39' order by block_time,block_number,transaction_index,log_index limit 400;
```

索引增加前执行计划
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

测试sql2
```sql
select block_number,transaction_index,log_index,block_time,account_id,broker_hash,address from partitioned_executed_trades where block_time >= '2024-06-28 19:57:39' and block_time < '2024-06-29 19:57:39'  order by block_time,block_number,transaction_index,log_index limit 400;
```


建议增加的索引

```sql
create index if not exists partitioned_executed_trades_block_time_cursor_idx on partitioned_executed_trades (block_time, block_number,transaction_index,log_index);
```

增加索引后的执行计划
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

### 2. 传入 broker_hash + 时间范围的查询

**查询形态**：`WHERE broker_hash = ? AND block_time BETWEEN ? AND ? ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400`。

**建议**：  
- **保留或统一使用现有** `partitioned_executed_trades_broker_index`：`btree (broker_hash, block_time)`。  
- **可选优化**：若分页游标是 `(block_time, block_number, transaction_index, log_index)` 且希望索引完全覆盖排序与 limit，可在**每个分区**上建：  
  `(broker_hash, block_time, block_number, transaction_index, log_index)`。  
- **原因**：  
  - `(broker_hash, block_time)` 能高效过滤 broker 并利用分区剪裁（时间范围）。  
  - 排序字段是 `block_time, block_number, transaction_index, log_index`，把 `block_number, transaction_index, log_index` 放进索引可以形成 **index-only 或最少回表** 的排序，避免大范围 sort；分页时用上一页最后一条的 `(block_time, block_number, transaction_index, log_index)` 做 cursor 也能走同一索引。

---

建议增加的索引
```sql
create index if not exists partitioned_executed_trades_broker_cursor_idx on partitioned_executed_trades (broker_hash,block_time, block_number,transaction_index,log_index);
```

### 3. 传入 account_id + 时间范围的查询

**查询形态**：`WHERE account_id = ? AND block_time BETWEEN ? AND ? ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400`。

**建议**：  
- **保留现有** `partitioned_executed_trades_account_id_index`：`btree (account_id, block_time)`。  
- **可选优化**：若仍存在大量 sort 或回表，可在**每个分区**上建：  
  `(account_id, block_time, block_number, transaction_index, log_index)`。  
- **原因**：  
  - `(account_id, block_time)` 已能快速定位到「某用户 + 时间区间」的数据，并配合分区键做 partition pruning。  
  - 加上 `block_number, transaction_index, log_index` 后，排序与 limit 400 可直接在索引上完成，分页游标与 2 类似，利于稳定延迟。


建议增加的索引
```sql
create index if not exists partitioned_executed_trades_account_cursor_idx on partitioned_executed_trades (account_id,block_time, block_number,transaction_index,log_index);
```

---

### 4. 传入 address + 时间范围的查询

**查询形态**：`WHERE address = ? AND block_time BETWEEN ? AND ? ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400`。

**建议**：  
- **新增索引**：在分区表上建 `(address, block_time)` 的 btree 索引（若当前没有）。  
- **可选优化**：同 2、3，若需要更稳的分页性能，在每个分区上建：  
  `(address, block_time, block_number, transaction_index, log_index)`。  
- **原因**：  
  - 当前文档中的索引只有主键、`account_id`、`broker_hash`，没有 `address`。按 address 过滤会变成分区内全表扫描，数据量在千万级时成本高。  
  - 加上 `(address, block_time)` 后能先按 address 和时间缩小范围，再在索引（或扩展为含排序列的索引）上做排序和 limit，避免大范围 sort 和随机回表。

建议增加的索引
```sql
create index if not exists partitioned_executed_trades_address_cursor_idx on partitioned_executed_trades (address,block_time, block_number,transaction_index,log_index);
```

---

### 5. 传入 symbol_hash + 时间范围的查询

**查询形态**：`WHERE symbol_hash = ? AND block_time BETWEEN ? AND ? ORDER BY block_time, block_number, transaction_index, log_index LIMIT 400`。

**建议**：  
- **新增索引**：在分区表上建 `(symbol_hash, block_time)` 的 btree 索引（当前仅有主键、`account_id`、`broker_hash`，无 `symbol_hash` 索引）。  
- **可选优化**：若需要更稳的分页性能，在每个分区上建：  
  `(symbol_hash, block_time, block_number, transaction_index, log_index)`。  
- **原因**：  
  - 按 symbol_hash 过滤时没有对应索引会触发分区内全表扫描，千万级分区下成本高。  
  - `(symbol_hash, block_time)` 可先按交易对和时间缩小范围，再配合分区键做 partition pruning。  
  - 将 `block_number, transaction_index, log_index` 加入索引后，排序与 limit 400、游标分页均可直接在索引上完成，避免大范围 sort 和随机回表。

建议增加的索引
```sql
create index if not exists partitioned_executed_trades_symbol_hash_cursor_idx on partitioned_executed_trades (symbol_hash, block_time, block_number, transaction_index, log_index);
```

---

### 小结

| 查询条件 | 建议 |
|----------|------|
| 仅时间范围 | 依赖分区剪裁 + 主键；必要时每分区建 `(block_time, block_number, transaction_index, log_index)` |
| broker_hash + 时间 | 使用现有 `(broker_hash, block_time)`；可选每分区 `(broker_hash, block_time, block_number, transaction_index, log_index)` |
| account_id + 时间 | 使用现有 `(account_id, block_time)`；可选每分区 `(account_id, block_time, block_number, transaction_index, log_index)` |
| address + 时间 | 新增 `(address, block_time)`；可选每分区 `(address, block_time, block_number, transaction_index, log_index)` |
| symbol_hash + 时间 | 新增 `(symbol_hash, block_time)`；可选每分区 `(symbol_hash, block_time, block_number, transaction_index, log_index)` |

**通用原因**：分页需要稳定、可复现的排序（文档要求 `block_time, block_number, transaction_index, log_index`），在索引中包含这些排序列能避免大结果集的 filesort，并让 `LIMIT 400` 和基于游标的下页查询都只扫少量索引页，从而提高分页查询速度。

**复合索引与冗余索引**：对 broker_hash、account_id、address、symbol_hash 等条件，若已建立「筛选列 + block_time + 游标列」的长索引（如 `(symbol_hash, block_time, block_number, transaction_index, log_index)`），则**不需要**再单独建「仅筛选列 + block_time」的短索引（如 `(symbol_hash, block_time)`）。PostgreSQL 的 B-tree 复合索引满足左前缀规则，长索引已能覆盖按短前缀的过滤与排序，单独建短索引只会增加存储与写入成本，无查询收益。

# 查询语句构建
如果已经通过以下sql创建索引：
 ```sql
create index if not exists partitioned_executed_trades_block_time_cursor_idx on partitioned_executed_trades (block_time, block_number,transaction_index,log_index);
```

分页查询第一页的查询是：
```sql
select block_number,transaction_index,log_index,block_time,account_id,broker_hash,address from partitioned_executed_trades where block_time >= '2024-06-28 19:57:39' and block_time < '2024-06-29 19:57:39'  order by block_time,block_number,transaction_index,log_index limit 400;
```
第一页查询结果的最后一条记录将作为下一页查询的过滤条件，也就是说下一页要从这次查询结果limit的后面一条记录开始返回，该怎么写查询语句使得sql更高效

### 思路（游标分页 / keyset pagination）

- 不用 `OFFSET`：`OFFSET 400`、`OFFSET 800` 等在大表上会让数据库先扫过前面所有行再丢弃，页数越大越慢。
- 用「上一页最后一条」的排序键做游标：用 `(block_time, block_number, transaction_index, log_index)` 四个字段组成游标，下一页条件为「排序键严格大于游标」，这样可以从索引上从游标之后的位置开始扫描，只取 400 行即停止。
- 索引 `(block_time, block_number, transaction_index, log_index)` 既能满足时间范围过滤，又能满足「按该四元组排序」以及「大于某四元组」的游标条件，因此下一页查询可以走同一索引，避免大范围 sort 和随机回表。

### 第一页（不变）

```sql
SELECT block_number, transaction_index, log_index, block_time, account_id, broker_hash, address
FROM partitioned_executed_trades
WHERE block_time >= '2024-06-28 19:57:39'
  AND block_time < '2024-06-29 19:57:39'
ORDER BY block_time, block_number, transaction_index, log_index
LIMIT 400;
```

### 下一页（游标分页）

设上一页最后一条的四个字段为：`last_block_time`, `last_block_number`, `last_transaction_index`, `last_log_index`。

**写法一（行比较，推荐，便于走索引）：**

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

（`:last_*` 替换为实际参数占位符，如 `$1,$2,$3,$4` 或 `?` 等。）

如：
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

**写法二（等价展开，便于在不支持行比较的客户端里使用）：**

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

### 为何更高效

- 条件 `(block_time, block_number, transaction_index, log_index) > (last_...)` 与 `ORDER BY` 一致，索引可直接从「游标之后」的位置开始扫描并返回下 400 行，无需大范围 sort，也不依赖 `OFFSET`，分页延迟稳定。

