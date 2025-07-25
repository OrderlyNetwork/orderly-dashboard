use std::str::FromStr;
use std::time::Instant;

use actix_web::{get, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use orderly_dashboard_analyzer::sync_broker::{cal_account_id, get_sol_account_id};
use orderly_dashboard_indexer::sdk::solana::pubkey::Pubkey;
use serde_derive::Deserialize;

use crate::config::get_common_cfg;
use crate::error_code::{
    ACCOUNT_ID_CONFLICT_OR_INVALID_ERR, ACCOUNT_ID_CONFLICT_OR_INVALID_ERR_MESSAGE,
};
use once_cell::sync::Lazy;
use orderly_dashboard_indexer::formats_external::trading_events::{
    AccountTradingEventsResponse, AccoutTradingCursor,
};
use orderly_dashboard_indexer::formats_external::{FailureResponse, IndexerQueryResponse};
use reqwest::Client;
use utoipa::ToSchema;

pub(crate) const QUERY_ACCOUNT_EVENT_CONTEXT: &str = "query_account_event_context";
pub(crate) static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct GetAccountEventsRequest {
    account_id: Option<String>,
    broker_id: Option<String>,
    address: Option<String>,
    #[serde(default = "two_weeks_ago")]
    from_time: i64,
    #[serde(default = "now_time")]
    to_time: i64,
    event_type: Option<String>,
}

impl GetAccountEventsRequest {
    pub fn check_account_id_valid_and_cal(&mut self) -> bool {
        if self.account_id.is_none() && self.address.is_none() && self.broker_id.is_none() {
            return true;
        }
        if self.account_id.is_some() && self.address.is_none() && self.broker_id.is_none() {
            return true;
        }
        if self.account_id.is_none() && self.address.is_some() && self.broker_id.is_some() {
            let address = self.address.clone().unwrap();
            let broker_id = self.broker_id.clone().unwrap();
            if address.starts_with("0x") {
                match cal_account_id(&broker_id, &address) {
                    Ok(account_id) => {
                        self.account_id = Some(account_id);
                    }
                    Err(_) => {
                        return false;
                    }
                };
            } else {
                let sol_key = Pubkey::from_str(&address);
                if sol_key.is_err() {
                    return false;
                }
                let sol_key = sol_key.unwrap();
                match get_sol_account_id(&sol_key, &broker_id) {
                    Ok(account_id) => {
                        self.account_id = Some(account_id);
                    }
                    Err(_) => {
                        return false;
                    }
                };
            };
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct GetAccountEventsV2Request {
    account_id: Option<String>,
    broker_id: Option<String>,
    address: Option<String>,
    #[serde(default = "two_weeks_ago")]
    from_time: i64,
    #[serde(default = "now_time")]
    to_time: i64,
    event_type: Option<String>,
    trading_event_next_cursor: Option<AccoutTradingCursor>,
}

impl GetAccountEventsV2Request {
    pub fn check_account_id_valid_and_cal(&mut self) -> bool {
        if self.account_id.is_none() && self.address.is_none() && self.broker_id.is_none() {
            return true;
        }
        if self.account_id.is_some() && self.address.is_none() && self.broker_id.is_none() {
            return true;
        }
        if self.account_id.is_none() && self.address.is_some() && self.broker_id.is_some() {
            let address = self.address.clone().unwrap();
            let broker_id = self.broker_id.clone().unwrap();
            if address.starts_with("0x") {
                match cal_account_id(&broker_id, &address) {
                    Ok(account_id) => {
                        self.account_id = Some(account_id);
                    }
                    Err(_) => {
                        return false;
                    }
                };
            } else {
                let sol_key = Pubkey::from_str(&address);
                if sol_key.is_err() {
                    return false;
                }
                let sol_key = sol_key.unwrap();
                match get_sol_account_id(&sol_key, &broker_id) {
                    Ok(account_id) => {
                        self.account_id = Some(account_id);
                    }
                    Err(_) => {
                        return false;
                    }
                };
            };
            return true;
        }
        false
    }
}

fn two_weeks_ago() -> i64 {
    let past = Utc::now() - Duration::days(14);
    past.timestamp()
}

fn now_time() -> i64 {
    Utc::now().timestamp()
}

/// Get user token/trading events informations
#[utoipa::path(
    responses(
        (status = 200, description = "Get Account events response on orderly", body = IndexerQueryResponse<AccountTradingEventsResponse>),
        (status = 1000, description = "Invalid Request")
    ),
    params(("param" = GetAccountEventsRequest, Query, description = "account events, filter by `account_id` or `broker_id` + `address`, timestamp of `from_time` and `to_time`, `from_time` has a defualt value of two weeks ago, `to_time` has a default value of current timestamp, and `event_type` is optinal enum with value \"TRANSACTION | PERPTRADE | SETTLEMENT | LIQUIDATION | ADL\"")),
)]
#[get("/events")]
pub async fn list_events(
    mut param: web::Query<GetAccountEventsRequest>,
) -> actix_web::Result<HttpResponse> {
    tracing::info!(
        target: QUERY_ACCOUNT_EVENT_CONTEXT,
        "query account events start broker_id: {:?}, address: {:?}, from_time: {}, to_time: {}, event_type: {:?}",
        param.broker_id, param.address, param.from_time, param.to_time, param.event_type,
    );
    let inst = Instant::now();
    if !param.check_account_id_valid_and_cal() {
        let resp = FailureResponse::new(
            ACCOUNT_ID_CONFLICT_OR_INVALID_ERR,
            ACCOUNT_ID_CONFLICT_OR_INVALID_ERR_MESSAGE.to_string(),
        );
        return Ok(HttpResponse::Ok().json(resp));
    }
    // let user_info_res = match UserInfo::try_new(param.broker_id.clone(), param.address.clone()) {
    //     Ok(user_info_res) => user_info_res,
    //     Err(err) => {
    //         let resp =
    //             FailureResponse::new(1000, format!("parse account_id failed with err: {}", err));
    //         return Ok(HttpResponse::Ok().json(resp));
    //     }
    // };
    let elapsed_new_user = inst.elapsed().as_millis();

    let indexer_data = get_indexer_data(
        param.from_time,
        param.to_time,
        param.account_id.clone().unwrap_or_default(),
        param.event_type.as_deref().map(str::to_uppercase),
        get_common_cfg().indexer_address.clone(),
    )
    .await;
    let elapse_get_data = inst.elapsed().as_millis() - elapsed_new_user;

    match indexer_data {
        Ok(response) => {
            let length = match &response {
                IndexerQueryResponse::Success(sucs) => match sucs.as_data() {
                    Some(data) => data.events.len(),
                    None => 0 as usize,
                },
                _ => 0 as usize,
            };
            tracing::info!(
                target: QUERY_ACCOUNT_EVENT_CONTEXT,
                "query account events sucs account_id: {:?}, broker_id: {:?}, address: {:?}, from_time: {}, to_time: {}, event_type: {:?}, result len: {}, cost: {} ms, elapsed_new_user: {} ms, elapsed_get_data: {} ms",
                param.account_id, param.broker_id, param.address, param.from_time, param.to_time, param.event_type, length, inst.elapsed().as_millis(), elapsed_new_user, elapse_get_data,
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
                "query account events failed account_id: {:?}, broker_id: {:?}, address: {:?}, from_time: {}, to_time: {}, event_type: {:?} with err: {}, cost: {} ms",
                param.account_id, param.broker_id, param.address, param.from_time, param.to_time, param.event_type, err, inst.elapsed().as_millis(),
            );
            return Ok(HttpResponse::Ok().json(resp));
        }
    };
}

/// Get user token/trading events informations with pagelization
#[utoipa::path(
    responses(
        (status = 200, description = "Get Account events response on orderly", body = IndexerQueryResponse<AccountTradingEventsResponse>),
        (status = 1000, description = "Invalid Request")
    ),
    params(("param" = GetAccountEventsV2Request, Query, description = "account events, filter by `account_id` or `broker_id` + `address`, timestamp of `from_time` and `to_time`, `from_time` has a defualt value of two weeks ago, `to_time` has a default value of current timestamp, and `event_type` is optinal enum with value \"TRANSACTION | PERPTRADE | SETTLEMENT | LIQUIDATION | ADL\"")),
)]
#[get("/events_v2")] // <- define path parameters
pub async fn list_events_v2(
    mut param: web::Query<GetAccountEventsV2Request>,
) -> actix_web::Result<HttpResponse> {
    tracing::info!(
        target: QUERY_ACCOUNT_EVENT_CONTEXT,
        "query account events v2 start broker_id: {:?}, address: {:?}, from_time: {}, to_time: {}, event_type: {:?}, trading_event_next_cursor: {:?}",
        param.broker_id, param.address, param.from_time, param.to_time, param.event_type, param.trading_event_next_cursor,
    );
    if !param.check_account_id_valid_and_cal() {
        let resp = FailureResponse::new(
            ACCOUNT_ID_CONFLICT_OR_INVALID_ERR,
            ACCOUNT_ID_CONFLICT_OR_INVALID_ERR_MESSAGE.to_string(),
        );
        return Ok(HttpResponse::Ok().json(resp));
    }
    let inst = Instant::now();
    let elapsed_new_user = inst.elapsed().as_millis();

    let indexer_data = get_indexer_v2_data(
        param.from_time,
        param.to_time,
        param.account_id.clone().unwrap_or_default(),
        param.event_type.as_deref().map(str::to_uppercase),
        &param.trading_event_next_cursor,
        get_common_cfg().indexer_address.clone(),
    )
    .await;
    let elapse_get_data = inst.elapsed().as_millis() - elapsed_new_user;

    match indexer_data {
        Ok(response) => {
            let length = match &response {
                IndexerQueryResponse::Success(sucs) => match sucs.as_data() {
                    Some(data) => data.events.len(),
                    None => 0 as usize,
                },
                _ => 0 as usize,
            };
            tracing::info!(
                target: QUERY_ACCOUNT_EVENT_CONTEXT,
                "query account v2 events sucs broker_id: {:?}, address: {:?}, from_time: {}, to_time: {}, event_type: {:?}, result len: {}, cost: {} ms, elapsed_new_user: {} ms, elapsed_get_data: {} ms",
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
                "query account events failed broker_id: {:?}, address: {:?}, from_time: {}, to_time: {}, event_type: {:?} with err: {}, cost: {} ms",
                param.broker_id, param.address, param.from_time, param.to_time, param.event_type, err, inst.elapsed().as_millis(),
            );
            return Ok(HttpResponse::Ok().json(resp));
        }
    };
}

