use std::collections::HashMap;
use orderly_dashboard_indexer::formats_external::trading_events::Trade;

use crate::db::hourly_orderly_perp::{create_or_update_hourly_perp, find_hourly_orderly_perp, HourlyOrderlyPerp, HourlyOrderlyPerpKey};
use crate::db::hourly_user_perp::{create_or_update_hourly_user_perp, find_hourly_user_perp, HourlyUserPerp, HourlyUserPerpKey};
use crate::db::orderly_perp_summary::{create_or_update_orderly_perp_summary, find_orderly_perp_summary, OrderlyPerpSummary};
use crate::db::user_perp_summary::{create_or_update_user_perp_summary, find_user_perp_summary, UserPerpSummary, UserPerpSummaryKey};

pub async fn analyzer_perp_trade(trades: Vec<Trade>, block_hour: i64, pulled_block_time: i64, pulled_block_height: i64) {
    let mut map: HashMap<HourlyOrderlyPerpKey, HourlyOrderlyPerp> = HashMap::new();
    let mut user_map: HashMap<HourlyUserPerpKey, HourlyUserPerp> = HashMap::new();
    let mut orderly_summary_map: HashMap<String, OrderlyPerpSummary> = HashMap::new();
    let mut user_summary_map: HashMap<UserPerpSummaryKey, UserPerpSummary> = HashMap::new();

    for perp_trade in trades {

        // hourly_orderly
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
        hourly_orderly_perp.new_trade(perp_trade.fee.parse().unwrap(), perp_trade.trade_qty.parse().unwrap(), pulled_block_time.clone(), pulled_block_height.clone());

        //hourly_user
        let hourly_user_perp_key = HourlyUserPerpKey {
            account_id: perp_trade.account_id.clone(),
            symbol: perp_trade.symbol_hash.clone(),
            block_hour: block_hour.clone(),
        };

        let mut hourly_user_perp_op = user_map.get_mut(&hourly_user_perp_key.clone());
        match hourly_user_perp_op {
            None => {
                let perp_key_clone = hourly_user_perp_key.clone();
                let saved_data = find_hourly_user_perp(perp_key_clone.account_id, perp_key_clone.symbol, block_hour.clone()).await.unwrap();
                user_map.insert(hourly_user_perp_key.clone(), saved_data);
            }
            Some(_) => {}
        }

        let mut hourly_user_perp = user_map.get_mut(&hourly_user_perp_key.clone()).unwrap();
        let new_hourly_user = hourly_user_perp.new_trade(perp_trade.fee.parse().unwrap(), perp_trade.trade_qty.parse().unwrap(), pulled_block_time.clone(), pulled_block_height.clone());
        if new_hourly_user {
            hourly_orderly_perp.new_user();
        }

        //orderly_summary
        let perp_symbol = perp_trade.symbol_hash.clone();
        let mut orderly_perp_summary_op = orderly_summary_map.get_mut(&perp_symbol.clone());
        match orderly_perp_summary_op {
            None => {
                let saved_summary = find_orderly_perp_summary(perp_symbol.clone()).await.unwrap();
                orderly_summary_map.insert(perp_symbol.clone(), saved_summary);
            }
            Some(_) => {}
        }

        let mut orderly_perp_summary = orderly_summary_map.get_mut(&perp_symbol.clone()).unwrap();
        orderly_perp_summary.new_trade(perp_trade.fee.parse().unwrap(), perp_trade.trade_qty.parse().unwrap(),
                                       pulled_block_time.clone(), pulled_block_height.clone(), perp_trade.side.clone());

        //user_summary
        let user_perp_summary_key = UserPerpSummaryKey { account_id: perp_trade.account_id.clone(), symbol: perp_symbol.clone() };
        let mut user_perp_summary_op = user_summary_map.get_mut(&user_perp_summary_key.clone());
        match user_perp_summary_op {
            None => {
                let saved_summary = find_user_perp_summary(user_perp_summary_key.account_id.clone(), perp_symbol.clone()).await.unwrap();
                user_summary_map.insert(user_perp_summary_key.clone(), saved_summary);
            }
            Some(_) => {}
        }

        let mut user_perp_summary = user_summary_map.get_mut(&user_perp_summary_key.clone()).unwrap();
        let (opening, new_user) = user_perp_summary.new_trade(perp_trade.fee.parse().unwrap(), perp_trade.trade_qty.parse().unwrap(), pulled_block_time.clone(), pulled_block_height.clone());
        if opening {
            hourly_orderly_perp.new_opening();
        }
        if new_user {
            orderly_perp_summary.new_user();
        }
    }

    create_or_update_hourly_user_perp(Vec::from_iter(user_map.values().into_iter())).await.unwrap();
    create_or_update_hourly_perp(Vec::from_iter(map.values().into_iter())).await.unwrap();
    create_or_update_orderly_perp_summary(Vec::from_iter(orderly_summary_map.values().into_iter())).await.unwrap();
    create_or_update_user_perp_summary(Vec::from_iter(user_summary_map.values().into_iter())).await.unwrap();
}

