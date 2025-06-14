#![allow(non_local_definitions)]

#[allow(dead_code)]
pub mod api;
pub mod consume_data_task;
pub mod formats_external;
pub mod schema;
extern crate diesel;
pub mod constants;
pub mod db;
mod service_base;
pub use service_base::runtime;
pub use service_base::sdk;
pub mod config;
#[allow(dead_code)]
pub mod contract;
#[allow(dead_code)]
pub mod eth_rpc;

#[allow(dead_code)]
pub mod bindings;
pub mod handler;
#[allow(dead_code)]
pub(crate) mod init;
pub mod transform;
pub mod utils;
pub use formats_external::trading_events::Events;
