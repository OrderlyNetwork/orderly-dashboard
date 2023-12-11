pub mod block_summary;
pub mod hourly_orderly_perp;
pub mod orderly_perp_summary;
pub mod hourly_orderly_token;
pub mod hourly_user_perp;
pub mod orderly_token_summary;
pub mod user_token_summary;
pub mod user_perp_summary;
pub mod hourly_user_token;

use actix_diesel::Database;
use diesel::PgConnection;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub const DB_CONTEXT: &str = "DB_operation";

pub static POOL: Lazy<Database<PgConnection>> = Lazy::new(|| establish_connection());

pub fn establish_connection() -> Database<PgConnection> {
    let database_url = get_database_credentials();
    Database::builder().pool_max_size(30).open(&database_url)
}

fn get_database_credentials() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file")
}