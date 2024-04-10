use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_web::{get, HttpResponse, Responder, web};
use chrono::{Duration, Utc};
use diesel::{ExpressionMethods, QueryDsl};
use serde_derive::Deserialize;

use orderly_dashboard_analyzer::db::POOL;
use orderly_dashboard_analyzer::db::user_info::UserInfo;
use orderly_dashboard_analyzer::schema::user_info::dsl::*;
use orderly_dashboard_indexer::formats_external::{FailureResponse, Response};
use orderly_dashboard_indexer::formats_external::trading_events::{AccountTradingEventsResponse};

use crate::events::events_api::HTTPException::Timeout;

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccountEventsRequest {
    broker_id: String,
    address: String,
    #[serde(default = "two_weeks_ago")]
    from_time: i64,
    #[serde(default = "now_time")]
    to_time: i64,
    event_type: String,
}

fn two_weeks_ago() -> i64 {
    let past = Utc::now() - Duration::days(14);
    past.timestamp()
}

fn now_time() -> i64 {
    Utc::now().timestamp()
}

#[get("/events")] // <- define path parameters
pub async fn list_events(param: web::Query<GetAccountEventsRequest>) -> actix_web::Result<impl Responder> {
    let select_result: Result<UserInfo, _> = user_info
        .filter(broker_id.eq(param.broker_id.clone()))
        .filter(address.eq(param.address.clone()))
        .first_async::<UserInfo>(&POOL).await;

     match select_result {
        Ok(user_info_res) => {
            let indexer_data = get_indexer_data(
                param.from_time,
                param.to_time,
                user_info_res.account_id,
                param.event_type.to_uppercase(),
                "http://localhost:8018".to_string(),
            ).await;

             match indexer_data {
                Ok(res_json_str) => {
                    let result: Result<Response<AccountTradingEventsResponse>, serde_json::Error>
                        = serde_json::from_str(&*res_json_str);

                    match result {
                        Ok(response) => {
                            return Ok(HttpResponse::Ok().json(response));
                        }
                        Err(err) => {
                            let resp = FailureResponse::new(
                                1000,
                                format!("parse event_type failed with err: {}", err),
                            );
                            return Ok(HttpResponse::Ok().json(resp));
                        }
                    }
                }
                Err(err) => {
                    let resp = FailureResponse::new(
                        1000,
                        format!("parse event_type failed with err: {:?}", err),
                    );
                    return Ok(HttpResponse::Ok().json(resp));
                }
            };
        }
        Err(_) => {
            let resp = FailureResponse::new(
                1000,
                "user not found".to_string(),
            );
            return Ok(HttpResponse::Ok().json(resp));
        }
    };
}


async fn get_indexer_data(
    from_time: i64,
    to_time: i64,
    p_account_id: String,
    event_type: String,
    base_url: String,
) -> Result<String, HTTPException> {
    let indexer_url = format!(
        "{}/pull_account_trading_events?account_id={}&from_time={}&to_time={}&event_type={}",
        base_url, p_account_id, from_time, to_time, event_type
    );
    let response = reqwest::get(indexer_url).await;
    match response {
        Ok(res) => Ok(res.text().await.unwrap()),
        Err(_) => Err(Timeout),
    }
}

#[derive(Debug)]
pub enum HTTPException {
    Timeout,
}
