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
use crate::analyzer::perp_analyzer::analyzer_perp_trade;
use crate::analyzer::settlement_analyzer::analyzer_settlement;
use crate::analyzer::transaction_analyzer::analyzer_transaction;
use crate::db::block_summary::{create_or_update_block_summary, find_block_summary};

use super::liquidation_analyzer::{analyzer_liquidation_v1, analyzer_liquidation_v2};

const ANALYZER_CONTEXT: &str = "Analyzer-Job";

#[allow(deprecated)]
pub fn start_analyzer_trade_job(
    interval_seconds: u64,
    base_url: String,
    start_block: i64,
    batch_block_num: u64,
) {
    let mut interval_ms = interval_seconds * 1000;
    tokio::spawn(async move {
        let mut block_summary = find_block_summary("trade".to_string()).await.unwrap();
        let mut from_block = max(block_summary.pulled_block_height + 1, start_block.clone());
        let mut max_block = block_summary.latest_block_height;

        loop {
            let round_from_block = from_block;
            let round_to_block = max(
                round_from_block,
                min(round_from_block + batch_block_num as i64, max_block),
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

                    if round_to_block > latest_block_height {
                        tracing::info!(target: ANALYZER_CONTEXT,"continue to pull block from {} to {}. cost:{}",round_from_block,round_to_block,Utc::now().timestamp_millis()-timestamp);
                        time::sleep(Duration::from_millis(interval_ms)).await;
                        continue;
                    }
                    if latest_block_height > round_from_block + 2 * (batch_block_num as i64) {
                        interval_ms = 10;
                    } else {
                        interval_ms = interval_seconds * 1000;
                    }

                    max_block = latest_block_height;
                    from_block = round_to_block + 1;

                    block_summary.pulled_block_height = round_to_block;
                    block_summary.pulled_block_time =
                        NaiveDateTime::from_timestamp_opt(pulled_block_time, 0).unwrap();
                    block_summary.latest_block_height = latest_block_height;
                    block_summary.pulled_perp_trade_id = pulled_perp_trade_id;
                    create_or_update_block_summary(block_summary.clone())
                        .await
                        .ok();
                    tracing::info!(target: ANALYZER_CONTEXT,"ok to pull block from {} to {}. cost:{}",round_from_block,round_to_block,Utc::now().timestamp_millis()-timestamp);
                }
                Err(error) => {
                    tracing::warn!(target: ANALYZER_CONTEXT, "get_indexer_data err: {:?}", error);
                    time::sleep(Duration::from_secs(5 * interval_seconds)).await;
                    continue;
                }
            }
            time::sleep(Duration::from_millis(interval_ms)).await;
        }
    });
}

#[allow(deprecated)]
async fn parse_and_analyzer(response: Response<TradingEventsResponse>) -> (i64, i64, i64) {
    let mut pulled_block_time = 0i64;
    let mut latest_block_height = 0i64;
    let mut latest_perp_trade_id = 0i64;
    let mut context = AnalyzeContext::new_context();

    match response {
        Response::Success(success_event) => {
            let trading_event: TradingEventsResponse = success_event.into_data().unwrap();
            latest_block_height = trading_event.last_block as i64;

            let events = trading_event.events;
            for event in events {
                let block_hour = convert_block_hour(event.block_timestamp as i64);
                let block_num = event.block_number as i64;
                let block_time =
                    NaiveDateTime::from_timestamp_opt(event.block_timestamp as i64, 0).unwrap();

                pulled_block_time = max(pulled_block_time, block_time.timestamp());
                let event_data = event.data;
                match event_data.clone() {
                    TradingEventInnerData::Transaction {
                        account_id,
                        sender: _,
                        receiver,
                        token_hash,
                        broker_hash,
                        chain_id,
                        side,
                        token_amount,
                        withdraw_nonce: _,
                        status: _,
                        fail_reason: _,
                        fee: _,
                    } => {
                        tracing::info!(target: ANALYZER_CONTEXT, "deposit/withdraw -- {:?}", event_data.clone());
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
                            receiver,
                            broker_hash,
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
                        insurance_account_id: _,
                        liquidated_asset_hash,
                        insurance_transfer_amount,
                        liquidation_transfers,
                    } => {
                        analyzer_liquidation_v1(
                            liquidated_account_id,
                            liquidated_asset_hash,
                            insurance_transfer_amount,
                            liquidation_transfers,
                            block_num,
                            block_hour.clone(),
                            block_time.clone(),
                            &mut context,
                        )
                        .await
                    }
                    TradingEventInnerData::AdlResult {
                        account_id,
                        insurance_account_id: _,
                        symbol_hash,
                        position_qty_transfer,
                        cost_position_transfer,
                        adl_price,
                        sum_unitary_fundings,
                    } => {
                        analyzer_adl(
                            account_id,
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
                    TradingEventInnerData::LiquidationResultV2 {
                        account_id,
                        liquidated_asset_hash,
                        insurance_transfer_amount,
                        liquidation_transfers,
                    } => {
                        analyzer_liquidation_v2(
                            account_id,
                            liquidated_asset_hash,
                            insurance_transfer_amount,
                            liquidation_transfers,
                            block_num,
                            block_hour.clone(),
                            block_time.clone(),
                            &mut context,
                        )
                        .await
                    }
                    TradingEventInnerData::AdlResultV2 {
                        account_id,
                        symbol_hash,
                        position_qty_transfer,
                        cost_position_transfer,
                        adl_price,
                        sum_unitary_fundings,
                    } => {
                        analyzer_adl(
                            account_id,
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

#[allow(deprecated)]
fn convert_block_hour(block_timestamp: i64) -> NaiveDateTime {
    let date_time = NaiveDateTime::from_timestamp_opt(block_timestamp, 0).unwrap();
    return date_time.with_second(0).unwrap().with_minute(0).unwrap();
}

#[derive(Debug)]
pub enum HTTPException {
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
