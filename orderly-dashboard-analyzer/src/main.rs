#[macro_use]
#[warn(deprecated)]
extern crate diesel;

use crate::analyzer::analyzer_gas_job::start_analyzer_gas_job;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use orderly_dashboard_analyzer::sync_broker::start_sync_brokers;
use serde_json::json;

use crate::analyzer::analyzer_job::start_analyzer_trade_job;
use crate::config::{AnalyzerConfig, Opts};
use crate::db::{get_database_credentials, init_database_url};

#[allow(unused_assignments)]
pub mod analyzer;
mod client;
mod config;
mod db;
mod schema;
mod sync_broker;

#[allow(dead_code)]
const ORDERLY_DASHBOARD_ANALYZER: &str = "orderly-dashboard-analyzer";

fn init_log() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(std::io::stderr)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}

fn start_analyze_job(config: AnalyzerConfig) {
    tracing::info!(target:ORDERLY_DASHBOARD_ANALYZER,"config loaded: {:?}",config);
    start_analyzer_trade_job(
        config.pull_interval,
        config.indexer_address.clone(),
        config.start_block,
        config.start_block_timestamp,
        config.batch_block_num,
    );
    start_analyzer_gas_job(
        config.pull_interval,
        config.indexer_address.clone(),
        config.start_block,
        config.batch_block_num,
    );
    tracing::info!(target:ORDERLY_DASHBOARD_ANALYZER,"start analyze block event")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is Health!")
}

#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().json(json!(
        {
            "success": true,
            "data": {
                "is_ready": true
            }
        }
    ))
}

#[warn(deprecated)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    init_log();
    let opts = Opts::parse();
    let raw_common_config =
        std::fs::read_to_string(&opts.config_path).expect("missing_common_config_file");
    let config: AnalyzerConfig =
        serde_json::from_str(&raw_common_config).expect("unable_to_deserialize_common_configs");
    init_database_url(get_database_credentials());
    let port = config.server_port;
    let sync_broker_url = config.get_broker_url.clone();

    start_analyze_job(config.clone());
    start_sync_brokers(sync_broker_url);
    HttpServer::new(|| App::new().service(health).service(status))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
