use crate::format_extern::trading_metrics::{
    AccountVolumeStatistic, AccountVolumeStatisticRequest, BrokerVolumeStatistic,
    BrokerVolumeStatisticRequest,
};
use crate::format_extern::QeuryServiceResponse;
use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::{
    db::trading_metrics::volume_statistic::{
        db_get_broker_volume_statistic, get_user_volume_statistic,
    },
    error_code::{ACCOUNT_NOT_FOUND, GENERAL_ERR},
    trading_metrics::{write_failed_response, write_response, TRADING_METRICS},
};

/// Get Account Volume Statistic information
///
#[utoipa::path(
    responses(
        (status = 200, description = "Get Account Volume Statistic Response", body = QeuryServiceResponse<AccountVolumeStatistic>),
        (status = 409, description = "Invalid Request")
    ),
    params(
        ("param" = AccountVolumeStatisticRequest, Query, description = "account_id is the query parma")
    ),
)]
#[get("/get_account_volume_statistic")]
pub async fn get_account_volume_statistic(
    _req: HttpRequest,
    param: web::Query<AccountVolumeStatisticRequest>,
) -> HttpResponse {
    tracing::debug!(target: TRADING_METRICS, "get_account_volume_statistic account_id: {}", param.account_id);
    match get_user_volume_statistic(param.account_id.clone()).await {
        Ok(Some(volume)) => {
            let res: AccountVolumeStatistic = volume.into();
            write_response(res)
        }
        Ok(None) => write_failed_response(ACCOUNT_NOT_FOUND, "account_id not found"),
        Err(err) => write_failed_response(GENERAL_ERR, &format!("db query err: {}", err)),
    }
}

/// Get Broker Volume Statistic information
///
#[utoipa::path(
    responses(
        (status = 200, description = "Get Broker Volume Statistic Response", body = QeuryServiceResponse<BrokerVolumeStatistic>),
        (status = 409, description = "Invalid Request")
    ),
    params(
        ("param" = BrokerVolumeStatisticRequest, Query, description = "account_id is the query parma")
    ),
)]
#[get("/get_broker_volume_statistic")]
pub async fn get_broker_volume_statistic(
    _req: HttpRequest,
    param: web::Query<BrokerVolumeStatisticRequest>,
) -> HttpResponse {
    tracing::debug!(target: TRADING_METRICS, "get_account_volume_statistic broker_id: {}", param.broker_id);
    match db_get_broker_volume_statistic(param.broker_id.clone()).await {
        Ok(Some(volume)) => {
            let res: BrokerVolumeStatistic = volume.into();
            write_response(res)
        }
        Ok(None) => write_failed_response(ACCOUNT_NOT_FOUND, "account_id not found"),
        Err(err) => write_failed_response(GENERAL_ERR, &format!("db query err: {}", err)),
    }
}
