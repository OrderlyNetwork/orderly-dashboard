use crate::db::partitioned_executed_trades::create_partitioned_executed_trades;
use crate::db::partitioned_executed_trades::DbPartitionedExecutedTrades;
use crate::db::{
    executed_trades::{delete_oldest_executed_trades, query_executed_trades},
    partitioned_executed_trades::{format_partition_name, PARTITIONED_EXECUTED_TRADES_TABLE_NAME},
    settings::{
        get_executed_trades_legacy_sync, get_executed_trades_partition,
        get_last_rpc_processed_height, update_executed_trades_legacy_sync,
        update_executed_trades_partition, ExecutedTradesPartitionConfig,
    },
    utils::{create_partition, PartitionBounds},
};
use crate::service_base::runtime::spawn_future;
use anyhow::Result;
use chrono::Datelike;
use std::time::{Duration, Instant};
#[allow(unused)]
use {
    crate::api::MIGRATE_TRADES_FINISHED_AND_QUERY_FROM_PARTITIONING, std::sync::atomic::Ordering,
};

pub async fn check_or_create_executed_trades_partition() -> Result<()> {
    let mut executed_trades_partition_cfg = get_executed_trades_partition().await?;
    check_and_gen_partition(&mut executed_trades_partition_cfg).await?;
    update_executed_trades_partition(executed_trades_partition_cfg).await?;
    spawn_future(check_and_gen_partition_task());

    Ok(())
}

async fn check_and_gen_partition_task() -> Result<()> {
    loop {
        // every 15 days
        tokio::time::sleep(Duration::from_secs(3600 * 24 * 15)).await;
        let mut executed_trades_partition_cfg = get_executed_trades_partition().await?;
        check_and_gen_partition(&mut executed_trades_partition_cfg).await?;
        update_executed_trades_partition(executed_trades_partition_cfg).await?;
    }
}

async fn check_and_gen_partition(
    executed_trades_partition_cfg: &mut ExecutedTradesPartitionConfig,
) -> Result<()> {
    let now = chrono::Utc::now();
    let target_date = now + Duration::from_secs(3600 * 24 * 31);
    let target_year = target_date.year() as u64;
    // target quarter is next quater
    let target_quarter = (target_date.month() as u64 - 1) / 3 + 2;
    let exist_year = executed_trades_partition_cfg.created_table_quarter / 100;
    let exist_quarter = executed_trades_partition_cfg.created_table_quarter % 100;
    let differ_quarters =
        gen_diff_table_quarters(target_year, target_quarter, exist_year, exist_quarter);
    tracing::info!("diff year quarters: {:?}", differ_quarters);
    for year_quarter in differ_quarters {
        if let Err(err) = create_partition(
            &format_partition_name(year_quarter),
            PartitionBounds::new_from_year_quarter(year_quarter),
            PARTITIONED_EXECUTED_TRADES_TABLE_NAME,
        )
        .await
        {
            tracing::warn!(
                "create_partition for year_quarter: {} failed with err: {}",
                year_quarter,
                err
            );
        }
    }
    executed_trades_partition_cfg.created_table_quarter = target_year * 100 + target_quarter;
    Ok(())
}

