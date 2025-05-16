pub mod rank_metrics;
pub mod trading_metrics;

use crate::db::raw_request::ExecutionResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct Response<T> {
    pub success: bool,
    pub err_code: i32,
    pub err_msg: Option<String>,
    pub data: Option<T>,
}

impl<T> Response<T> {
    #[allow(dead_code)]
    fn new_ok() -> Response<T> {
        Response {
            success: true,
            err_code: 0,
            err_msg: None,
            data: None,
        }
    }

    pub fn new_err(err_code: i32, err_msg: String) -> Response<T> {
        Response {
            success: false,
            err_code,
            err_msg: Some(err_msg),
            data: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawQueryRequest {
    pub query_str: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct RawQueryResponse {
    pub query_result: ExecutionResult,
}

impl RawQueryResponse {
    pub fn new(query_result: ExecutionResult) -> RawQueryResponse {
        RawQueryResponse { query_result }
    }
}
