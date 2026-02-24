pub mod user_info;

pub const ANLYZER_DB_CONTEXT: &str = "analyzer_db_context";
pub(crate) const ANLYZER_DB_CONN_ERR_MSG: &str = "Couldn't get anlyzer db connection from the pool";

use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use once_cell::sync::{Lazy, OnceCell};
use std::env;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::db::DbPool;

static POOL: Lazy<DbPool> = Lazy::new(|| initialize_db_pool());

static INITED_DATABASE_URL: OnceCell<String> = OnceCell::new();
static POOL_SIZE_CONFIG: AtomicU32 = AtomicU32::new(2);

#[allow(dead_code)]
pub fn init_anlyzer_database_setting(database_url: String, pool_size: u32) {
    INITED_DATABASE_URL.set(database_url).ok();
    if pool_size > 2 {
        tracing::warn!(target: ANLYZER_DB_CONTEXT, "pool size too big");
    }
    POOL_SIZE_CONFIG.store(pool_size, Ordering::Relaxed);
}

pub fn get_database_credentials() -> String {
    if let Some(db_url) = INITED_DATABASE_URL.get() {
        return db_url.to_string();
    }
    env::var("ANALYZER_DATABASE_URL").expect("ANALYZER_DATABASE_URL must be set in .env file")
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel-async/latest/diesel_async/pooled_connection/index.html#modules>.
fn initialize_db_pool() -> DbPool {
    let db_url = get_database_credentials();

    let connection_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder(connection_manager)
        .max_size(POOL_SIZE_CONFIG.load(Ordering::Relaxed) as usize)
        .build();
    if let Err(err) = &pool {
        tracing::error!("initialize_db_pool failed with err: {:?}", err);
    } else {
        tracing::info!("initialize_db_pool for anlyzer success");
    }
    pool.expect("pool connected normal")
}
