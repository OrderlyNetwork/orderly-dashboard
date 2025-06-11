pub mod raw_request;
pub mod trading_metrics;
use dotenv::dotenv;
use lazy_static::lazy_static;
#[allow(unused_imports, dead_code)]
use orderly_dashboard_analyzer::db::init_database_url as init_analyzer_database_url;
use parking_lot::Mutex;
use postgres::{Client, NoTls};
use std::env;

pub const DB_CONTEXT: &str = "DB_operation";
pub(crate) const DB_CONN_ERR_MSG: &str = "Couldn't get db connection from the pool";

lazy_static! {
    pub static ref ANALYZER_PG: Mutex<Client> = Mutex::new(
        Client::connect(
            &env::var("ANALYZER_DATABASE_URL")
                .expect("ANALYZER_DATABASE_URL must be set in .env file"),
            NoTls
        )
        .expect("Init analyzer database client failed")
    );
}

pub fn init_analyzer_db_url() {
    init_analyzer_database_url(get_database_credentials())
}

fn get_database_credentials() -> String {
    dotenv().ok();
    env::var("ANALYZER_DATABASE_URL").expect("ANALYZER_DATABASE_URL must be set in .env file")
}
