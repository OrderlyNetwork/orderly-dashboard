use std::collections::HashMap;

use orderly_dashboard_indexer::formats_external::trading_events::Trade;

use orderly_dashboard_analyzer::db::hourly_orderly_perp::HourlyOrderlyPerpKey;

use crate::db::hourly_orderly_perp::{create_or_update_hourly_perp, find_hourly_orderly_perp, HourlyOrderlyPerp};

pub async fn analyzer_perp_trade(trades: Vec<Trade>, block_hour: i64, pulled_block_time: i64, pulled_block_height: i64) {
    let mut map: HashMap<HourlyOrderlyPerpKey, HourlyOrderlyPerp> = HashMap::new();

    for perp_trade in trades {
        println!("1");
        let perp_key = HourlyOrderlyPerpKey { symbol: perp_trade.symbol_hash.clone(), block_hour: block_hour.clone() };
        let mut hourly_orderly_perp;
        let mut hourly_orderly_perp_op = map.get_mut(&perp_key);
        match hourly_orderly_perp_op {
            None => {
                let saved_hourly = find_hourly_orderly_perp(perp_key.symbol.clone(), perp_key.block_hour.clone()).await.unwrap();
                hourly_orderly_perp = &saved_hourly;
            }
            Some(perp) => {
                hourly_orderly_perp = perp;
            }
        }
    }

    create_or_update_hourly_perp(Vec::from_iter(map.values().into_iter())).await.unwrap();
}