pub mod trading_metrics;
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

    #[allow(dead_code)]
    fn new_err(err_code: i32, err_msg: String) -> Response<T> {
        Response {
            success: false,
            err_code,
            err_msg: Some(err_msg),
            data: None,
        }
    }
}
