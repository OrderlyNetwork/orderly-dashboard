#![feature(unwrap_infallible)]
#[macro_use]
extern crate diesel;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use clap::Parser;

use crate::analyzer::analyzer_job::start_analyzer_job;
use crate::config::{AnalyzerConfig, Opts};
use crate::db::{get_database_credentials, init_database_url};

#[allow(unused_assignments)]
mod analyzer;
mod client;
mod config;
mod db;
mod schema;

#[allow(dead_code)]
const ORDERLY_DASHBOARD_ANALYZER: &str = "orderly-dashboard-analyzer";

fn init_log() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(std::io::stderr)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}

fn start_analyze_job() {
    let opts = Opts::parse();
    let raw_common_config =
        std::fs::read_to_string(&opts.config_path).expect("missing_common_config_file");
    let config: AnalyzerConfig =
        serde_json::from_str(&raw_common_config).expect("unable_to_deserialize_common_configs");

    tracing::info!(target:ORDERLY_DASHBOARD_ANALYZER,"config loaded: {:?}",config);
    start_analyzer_job(
        config.pull_interval,
        config.indexer_address,
        config.start_block,
    );
    tracing::info!(target:ORDERLY_DASHBOARD_ANALYZER,"start analyze block event")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is Health!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_log();
    init_database_url(get_database_credentials());
    start_analyze_job();
    HttpServer::new(|| App::new().service(health))
        .bind(("127.0.0.1", 18080))?
        .run()
        .await
}
