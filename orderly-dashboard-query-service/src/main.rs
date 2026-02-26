#![allow(non_local_definitions)]
mod monitor;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, options, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;

use orderly_dashboard_query_service::config::init_config;
use orderly_dashboard_query_service::config::{CommonConfig, Opts};
use orderly_dashboard_query_service::trading_metrics::{
    average_trading_count, daily_trading_fee, daily_volume,
};
use orderly_dashboard_query_service::{add_base_header, ORDERLY_DASHBOARD_CONTEXT};

use orderly_dashboard_query_service::db::init_analyzer_db_url;
use orderly_dashboard_query_service::events::events_api::{
    list_events, list_events_v2, list_sol_events,
};
use orderly_dashboard_query_service::indexer_db::init_indexer_db_url;
use orderly_dashboard_query_service::network_info::get_network_info;
use orderly_dashboard_query_service::raw_query::analyzer_raw_query;
use orderly_dashboard_query_service::service_base::runtime::spawn_future;
use orderly_dashboard_query_service::status::get_status;
use orderly_dashboard_query_service::swagger_docs::ApiDoc;
use orderly_dashboard_query_service::trades::{get_trades_status, query_trades, update_settings_task};
use orderly_dashboard_query_service::trading_metrics;
use orderly_dashboard_query_service::trading_metrics::{
    average_opening_count, average_trading_fee, average_trading_volume, block_height,
    deposit_gas_fee, event_gas_fee, get_account_volume_statistic, get_broker_volume_statistic,
    get_daily_orderly_token, get_perp_holding_rank, get_perp_recent_days_pnl_rank, perp_gas_fee,
    update_positions_task, update_realized_pnl_task,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[get("/")]
async fn hello() -> impl Responder {
    println!("request /");
    let mut resp = HttpResponse::Ok().body("data from query service");
    add_base_header(&mut resp);
    resp
}

#[options("/")]
async fn hello2() -> impl Responder {
    println!("request options");
    let mut resp = HttpResponse::Ok().body("");
    add_base_header(&mut resp);

    resp
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn init_log(is_debug: bool) {
    let log_level = if is_debug {
        tracing_subscriber::filter::LevelFilter::DEBUG
    } else {
        tracing_subscriber::filter::LevelFilter::INFO
    };
    tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(std::io::stderr)
        .with_max_level(log_level)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}

fn init(is_debug: bool) {
    init_log(is_debug);
    init_analyzer_db_url();
    init_indexer_db_url()
}

fn set_envs() {
    std::env::set_var("RUST_BACKTRACE", "1");
}

fn service_config(conf: &mut web::ServiceConfig) {
    conf.service(trading_metrics::get_daily_orderly_perp)
        .service(daily_volume)
        .service(daily_trading_fee)
        .service(trading_metrics::get_trading_volume_rank)
        .service(trading_metrics::get_position_rank)
        .service(trading_metrics::get_realized_pnl_rank)
        .service(trading_metrics::get_token_deposit_rank)
        .service(trading_metrics::get_token_withdraw_rank)
        .service(get_perp_recent_days_pnl_rank)
        .service(list_events)
        .service(list_events_v2)
        .service(get_account_volume_statistic)
        .service(get_broker_volume_statistic);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    set_envs();
    let opts = Opts::parse();
    let raw_common_config =
        std::fs::read_to_string(&opts.config_path).expect("missing_common_config_file");
    let config: CommonConfig =
        serde_json::from_str(&raw_common_config).expect("unable_to_deserialize_common_configs");
    init(config.is_debug);
    init_config(config.clone());
    tracing::info!(target: ORDERLY_DASHBOARD_CONTEXT, "orderly dashboard config info: {:?}, cpu num: {}", config, num_cpus::get());
    std::thread::spawn(|| {
        spawn_future(async {
            tracing::info!(target: ORDERLY_DASHBOARD_CONTEXT, "start new thread pool");
            let monitor = crate::monitor::memory::MemoryMonitor::new(
                12288.0, // 12GB
                8192.0,  // 8GB
                std::time::Duration::from_secs(60),
            );
            monitor.start_monitoring().await;
            Ok(())
        });
    });
    crate::update_positions_task();
    crate::update_realized_pnl_task();
    // Start settings update task
    std::thread::spawn(|| {
        spawn_future(async {
            update_settings_task().await
        });
    });
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                header::ACCESS_CONTROL_ALLOW_METHODS,
            ])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .configure(service_config)
            .service(hello)
            .service(echo)
            .service(hello2)
            .service(average_trading_count)
            .service(average_trading_fee)
            .service(average_trading_volume)
            .service(perp_gas_fee)
            .service(event_gas_fee)
            .service(deposit_gas_fee)
            .service(get_daily_orderly_token)
            // duplicate with get_position_rank
            .service(get_perp_holding_rank)
            .service(average_opening_count)
            // duplicate with get_realized_pnl_rank
            .service(block_height)
            .service(get_network_info)
            .service(analyzer_raw_query)
            .service(get_status)
            .service(list_sol_events)
            .service(query_trades)
            .service(get_trades_status)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route("/hey", web::get().to(manual_hello))
    })
    .workers(num_cpus::get() * 2)
    .backlog(1024)
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
