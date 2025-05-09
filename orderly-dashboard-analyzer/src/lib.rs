#![allow(non_local_definitions)]
#![warn(dead_code)]
#[macro_use]
extern crate diesel;
pub mod analyzer;
pub mod db;
pub mod schema;
pub mod sync_broker;
