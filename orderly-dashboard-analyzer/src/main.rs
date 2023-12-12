#![feature(unwrap_infallible)]
#[macro_use]
extern crate diesel;

use std::any::Any;
use std::fmt::Debug;
use std::str::FromStr;

use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use chrono::{TimeZone, Utc};
use clap::Parser;
use orderly_dashboard_indexer::formats_external::{Response, trading_events::*};
use reqwest;

use crate::analyzer::block_event_analyzer::BlockEventAnalyzer;
use crate::analyzer::perp_analyzer::analyzer_perp_trade;
use crate::analyzer::transaction_analyzer::analyzer_transaction;
use crate::indexer::indexer_client::IndexerClient;

mod indexer;
mod config;
mod analyzer;
mod db;
mod schema;

const ORDERLY_DASHBOARD_ANALYZER: &str = "orderly-dashboard-analyzer";

fn init_log() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(std::io::stderr)
        // .with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}


fn convert_block_hour(block_timestamp: i64) -> i64 {
    let date_time = Utc.timestamp_opt(block_timestamp, 0).unwrap();
    let time_str = date_time.format("%Y%m%d%H").to_string();
    time_str.parse::<i64>().unwrap()
}


#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is Health!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_log();

    let indexer_url = "http://localhost:8018/pull_perp_trading_events?from_block=1145079&to_block=1148065";
    let response = reqwest::get(indexer_url);
    // 获取响应的字符串内容
    let body = response.await;
    let ss = body.unwrap().text().await;


    match ss {
        Ok(res_text) => {
            println!("{}", res_text.clone());
            let rs_clone = res_text.clone();
            let result: Result<Response<TradingEventsResponse>, serde_json::Error> = serde_json::from_str(&*rs_clone);
            match result {
                Ok(event_res) => {
                    match event_res {
                        Response::Success(success_event) => {
                            let trading_event: TradingEventsResponse = success_event.into_data().unwrap();
                            let events = trading_event.events;
                            for event in events {
                                let block_hour = convert_block_hour(event.block_timestamp as i64);
                                let block_num = event.block_number as i64;
                                let block_time = event.block_timestamp as i64;
                                let event_data = event.data;
                                match event_data {
                                    TradingEventInnerData::Transaction { account_id, sender, receiver, token_hash, broker_hash, chain_id, side, token_amount, withdraw_nonce, status, fail_reason, fee } => {
                                        analyzer_transaction(account_id, token_hash, chain_id, side, token_amount, &block_hour, &block_num, block_time).await;
                                    }
                                    TradingEventInnerData::ProcessedTrades { batch_id, trades } => {
                                        analyzer_perp_trade(trades, block_hour, block_time, block_num).await;
                                    }
                                    TradingEventInnerData::SettlementResult { account_id, settled_amount, settled_asset_hash, insurance_account_id, insurance_transfer_amount, settlement_executions } => {}
                                    TradingEventInnerData::LiquidationResult { .. } => {}
                                    TradingEventInnerData::AdlResult { .. } => {}
                                }
                            }
                        }
                        Response::Failure(_) => {}
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {
            println!("error")
        }
    }
    HttpServer::new(|| {
        App::new()
            .service(health)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}