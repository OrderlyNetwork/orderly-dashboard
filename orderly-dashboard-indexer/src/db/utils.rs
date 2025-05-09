use actix_diesel::dsl::AsyncRunQueryDsl;
use diesel::sql_types::BigInt;

use crate::db::POOL;
use anyhow::Result;

use diesel::sql_query;

#[derive(QueryableByName, Debug)]
pub struct Count {
    #[sql_type = "BigInt"]
    pub count: i64,
}

#[allow(dead_code)]
pub async fn check_table_is_exist(table_name: String) -> Result<bool> {
    let s = format!(
        "select count(*) from pg_class where relname = '{}';",
        table_name
    );
    let sql = sql_query(s);
    let result = sql.get_result_async::<Count>(&POOL).await?;
    if result.count == 0 {
        return Ok(false);
    }
    Ok(true)
}

#[derive(Debug)]
pub struct PartitionBounds {
    pub from_bound: String,
    pub to_bound: String,
}

/**
CREATE TABLE executed_trades_y2024m01 PARTITION OF partitioned_executed_trades
   FOR VALUES FROM ('2024-01-01 00:00:00') TO ('2024-02-01 00:00:00');
 */
impl PartitionBounds {
    #[allow(dead_code)]
    pub fn new(from_bound: &str, to_bound: &str) -> PartitionBounds {
        PartitionBounds {
            from_bound: from_bound.to_string(),
            to_bound: to_bound.to_string(),
        }
    }

    pub fn new_from_year_quarter(year_quarter: u64) -> PartitionBounds {
        let year = year_quarter / 100;
        let month_start = (year_quarter % 100 - 1) * 3 + 1;
        let (to_year, to_month) = if month_start < 12 {
            (year, month_start + 3)
        } else {
            (year + 1, 1)
        };
        PartitionBounds {
            from_bound: format!("{}-{:02}-01 00:00:00", year, month_start),
            to_bound: format!("{}-{:02}-01 00:00:00", to_year, to_month),
        }
    }
}

pub async fn create_partition(
    table_name: &str,
    partition_bounds: PartitionBounds,
    parent_table: &str,
) -> Result<bool> {
    let (from_bound, to_bound) = (partition_bounds.from_bound, partition_bounds.to_bound);
    let s = format!(
        "
        CREATE TABLE {} PARTITION OF {}
            FOR VALUES FROM ('{}') TO ('{}');
    ",
        table_name, parent_table, from_bound, to_bound
    );
    let sql = sql_query(s);
    let result = sql.execute_async(&POOL).await?;
    tracing::info!("create_partition result: {}", result);

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[ignore]
    #[test]
    fn test_check_table_is_exist() {
        dotenv::dotenv().ok();
        let system = actix::System::new();
        system.block_on(async move {
            let result = check_table_is_exist("settings".to_string()).await.unwrap();
            println!("result: {}", result);
            actix::System::current().stop();
        });

        system.run().unwrap();
    }

    #[ignore]
    #[test]
    fn test_create_partition() {
        dotenv::dotenv().ok();
        let system = actix::System::new();
        system.block_on(async move {
            // CREATE TABLE executed_trades_y2024m07 PARTITION OF partitioned_executed_trades
            //     FOR VALUES FROM ('2024-07-01 00:00:00') TO ('2024-08-01 00:00:00');
            create_partition(
                "executed_trades_y2024m07",
                PartitionBounds::new("2024-07-01 00:00:00", "2024-08-01 00:00:00"),
                "partitioned_executed_trades",
            )
            .await
            .map_err(|err| {
                println!("create_partition err: {}", err);
                err
            })
            .ok();
            actix::System::current().stop();
        });

        system.run().unwrap();
    }

    #[test]
    fn test_partition_bounds() {
        let year_month = 202408;
        let partition_bounds = PartitionBounds::new_from_year_quarter(year_month);
        println!(
            "partition_bounds for year_month: {} is {:?}",
            year_month, partition_bounds
        );
        assert_eq!(partition_bounds.from_bound, "2024-08-01 00:00:00");
        assert_eq!(partition_bounds.to_bound, "2024-09-01 00:00:00");

        let year_month = 202312;
        let partition_bounds = PartitionBounds::new_from_year_quarter(year_month);
        println!(
            "partition_bounds for year_month: {} is {:?}",
            year_month, partition_bounds
        );
        assert_eq!(partition_bounds.from_bound, "2023-12-01 00:00:00");
        assert_eq!(partition_bounds.to_bound, "2024-01-01 00:00:00");
    }
}
