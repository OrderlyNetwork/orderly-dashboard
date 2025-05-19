use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Insertable, Queryable};

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::QueryError;
use crate::db::DB_CONTEXT;
use crate::db::POOL;
use crate::schema::block_summary;

pub const TRADE_METRIC: &str = "trade";
pub const GAS_METRIC: &str = "gas_fee";
#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name = "block_summary"]
pub struct BlockSummary {
    id: i32,
    pub latest_block_height: i64,

    pub pulled_block_height: i64,
    pub pulled_block_time: NaiveDateTime,

    pub pulled_event_id: i64,
    pub pulled_perp_trade_id: i64,

    pub metrics_type: String,
}

pub async fn find_block_summary(p_metric: String) -> Result<BlockSummary, DBException> {
    use crate::schema::block_summary::dsl::*;
    let select_result = block_summary
        .filter(metrics_type.eq(p_metric.clone()))
        .first_async::<BlockSummary>(&POOL)
        .await;
    let id_ = if p_metric == GAS_METRIC {
        2
    } else if p_metric == TRADE_METRIC {
        3
    } else {
        4
    };

    match select_result {
        Ok(result) => Ok(result),
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                let result = BlockSummary {
                    id: id_,
                    latest_block_height: 0,
                    pulled_block_height: 0,
                    pulled_block_time: Default::default(),
                    pulled_event_id: 0,
                    pulled_perp_trade_id: 0,
                    metrics_type: p_metric.clone(),
                };
                Ok(result)
            }
            _ => {
                tracing::warn!(target: DB_CONTEXT,"get_block_summary error. err:{:?}",error);
                Err(QueryError)
            }
        },
    }
}

pub async fn create_or_update_block_summary(summary: BlockSummary) -> Result<(), String> {
    use crate::schema::block_summary::dsl::*;
    let init_result = diesel::insert_into(block_summary)
        .values(summary.clone())
        .on_conflict(id)
        .do_update()
        .set((
            latest_block_height.eq(summary.latest_block_height.clone()),
            pulled_block_height.eq(summary.pulled_block_height.clone()),
            pulled_block_time.eq(summary.pulled_block_time.clone()),
            pulled_event_id.eq(summary.pulled_event_id.clone()),
            pulled_perp_trade_id.eq(summary.pulled_perp_trade_id.clone()),
        ))
        .execute_async(&POOL)
        .await;

    match init_result {
        Ok(_) => Ok(()),
        Err(error) => {
            tracing::warn!(target: DB_CONTEXT,"init_or_update_block_summary error. err:{:?}",error);
            Err("init_block_summary error".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_log() {
        tracing_subscriber::fmt::Subscriber::builder()
            .with_writer(std::io::stderr)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
    }
    // #[ignore]
    #[actix_web::test]
    async fn test_create_or_update_block_summary() {
        dotenv::dotenv().ok();
        init_log();

        let sumary = BlockSummary {
            id: 5,
            latest_block_height: 100,
            pulled_block_height: 100,
            pulled_block_time: NaiveDateTime::from_timestamp_opt(1747674014, 0).unwrap(),
            pulled_event_id: 100,
            pulled_perp_trade_id: 10000,
            metrics_type: "test".to_string(),
        };
        create_or_update_block_summary(sumary).await.unwrap();
    }
}