pub async fn migrate_executed_trades_data(
    option_query_from_partitioning_executed_trades: bool,
) -> Result<()> {
    let mut executed_trades_legacy_sync = get_executed_trades_legacy_sync().await?;
    if !executed_trades_legacy_sync.finished {
        if executed_trades_legacy_sync.finished_block.is_none() {
            match get_last_rpc_processed_height().await? {
                Some(last_processed_block) => {
                    executed_trades_legacy_sync.finished_block = Some(last_processed_block);
                }
                None => {
                    executed_trades_legacy_sync.finished = true;
                }
            }
        }
    }
    update_executed_trades_legacy_sync(&executed_trades_legacy_sync).await?;

    // todo: reopen it if sync data and query data ok
    if executed_trades_legacy_sync.finished && option_query_from_partitioning_executed_trades {
        // todo: spawn a delete trade task
        // spawn_future(delete_legacy_trades_task());
        tracing::info!("set MIGRATE_TRADES_FINISHED_AND_QUERY_FROM_PARTITIONING to true");
        MIGRATE_TRADES_FINISHED_AND_QUERY_FROM_PARTITIONING.store(true, Ordering::Relaxed);
        return Ok(());
    }
    tracing::info!(
        "executed_trades_legacy_sync data: {:?}",
        executed_trades_legacy_sync
    );
    let mut current_block = executed_trades_legacy_sync
        .current_block
        .unwrap_or_default();
    let finished_block = executed_trades_legacy_sync
        .finished_block
        .unwrap_or_default();
    if finished_block == 0 {
        return Ok(());
    }
    const PAGE_SIZE: u64 = 100;
    let mut counter = 0;
    spawn_future(async move {
        loop {
            let to_block = current_block + PAGE_SIZE;
            migrate_executed_trades_one_batch(current_block, to_block).await?;
            current_block += PAGE_SIZE;
            if current_block >= finished_block {
                executed_trades_legacy_sync.finished = true;
                executed_trades_legacy_sync.current_block = Some(to_block);
                update_executed_trades_legacy_sync(&executed_trades_legacy_sync).await?;
                return Ok(());
            }
            counter += 1;
            if counter % 20 == 0 {
                executed_trades_legacy_sync.current_block = Some(to_block);
                update_executed_trades_legacy_sync(&executed_trades_legacy_sync).await?;
            }
            if counter % 200 == 0 {
                tracing::info!("executed trades migrated to block: {}", to_block);
            }
            counter = 0;
            // to avoid cpu usage rate too high
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    });

    Ok(())
}

async fn migrate_executed_trades_one_batch(from_block: u64, to_block: u64) -> Result<()> {
    let trades = query_executed_trades(from_block as i64, to_block as i64).await?;
    let mut trades = trades
        .into_iter()
        .map(DbPartitionedExecutedTrades::from)
        .collect::<Vec<DbPartitionedExecutedTrades>>();
    if !trades.is_empty() {
        tracing::info!(
            "migrate trades from_block {}, to_block: {}",
            from_block,
            to_block
        );
        loop {
            if trades.len() > 200 {
                let (left, right) = trades.split_at(200);
                create_partitioned_executed_trades(left.to_vec()).await?;
                trades = right.to_vec();
            } else {
                create_partitioned_executed_trades(trades.to_vec()).await?;
                break;
            }
        }
    }
    Ok(())
}

fn gen_diff_table_quarters(
    target_year: u64,
    target_quarter: u64,
    mut exist_year: u64,
    mut exist_quarter: u64,
) -> Vec<u64> {
    let mut results: Vec<u64> = Vec::new();
    while exist_year <= target_year {
        exist_quarter += 1;
        while (exist_year < target_year && exist_quarter <= 4)
            || (exist_year == target_year && exist_quarter <= target_quarter)
        {
            results.push(exist_year * 100 + exist_quarter);
            exist_quarter += 1;
        }
        exist_quarter = 0;
        exist_year += 1;
    }
    results
}

#[allow(unused)]
async fn delete_legacy_trades_task() -> Result<()> {
    loop {
        let inst = Instant::now();
        match delete_oldest_executed_trades(10000).await {
            Ok(rows) => {
                if rows == 0 {
                    break;
                }
            }
            Err(err) => {
                tracing::warn!("delete_oldest_executed_trades failed with err: {}", err);
            }
        }
        let elapse_ms = inst.elapsed().as_millis();
        if elapse_ms > 200 {
            tracing::warn!(
                "delete_oldest_executed_trades execute time {} ms has over limit",
                elapse_ms
            );
        }

        tokio::time::sleep(Duration::from_millis(100)).await
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::gen_diff_table_quarters;

    #[test]
    fn test_gen_diff_table_months() {
        let target_year = 2025;
        let target_quarter = 3;
        let exist_year = 2023;
        let exist_quater = 3;
        let res = gen_diff_table_quarters(target_year, target_quarter, exist_year, exist_quater);
        let expected = vec![
            202304, 202401, 202402, 202403, 202404, 202501, 202502, 202503,
        ];
        println!("results: {:?}", res);
        assert_eq!(res, expected);
    }
}
