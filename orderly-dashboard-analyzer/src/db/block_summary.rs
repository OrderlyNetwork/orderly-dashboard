use diesel::{Insertable, Queryable};
use diesel::prelude::*;

use crate::schema::block_summary;

#[derive(Insertable, Queryable, Debug)]
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