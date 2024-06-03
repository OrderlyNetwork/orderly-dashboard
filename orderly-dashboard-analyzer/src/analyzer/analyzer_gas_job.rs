use std::cmp::{max, min};
use std::ops::Div;
use std::time::Duration;

use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, Timelike, Utc};

use orderly_dashboard_indexer::formats_external::gas_consumption::{
    GasConsumptionResponse, TransactionGasCostData,
};
use orderly_dashboard_indexer::formats_external::Response;
use tokio::time;

use crate::analyzer::analyzer_gas_context::GasFeeContext;
use crate::analyzer::analyzer_job::HTTPException;
use crate::analyzer::analyzer_job::HTTPException::Timeout;
use crate::db::block_summary::{create_or_update_block_summary, find_block_summary};
use crate::db::hourly_gas_fee::HourlyGasFeeKey;

use super::get_gas_prec;

const ANALYZER_CONTEXT: &str = "Analyzer-Gas-Job";

pub fn start_analyzer_gas_job(
    interval_seconds: u64,
    base_url: String,
    start_block: i64,
    batch_block_num: u64,
) {
    tokio::spawn(async move {
        let mut block_summary = find_block_summary("gas_fee".to_string()).await.unwrap();
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
                    let result: Result<Response<GasConsumptionResponse>, serde_json::Error> =
                        serde_json::from_str(&*json_str);
                    if result.is_err() {
                        tracing::warn!(target:ANALYZER_CONTEXT, "parse data err, json_str: {}", json_str);
                        time::sleep(Duration::from_secs(5 * interval_seconds)).await;
                        continue;
                    }

                    let (pulled_block_time, latest_block_height, _) =
                        parse_and_analyzer(result.unwrap()).await;

                    if round_to_block > latest_block_height {
                        tracing::info!(target: ANALYZER_CONTEXT,"continue to pull block from {} to {}. cost:{}",round_from_block,round_to_block,Utc::now().timestamp_millis()-timestamp);
                        continue;
                    }

                    max_block = latest_block_height;
                    from_block = round_to_block + 1;

                    block_summary.pulled_block_height = round_to_block;
                    block_summary.pulled_block_time =
                        NaiveDateTime::from_timestamp_opt(pulled_block_time, 0).unwrap();
                    block_summary.latest_block_height = latest_block_height;
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
            time::sleep(Duration::from_secs(interval_seconds)).await;
        }
    });
}

async fn parse_and_analyzer(response: Response<GasConsumptionResponse>) -> (i64, i64, i64) {
    let mut pulled_block_time = 0i64;
    let mut latest_block_height = 0i64;
    let latest_perp_trade_id = 0i64;

    match response {
        Response::Success(success_event) => {
            let trading_event: GasConsumptionResponse = success_event.into_data().unwrap();
            latest_block_height = trading_event.last_block as i64;

            let mut context: GasFeeContext = GasFeeContext::new_context();
            let events = trading_event.transactions;
            for gas_event in events {
                let block_hour = convert_block_hour(gas_event.block_timestamp as i64);
                let block_num = gas_event.block_number as i64;
                let block_time =
                    NaiveDateTime::from_timestamp_opt(gas_event.block_timestamp as i64, 0).unwrap();

                pulled_block_time = max(pulled_block_time, block_time.timestamp());
                let event_data = gas_event.transaction_gas_data;
                match event_data {
                    TransactionGasCostData::Deposit { .. } => {
                        let p_key = HourlyGasFeeKey::new_key("deposit".to_string(), block_hour);
                        let gs = context.get_hourly_gas(&p_key).await;
                        let l1_fee: BigDecimal = gas_event.fee_data.clone().l1_fee.parse().unwrap();
                        let l2_fee: BigDecimal = gas_event.fee_data.clone().l2_fee.parse().unwrap();
                        let fixed_l1_fee = l1_fee.div(get_gas_prec());
                        let fixed_l2_fee = l2_fee.div(get_gas_prec());
                        gs.new_event(fixed_l1_fee + fixed_l2_fee, block_num, block_time.clone());
                    }
                    TransactionGasCostData::PerpTradesUpload { .. } => {
                        let p_key = HourlyGasFeeKey::new_key("perp_trade".to_string(), block_hour);
                        let gs = context.get_hourly_gas(&p_key).await;
                        let l1_fee: BigDecimal = gas_event.fee_data.clone().l1_fee.parse().unwrap();
                        let l2_fee: BigDecimal = gas_event.fee_data.clone().l2_fee.parse().unwrap();
                        let fixed_l1_fee = l1_fee.div(get_gas_prec());
                        let fixed_l2_fee = l2_fee.div(get_gas_prec());
                        gs.new_event(fixed_l1_fee + fixed_l2_fee, block_num, block_time.clone());
                    }

                    TransactionGasCostData::EventUpload { .. } => {
                        let p_key =
                            HourlyGasFeeKey::new_key("event_upload".to_string(), block_hour);
                        let gs = context.get_hourly_gas(&p_key).await;
                        let l1_fee: BigDecimal = gas_event.fee_data.clone().l1_fee.parse().unwrap();
                        let l2_fee: BigDecimal = gas_event.fee_data.clone().l2_fee.parse().unwrap();
                        let fixed_l1_fee = l1_fee.div(get_gas_prec());
                        let fixed_l2_fee = l2_fee.div(get_gas_prec());
                        gs.new_event(fixed_l1_fee + fixed_l2_fee, block_num, block_time.clone());
                    }
                }
            }
            context.save_analyze_result().await
        }
        Response::Failure(_) => {}
    }
    (pulled_block_time, latest_block_height, latest_perp_trade_id)
}

fn convert_block_hour(block_timestamp: i64) -> NaiveDateTime {
    let date_time = NaiveDateTime::from_timestamp_opt(block_timestamp, 0).unwrap();
    return date_time.with_second(0).unwrap().with_minute(0).unwrap();
}

async fn get_indexer_data(
    from_block: i64,
    to_block: i64,
    base_url: String,
) -> Result<String, HTTPException> {
    let indexer_url = format!(
        "{}/pull_transaction_gas_cost?from_block={}&to_block={}",
        base_url, from_block, to_block
    );
    let response = reqwest::get(indexer_url).await;

    match response {
        Ok(res) => Ok(res.text().await.unwrap()),
        Err(_) => Err(Timeout),
    }
}
