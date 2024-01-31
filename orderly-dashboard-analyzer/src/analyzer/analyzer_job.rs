use std::cmp::{max, min};
use std::time::Duration;

use chrono::{NaiveDateTime, Timelike, Utc};
use tokio::time;

use orderly_dashboard_indexer::formats_external::trading_events::{
    TradingEventInnerData, TradingEventsResponse,
};
use orderly_dashboard_indexer::formats_external::Response;

use crate::analyzer::adl_analyzer::analyzer_adl;
use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::analyzer_job::HTTPException::Timeout;
use crate::analyzer::liquidation_analyzer::analyzer_liquidation;
use crate::analyzer::perp_analyzer::analyzer_perp_trade;
use crate::analyzer::settlement_analyzer::analyzer_settlement;
use crate::analyzer::transaction_analyzer::analyzer_transaction;
use crate::db::block_summary::{create_or_update_block_summary, find_block_summary};

const PULL_NUM: i32 = 3000;

const ANALYZER_CONTEXT: &str = "Analyzer-Job";

pub fn start_analyzer_job(
    interval_seconds: u64,
    base_url: String,
    start_block: i64,
    _batch_block_num: u64,
) {
    tokio::spawn(async move {
        let mut block_summary = find_block_summary().await.unwrap();
        let mut from_block = max(block_summary.pulled_block_height + 1, start_block.clone());
        let mut max_block = block_summary.latest_block_height;

        loop {
            let round_from_block = from_block;
            let round_to_block = max(
                round_from_block,
                min(round_from_block + PULL_NUM as i64, max_block),
            );
            let timestamp = Utc::now().timestamp_millis();
            let response_str =
                get_indexer_data(round_from_block, round_to_block, base_url.clone()).await;
            match response_str {
                Ok(json_str) => {
                    let result: Result<Response<TradingEventsResponse>, serde_json::Error> =
                        serde_json::from_str(&*json_str);
                    if result.is_err() {
                        tracing::warn!(target:ANALYZER_CONTEXT, "parse data err, json_str: {}", json_str);
                        time::sleep(Duration::from_secs(5 * interval_seconds)).await;
                        continue;
                    }

                    let (pulled_block_time, latest_block_height, pulled_perp_trade_id) =
                        parse_and_analyzer(result.unwrap()).await;

                    if round_to_block == latest_block_height {
                        tracing::info!(target:ANALYZER_CONTEXT,"pull task blocked, latest_block:{}",block_summary.latest_block_height);
                        time::sleep(Duration::from_secs(interval_seconds)).await;
                        continue;
                    }

                    block_summary.pulled_block_height = min(round_to_block, latest_block_height);
                    block_summary.pulled_block_time =
                        NaiveDateTime::from_timestamp_opt(pulled_block_time, 0).unwrap();
                    block_summary.latest_block_height = latest_block_height;
                    block_summary.pulled_perp_trade_id = pulled_perp_trade_id;
                    create_or_update_block_summary(block_summary.clone())
                        .await
                        .ok();

                    max_block = latest_block_height;
                    from_block = block_summary.pulled_block_height + 1;
                }
                Err(error) => {
                    tracing::warn!(target: ANALYZER_CONTEXT, "get_indexer_data err: {:?}", error);
                    time::sleep(Duration::from_secs(5 * interval_seconds)).await;
                    continue;
                }
            }
            tracing::info!(target: ANALYZER_CONTEXT,"pull block from {} to {}. cost:{}",round_from_block,round_to_block,Utc::now().timestamp_millis()-timestamp);
            time::sleep(Duration::from_secs(interval_seconds.clone())).await;
        }
    });
}

