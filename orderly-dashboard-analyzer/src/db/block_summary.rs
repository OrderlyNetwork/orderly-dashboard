use actix_diesel::AsyncError;
use actix_diesel::dsl::AsyncRunQueryDsl;
use diesel::{Insertable, Queryable};
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::DB_CONTEXT;
use crate::db::POOL;
use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::QueryError;
use crate::schema::block_summary;
use crate::schema::block_summary::id;

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name = "block_summary"]
pub struct BlockSummary {
    id: i32,
    latest_block_height: i64,

    pulled_block_height: i64,
    pulled_block_time: i64,

    pulled_event_id: i64,
    pulled_spot_trade_id: i64,
    pulled_perp_trade_id: i64,
}

pub async fn get_block_summary() -> Result<BlockSummary, DBException> {
    use crate::schema::block_summary::dsl::*;
    let select_result = block_summary
        .filter(id.eq(1i32))
        .first_async::<BlockSummary>(&POOL)
        .await;

    match select_result {
        Ok(result) => {
            Ok(result)
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                let result = BlockSummary {
                    id: 1i32,
                    latest_block_height: 1143278,
                    pulled_block_height: 1143268,
                    pulled_block_time: 0,
                    pulled_event_id: 0,
                    pulled_spot_trade_id: 0,
                    pulled_perp_trade_id: 0,
                };
                Ok(result)
            }
            _ => {
                tracing::warn!(target: DB_CONTEXT,"get_block_summary error. err:{:?}",error);
                Err(QueryError)
            }
        }
    }
}

pub async fn create_or_update_block_summary(summary: BlockSummary) {
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
            pulled_spot_trade_id.eq(summary.pulled_spot_trade_id.clone()),
            pulled_perp_trade_id.eq(summary.pulled_perp_trade_id.clone())
        ))
        .execute_async(&POOL)
        .await;

    match init_result {
        Ok(_) => {
            tracing::info!(target: DB_CONTEXT,"init_or_update_block_summary succeed.");
        }
        Err(error) => {
            tracing::warn!(target: DB_CONTEXT,"init_or_update_block_summary error. err:{:?}",error);
            panic!("init_block_summary error")
        }
    }
}