#![feature(unwrap_infallible)]
#[macro_use]
extern crate diesel;

use std::any::Any;
use std::fmt::Debug;
use std::str::FromStr;

use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use bigdecimal::BigDecimal;
use chrono::{TimeZone, Utc,DateTime};
use clap::Parser;
use orderly_dashboard_indexer::formats_external::{Response, trading_events::*};
use reqwest;

use crate::analyzer::block_event_analyzer::BlockEventAnalyzer;
use crate::db::user_token_summary::{create_user_token, DBException, select_user_token_summary, update_user_token, UserTokenSummary};
use crate::indexer::indexer_client::IndexerClient;
use crate::db::hourly_user_token::{select_by_key, HourlyUserToken, create_or_update_hourly_data};

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
    println!("{}", date_time);
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

    let indexer_url = "http://localhost:8018/pull_perp_trading_events?from_block=1143269&to_block=1145079";
    let response = reqwest::get(indexer_url);
    // 获取响应的字符串内容
    let body = response.await;
    let ss = body.unwrap().text().await;

    match ss {
        Ok(res_text) => {
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
                                let event_data = event.data;
                                match event_data {
                                    TradingEventInnerData::Transaction {
                                        account_id,
                                        sender,
                                        receiver, // receiver address
                                        token_hash,
                                        broker_hash,
                                        chain_id,
                                        side, // “deposit｜withdraw"
                                        token_amount,
                                        withdraw_nonce, // optional
                                        status,   // "succeed|failed"
                                        fail_reason,    // optional
                                        fee
                                    } => {
                                        let ori = select_user_token_summary(account_id.clone(), token_hash.clone(), chain_id.clone()).await;
                                        let hour_ori = select_by_key(account_id.clone(), token_hash.clone(), block_hour, chain_id.clone()).await;
                                        let mut hourly_data;
                                        match hour_ori {
                                            Ok(ori) => {
                                                match ori {
                                                    None => {
                                                        hourly_data = HourlyUserToken {
                                                            account_id: account_id.clone(),
                                                            token: token_hash.clone(),
                                                            block_hour: block_hour,
                                                            chain_id: chain_id.clone(),
                                                            withdraw_amount: Default::default(),
                                                            withdraw_count: 0,
                                                            deposit_amount: Default::default(),
                                                            deposit_count: 0,
                                                            pulled_block_height: 0,
                                                            pulled_block_time: 0,
                                                        }
                                                    }
                                                    Some(ori_data) => {
                                                        hourly_data = ori_data;
                                                    }
                                                }
                                            }
                                            Err(error) => {
                                                println!("{:?}", error);
                                                panic!()
                                            }
                                        }

                                        match ori {
                                            None => {
                                                let mut insert_sum = UserTokenSummary {
                                                    account_id: account_id.clone(),
                                                    token: token_hash.clone(),
                                                    chain_id: chain_id.clone(),
                                                    balance: Default::default(),
                                                    total_withdraw_amount: Default::default(),
                                                    total_deposit_amount: Default::default(),
                                                    total_withdraw_count: 0,
                                                    total_deposit_count: 0,
                                                    pulled_block_height: event.block_number as i64,
                                                    pulled_block_time: event.block_timestamp as i64,
                                                };
                                                match side {
                                                    TransactionSide::Deposit => {
                                                        insert_sum.balance = BigDecimal::from_str(token_amount.as_str()).unwrap();
                                                        insert_sum.total_deposit_count = 1;
                                                        insert_sum.total_deposit_amount = BigDecimal::from(1);
                                                    }
                                                    TransactionSide::Withdraw => {}
                                                }
                                                create_user_token(vec![insert_sum]).await.expect("TODO: panic message");
                                            }
                                            Some(mut saved_summary) => {
                                                match side {
                                                    TransactionSide::Deposit => {
                                                        saved_summary.balance = saved_summary.balance + BigDecimal::from_str(token_amount.as_str()).unwrap();
                                                        saved_summary.total_deposit_count += 1;
                                                        saved_summary.total_deposit_amount = saved_summary.total_deposit_amount + BigDecimal::from_str(token_amount.as_str()).unwrap();

                                                        hourly_data.deposit_count += 1;
                                                        hourly_data.deposit_amount = hourly_data.deposit_amount + BigDecimal::from_str(token_amount.as_str()).unwrap();
                                                    }
                                                    TransactionSide::Withdraw => {
                                                        saved_summary.balance = saved_summary.balance - BigDecimal::from_str(token_amount.as_str()).unwrap();
                                                        saved_summary.total_withdraw_count += 1;
                                                        saved_summary.total_withdraw_amount = saved_summary.total_withdraw_amount + BigDecimal::from_str(token_amount.as_str()).unwrap();


                                                        hourly_data.withdraw_count += 1;
                                                        hourly_data.withdraw_amount = hourly_data.withdraw_amount + BigDecimal::from_str(token_amount.as_str()).unwrap();
                                                    }
                                                }
                                                update_user_token(vec![saved_summary]).await.expect("TODO: panic message");
                                            }
                                        }

                                        create_or_update_hourly_data(vec![hourly_data]).await.expect("TODO: panic message");
                                    }
                                    TradingEventInnerData::ProcessedTrades { .. } => {
                                        println!("2")
                                    }
                                    TradingEventInnerData::SettlementResult { .. } => {
                                        println!("3")
                                    }
                                    TradingEventInnerData::LiquidationResult { .. } => {
                                        println!("4")
                                    }
                                    TradingEventInnerData::AdlResult { .. } => {
                                        println!("5")
                                    }
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