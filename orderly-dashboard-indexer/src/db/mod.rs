pub mod adl_result;
pub mod executed_trades;
pub mod fee_distribution;
pub mod liquidation_result;
pub mod liquidation_transfer;
pub mod serial_batches;
pub mod settings;
pub mod settlement_execution;
pub mod settlement_result;
pub mod symbols_config;
pub mod transaction_events;

use actix_diesel::Database;
use diesel::PgConnection;
use dotenv::dotenv;
use once_cell::sync::{Lazy, OnceCell};
use std::env;
use std::sync::atomic::{AtomicU32, Ordering};

pub const DB_CONTEXT: &str = "db_context";

pub static POOL: Lazy<Database<PgConnection>> = Lazy::new(|| establish_connection());

pub fn establish_connection() -> Database<PgConnection> {
    let database_url = get_database_credentials();
    Database::builder()
        .pool_max_size(POOL_SIZE_CONFIG.load(Ordering::Relaxed))
        .open(&database_url)
}

pub static INITED_DATABASE_URL: OnceCell<String> = OnceCell::new();
pub static POOL_SIZE_CONFIG: AtomicU32 = AtomicU32::new(8);

#[allow(dead_code)]
pub fn init_database_setting(database_url: String, pool_size: u32) {
    INITED_DATABASE_URL.set(database_url).ok();
    if pool_size > 30 {
        tracing::warn!(target: DB_CONTEXT, "pool size too big");
    }
    POOL_SIZE_CONFIG.store(pool_size, Ordering::Relaxed);
}

pub fn get_database_credentials() -> String {
    if let Some(db_url) = INITED_DATABASE_URL.get() {
        return db_url.to_string();
    }
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file")
}
