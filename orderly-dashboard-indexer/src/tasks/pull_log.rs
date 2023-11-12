use crate::client::HttpClient;
use anyhow::Result;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::cmp::min;
use std::collections::BTreeSet;
use std::time::Duration;

const PULL_LOG: &str = "pull_log";

pub(crate) async fn pull_consume_log_task(
    http_client: HttpClient,
    start_block: u64,
    end_block: u64,
) -> Result<()> {
    todo!();

    Ok(())
}
