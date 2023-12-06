use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use tokio::time::{Duration, sleep};

use crate::analyzer::block_event_analyzer::BlockEventAnalyzer;
use crate::indexer::indexer_client::IndexerClient;

mod indexer;
mod config;
mod analyzer;
mod db;
mod schema;

async fn pull_block_timer(puller: &dyn IndexerClient, analyzer: &dyn BlockEventAnalyzer) {
    loop {
        let result = puller.pull_block(1i64, 20i64);
        match result {
            Ok(result) => {
                analyzer.analyzer_event(result.as_data());
                //TODO update block summary
            }
            Err(_) => {
                //TODO log warning
            }
        }

        sleep(Duration::from_secs(5)).await; // 设置定时任务间隔为5秒
    }
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is Health!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}