#[get("/sol_events")] // <- define path parameters
pub async fn list_sol_events(
    mut param: web::Query<GetAccountEventsRequest>,
) -> actix_web::Result<impl Responder> {
    let inst = Instant::now();
    // let user_info_res = match UserInfo::try_new(param.broker_id.clone(), param.address.clone()) {
    //     Ok(user_info_res) => user_info_res,
    //     Err(err) => {
    //         let resp =
    //             FailureResponse::new(1000, format!("parse account_id failed with err: {}", err));
    //         return Ok(HttpResponse::Ok().json(resp));
    //     }
    // };
    if !param.check_account_id_valid_and_cal() {
        let resp = FailureResponse::new(
            ACCOUNT_ID_CONFLICT_OR_INVALID_ERR,
            ACCOUNT_ID_CONFLICT_OR_INVALID_ERR_MESSAGE.to_string(),
        );
        return Ok(HttpResponse::Ok().json(resp));
    }

    let indexer_data = get_indexer_sol_data(
        param.from_time,
        param.to_time,
        param.account_id.clone().unwrap_or_default(),
        param.event_type.as_deref().map(str::to_uppercase),
        get_common_cfg().indexer_address.clone(),
    )
    .await;

    match indexer_data {
        Ok(response) => {
            let length = match &response {
                IndexerQueryResponse::Success(sucs) => match sucs.as_data() {
                    Some(data) => data.events.len(),
                    None => 0 as usize,
                },
                _ => 0 as usize,
            };
            tracing::info!(
                target: QUERY_ACCOUNT_EVENT_CONTEXT,
                "query sol account events sucs broker_id: {:?}, address: {:?}, from_time: {}, to_time: {}, event_type: {:?}, result len: {}, cost: {} ms",
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
                "query sol account events failed broker_id: {:?}, address: {:?}, from_time: {}, to_time: {}, event_type: {:?} with err: {}, cost: {} ms",
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
) -> anyhow::Result<IndexerQueryResponse<AccountTradingEventsResponse>> {
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
) -> anyhow::Result<IndexerQueryResponse<AccountTradingEventsResponse>> {
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
    trading_event_next_cursor: &Option<AccoutTradingCursor>,
    base_url: String,
) -> anyhow::Result<IndexerQueryResponse<AccountTradingEventsResponse>> {
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
    if let Some(trading_event_next_cursor) = trading_event_next_cursor {
        indexer_url = format!(
            "{}&offset_block_time={}&offset_block_number={}&offset_transaction_index={}&offset_log_index={}", 
            indexer_url, trading_event_next_cursor.block_time, trading_event_next_cursor.block_number, trading_event_next_cursor.transaction_index, trading_event_next_cursor.log_index,
        );
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
