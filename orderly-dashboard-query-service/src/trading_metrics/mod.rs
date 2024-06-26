use actix_web::{get, web, HttpResponse, Responder, Result};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use serde::Serialize;
use serde_derive::Deserialize;

use crate::db::trading_metrics::average::get_average;
use crate::db::trading_metrics::orderly_daily_perp::{daily_gas_fee, daily_orderly_perp};
use crate::db::trading_metrics::orderly_daily_token::get_daily_token;
use crate::db::trading_metrics::ranking::{
    get_daily_trading_volume_ranking, get_pnl_ranking, get_token_ranking,
    get_user_perp_holding_ranking,
};
use crate::db::trading_metrics::{get_block_height, get_daily_trading_fee, get_daily_volume};
use crate::{add_base_header, format_extern::Response};

const TRADING_METRICS: &str = "trading_metrics_context";

#[derive(Debug, Clone, Deserialize)]
pub struct DailyRequest {
    #[serde(default = "default_past")]
    from_day: String,
    #[serde(default = "default_now")]
    end_day: String,
}

fn default_now() -> String {
    Local::now().naive_utc().format("%Y-%m-%d").to_string()
}

fn default_past() -> String {
    let past = Local::now().naive_utc() - Duration::days(90);
    past.format("%Y-%m-%d").to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct VolumeRankingRequest {
    #[serde(default = "default_days")]
    days: i32,
    #[serde(default = "default_size")]
    size: i32,
}

fn default_days() -> i32 {
    30
}

fn default_size() -> i32 {
    10
}

fn test_symbol() -> String {
    "test".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct PerpHoldingRankingRequest {
    #[serde(default = "test_symbol")]
    symbol: String,
    #[serde(default = "default_size")]
    size: i32,
}

impl VolumeRankingRequest {
    fn to_hour(&self) -> i64 {
        (self.days * 24) as i64
    }
}

impl DailyRequest {
    pub fn parse_day(&self) -> (NaiveDateTime, NaiveDateTime) {
        let date_format = "%Y-%m-%d";
        let from_time = NaiveDate::parse_from_str(&self.from_day, date_format)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let end_time = NaiveDate::parse_from_str(&self.end_day, date_format)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        (from_time, end_time)
    }
}

pub fn write_response<T: Serialize>(res_data: T) -> HttpResponse {
    let success_response = Response {
        success: true,
        err_code: 0,
        err_msg: None,
        data: Some(res_data),
    };
    let mut resp = HttpResponse::Ok().json(success_response);
    add_base_header(&mut resp);
    resp
}

pub fn write_failed_response(err_code: i32, err_msg: &str) -> HttpResponse {
    let failed_response: Response<()> = Response::new_err(err_code, err_msg.to_string());
    let mut resp = HttpResponse::Ok().json(failed_response);
    add_base_header(&mut resp);
    resp
}

#[get("/block_height")] // <- define path parameters
pub async fn block_height() -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "block_height request");
    Ok(write_response(get_block_height().await))
}

#[get("/daily_orderly_perp")] // <- define path parameters
pub async fn get_daily_orderly_perp(param: web::Query<DailyRequest>) -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "daily_orderly_perp from day: {}, end_day: {}", param.from_day, param.end_day);
    let (from_time, end_time) = param.parse_day();
    Ok(write_response(
        daily_orderly_perp(from_time, end_time).await,
    ))
}

#[get("/daily_gas_fee/perp_trade")] // <- define path parameters
pub async fn perp_gas_fee(param: web::Query<DailyRequest>) -> Result<impl Responder> {
    let (from_time, end_time) = param.parse_day();
    Ok(write_response(
        daily_gas_fee(from_time, end_time, "perp_trade".to_string()).await,
    ))
}

#[get("/daily_gas_fee/event")] // <- define path parameters
pub async fn event_gas_fee(param: web::Query<DailyRequest>) -> Result<impl Responder> {
    let (from_time, end_time) = param.parse_day();
    Ok(write_response(
        daily_gas_fee(from_time, end_time, "event_upload".to_string()).await,
    ))
}

