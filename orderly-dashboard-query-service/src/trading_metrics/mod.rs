use crate::{
    add_base_header,
    format_extern::{
        trading_metrics::{DailyTradingFeeExtern, DailyVolumeExtern},
        Response,
    },
};
use actix_web::{get, HttpResponse, Responder, Result};
use serde_json::json;

/// extract path info using serde
#[get("/daily_volume")] // <- define path parameters
pub async fn daily_volume() -> Result<impl Responder> {
    let example_data = serde_json::to_string(&json!({
        "success": true,
        "err_code": 0,
        "err_msg": null,
        "data": {
            "daytime": ["20231213", "20231214", "20231215", "20231216", "20231217", "20231218", "20231219"],
            "volume": [1950000, 2120000, 2240000, 2170000, 1110000, 2030000, 2080000]
        }
    }))?;
    let data: Response<DailyVolumeExtern> = serde_json::from_str(&example_data)?;
    let mut resp = HttpResponse::Ok().json(data);
    add_base_header(&mut resp);

    Ok(resp)
}

/// extract path info using serde
#[get("/daily_trading_fee")] // <- define path parameters
pub async fn daily_trading_fee() -> Result<impl Responder> {
    let example_data = serde_json::to_string(&json!({
        "success": true,
        "err_code": 0,
        "err_msg": null,
        "data": {
            "daytime": ["20231213", "20231214", "20231215", "20231216", "20231217", "20231218", "20231219"],
            "fee_amount": [1950000.6, 2120000.0, 2240000.0, 2170000.8, 1110000.0, 2030000.0, 2080000.0]
        }
    }))?;
    let data: Response<DailyTradingFeeExtern> = serde_json::from_str(&example_data)?;
    let mut resp = HttpResponse::Ok().json(data);
    add_base_header(&mut resp);

    Ok(resp)
}
