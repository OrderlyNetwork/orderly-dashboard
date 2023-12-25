pub mod trading_metrics;
use dotenv::dotenv;
#[allow(unused_imports, dead_code)]
use orderly_dashboard_analyzer::db::init_database_url as init_analyzer_database_url;
use std::env;

#[allow(dead_code)]
pub const DB_CONTEXT: &str = "DB_operation";

pub fn init_analyzer_db_url() {
    init_analyzer_database_url(get_database_credentials())
}

fn get_database_credentials() -> String {
    dotenv().ok();
    env::var("ANALYZER_DATABASE_URL").expect("ANALYZER_DATABASE_URL must be set in .env file")
}
