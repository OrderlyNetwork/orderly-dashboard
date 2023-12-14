#![feature(unwrap_infallible)]
#[macro_use]
extern crate diesel;

use std::any::Any;
use std::fmt::Debug;
use std::str::FromStr;

use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use chrono::TimeZone;
use clap::Parser;
use reqwest;

use crate::analyzer::analyzer_job::start_analyzer_job;

mod analyzer;
mod db;
mod schema;
mod config;

const ORDERLY_DASHBOARD_ANALYZER: &str = "orderly-dashboard-analyzer";

fn init_log() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(std::io::stderr)
        // .with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is Health!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    init_log();
    start_analyzer_job(5, "http://localhost:8018".to_string(), 1145079);
    HttpServer::new(|| {
        App::new()
            .service(health)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}