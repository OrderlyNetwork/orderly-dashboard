use std::time::Instant;

use actix_web::{get, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use serde_derive::Deserialize;

use crate::config::get_common_cfg;
use once_cell::sync::Lazy;
use orderly_dashboard_analyzer::db::user_info::UserInfo;
use orderly_dashboard_indexer::formats_external::trading_events::AccountTradingEventsResponse;
use orderly_dashboard_indexer::formats_external::{FailureResponse, Response};
use reqwest::Client;

pub(crate) const QUERY_ACCOUNT_EVENT_CONTEXT: &str = "query_account_event_context";
pub(crate) static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

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

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccountEventsV2Request {
    broker_id: String,
    address: String,
    #[serde(default = "two_weeks_ago")]
    from_time: i64,
    #[serde(default = "now_time")]
    to_time: i64,
    event_type: Option<String>,
    offset: Option<u32>,
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
    tracing::info!(
        target: QUERY_ACCOUNT_EVENT_CONTEXT,
        "query account events start broker_id: {}, address: {}, from_time: {}, to_time: {}, event_type: {:?}",
        param.broker_id, param.address, param.from_time, param.to_time, param.event_type,
    );
    let inst = Instant::now();
    let user_info_res = match UserInfo::try_new(param.broker_id.clone(), param.address.clone()) {
        Ok(user_info_res) => user_info_res,
        Err(err) => {
            let resp =
                FailureResponse::new(1000, format!("parse account_id failed with err: {}", err));
            return Ok(HttpResponse::Ok().json(resp));
        }
    };
    let elapsed_new_user = inst.elapsed().as_millis();

    let indexer_data = get_indexer_data(
        param.from_time,
        param.to_time,
        user_info_res.account_id,
        param.event_type.as_deref().map(str::to_uppercase),
        get_common_cfg().indexer_address.clone(),
    )
    .await;
    let elapse_get_data = inst.elapsed().as_millis() - elapsed_new_user;

    match indexer_data {
        Ok(response) => {
            let length = match &response {
                Response::Success(sucs) => match sucs.as_data() {
                    Some(data) => data.events.len(),
                    None => 0 as usize,
                },
                _ => 0 as usize,
            };
            tracing::info!(
                target: QUERY_ACCOUNT_EVENT_CONTEXT,
                "query account events sucs broker_id: {}, address: {}, from_time: {}, to_time: {}, event_type: {:?}, result len: {}, cost: {} ms, elapsed_new_user: {} ms, elapsed_get_data: {} ms",
                param.broker_id, param.address, param.from_time, param.to_time, param.event_type, length, inst.elapsed().as_millis(), elapsed_new_user, elapse_get_data,
            );
            return Ok(HttpResponse::Ok().json(response));
        }
        Err(err) => {
            let resp = FailureResponse::new(
                1000,
                format!("get_indexer_data parse event_type failed with err: {}", err),
            );
            tracing::warn!(
                target: QUERY_ACCOUNT_EVENT_CONTEXT,
                "query account events failed broker_id: {}, address: {}, from_time: {}, to_time: {}, event_type: {:?} with err: {}, cost: {} ms",
                param.broker_id, param.address, param.from_time, param.to_time, param.event_type, err, inst.elapsed().as_millis(),
            );
            return Ok(HttpResponse::Ok().json(resp));
        }
    };
}

#[get("/events_v2")] // <- define path parameters
pub async fn list_events_v2(
    param: web::Query<GetAccountEventsV2Request>,
) -> actix_web::Result<impl Responder> {
    tracing::info!(
        target: QUERY_ACCOUNT_EVENT_CONTEXT,
        "query account events v2 start broker_id: {}, address: {}, from_time: {}, to_time: {}, event_type: {:?}, offset: {:?}",
        param.broker_id, param.address, param.from_time, param.to_time, param.event_type, param.offset,
    );
    let inst = Instant::now();
    let user_info_res = match UserInfo::try_new(param.broker_id.clone(), param.address.clone()) {
        Ok(user_info_res) => user_info_res,
        Err(err) => {
            let resp =
                FailureResponse::new(1000, format!("parse account_id failed with err: {}", err));
            return Ok(HttpResponse::Ok().json(resp));
        }
    };
    let elapsed_new_user = inst.elapsed().as_millis();

    let indexer_data = get_indexer_v2_data(
        param.from_time,
        param.to_time,
        user_info_res.account_id,
        param.event_type.as_deref().map(str::to_uppercase),
        param.offset,
        get_common_cfg().indexer_address.clone(),
    )
    .await;
    let elapse_get_data = inst.elapsed().as_millis() - elapsed_new_user;

    match indexer_data {
        Ok(response) => {
            let length = match &response {
                Response::Success(sucs) => match sucs.as_data() {
                    Some(data) => data.events.len(),
                    None => 0 as usize,
                },
                _ => 0 as usize,
            };
            tracing::info!(
                target: QUERY_ACCOUNT_EVENT_CONTEXT,
                "query account v2 events sucs broker_id: {}, address: {}, from_time: {}, to_time: {}, event_type: {:?}, result len: {}, cost: {} ms, elapsed_new_user: {} ms, elapsed_get_data: {} ms",
                param.broker_id, param.address, param.from_time, param.to_time, param.event_type, length, inst.elapsed().as_millis(), elapsed_new_user, elapse_get_data,
            );
            return Ok(HttpResponse::Ok().json(response));
        }
        Err(err) => {
            let resp = FailureResponse::new(
                1000,
                format!("get_indexer_data parse event_type failed with err: {}", err),
            );
            tracing::warn!(
                target: QUERY_ACCOUNT_EVENT_CONTEXT,
                "query account events failed broker_id: {}, address: {}, from_time: {}, to_time: {}, event_type: {:?} with err: {}, cost: {} ms",
                param.broker_id, param.address, param.from_time, param.to_time, param.event_type, err, inst.elapsed().as_millis(),
            );
            return Ok(HttpResponse::Ok().json(resp));
        }
    };
}

#[get("/sol_events")] // <- define path parameters
pub async fn list_sol_events(
    param: web::Query<GetAccountEventsRequest>,
) -> actix_web::Result<impl Responder> {
    let inst = Instant::now();
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
        Ok(response) => {
            let length = match &response {
                Response::Success(sucs) => match sucs.as_data() {
                    Some(data) => data.events.len(),
                    None => 0 as usize,
                },
                _ => 0 as usize,
            };
            tracing::info!(
                target: QUERY_ACCOUNT_EVENT_CONTEXT,
                "query sol account events sucs broker_id: {}, address: {}, from_time: {}, to_time: {}, event_type: {:?}, result len: {}, cost: {} ms",
                param.broker_id, param.address, param.from_time, param.to_time, param.event_type, length, inst.elapsed().as_millis()
            );
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
            tracing::warn!(
                target: QUERY_ACCOUNT_EVENT_CONTEXT,
                "query sol account events failed broker_id: {}, address: {}, from_time: {}, to_time: {}, event_type: {:?} with err: {}, cost: {} ms",
                param.broker_id, param.address, param.from_time, param.to_time, param.event_type, err, inst.elapsed().as_millis(),
            );
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
) -> anyhow::Result<Response<AccountTradingEventsResponse>> {
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

    // let response = reqwest::get(indexer_url).await;
    let response = CLIENT.get(indexer_url).send().await;
    match response {
        Ok(res) => Ok(res.json().await?),
        Err(err) => Err(anyhow::anyhow!("reqwest failed with: {}", err)),
    }
}

async fn get_indexer_sol_data(
    from_time: i64,
    to_time: i64,
    p_account_id: String,
    event_type: Option<String>,
    base_url: String,
) -> anyhow::Result<Response<AccountTradingEventsResponse>> {
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
    let response = CLIENT.get(indexer_url).send().await;
    match response {
        Ok(res) => Ok(res.json().await?),
        Err(err) => Err(anyhow::anyhow!("reqwest failed with: {}", err)),
    }
}

async fn get_indexer_v2_data(
    from_time: i64,
    to_time: i64,
    p_account_id: String,
    event_type: Option<String>,
    offset: Option<u32>,
    base_url: String,
) -> anyhow::Result<Response<AccountTradingEventsResponse>> {
    let mut indexer_url = if let Some(event_type) = event_type {
        format!(
            "{}/pull_account_trading_events_v2?account_id={}&from_time={}&to_time={}&event_type={}",
            base_url, p_account_id, from_time, to_time, event_type
        )
    } else {
        format!(
            "{}/pull_account_trading_events_v2?account_id={}&from_time={}&to_time={}",
            base_url, p_account_id, from_time, to_time,
        )
    };
    if let Some(offset) = offset {
        indexer_url = format!("{}&offset={}", indexer_url, offset);
    }
    let response = CLIENT.get(indexer_url).send().await;
    match response {
        Ok(res) => Ok(res.json().await?),
        Err(err) => Err(anyhow::anyhow!("reqwest failed with: {}", err)),
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    #[ignore]
    #[tokio::test]
    async fn test_get_user_trading_events() {
        let mut counter = 0;
        loop {
            let fut = async {
                print!("task start.....................");
                let indexer_url = "http://127.0.0.1:8020/events?address=0xd51C5283b8727206bf9Be2b2DB4e5673EfAF519C&broker_id=woofi_pro&from_time=1733834818&to_time=1735044418&from=12/10/2024&to=12/24/2024";
                let response = reqwest::get(indexer_url).await;
                println!("response: {:?}", response);
            };
            tokio::spawn(fut);
            tokio::time::sleep(tokio::time::Duration::from_millis(33)).await;
            counter += 1;
            println!("counter: {}", counter);
            if counter > 1000 {
                break;
            }
        }
    }

    #[test]
    fn test_new_client() {
        let inst = std::time::Instant::now();
        let _client = Client::new();
        println!("elapsed: {:?}", inst.elapsed());

        let inst2 = std::time::Instant::now();
        let _client2 = Client::new();
        println!("elapsed2: {:?}", inst2.elapsed());
    }

    #[test]
    fn test_printlog_time() {
        let inst = std::time::Instant::now();
        println!("test log");
        let elapsed = inst.elapsed();
        println!("elapsed: {:?}", elapsed);
    }
}
