use actix_cors::Cors;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::{get, options, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;

use config::{CommonConfig, Opts};
use trading_metrics::{average_trading_count, daily_trading_fee, daily_volume};

use crate::db::init_analyzer_db_url;
use crate::status::get_status;
use crate::trading_metrics::{
    average_opening_count, average_trading_fee, average_trading_volume, block_height,
    get_daily_orderly_perp, get_daily_orderly_token, get_perp_holding_rank, get_perp_pnl_rank,
    get_token_deposit_rank, get_token_withdraw_rank, get_trading_volume_rank,
};

mod config;
mod db;
mod format_extern;
mod network_info;
mod status;
mod trading_metrics;
use crate::network_info::{get_network_info, init_indexer_db_url};
const ORDERLY_DASHBOARD_CONTEXT: &str = "orderly_dashboard_context";

fn add_base_header(resp: &mut HttpResponse) {
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

pub fn set_envs() {
    std::env::set_var("RUST_BACKTRACE", "1");
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
    tracing::info!(target: ORDERLY_DASHBOARD_CONTEXT, "orderly dashboard config info: {:?}", config);
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
            .service(hello)
            .service(echo)
            .service(hello2)
            .service(daily_volume)
            .service(daily_trading_fee)
            .service(average_trading_count)
            .service(average_trading_fee)
            .service(average_trading_volume)
            .service(get_trading_volume_rank)
            .service(get_daily_orderly_perp)
            .service(get_daily_orderly_token)
            .service(get_perp_holding_rank)
            .service(average_opening_count)
            .service(get_perp_pnl_rank)
            .service(get_token_deposit_rank)
            .service(get_token_withdraw_rank)
            .service(block_height)
            .service(get_network_info)
            .service(get_status)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
