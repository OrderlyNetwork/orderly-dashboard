pub mod volume_statistic;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use orderly_dashboard_analyzer::sync_broker::{
    cal_account_id, cal_symbol_hash, get_sol_account_id,
};
use orderly_dashboard_indexer::formats_external::FailureResponse;
use orderly_dashboard_indexer::sdk::solana::pubkey::Pubkey;
use serde::Serialize;
use serde_derive::Deserialize;

use crate::db::trading_metrics::average::get_average;
use crate::db::trading_metrics::orderly_daily_perp::{daily_gas_fee, daily_orderly_perp};
use crate::db::trading_metrics::orderly_daily_token::get_daily_token;
use crate::db::trading_metrics::ranking::{
    get_daily_trading_volume_ranking, get_pnl_ranking, get_token_ranking,
    get_user_perp_holding_ranking, query_user_perp_holding_by_address_and_opt_broker,
    query_user_perp_max_symbol_holding, query_user_perp_max_symbol_realized_pnl,
    UserSymbolSummaryRank,
};
use crate::db::trading_metrics::{get_block_height, get_daily_trading_fee, get_daily_volume};
use crate::error_code::{
    ACCOUNT_ID_CONFLICT_OR_INVALID_ERR, ACCOUNT_ID_CONFLICT_OR_INVALID_ERR_MESSAGE,
    DAYS_RAGE_OVER_LIMIT, DAYS_RAGE_OVER_LIMIT_ERR_MESSAGE, QUERY_OVER_EXECUTION_ERR,
    QUERY_OVER_LIMIT_ERR, SIZE_OVER_LIMIT, SIZE_OVER_LIMIT_ERR_MESSAGE,
};
use crate::format_extern::rank_metrics::UserSummaryRankExtern;
use crate::format_extern::trading_metrics::{
    DailyData, DailyTradingFeeExtern, DailyVolumeExtern, OrderlyPerpDaily, TokenAmountRanking,
    TradingPnlRanking, TradingVolumeRanking,
};
use crate::utils::gen_rand;
use crate::{add_base_header, format_extern::QeuryServiceResponse};
use dashmap::DashMap;
use fxhash::FxBuildHasher;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;
use typescript_type_def::TypeDef;
use utoipa::ToSchema;

pub use volume_statistic::{get_account_volume_statistic, get_broker_volume_statistic};

const TRADING_METRICS: &str = "trading_metrics_context";

lazy_static! {
    pub static ref TOP_POSITIONS: RwLock<Vec<UserSumaryRankingData>> =
        RwLock::new(Vec::with_capacity(1000));
    pub static ref TOP_REALIZED_PNL_ASC: RwLock<Vec<UserSumaryRankingData>> =
        RwLock::new(Vec::with_capacity(1000));
    pub static ref TOP_REALIZED_PNL_DESC: RwLock<Vec<UserSumaryRankingData>> =
        RwLock::new(Vec::with_capacity(1000));
}

// top 1000 cache
pub static BROKER_TOP_POSITIONS: Lazy<
    DashMap<String, Arc<RwLock<(Vec<UserSumaryRankingData>, Instant)>>, FxBuildHasher>,
> = Lazy::new(|| DashMap::with_hasher(FxBuildHasher::default()));

pub static SYMBOL_TOP_POSITIONS: Lazy<
    DashMap<String, Arc<RwLock<(Vec<UserSumaryRankingData>, Instant)>>, FxBuildHasher>,
> = Lazy::new(|| DashMap::with_hasher(FxBuildHasher::default()));

pub static SYMBOL_TOP_REALIZED_PNL_ASC: Lazy<
    DashMap<String, Arc<RwLock<(Vec<UserSumaryRankingData>, Instant)>>, FxBuildHasher>,
> = Lazy::new(|| DashMap::with_hasher(FxBuildHasher::default()));

pub static SYMBOL_TOP_REALIZED_PNL_DESC: Lazy<
    DashMap<String, Arc<RwLock<(Vec<UserSumaryRankingData>, Instant)>>, FxBuildHasher>,
> = Lazy::new(|| DashMap::with_hasher(FxBuildHasher::default()));

// 1000
pub fn update_positions_task() {
    actix_web::rt::spawn(update_positions());
}

pub fn update_realized_pnl_task() {
    actix_web::rt::spawn(update_realized_pnl_asc());
    actix_web::rt::spawn(update_realized_pnl_desc());
}

