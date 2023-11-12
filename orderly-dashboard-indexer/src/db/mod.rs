pub mod settings;

use actix_diesel::Database;
use diesel::PgConnection;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;
pub(crate) const DB_CONTEXT: &str = "db_context";

pub static POOL: Lazy<Database<PgConnection>> = Lazy::new(|| establish_connection());

pub fn establish_connection() -> Database<PgConnection> {
    let database_url = get_database_credentials();
    Database::builder().pool_max_size(30).open(&database_url)
}

fn get_database_credentials() -> String {
    dotenv().ok();

    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file")
}
