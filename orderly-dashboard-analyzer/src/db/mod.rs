use std::env;
use std::hash::Hash;

use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use dotenv::dotenv;
use once_cell::sync::{Lazy, OnceCell};

pub mod block_summary;
pub mod broker_info;
pub mod hourly_gas_fee;
pub mod hourly_orderly_perp;
pub mod hourly_orderly_token;
pub mod hourly_user_perp;
pub mod hourly_user_token;
pub mod market_info;
pub mod orderly_perp_summary;
pub mod orderly_token_summary;
pub mod symbols;
pub mod user_info;
pub mod user_perp_summary;
pub mod user_token_summary;

pub const DB_CONTEXT: &str = "DB_operation";
pub const BATCH_UPSERT_LEN: usize = 300;
pub(crate) const DB_CONN_ERR_MSG: &str = "Couldn't get db connection from the pool";

type DbPool = Pool<AsyncPgConnection>;

pub static POOL: Lazy<DbPool> = Lazy::new(|| initialize_db_pool());

pub static INITED_DATABASE_URL: OnceCell<String> = OnceCell::new();

pub fn init_database_url(database_url: String) {
    INITED_DATABASE_URL.set(database_url).ok();
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel-async/latest/diesel_async/pooled_connection/index.html#modules>.
fn initialize_db_pool() -> DbPool {
    let db_url = get_database_credentials();

    let connection_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder(connection_manager)
        .max_size(6)
        .build()
        .expect("analyzer db should connected success");
    pool
}

pub fn get_database_credentials() -> String {
    if let Some(db_url) = INITED_DATABASE_URL.get() {
        return db_url.to_string();
    }
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file")
}

#[allow(dead_code)]
pub trait PrimaryKey: PartialEq + Eq + Hash {}
