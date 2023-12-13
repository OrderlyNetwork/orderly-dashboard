use std::cmp::{max, min};
use std::time::Duration;

use chrono::{TimeZone, Utc};
use orderly_dashboard_indexer::formats_external::Response;
use orderly_dashboard_indexer::formats_external::trading_events::{TradingEventInnerData, TradingEventsResponse};
use tokio::time;

use crate::analyzer::analyzer_job::HTTPException::Timeout;
use crate::analyzer::perp_analyzer::analyzer_perp_trade;
use crate::analyzer::transaction_analyzer::analyzer_transaction;
use crate::db::block_summary::{create_or_update_block_summary, find_block_summary};

const ANALYZER_CONTEXT: &str = "Analyzer-Job";

pub fn start_analyzer_job(interval_seconds: u64, base_url: String, start_block: i64) {
    tokio::spawn(async move {
        loop {
            let mut block_summary = find_block_summary().await.unwrap();
            let from_block = max(block_summary.pulled_block_height + 1, start_block.clone());
            let to_block = min(from_block + 50, block_summary.latest_block_height);

            let timestamp = Utc::now().timestamp_millis();
            tracing::info!(target: ANALYZER_CONTEXT,"start pull block from {} to {}.",from_block,to_block);
            let response_str = get_indexer_data(from_block, to_block, base_url.clone()).await;
            match response_str {
                Ok(json_str) => {
                    let result: Result<Response<TradingEventsResponse>, serde_json::Error> = serde_json::from_str(&*json_str);
                    let (pulled_block_time, latest_block_height, pulled_perp_trade_id) = parse_and_analyzer(result.unwrap()).await;
                    block_summary.pulled_block_height = to_block;
                    block_summary.pulled_block_time = pulled_block_time;
                    block_summary.latest_block_height = latest_block_height;
                    block_summary.pulled_perp_trade_id = pulled_perp_trade_id;
                    create_or_update_block_summary(block_summary).await;
                }
                Err(_) => {}
            }
            tracing::info!(target: ANALYZER_CONTEXT,"end pull block from {} to {}. cost:{}",from_block,to_block,Utc::now().timestamp_millis()-timestamp);
            time::sleep(Duration::from_secs(interval_seconds.clone())).await;
        }
    });
}

async fn parse_and_analyzer(response: Response<TradingEventsResponse>) -> (i64, i64, i64) {
    let mut pulled_block_time = 0i64;
    let mut latest_block_height = 0i64;
    let mut latest_perp_trade_id = 0i64;

    match response {
        Response::Success(success_event) => {
            let trading_event: TradingEventsResponse = success_event.into_data().unwrap();
            latest_block_height = trading_event.last_block as i64;

            let events = trading_event.events;
            for event in events {
                let block_hour = convert_block_hour(event.block_timestamp as i64);
                let block_num = event.block_number as i64;
                let block_time = event.block_timestamp as i64;
                pulled_block_time = max(pulled_block_time, block_time);
                let event_data = event.data;
                match event_data {
                    TradingEventInnerData::Transaction { account_id, sender, receiver, token_hash, broker_hash, chain_id, side, token_amount, withdraw_nonce, status, fail_reason, fee } => {
                        analyzer_transaction(account_id, token_hash, chain_id, side, token_amount, &block_hour, &block_num, block_time).await;
                    }
                    TradingEventInnerData::ProcessedTrades { batch_id, trades } => {
                        let trade_id = analyzer_perp_trade(trades, block_hour, block_time, block_num).await;
                        latest_perp_trade_id = max(latest_perp_trade_id, trade_id);
                    }
                    TradingEventInnerData::SettlementResult { account_id, settled_amount, settled_asset_hash, insurance_account_id, insurance_transfer_amount, settlement_executions } => {}
                    TradingEventInnerData::LiquidationResult { .. } => {}
                    TradingEventInnerData::AdlResult { .. } => {}
                }
            }
        }
        Response::Failure(_) => {}
    }

    (pulled_block_time, latest_block_height, latest_perp_trade_id)
}

fn convert_block_hour(block_timestamp: i64) -> i64 {
    let date_time = Utc.timestamp_opt(block_timestamp, 0).unwrap();
    let time_str = date_time.format("%Y%m%d%H").to_string();
    time_str.parse::<i64>().unwrap()
}


enum HTTPException {
    Timeout
}

async fn get_indexer_data(from_block: i64, to_block: i64, base_url: String) -> Result<String, HTTPException> {
    let indexer_url = format!("{}/pull_perp_trading_events?from_block={}&to_block={}", base_url, from_block, to_block);
    let response = reqwest::get(indexer_url).await.unwrap().text().await;
    match response {
        Ok(result) => {
            Ok(result)
        }
        Err(_) => {
            Err(Timeout)
        }
    }
}