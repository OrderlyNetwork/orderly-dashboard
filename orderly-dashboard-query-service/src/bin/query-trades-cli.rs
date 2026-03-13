use anyhow::{Context, Result};
use clap::Parser;
use orderly_dashboard_query_service::trades::trades_api::{
    PaginationCursorRequest, QueryTradesRequest, QueryTradesResponse, TradesStatusResponse,
};
use reqwest::Client;
use std::time::Duration;

#[derive(Debug, Parser)]
#[command(name = "query-trades-cli")]
#[command(about = "Call /trades to query trades for broker=orderly (last N days, default 10)")]
struct Args {
    /// Service base url, e.g. https://orderly-dashboard-query-service.orderly.network
    #[arg(
        long,
        env = "QUERY_SERVICE_BASE_URL",
        default_value = "https://orderly-dashboard-query-service.orderly.network"
    )]
    base_url: String,

    /// How many recent days to query (default 10)
    #[arg(long, default_value_t = 10)]
    days: i64,

    /// Output pretty JSON (by default one JSON line per page)
    #[arg(long, default_value_t = false)]
    pretty: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let http = Client::new();

    let status = get_trades_status(&http, &args.base_url)
        .await
        .context("request /trades_status failed")?;

    let to_time = status.last_rpc_process_timestamp as i64;
    let from_time = to_time - args.days * 24 * 3600;
    let max_range = status.max_time_range_seconds.max(1);

    let mut range_start = from_time;
    while range_start < to_time {
        let range_end = (range_start + max_range).min(to_time);
        query_one_range(&http, &args.base_url, range_start, range_end, args.pretty)
            .await
            .with_context(|| {
                format!(
                    "query /trades failed (from_time={}, to_time={})",
                    range_start, range_end
                )
            })?;
        range_start = range_end;
    }

    Ok(())
}

async fn get_trades_status(http: &Client, base_url: &str) -> Result<TradesStatusResponse> {
    let url = format!("{}/trades_status", base_url.trim_end_matches('/'));
    let resp = http.get(url).send().await?.error_for_status()?;
    Ok(resp.json::<TradesStatusResponse>().await?)
}

async fn query_one_range(
    http: &Client,
    base_url: &str,
    from_time: i64,
    to_time: i64,
    pretty: bool,
) -> Result<()> {
    let url = format!("{}/trades", base_url.trim_end_matches('/'));
    let mut next_cursor: Option<PaginationCursorRequest> = None;

    loop {
        let req = QueryTradesRequest {
            broker_id: Some("orderly".to_string()),
            account_id: None,
            address: None,
            symbol: None,
            from_time,
            to_time,
            next_cursor,
        };

        let resp = http
            .post(&url)
            .json(&req)
            .send()
            .await?
            .error_for_status()?;
        let body: QueryTradesResponse = resp.json().await?;

        if pretty {
            println!("{}", serde_json::to_string_pretty(&body)?);
        } else {
            println!("{}", serde_json::to_string(&body)?);
        }

        next_cursor = body.next_cursor.map(|c| PaginationCursorRequest {
            block_number: c.block_number,
            transaction_index: c.transaction_index,
            log_index: c.log_index,
            block_time: c.block_time,
        });

        if next_cursor.is_none() {
            break;
        }

        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    Ok(())
}
