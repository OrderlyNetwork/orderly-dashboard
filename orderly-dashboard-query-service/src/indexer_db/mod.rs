pub mod trades;

use dotenv::dotenv;
use std::env;

/// Initialize the indexer database connection pool
///
/// This function sets up the database connection pool for the indexer database
/// using the INDEXER_DATABASE_URL environment variable.
pub fn init_indexer_db_url() {
    orderly_dashboard_indexer::db::init_database_setting(get_database_credentials(), 1);
}

fn get_database_credentials() -> String {
    dotenv().ok();
    env::var("INDEXER_DATABASE_URL").expect("INDEXER_DATABASE_URL must be set in .env file")
}