#[get("/daily_gas_fee/deposit")] // <- define path parameters
pub async fn deposit_gas_fee(param: web::Query<DailyRequest>) -> Result<impl Responder> {
    let (from_time, end_time) = param.parse_day();
    Ok(write_response(
        daily_gas_fee(from_time, end_time, "deposit".to_string()).await,
    ))
}

#[get("/daily_orderly_token")] // <- define path parameters
pub async fn get_daily_orderly_token(param: web::Query<DailyRequest>) -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "daily_orderly_token from day: {}, end_day: {}", param.from_day, param.end_day);
    let (from_time, end_time) = param.parse_day();
    Ok(write_response(get_daily_token(from_time, end_time).await))
}

#[get("/daily_volume")] // <- define path parameters
pub async fn daily_volume(param: web::Query<DailyRequest>) -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "daily_volume from day: {}, end_day: {}", param.from_day, param.end_day);
    let (from_time, end_time) = param.parse_day();
    Ok(write_response(get_daily_volume(from_time, end_time).await))
}

#[get("/daily_trading_fee")] // <- define path parameters
pub async fn daily_trading_fee(param: web::Query<DailyRequest>) -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "daily_trading_fee from day: {}, end_day: {}", param.from_day, param.end_day);
    let (from_time, end_time) = param.parse_day();
    Ok(write_response(
        get_daily_trading_fee(from_time, end_time).await,
    ))
}

#[get("/average_trading_count")]
pub async fn average_trading_count() -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "average_trading_count request");
    Ok(write_response(get_average("trading_count").await))
}

#[get("/average_trading_fee")]
pub async fn average_trading_fee() -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "average_trading_fee request");
    Ok(write_response(get_average("trading_fee").await))
}

#[get("/average_trading_volume")]
pub async fn average_trading_volume() -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "average_trading_volume request");
    Ok(write_response(get_average("trading_volume").await))
}

#[get("/average_opening_count")]
pub async fn average_opening_count() -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "average_opening_count request");
    Ok(write_response(get_average("opening_count").await))
}

#[get("/ranking/trading_volume")]
pub async fn get_trading_volume_rank(
    param: web::Query<VolumeRankingRequest>,
) -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "/ranking/trading_volume request, days: {}, size: {}", param.days, param.size);
    Ok(write_response(
        get_daily_trading_volume_ranking(param.to_hour(), param.size as i64).await,
    ))
}

#[get("/ranking/perp_holding")]
pub async fn get_perp_holding_rank(
    param: web::Query<PerpHoldingRankingRequest>,
) -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "/ranking/perp_holding request, days: {}, size: {}", param.symbol, param.size);
    Ok(write_response(
        get_user_perp_holding_ranking(param.symbol.clone(), param.size as i64).await,
    ))
}

#[get("/ranking/pnl")]
pub async fn get_perp_pnl_rank(param: web::Query<VolumeRankingRequest>) -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "/ranking/pnl, days: {}, size: {}", param.days, param.size);
    Ok(write_response(
        get_pnl_ranking(param.to_hour(), param.size as i64).await,
    ))
}

#[get("/ranking/deposit")]
pub async fn get_token_deposit_rank(
    param: web::Query<VolumeRankingRequest>,
) -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "/ranking/deposit, days: {}, size: {}", param.days, param.size);
    Ok(write_response(
        get_token_ranking(param.to_hour(), param.size as i64, false).await,
    ))
}

#[get("/ranking/withdraw")]
pub async fn get_token_withdraw_rank(
    param: web::Query<VolumeRankingRequest>,
) -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "/ranking/withdraw, days: {}, size: {}", param.days, param.size);
    Ok(write_response(
        get_token_ranking(param.to_hour(), param.size as i64, true).await,
    ))
}
