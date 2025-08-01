pub mod adl_result;
pub mod balance_transfer;
pub mod executed_trades;
pub mod fee_distribution;
pub mod liquidation_result;
pub mod liquidation_transfer;
pub mod partitioned_executed_trades;
pub mod rebalance_events;
pub mod serial_batches;
pub mod settings;
pub mod settlement_execution;
pub mod settlement_result;
pub mod sol_transaction_events;
pub mod swap_result_uploaded;
pub mod symbols_config;
pub mod transaction_events;

pub mod utils;
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use once_cell::sync::{Lazy, OnceCell};
use std::env;
use std::sync::atomic::{AtomicU32, Ordering};
type DbPool = Pool<AsyncPgConnection>;

pub const DB_CONTEXT: &str = "db_context";
pub(crate) const DB_CONN_ERR_MSG: &str = "Couldn't get db connection from the pool";

// pub static POOL: Lazy<Database<PgConnection>> = Lazy::new(|| establish_connection());
pub static POOL: Lazy<DbPool> = Lazy::new(|| initialize_db_pool());

pub static INITED_DATABASE_URL: OnceCell<String> = OnceCell::new();
pub static POOL_SIZE_CONFIG: AtomicU32 = AtomicU32::new(3);

#[allow(dead_code)]
pub fn init_database_setting(database_url: String, pool_size: u32) {
    INITED_DATABASE_URL.set(database_url).ok();
    if pool_size > 180 {
        tracing::warn!(target: DB_CONTEXT, "pool size too big");
    }
    POOL_SIZE_CONFIG.store(pool_size, Ordering::Relaxed);
}

pub fn get_database_credentials() -> String {
    if let Some(db_url) = INITED_DATABASE_URL.get() {
        return db_url.to_string();
    }
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file")
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel-async/latest/diesel_async/pooled_connection/index.html#modules>.
fn initialize_db_pool() -> DbPool {
    let db_url = get_database_credentials();

    let connection_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder(connection_manager)
        .max_size(POOL_SIZE_CONFIG.load(Ordering::Relaxed) as usize)
        .build()
        .expect("pool connected normal");
    pool
}
