use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_web::{get, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use diesel::{ExpressionMethods, QueryDsl};
use serde_derive::Deserialize;

use crate::config::get_common_cfg;
use orderly_dashboard_analyzer::db::user_info::UserInfo;
use orderly_dashboard_analyzer::db::POOL;
use orderly_dashboard_analyzer::schema::user_info::dsl::*;
use orderly_dashboard_indexer::formats_external::trading_events::AccountTradingEventsResponse;
use orderly_dashboard_indexer::formats_external::{FailureResponse, Response, SuccessResponse};

pub(crate) const QUERY_ACCOUNT_EVENT_CONTEXT: &str = "query_account_event_context";

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccountEventsRequest {
    broker_id: String,
    address: String,
    #[serde(default = "two_weeks_ago")]
    from_time: i64,
    #[serde(default = "now_time")]
    to_time: i64,
    event_type: Option<String>,
}

fn two_weeks_ago() -> i64 {
    let past = Utc::now() - Duration::days(14);
    past.timestamp()
}

fn now_time() -> i64 {
    Utc::now().timestamp()
}

#[get("/events")] // <- define path parameters
pub async fn list_events(
    param: web::Query<GetAccountEventsRequest>,
) -> actix_web::Result<impl Responder> {
    tracing::info!(target: QUERY_ACCOUNT_EVENT_CONTEXT, "query account events params: {:?}", param);
    let mut select_result: Result<UserInfo, _> = user_info
        .filter(address.eq(param.address.clone()))
        .filter(broker_id.eq(param.broker_id.clone()))
        .first_async::<UserInfo>(&POOL)
        .await;
    if select_result.is_err() {
        match UserInfo::try_new(param.broker_id.clone(), param.address.clone()) {
            Err(err) => {
                let resp = FailureResponse::new(
                    1000,
                    format!("parse account_id failed with err: {}", err),
                );
                return Ok(HttpResponse::Ok().json(resp));
            }
            Ok(user_info_) => {
                select_result = Ok(user_info_);
            }
        }
    }

    match select_result {
        Ok(user_info_res) => {
            let indexer_data = get_indexer_data(
                param.from_time,
                param.to_time,
                user_info_res.account_id,
                param.event_type.as_deref().map(str::to_uppercase),
                get_common_cfg().indexer_address.clone(),
            )
            .await;

            match indexer_data {
                Ok(res_json_str) => {
                    let result: Result<Response<AccountTradingEventsResponse>, serde_json::Error> =
                        serde_json::from_str(&*res_json_str);

                    match result {
                        Ok(response) => {
                            return Ok(HttpResponse::Ok().json(response));
                        }
                        Err(err) => {
                            let resp = FailureResponse::new(
                                1000,
                                format!(
                                    "get_indexer_data parse event_type failed with err: {}",
                                    err
                                ),
                            );
                            return Ok(HttpResponse::Ok().json(resp));
                        }
                    }
                }
                Err(err) => {
                    let resp = FailureResponse::new(
                        1000,
                        format!("get user info failed with err: {}", err),
                    );
                    return Ok(HttpResponse::Ok().json(resp));
                }
            };
        }
        Err(_) => {
            let resp: Response<AccountTradingEventsResponse> =
                Response::Success(SuccessResponse::new(AccountTradingEventsResponse::default()));
            return Ok(HttpResponse::Ok().json(resp));
        }
    };
}

#[get("/sol_events")] // <- define path parameters
pub async fn list_sol_events(
    param: web::Query<GetAccountEventsRequest>,
) -> actix_web::Result<impl Responder> {
    tracing::info!(target: QUERY_ACCOUNT_EVENT_CONTEXT, "query sol account events params: {:?}", param);
    let user_info_res = match UserInfo::try_new(param.broker_id.clone(), param.address.clone()) {
        Ok(user_info_res) => user_info_res,
        Err(err) => {
            let resp =
                FailureResponse::new(1000, format!("parse account_id failed with err: {}", err));
            return Ok(HttpResponse::Ok().json(resp));
        }
    };

    let indexer_data = get_indexer_sol_data(
        param.from_time,
        param.to_time,
        user_info_res.account_id,
        param.event_type.as_deref().map(str::to_uppercase),
        get_common_cfg().indexer_address.clone(),
    )
    .await;

    match indexer_data {
        Ok(res_json_str) => {
            let result: Result<Response<AccountTradingEventsResponse>, serde_json::Error> =
                serde_json::from_str(&*res_json_str);

            match result {
                Ok(response) => {
                    return Ok(HttpResponse::Ok().json(response));
                }
                Err(err) => {
                    let resp = FailureResponse::new(
                        1000,
                        format!(
                            "get_indexer_sol_data parse event_type failed with err: {}",
                            err
                        ),
                    );
                    return Ok(HttpResponse::Ok().json(resp));
                }
            }
        }
        Err(err) => {
            let resp =
                FailureResponse::new(1000, format!("get user info failed with err: {}", err));
            return Ok(HttpResponse::Ok().json(resp));
        }
    };
}

async fn get_indexer_data(
    from_time: i64,
    to_time: i64,
    p_account_id: String,
    event_type: Option<String>,
    base_url: String,
) -> anyhow::Result<String> {
    let indexer_url = if let Some(event_type) = event_type {
        format!(
            "{}/pull_account_trading_events?account_id={}&from_time={}&to_time={}&event_type={}",
            base_url, p_account_id, from_time, to_time, event_type
        )
    } else {
        format!(
            "{}/pull_account_trading_events?account_id={}&from_time={}&to_time={}",
            base_url, p_account_id, from_time, to_time,
        )
    };
    let response = reqwest::get(indexer_url).await;
    match response {
        Ok(res) => Ok(res.text().await?),
        Err(err) => Err(anyhow::anyhow!("reqwest failed with: {}", err)),
    }
}

async fn get_indexer_sol_data(
    from_time: i64,
    to_time: i64,
    p_account_id: String,
    event_type: Option<String>,
    base_url: String,
) -> anyhow::Result<String> {
    let indexer_url = if let Some(event_type) = event_type {
        format!(
            "{}/pull_account_sol_events?account_id={}&from_time={}&to_time={}&event_type={}",
            base_url, p_account_id, from_time, to_time, event_type
        )
    } else {
        format!(
            "{}/pull_account_sol_events?account_id={}&from_time={}&to_time={}",
            base_url, p_account_id, from_time, to_time,
        )
    };
    let response = reqwest::get(indexer_url).await;
    match response {
        Ok(res) => Ok(res.text().await?),
        Err(err) => Err(anyhow::anyhow!("reqwest failed with: {}", err)),
    }
}
