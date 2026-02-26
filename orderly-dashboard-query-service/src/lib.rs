#![allow(non_local_definitions)]
pub mod config;
pub mod db;
pub mod error_code;
pub mod events;
pub mod format_extern;
pub mod indexer_db;
pub mod network_info;
pub mod raw_query;
pub mod service_base;
pub mod status;
pub mod swagger_docs;
pub mod trades;
pub mod trading_metrics;
pub mod utils;

pub use format_extern::trading_metrics::Api;

use actix_web::HttpResponse;
use reqwest::header::{self, HeaderValue};

pub const ORDERLY_DASHBOARD_CONTEXT: &str = "orderly_dashboard_context";

pub fn add_base_header(resp: &mut HttpResponse) {
    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("*"),
    );
    resp.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("*"),
    );
}
