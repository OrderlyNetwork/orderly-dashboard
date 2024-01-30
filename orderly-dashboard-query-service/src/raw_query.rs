use crate::db::raw_request::raw_query;
use crate::error_code::{
    GENERAL_ERR, RAW_QUERY_EXECUTE_ERR, RAW_QUERY_EXECUTE_ERR_MESSAGE, RAW_QUERY_OVERLIMIT_ERR,
    RAW_QUERY_OVERLIMIT_ERR_MESSAGE,
};
use crate::format_extern::{RawQueryRequest, RawQueryResponse};
use crate::trading_metrics::{write_failed_response, write_response};
use actix_web::{post, web, Responder, Result};

#[post("/analyzer_raw_query")] // <- define path parameters
pub async fn analyzer_raw_query(param: web::Json<RawQueryRequest>) -> Result<impl Responder> {
    let query_str = param.query_str.clone();
    match raw_query(query_str).await {
        Ok((code, res)) => {
            if code == 0 {
                Ok(write_response(RawQueryResponse::new(res)))
            } else if code == RAW_QUERY_OVERLIMIT_ERR {
                Ok(write_failed_response(code, RAW_QUERY_OVERLIMIT_ERR_MESSAGE))
            } else if code == RAW_QUERY_EXECUTE_ERR {
                Ok(write_failed_response(code, RAW_QUERY_EXECUTE_ERR_MESSAGE))
            } else {
                Ok(write_failed_response(code, "raw_query execute failed"))
            }
        }
        Err(err) => Ok(write_failed_response(GENERAL_ERR, &err.to_string())),
    }
}
