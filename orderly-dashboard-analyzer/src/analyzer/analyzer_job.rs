use std::cmp::{max, min};
use std::time::Duration;

use chrono::{NaiveDateTime, Timelike, Utc};
use tokio::time;

use orderly_dashboard_indexer::formats_external::trading_events::{
    TradingEventInnerData, TradingEventsResponse,
};
use orderly_dashboard_indexer::formats_external::{BlockTimeResponse, Response};

use crate::analyzer::adl_analyzer::analyzer_adl;
use crate::analyzer::analyzer_context::AnalyzeContext;
use crate::analyzer::analyzer_job::HTTPException::Timeout;
use crate::analyzer::perp_analyzer::analyzer_perp_trade;
use crate::analyzer::settlement_analyzer::analyzer_settlement;
use crate::analyzer::transaction_analyzer::analyzer_transaction;
use crate::db::block_summary::{create_or_update_block_summary, find_block_summary, TRADE_METRIC};

use super::adl_analyzer::analyzer_adl_v2;
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
        let mut block_summary = find_block_summary(TRADE_METRIC.to_string()).await.unwrap();
        let mut from_block = max(block_summary.pulled_block_height + 1, start_block.clone());
        let mut max_block = block_summary.latest_block_height;
        let start_timestamp = get_block_timestamp_with_retry(base_url.clone(), start_block).await;
        // to ensure from_time and to_time has covered the block range
        let mut from_time = max(block_summary.pulled_block_time.timestamp(), start_timestamp);
        let mut to_time = cal_to_time(from_time, batch_block_num);
        let mut empty_counter = 0_i64;
        loop {
            let round_from_block = from_block;
            let round_to_block = max(
                round_from_block,
                min(round_from_block + batch_block_num as i64, max_block),
            );
            let timestamp = Utc::now().timestamp_millis();
            let response_str = get_indexer_data(
                round_from_block,
                round_to_block,
                from_time,
                to_time,
                base_url.clone(),
            )
            .await;
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
                        tracing::info!(target: ANALYZER_CONTEXT,"continue to pull blocks, from time: {}, to_time: {}, diff mins: {}, from num {} to numb {}, latest_block_height: {}. cost:{}", from_time, to_time, (to_time - from_time) / 60,round_from_block, round_to_block, latest_block_height, Utc::now().timestamp_millis()-timestamp);
                        time::sleep(Duration::from_millis(interval_ms)).await;
                        continue;
                    }
                    if latest_block_height > round_from_block + 2 * (batch_block_num as i64) {
                        interval_ms = 5;
                    } else {
                        interval_ms = interval_seconds * 1000;
                    }

                    max_block = latest_block_height;
                    from_block = round_to_block + 1;
                    if pulled_block_time != 0 {
                        from_time = pulled_block_time;
                        block_summary.pulled_block_time =
                            NaiveDateTime::from_timestamp_opt(pulled_block_time, 0).unwrap();
                    } else {
                        empty_counter += 1;
                        const TWO_DAY_SEC: i64 = 86400;
                        if empty_counter > 10 && max_block - from_block > TWO_DAY_SEC
                            || empty_counter > 500
                        {
                            tracing::info!("force fetch pulled_block_time at empty_counter: {}, block diff: {}", empty_counter, max_block - from_block);
                            empty_counter = 0;
                            // fetch pulled_block_time with param round_to_block
                            let pulled_block_time =
                                get_block_timestamp_with_retry(base_url.clone(), round_to_block)
                                    .await;
                            from_time = pulled_block_time;
                            block_summary.pulled_block_time =
                                NaiveDateTime::from_timestamp_opt(pulled_block_time, 0).unwrap();
                        }
                    }
                    // generate 1 block for 2s
                    to_time = cal_to_time(
                        from_time + empty_counter * (batch_block_num as i64) * 2,
                        batch_block_num,
                    );

                    block_summary.pulled_block_height = round_to_block;
                    block_summary.latest_block_height = latest_block_height;
                    if pulled_perp_trade_id != 0 {
                        block_summary.pulled_perp_trade_id = pulled_perp_trade_id;
                    }
                    create_or_update_block_summary(block_summary.clone())
                        .await
                        .ok();
                    tracing::info!(target: ANALYZER_CONTEXT,"trade job ok to pull block from time: {}, to_time: {},from {} to {} block_summary: {:?}. cost:{}", from_time, to_time,round_from_block,round_to_block, block_summary, Utc::now().timestamp_millis()-timestamp);
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

