#![feature(unwrap_infallible)]
#[macro_use]
extern crate diesel;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use clap::Parser;

use crate::analyzer::analyzer_job::start_analyzer_job;
use crate::config::{AnalyzerConfig, Opts};
use crate::db::init_database_url;

mod analyzer;
mod config;
mod db;
mod schema;

#[allow(dead_code)]
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

    let opts = Opts::parse();
    let raw_common_config =
        std::fs::read_to_string(&opts.config_path).expect("missing_common_config_file");
    let config: AnalyzerConfig =
        serde_json::from_str(&raw_common_config).expect("unable_to_deserialize_common_configs");
    init_database_url(config.database_url);

    start_analyzer_job(
        config.pull_interval,
        config.indexer_address,
        config.start_block,
    );

    HttpServer::new(|| App::new().service(health))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
