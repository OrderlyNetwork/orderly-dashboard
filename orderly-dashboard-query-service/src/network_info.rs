use crate::trading_metrics::write_response;
use actix_web::{get, Responder, Result};
use dotenv::dotenv;
use orderly_dashboard_indexer::db::settings::get_db_network_info;
use orderly_dashboard_indexer::formats_external::NetworkInfo;
use std::env;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

pub fn init_indexer_db_url() {
    orderly_dashboard_indexer::db::init_database_setting(get_database_credentials(), 1);
}

fn get_database_credentials() -> String {
    dotenv().ok();
    env::var("INDEXER_DATABASE_URL").expect("INDEXER_DATABASE_URL must be set in .env file")
}

static LAST_CALL: AtomicI64 = AtomicI64::new(0);
static CACHE_HEIGHT: AtomicU64 = AtomicU64::new(0);

#[get("/network_info")]
pub async fn get_network_info() -> Result<impl Responder> {
    let now = chrono::Utc::now().timestamp();
    let last = LAST_CALL.load(Ordering::Relaxed);
    let cache_height = CACHE_HEIGHT.load(Ordering::Relaxed);
    if (last - now).abs() > 5 || cache_height == 0 {
        let info = get_db_network_info()
            .await
            .unwrap_or_else(|_err| NetworkInfo::default());
        if let Some(height) = info.finalized_height {
            CACHE_HEIGHT.store(height, Ordering::Relaxed);
        }
        return Ok(write_response(info));
    };
    Ok(write_response(NetworkInfo {
        finalized_height: Some(cache_height),
    }))
}