async fn parse_and_analyzer(response: Response<TradingEventsResponse>) -> (i64, i64, i64) {
    let mut pulled_block_time = 0i64;
    let mut latest_block_height = 0i64;
    let mut latest_perp_trade_id = 0i64;
    let mut context = AnalyzeContext::new_context();

    match response {
        Response::Success(success_event) => {
            let trading_event: TradingEventsResponse = success_event.into_data().unwrap();
            // tracing::info!(target:ANALYZER_CONTEXT,"indexer-response: {:?}",trading_event.clone());
            latest_block_height = trading_event.last_block as i64;

            let events = trading_event.events;
            for event in events {
                let block_hour = convert_block_hour(event.block_timestamp as i64);
                let block_num = event.block_number as i64;
                let block_time =
                    NaiveDateTime::from_timestamp_opt(event.block_timestamp as i64, 0).unwrap();

                pulled_block_time = max(pulled_block_time, block_time.timestamp());
                let event_data = event.data;
                match event_data {
                    TradingEventInnerData::Transaction {
                        account_id,
                        sender: _,
                        receiver: _,
                        token_hash,
                        broker_hash: _,
                        chain_id,
                        side,
                        token_amount,
                        withdraw_nonce: _,
                        status: _,
                        fail_reason: _,
                        fee: _,
                    } => {
                        analyzer_transaction(
                            account_id,
                            token_hash,
                            chain_id,
                            side,
                            token_amount,
                            &block_hour,
                            block_num,
                            block_time.clone(),
                            &mut context,
                        )
                        .await;
                    }
                    TradingEventInnerData::ProcessedTrades {
                        batch_id: _,
                        trades,
                    } => {
                        let trade_id =
                            analyzer_perp_trade(trades, block_time, block_num, &mut context).await;
                        latest_perp_trade_id = max(latest_perp_trade_id, trade_id);
                    }
                    TradingEventInnerData::SettlementResult {
                        account_id,
                        settled_amount,
                        settled_asset_hash,
                        insurance_account_id,
                        insurance_transfer_amount,
                        settlement_executions,
                    } => {
                        analyzer_settlement(
                            account_id,
                            settled_amount,
                            settled_asset_hash,
                            insurance_account_id,
                            insurance_transfer_amount,
                            settlement_executions,
                            block_hour,
                            block_num,
                            block_time.clone(),
                            &mut context,
                        )
                        .await
                    }
                    TradingEventInnerData::LiquidationResult {
                        liquidated_account_id,
                        insurance_account_id,
                        liquidated_asset_hash,
                        insurance_transfer_amount,
                        liquidation_transfers,
                    } => {
                        analyzer_liquidation(
                            liquidated_account_id,
                            insurance_account_id,
                            liquidated_asset_hash,
                            insurance_transfer_amount,
                            liquidation_transfers,
                            block_num.clone(),
                            block_hour.clone(),
                            block_time.clone(),
                            &mut context,
                        )
                        .await
                    }
                    TradingEventInnerData::AdlResult {
                        account_id,
                        insurance_account_id,
                        symbol_hash,
                        position_qty_transfer,
                        cost_position_transfer,
                        adl_price,
                        sum_unitary_fundings,
                    } => {
                        analyzer_adl(
                            account_id,
                            insurance_account_id,
                            symbol_hash,
                            position_qty_transfer,
                            cost_position_transfer,
                            adl_price,
                            sum_unitary_fundings,
                            block_hour,
                            block_time.clone(),
                            block_num,
                            &mut context,
                        )
                        .await;
                    }
                }
            }
        }
        Response::Failure(_) => {}
    }

    context.save_analyze_result().await;
    (pulled_block_time, latest_block_height, latest_perp_trade_id)
}

fn convert_block_hour(block_timestamp: i64) -> NaiveDateTime {
    let date_time = NaiveDateTime::from_timestamp_opt(block_timestamp, 0).unwrap();
    return date_time.with_second(0).unwrap().with_minute(0).unwrap();
}

#[derive(Debug)]
enum HTTPException {
    Timeout,
}

async fn get_indexer_data(
    from_block: i64,
    to_block: i64,
    base_url: String,
) -> Result<String, HTTPException> {
    let indexer_url = format!(
        "{}/pull_perp_trading_events?from_block={}&to_block={}",
        base_url, from_block, to_block
    );
    let response = reqwest::get(indexer_url).await;

    match response {
        Ok(res) => Ok(res.text().await.unwrap()),
        Err(_) => Err(Timeout),
    }
}