fn cal_to_time(from_time: i64, batch_block_num: u64) -> i64 {
    from_time + batch_block_num as i64 * 4 + 100
}

#[allow(deprecated)]
pub async fn parse_and_analyzer(response: Response<TradingEventsResponse>) -> (i64, i64, i64) {
    let mut pulled_block_time = 0i64;
    let mut pulled_block_height = 0i64;
    let mut latest_block_height = 0i64;
    let mut latest_perp_trade_id = 0i64;
    let mut context = AnalyzeContext::new_context();

    match response {
        Response::Success(success_event) => {
            let trading_event: TradingEventsResponse = success_event.into_data().unwrap();
            pulled_block_time = trading_event.last_block_timestamp;
            latest_block_height = trading_event.last_block as i64;

            let events = trading_event.events;
            if !events.is_empty() {
                if let Some(event) = events.last() {
                    pulled_block_height = event.block_number as i64;
                }
            }
            for event in events {
                let block_hour = convert_block_hour(event.block_timestamp as i64);
                let block_num = event.block_number as i64;
                let block_time =
                    NaiveDateTime::from_timestamp_opt(event.block_timestamp as i64, 0).unwrap();

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
                        let trade_id = analyzer_perp_trade(trades, block_num, &mut context).await;
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
                        analyzer_liquidation_v1(
                            liquidated_account_id,
                            insurance_account_id,
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
                        analyzer_adl_v2(
                            account_id,
                            symbol_hash,
                            position_qty_transfer,
                            cost_position_transfer,
                            adl_price,
                            sum_unitary_fundings,
                            block_hour,
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

    context
        .save_analyze_result(
            pulled_block_height,
            NaiveDateTime::from_timestamp_opt(pulled_block_time, 0).unwrap(),
        )
        .await;
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
    from_timme: i64,
    to_time: i64,
    base_url: String,
) -> Result<String, HTTPException> {
    let indexer_url = format!(
        "{}/pull_perp_trading_events?from_block={}&to_block={}&from_time={}&to_time={}",
        base_url, from_block, to_block, from_timme, to_time,
    );
    let response = reqwest::get(indexer_url).await;

    match response {
        Ok(res) => Ok(res.text().await.unwrap()),
        Err(_) => Err(Timeout),
    }
}

pub async fn get_block_timestamp_with_retry(base_url: String, block_num: i64) -> i64 {
    loop {
        match get_block_timestamp(&base_url, block_num).await {
            Ok(data) => match data {
                Response::Success(data) => {
                    let timetamp = data.into_data().unwrap().block_timestamp;
                    tracing::info!("get_block_timestamp success with: {:?}", timetamp);
                    return timetamp;
                }
                Response::Failure(err) => {
                    tracing::warn!("get_block_timestamp response failed with err: {:?}", err);
                    tokio::time::sleep(Duration::from_secs(30)).await;
                }
            },
            Err(err) => {
                tracing::info!("get_block_timestamp result failed with err: {}", err);
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        }
    }
}

pub async fn get_block_timestamp(
    base_url: &str,
    block_num: i64,
) -> anyhow::Result<Response<BlockTimeResponse>> {
    let indexer_url = format!(
        "{}/pull_block_timestamp?block_number={}",
        base_url, block_num,
    );
    let response = reqwest::get(indexer_url).await?;

    Ok(response.json().await?)
}
