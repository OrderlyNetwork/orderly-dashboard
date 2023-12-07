use diesel::{Insertable, Queryable};
use diesel::prelude::*;

#[allow(dead_code)]
#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = "block_summary")]
pub struct BlockSummary<'a> {
    id: i64,

    pulled_block_height: i64,
    pulled_block_timestamp: i64,
    pulled_block_hash: &'a String,

    pulled_event_id: i64,
    pulled_spot_trade_id: i64,
    pulled_perp_trade_id: i64,

    created_time: i64,
    updated_time: i64,
}