use std::collections::HashMap;

use orderly_dashboard_indexer::formats_external::trading_events::Trade;

use orderly_dashboard_analyzer::db::hourly_orderly_perp::HourlyOrderlyPerpKey;

use crate::db::hourly_orderly_perp::{create_or_update_hourly_perp, find_hourly_orderly_perp, HourlyOrderlyPerp};
use crate::db::hourly_user_perp::HourlyUserPerp;

pub async fn analyzer_perp_trade(trades: Vec<Trade>, block_hour: i64, pulled_block_time: i64, pulled_block_height: i64) {
    let mut map: HashMap<HourlyOrderlyPerpKey, HourlyOrderlyPerp> = HashMap::new();

    for perp_trade in trades {
        let perp_key = HourlyOrderlyPerpKey { symbol: perp_trade.symbol_hash.clone(), block_hour: block_hour.clone() };
        let mut hourly_orderly_perp_op = map.get_mut(&perp_key.clone());
        match hourly_orderly_perp_op {
            None => {
                let perp_key_clone = perp_key.clone();
                let saved_hourly = find_hourly_orderly_perp(perp_key_clone.symbol, perp_key_clone.block_hour).await.unwrap();
                map.insert(perp_key.clone(), saved_hourly);
            }
            Some(_) => {}
        }

        let mut hourly_orderly_perp = map.get_mut(&perp_key.clone()).unwrap();
        hourly_orderly_perp.new_trade(perp_trade.fee.parse().unwrap(), perp_trade.trade_qty.parse().unwrap())
    }
    create_or_update_hourly_perp(Vec::from_iter(map.values().into_iter())).await.unwrap();
}

