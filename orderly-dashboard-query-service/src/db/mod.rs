pub mod trading_metrics;
use actix_diesel::Database;
use diesel::PgConnection;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

#[allow(dead_code)]
pub const DB_CONTEXT: &str = "DB_operation";

#[allow(dead_code)]
pub static ANALYZER_POOL: Lazy<Database<PgConnection>> = Lazy::new(|| establish_connection());

#[allow(dead_code)]
pub fn establish_connection() -> Database<PgConnection> {
    let database_url = get_database_credentials();
    Database::builder().pool_max_size(30).open(&database_url)
}

fn get_database_credentials() -> String {
    dotenv().ok();
    env::var("ANALYZER_DATABASE_URL").expect("ANALYZER_DATABASE_URL must be set in .env file")
}