async fn update_positions() -> anyhow::Result<()> {
    loop {
        match query_user_perp_max_symbol_holding(0, 1000, None, None, None).await {
            Ok(user_perp_holding) => {
                let user_perp_holding = user_perp_holding
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<UserSumaryRankingData>>();
                *TOP_POSITIONS.write() = user_perp_holding;
            }
            Err(err) => {
                tracing::warn!(
                    "query_user_perp_max_symbol_holding failed with err: {}",
                    err
                );
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
    #[allow(unreachable_code)]
    Ok(())
}

async fn update_realized_pnl_asc() -> anyhow::Result<()> {
    loop {
        match query_user_perp_max_symbol_realized_pnl(0, 1000, None, None, "ASC".to_string()).await
        {
            Ok(user_perp_realized_pnl) => {
                let user_perp_realized_pnl = user_perp_realized_pnl
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<UserSumaryRankingData>>();
                *TOP_REALIZED_PNL_ASC.write() = user_perp_realized_pnl;
            }
            Err(err) => {
                tracing::warn!(
                    "query_user_perp_max_symbol_realized_pnl failed with err: {}",
                    err
                );
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
    #[allow(unreachable_code)]
    Ok(())
}

async fn update_realized_pnl_desc() -> anyhow::Result<()> {
    loop {
        match query_user_perp_max_symbol_realized_pnl(0, 1000, None, None, "DESC".to_string()).await
        {
            Ok(user_perp_realized_pnl) => {
                let user_perp_realized_pnl = user_perp_realized_pnl
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<UserSumaryRankingData>>();
                *TOP_REALIZED_PNL_DESC.write() = user_perp_realized_pnl;
            }
            Err(err) => {
                tracing::warn!(
                    "query_user_perp_max_symbol_realized_pnl failed with err: {}",
                    err
                );
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(gen_rand(60, 120))).await;
    }
    #[allow(unreachable_code)]
    Ok(())
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct VolumeRankingRequest {
    #[serde(default = "default_days")]
    days: u32,
    #[serde(default = "default_size")]
    size: u32,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct DepositWithdrawRankingRequest {
    #[serde(default = "default_days")]
    days: u32,
    #[serde(default = "default_size")]
    size: u32,
    token: String,
}

impl DepositWithdrawRankingRequest {
    fn to_hour(&self) -> i64 {
        (self.days * 24) as i64
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PnlRankingRequest {
    #[serde(default = "default_days")]
    days: u32,
    #[serde(default = "default_size")]
    size: u32,
    symbol: Option<String>,
}

impl PnlRankingRequest {
    fn to_hour(&self) -> i64 {
        (self.days * 24) as i64
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, TypeDef, ToSchema)]
pub struct UserSumaryRankingData {
    pub account_id: String,
    pub address: String,
    pub broker_id: String,
    pub symbol: String,
    pub symbol_hash: String,
    pub holding: String,
    pub total_realized_pnl: String,
    pub index_price: String,
    pub mark_price: String,
    pub holding_value: String,
    pub opening_cost: String,
    pub average_entry_price: String,
    pub un_realized_pnl: String,
}

impl From<UserSymbolSummaryRank> for UserSumaryRankingData {
    fn from(value: UserSymbolSummaryRank) -> Self {
        let average_entry_price = if value.holding.sign() == num_bigint::Sign::Minus {
            (-value.opening_cost.clone() / value.holding.clone())
                .with_scale_round(8, bigdecimal::RoundingMode::Down)
        } else if value.holding.sign() == num_bigint::Sign::Plus {
            (-value.opening_cost.clone() / value.holding.clone())
                .with_scale_round(8, bigdecimal::RoundingMode::Up)
        } else {
            0.into()
        };
        UserSumaryRankingData {
            account_id: value.account_id,
            address: value.address,
            broker_id: value.broker_id,
            symbol: value.symbol,
            symbol_hash: value.symbol_hash,
            holding: value.holding.to_string(),
            total_realized_pnl: value.total_realized_pnl.to_string(),
            index_price: value.index_price.to_string(),
            mark_price: value.mark_price.to_string(),
            holding_value: value.holding_value.to_string(),
            opening_cost: value.opening_cost.to_string(),
            average_entry_price: average_entry_price.to_string(),
            un_realized_pnl: (value.holding * (value.mark_price - average_entry_price))
                .with_scale_round(8, bigdecimal::RoundingMode::Down)
                .to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VolumeRankingResponse {
    pub rows: Vec<UserSumaryRankingData>,
}

fn default_days() -> u32 {
    30
}

fn default_size() -> u32 {
    10
}

fn test_symbol() -> String {
    "test".to_string()
}

fn default_offset() -> i32 {
    0
}

fn default_limit() -> i32 {
    50
}

fn default_order_by() -> RealizedPnlRankingOrder {
    RealizedPnlRankingOrder::DESC
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PositionRankingRequest {
    account_id: Option<String>,
    address: Option<String>,
    broker_id: Option<String>,
    symbol: Option<String>,
    #[serde(default = "default_offset")]
    offset: i32,
    #[serde(default = "default_limit")]
    limit: i32,
}

impl PositionRankingRequest {
    pub fn check_account_id_valid_and_cal(&mut self) -> bool {
        if self.account_id.is_none() && self.address.is_none() {
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
pub enum RealizedPnlRankingOrder {
    ASC,
    DESC,
}

impl std::fmt::Display for RealizedPnlRankingOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealizedPnlRankingOrder::ASC => write!(f, "ASC"),
            RealizedPnlRankingOrder::DESC => write!(f, "DESC"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RealizedPnlRankingRequest {
    account_id: Option<String>,
    address: Option<String>,
    broker_id: Option<String>,
    symbol: Option<String>,
    #[serde(default = "default_offset")]
    offset: i32,
    #[serde(default = "default_limit")]
    limit: i32,
    #[serde(default = "default_order_by")]
    order_by: RealizedPnlRankingOrder,
}

impl RealizedPnlRankingRequest {
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

#[derive(Debug, Clone, Deserialize)]
pub struct PerpHoldingRankingRequest {
    #[serde(default = "test_symbol")]
    symbol: String,
    #[serde(default = "default_size")]
    size: u32,
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
            .and_hms_opt(23, 59, 59)
            .unwrap();
        (from_time, end_time)
    }
}

pub fn write_response<T: Serialize>(res_data: T) -> HttpResponse {
    let success_response = QeuryServiceResponse {
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
    let failed_response: QeuryServiceResponse<()> =
        QeuryServiceResponse::new_err(err_code, err_msg.to_string());
    let mut resp = HttpResponse::Ok().json(failed_response);
    add_base_header(&mut resp);
    resp
}

#[get("/block_height")] // <- define path parameters
pub async fn block_height() -> Result<impl Responder> {
    tracing::debug!(target: TRADING_METRICS, "block_height request");
    Ok(write_response(get_block_height().await))
}

/// Get daily Perpetual trading information
///
/// This api will return `trading_fee`, `trading_count`, `trading_user_count`, `liquidation_amount`: `liquidation_count` and `opening_count` infomation of OrderlyNetwork between `from_days` and `end_days`.
#[utoipa::path(
    responses(
        (status = 200, description = "Get Daily Orderly Perp Response", body = QeuryServiceResponse<DailyData<OrderlyPerpDaily>>),
        (status = 409, description = "Invalid Request")
    ),
    params(
        ("param" = DailyRequest, Query, description = "daily orderly perp params \n
example: {\"from_day\": \"2025-07-12\", \"end_day\": \"2025-07-17\" }
        ")
    ),
)]
#[get("/daily_orderly_perp")] // <- define path parameters
pub async fn get_daily_orderly_perp(
    _req: HttpRequest,
    param: web::Query<DailyRequest>,
) -> HttpResponse {
    tracing::debug!(target: TRADING_METRICS, "daily_orderly_perp from day: {}, end_day: {}", param.from_day, param.end_day);
    let (from_time, end_time) = param.parse_day();
    write_response(daily_orderly_perp(from_time, end_time).await)
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

/// Get daily Trading volume information
///
#[utoipa::path(
    responses(
        (status = 200, description = "Get Daily Trading Volume", body = QeuryServiceResponse<DailyVolumeExtern>),
        (status = 409, description = "Invalid Request")
    ),
    params(("param" = DailyRequest, Query, 
        description = "Daily volume params, with day format %Y-%m-%d \n
example: {\"from_day\": \"2025-07-12\", \"end_day\": \"2025-07-17\" }
        "
    )),
)]
#[get("/daily_volume")] // <- define path parameters
pub async fn daily_volume(param: web::Query<DailyRequest>) -> HttpResponse {
    tracing::debug!(target: TRADING_METRICS, "daily_volume from day: {}, end_day: {}", param.from_day, param.end_day);
    let (from_time, end_time) = param.parse_day();
    write_response(get_daily_volume(from_time, end_time).await)
}

/// Get daily Trading fee information
///
#[utoipa::path(
    responses(
        (status = 200, description = "Get Daily Trading Fee", body = QeuryServiceResponse<DailyTradingFeeExtern>),
        (status = 409, description = "Invalid Request")
    ),
    params(("param" = DailyRequest, Query, 
        description = "Daily volume params \n
example: {\"from_day\": \"2025-07-12\", \"end_day\": \"2025-07-17\" }
        "
    )),
)]
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

/// Get daily Trading Volume ranking information
///
#[utoipa::path(
    responses(
        (status = 200, description = "Get Trading Volume ranking response", body = QeuryServiceResponse<TradingVolumeRanking>),
        (status = 409, description = "Invalid Request")
    ),
    params(("param" = VolumeRankingRequest, Query, 
        description = "Position ranking params, days param is the range of recent days \n
example: {\"days\": 1, \"size\": 10}
        "
    )),
)]
#[get("/ranking/trading_volume")]
pub async fn get_trading_volume_rank(param: web::Query<VolumeRankingRequest>) -> HttpResponse {
    tracing::debug!(target: TRADING_METRICS, "/ranking/trading_volume request, days: {}, size: {}", param.days, param.size);
    write_response(get_daily_trading_volume_ranking(param.to_hour(), param.size as i64).await)
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

/// Get Pnl Ranking recent days information
///
#[utoipa::path(
    responses(
        (status = 200, description = "Get Pnl Ranking recent days response", body = QeuryServiceResponse<Vec<TradingPnlRanking>>),
        (status = 409, description = "Invalid Request")
    ),
    params(("param" = PnlRankingRequest, Query, 
        description = "Pnl ranking params, symbol is optional \n
example1: {\"days\": 3, \"size\": 10, \"symbol\": \"PERP_BTC_USDC\"} \n
example2: {\"days\": 3, \"size\": 10} \n
        "
    )),
)]
#[get("/ranking/recent_days_perp_pnl")]
pub async fn get_perp_recent_days_pnl_rank(param: web::Query<PnlRankingRequest>) -> HttpResponse {
    tracing::debug!(target: TRADING_METRICS, "/ranking/pnl, days: {}, size: {}", param.days, param.size);
    write_response(
        get_pnl_ranking(
            param.to_hour(),
            param.size as i64,
            param.symbol.as_ref().map(|s| cal_symbol_hash(&s)),
        )
        .await,
    )
}

/// Get token deposit in recent days information
///
#[utoipa::path(
    responses(
        (status = 200, description = "Get Token Deposit in recent days response", body = QeuryServiceResponse<Vec<TokenAmountRanking>>),
        (status = 409, description = "Invalid Request")
    ),
    params(("param" = DepositWithdrawRankingRequest, Query, 
        description = "Token Depoit ranking params \n
example1: {\"days\": 3, \"size\": 10, \"token\": \"USDC\"} \n
        "
    )),
)]
#[get("/ranking/deposit")]
pub async fn get_token_deposit_rank(
    param: web::Query<DepositWithdrawRankingRequest>,
) -> HttpResponse {
    tracing::debug!(target: TRADING_METRICS, "/ranking/deposit, days: {}, size: {}", param.days, param.size);
    const MAX_DAYS: u32 = 90;
    if param.days > MAX_DAYS {
        let resp = FailureResponse::new(
            DAYS_RAGE_OVER_LIMIT,
            format!(
                "{}, max days is {}",
                DAYS_RAGE_OVER_LIMIT_ERR_MESSAGE, MAX_DAYS
            ),
        );
        return HttpResponse::Ok().json(resp);
    }
    const MAX_SIZE: u32 = 100;
    if param.size > MAX_SIZE {
        let resp = FailureResponse::new(
            SIZE_OVER_LIMIT,
            format!("{}, max size is {}", SIZE_OVER_LIMIT_ERR_MESSAGE, MAX_SIZE),
        );
        return HttpResponse::Ok().json(resp);
    }
    write_response(
        get_token_ranking(
            param.to_hour(),
            param.size as i64,
            false,
            cal_symbol_hash(&param.token),
        )
        .await,
    )
}

/// Get token withdrawal in recent days information
///
#[utoipa::path(
    responses(
        (status = 200, description = "Get Token Withdraw in recent days response", body = QeuryServiceResponse<Vec<TokenAmountRanking>>),
        (status = 409, description = "Invalid Request")
    ),
    params(("param" = DepositWithdrawRankingRequest, Query, 
        description = "Token Withdraw ranking params \n
example1: {\"days\": 3, \"size\": 10, \"token\": \"USDC\"} \n
        "
    )),
)]
#[get("/ranking/withdraw")]
pub async fn get_token_withdraw_rank(
    _req: HttpRequest,
    param: web::Query<DepositWithdrawRankingRequest>,
) -> HttpResponse {
    tracing::debug!(target: TRADING_METRICS, "/ranking/withdraw, days: {}, size: {}", param.days, param.size);
    const MAX_DAYS: u32 = 90;
    if param.days > MAX_DAYS {
        let resp = FailureResponse::new(
            DAYS_RAGE_OVER_LIMIT,
            format!(
                "{}, max days is {}",
                DAYS_RAGE_OVER_LIMIT_ERR_MESSAGE, MAX_DAYS
            ),
        );
        return HttpResponse::Ok().json(resp);
    }
    const MAX_SIZE: u32 = 100;
    if param.size > MAX_SIZE {
        let resp = FailureResponse::new(
            SIZE_OVER_LIMIT,
            format!("{}, max size is {}", SIZE_OVER_LIMIT_ERR_MESSAGE, MAX_SIZE),
        );
        return HttpResponse::Ok().json(resp);
    }
    write_response(
        get_token_ranking(
            param.to_hour(),
            param.size as i64,
            false,
            cal_symbol_hash(&param.token),
        )
        .await,
    )
}

/// Get User perp position info ranking by holding
///
/// This api will retur `account_id`, `address`,`broker_id`,`symbol`,`symbol_hash`,`holding`,`total_realized_pnl`,`index_price`,`mark_price`,`holding_value`,`opening_cost`,`average_entry_price`,`un_realized_pnl` informations
///
#[utoipa::path(
    responses(
        (status = 200, description = "Get Position ranking", body = QeuryServiceResponse<UserSummaryRankExtern>),
        (status = 409, description = "Invalid Request")
    ),
    params(("param" = PositionRankingRequest, Query, description = "position ranking params, `address` nor `broker_id` should not apear with `account_id`, `symbol` is optional, `offset` and `limit` has default value of `0` and `30`")),
)]
#[get("/ranking/positions")]
pub async fn get_position_rank(
    _req: HttpRequest,
    mut param: web::Query<PositionRankingRequest>,
) -> HttpResponse {
    tracing::info!(target: TRADING_METRICS, "/ranking/positions, params: {:?}", param.0);
    if param.limit > 200 {
        return write_failed_response(QUERY_OVER_LIMIT_ERR, "query number over limit 200");
    }
    if param.limit + param.offset > 1000 {
        return write_failed_response(
            QUERY_OVER_LIMIT_ERR,
            "query offset + number over limit 1000",
        );
    }
    if param.limit == 0 {
        return write_failed_response(QUERY_OVER_LIMIT_ERR, "query number should not be 0");
    }
    if let Some(address) = &param.address {
        param.address = Some(address.to_lowercase());
    }
    if param.account_id.is_none() {
        if let Some(address) = &param.address {
            // filter by address & broker id
            return match query_user_perp_holding_by_address_and_opt_broker(
                param.offset,
                param.limit,
                address.to_string(),
                param.symbol.clone(),
                param.broker_id.clone(),
            )
            .await
            {
                Ok(user_perp_holding) => {
                    let user_perp_holding = user_perp_holding
                        .into_iter()
                        .map(Into::into)
                        .collect::<Vec<UserSumaryRankingData>>();
                    write_response(UserSummaryRankExtern::new(user_perp_holding))
                }
                Err(err) => write_failed_response(QUERY_OVER_EXECUTION_ERR, &err.to_string()),
            };
        }
    }
    if !param.check_account_id_valid_and_cal() {
        return write_failed_response(
            ACCOUNT_ID_CONFLICT_OR_INVALID_ERR,
            ACCOUNT_ID_CONFLICT_OR_INVALID_ERR_MESSAGE,
        );
    }
    if param.broker_id.is_some() && param.address.is_none() {
        let broker_id = param.broker_id.clone().unwrap();
        // filter by broker id
        if param.symbol.is_none() {
            if param.offset + param.limit <= 1000 {
                if let Some(broker_top_positions) = BROKER_TOP_POSITIONS.get(&broker_id) {
                    let top_positions = broker_top_positions.read();
                    if top_positions.1.elapsed().as_secs() < 60 {
                        if (param.offset + param.limit) as usize <= top_positions.0.len() {
                            // use cache and return
                            let resp_data = top_positions.0
                                [param.offset as usize..(param.offset + param.limit) as usize]
                                .to_vec();
                            return write_response(UserSummaryRankExtern::new(resp_data));
                        }
                    }
                }
                match query_user_perp_max_symbol_holding(
                    0,
                    1000,
                    None,
                    None,
                    param.broker_id.clone(),
                )
                .await
                {
                    Ok(user_perp_holding) => {
                        tracing::info!(
                            "read from db and update cache for broker: {}, user_perp_holding len {}",
                            broker_id, user_perp_holding.len()
                        );
                        let user_perp_holding = user_perp_holding
                            .into_iter()
                            .map(Into::into)
                            .collect::<Vec<UserSumaryRankingData>>();
                        let resp_data = if param.offset as usize >= user_perp_holding.len() {
                            vec![]
                        } else {
                            let end = usize::min(
                                user_perp_holding.len(),
                                (param.offset + param.limit) as usize,
                            );
                            user_perp_holding[param.offset as usize..end].to_vec()
                        };
                        BROKER_TOP_POSITIONS.insert(
                            broker_id,
                            Arc::new(RwLock::new((user_perp_holding, Instant::now()))),
                        );
                        return write_response(UserSummaryRankExtern::new(resp_data));
                    }
                    Err(err) => {
                        return write_failed_response(QUERY_OVER_EXECUTION_ERR, &err.to_string());
                    }
                }
            }
        }
        let symbol_hash = if let Some(symbol) = &param.symbol {
            Some(cal_symbol_hash(symbol.as_str()))
        } else {
            None
        };
        // broker and symbol exist or offset + limit >= 1000, read from db and no cache
        return match query_user_perp_max_symbol_holding(
            param.offset,
            param.limit,
            None,
            symbol_hash,
            param.broker_id.clone(),
        )
        .await
        {
            Ok(user_perp_holding) => {
                let user_perp_holding = user_perp_holding
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<UserSumaryRankingData>>();
                write_response(UserSummaryRankExtern::new(user_perp_holding))
            }
            Err(err) => {
                return write_failed_response(QUERY_OVER_EXECUTION_ERR, &err.to_string());
            }
        };
    }

    if param.account_id.is_none() && param.symbol.is_none() {
        if param.offset + param.limit <= 1000 {
            let top_position_caches = TOP_POSITIONS.read();
            if (param.offset + param.limit) as usize <= top_position_caches.len() {
                let resp_data = top_position_caches
                    [param.offset as usize..(param.offset + param.limit) as usize]
                    .to_vec();
                return write_response(UserSummaryRankExtern::new(resp_data));
            }
        }
        return match query_user_perp_max_symbol_holding(param.offset, param.limit, None, None, None)
            .await
        {
            Ok(user_perp_holding) => {
                let user_perp_holding = user_perp_holding
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<UserSumaryRankingData>>();
                write_response(UserSummaryRankExtern::new(user_perp_holding))
            }
            Err(err) => {
                return write_failed_response(QUERY_OVER_EXECUTION_ERR, &err.to_string());
            }
        };
    }

    // symbol is some
    if let Some(symbol) = &param.symbol {
        let symbol_hash = cal_symbol_hash(symbol.as_str());
        // symbol is some and account_id is none
        if param.account_id.is_none() {
            if param.offset + param.limit <= 1000 {
                if let Some(top_position_caches) = SYMBOL_TOP_POSITIONS.get(&symbol_hash) {
                    let top_positions = top_position_caches.read();
                    if top_positions.1.elapsed().as_secs() < 60 {
                        if (param.offset + param.limit) as usize <= top_positions.0.len() {
                            // use cache and return
                            let resp_data = top_positions.0
                                [param.offset as usize..(param.offset + param.limit) as usize]
                                .to_vec();
                            return write_response(UserSummaryRankExtern::new(resp_data));
                        }
                    }
                }

                match query_user_perp_max_symbol_holding(
                    0,
                    1000,
                    None,
                    Some(symbol_hash.clone()),
                    None,
                )
                .await
                {
                    Ok(user_perp_holding) => {
                        tracing::info!(
                            "read from db and update cache for symbol hash: {}",
                            symbol_hash
                        );
                        let user_perp_holding = user_perp_holding
                            .into_iter()
                            .map(Into::into)
                            .collect::<Vec<UserSumaryRankingData>>();
                        let resp_data = if param.offset as usize >= user_perp_holding.len() {
                            vec![]
                        } else {
                            let end = usize::min(
                                user_perp_holding.len(),
                                (param.offset + param.limit) as usize,
                            );
                            user_perp_holding[param.offset as usize..end].to_vec()
                        };
                        SYMBOL_TOP_POSITIONS.insert(
                            symbol_hash,
                            Arc::new(RwLock::new((user_perp_holding, Instant::now()))),
                        );
                        return write_response(UserSummaryRankExtern::new(resp_data));
                    }
                    Err(err) => {
                        return write_failed_response(QUERY_OVER_EXECUTION_ERR, &err.to_string());
                    }
                }
            }

            return match query_user_perp_max_symbol_holding(
                param.offset,
                param.limit,
                None,
                Some(symbol_hash.clone()),
                None,
            )
            .await
            {
                Ok(user_perp_holding) => {
                    let user_perp_holding = user_perp_holding
                        .into_iter()
                        .map(Into::into)
                        .collect::<Vec<UserSumaryRankingData>>();
                    write_response(UserSummaryRankExtern::new(user_perp_holding))
                }
                Err(err) => {
                    return write_failed_response(QUERY_OVER_EXECUTION_ERR, &err.to_string());
                }
            };
        }
    }

    // account_id is some, symbol may be someor none
    let account_id = param.account_id.clone().unwrap_or_default();
    let symbol_hash = param.symbol.clone().map(|s| cal_symbol_hash(&s));
    match query_user_perp_max_symbol_holding(
        param.offset,
        param.limit,
        Some(account_id),
        symbol_hash,
        None,
    )
    .await
    {
        Ok(user_perp_holding) => {
            let user_perp_holding = user_perp_holding
                .into_iter()
                .map(Into::into)
                .collect::<Vec<UserSumaryRankingData>>();
            write_response(UserSummaryRankExtern::new(user_perp_holding))
        }
        Err(err) => {
            return write_failed_response(QUERY_OVER_EXECUTION_ERR, &err.to_string());
        }
    }
}

/// Get user perpetual info ranking by realized_pnl information
///
#[utoipa::path(
    responses(
        (status = 200, description = "Get Realized pnl ranking response", body = QeuryServiceResponse<UserSummaryRankExtern>),
        (status = 409, description = "Invalid Request")
    ),
    params(
        (
            "param" = RealizedPnlRankingRequest,
            Query,
            description = "pnl ranking params \n 
example1: {\"account_id\": \"0x85cf9694ff45a0230bb572d9c982126b124036e5bc790c285387d31e4fb482ad\", \"symbol\": \"PERP_BTC_USDC\", \"offset\": 0, \"limit\": 10, \"order_by\": \"DESC\"} \n
example2: {\"address\": \"0x32831ca2efa20ae6340224bc353d4b241b3d2541\", \"broker_id\": \"woofi_pro\", \"symbol\": \"PERP_BTC_USDC\", \"offset\": 0, \"limit\": 10, \"order_by\": \"DESC\"}"
        )
    ),
)]
#[get("/ranking/realized_pnl")]
pub async fn get_realized_pnl_rank(
    _req: HttpRequest,
    mut param: web::Query<RealizedPnlRankingRequest>,
) -> HttpResponse {
    tracing::debug!(target: TRADING_METRICS, "/ranking/realized_pnl, params: {:?}", param.0);
    if param.limit > 200 {
        return write_failed_response(QUERY_OVER_LIMIT_ERR, "query number over limit 200");
    }
    if param.limit == 0 {
        return write_failed_response(QUERY_OVER_LIMIT_ERR, "query number should not be 0");
    }
    if param.offset + param.limit > 1000 {
        return write_failed_response(
            QUERY_OVER_LIMIT_ERR,
            "query offset + limit should less than 1000",
        );
    }
    if !param.check_account_id_valid_and_cal() {
        return write_failed_response(
            ACCOUNT_ID_CONFLICT_OR_INVALID_ERR,
            ACCOUNT_ID_CONFLICT_OR_INVALID_ERR_MESSAGE,
        );
    }

    if param.account_id.is_none() && param.symbol.is_none() {
        if param.offset + param.limit <= 1000 {
            let top_position_caches = match param.order_by {
                RealizedPnlRankingOrder::ASC => TOP_REALIZED_PNL_ASC.read(),
                RealizedPnlRankingOrder::DESC => TOP_REALIZED_PNL_DESC.read(),
            };
            if (param.offset + param.limit) as usize <= top_position_caches.len() {
                let resp_data = top_position_caches
                    [param.offset as usize..(param.offset + param.limit) as usize]
                    .to_vec();
                return write_response(UserSummaryRankExtern::new(resp_data));
            }
        }
        return match query_user_perp_max_symbol_realized_pnl(
            param.offset,
            param.limit,
            None,
            None,
            param.order_by.to_string(),
        )
        .await
        {
            Ok(user_perp_holding) => {
                let user_perp_holding = user_perp_holding
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<UserSumaryRankingData>>();
                write_response(UserSummaryRankExtern::new(user_perp_holding))
            }
            Err(err) => {
                return write_failed_response(QUERY_OVER_EXECUTION_ERR, &err.to_string());
            }
        };
    }

    // symbol is some
    if let Some(symbol) = &param.symbol {
        let symbol_hash = cal_symbol_hash(symbol.as_str());
        // symbol is some and account_id is none
        if param.account_id.is_none() {
            if param.offset + param.limit <= 1000 {
                let top_realized_pnl_caches = match param.order_by {
                    RealizedPnlRankingOrder::ASC => SYMBOL_TOP_REALIZED_PNL_ASC.get(&symbol_hash),
                    RealizedPnlRankingOrder::DESC => SYMBOL_TOP_REALIZED_PNL_DESC.get(&symbol_hash),
                };
                if let Some(top_position_caches) = top_realized_pnl_caches {
                    let top_positions = top_position_caches.read();
                    if top_positions.1.elapsed().as_secs() < 5 {
                        if (param.offset + param.limit) as usize <= top_positions.0.len() {
                            // use cache and return
                            let resp_data = top_positions.0
                                [param.offset as usize..(param.offset + param.limit) as usize]
                                .to_vec();
                            return write_response(UserSummaryRankExtern::new(resp_data));
                        }
                    }
                }

                match query_user_perp_max_symbol_realized_pnl(
                    0,
                    1000,
                    None,
                    Some(symbol_hash.clone()),
                    param.order_by.to_string(),
                )
                .await
                {
                    Ok(user_perp_holding) => {
                        tracing::info!(
                            "read from db and update cache for symbol hash: {}",
                            symbol_hash
                        );
                        let user_perp_realized_pnl = user_perp_holding
                            .into_iter()
                            .map(Into::into)
                            .collect::<Vec<UserSumaryRankingData>>();
                        let resp_data = user_perp_realized_pnl
                            [param.offset as usize..(param.offset + param.limit) as usize]
                            .to_vec();
                        match param.order_by {
                            RealizedPnlRankingOrder::ASC => {
                                SYMBOL_TOP_REALIZED_PNL_ASC.insert(
                                    symbol_hash,
                                    Arc::new(RwLock::new((user_perp_realized_pnl, Instant::now()))),
                                );
                            }
                            RealizedPnlRankingOrder::DESC => {
                                SYMBOL_TOP_REALIZED_PNL_DESC.insert(
                                    symbol_hash,
                                    Arc::new(RwLock::new((user_perp_realized_pnl, Instant::now()))),
                                );
                            }
                        }
                        return write_response(UserSummaryRankExtern::new(resp_data));
                    }
                    Err(err) => {
                        return write_failed_response(QUERY_OVER_EXECUTION_ERR, &err.to_string());
                    }
                }
            }

            return match query_user_perp_max_symbol_realized_pnl(
                param.offset,
                param.limit,
                None,
                Some(symbol_hash.clone()),
                param.order_by.to_string(),
            )
            .await
            {
                Ok(user_perp_holding) => {
                    let user_perp_holding = user_perp_holding
                        .into_iter()
                        .map(Into::into)
                        .collect::<Vec<UserSumaryRankingData>>();
                    write_response(UserSummaryRankExtern::new(user_perp_holding))
                }
                Err(err) => {
                    return write_failed_response(QUERY_OVER_EXECUTION_ERR, &err.to_string());
                }
            };
        }
    }

    // account_id is some, symbol may be someor none
    let account_id = param.account_id.clone().unwrap_or_default();
    let symbol_hash = param.symbol.clone().map(|s| cal_symbol_hash(&s));
    match query_user_perp_max_symbol_realized_pnl(
        param.offset,
        param.limit,
        Some(account_id),
        symbol_hash,
        param.order_by.to_string(),
    )
    .await
    {
        Ok(user_perp_holding) => {
            let user_perp_holding = user_perp_holding
                .into_iter()
                .map(Into::into)
                .collect::<Vec<UserSumaryRankingData>>();
            write_response(UserSummaryRankExtern::new(user_perp_holding))
        }
        Err(err) => {
            return write_failed_response(QUERY_OVER_EXECUTION_ERR, &err.to_string());
        }
    }
}
