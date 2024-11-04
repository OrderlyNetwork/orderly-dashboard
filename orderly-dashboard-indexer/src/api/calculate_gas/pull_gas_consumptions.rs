use crate::db::serial_batches::get_serial_batches;
use crate::db::settings::{get_last_rpc_processed_height, get_last_rpc_processed_timestamp};
use crate::db::transaction_events::query_balance_transaction_executions;
use crate::formats_external::gas_consumption::{GasConsumptionResponse, TransactionGasCost};
use crate::formats_external::{Response, SuccessResponse};
use anyhow::{Context, Result};
use std::cmp::min;
use std::collections::HashMap;

pub async fn pull_gas_consumptions(
    params: &HashMap<String, String>,
) -> Result<Response<GasConsumptionResponse>> {
    let from_block = params
        .get("from_block")
        .context("param from_block not found")?;
    let to_block = params.get("to_block").context("param to_block not found")?;
    let response = gas_consumption_data(from_block.parse()?, to_block.parse()?).await?;

    Ok(Response::Success(SuccessResponse::new(response)))
}

pub async fn gas_consumption_data(
    from_block: i64,
    to_block: i64,
) -> Result<GasConsumptionResponse> {
    let last_block = get_last_rpc_processed_height().await?.unwrap_or_default();
    let last_timestamp = get_last_rpc_processed_timestamp()
        .await?
        .unwrap_or_default();
    let to_block = min(last_block as i64, to_block);
    let mut response = GasConsumptionResponse::default();
    if last_block == 0 {
        return Ok(response);
    }
    let deposit_gas_cost = get_deposit_gas_data(from_block, to_block).await?;
    let serial_batches_gas_cost = get_serial_batches_gas_data(from_block, to_block).await?;
    let mut gas_cost_data = [deposit_gas_cost, serial_batches_gas_cost].concat();
    gas_cost_data.sort();
    response.transactions = gas_cost_data;
    response.last_block = last_block;
    response.last_timestamp = last_timestamp;
    Ok(response)
}

pub async fn get_deposit_gas_data(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<TransactionGasCost>> {
    let balance_transactions = query_balance_transaction_executions(from_block, to_block).await?;
    let deposit_gas_data_vec: Vec<TransactionGasCost> = balance_transactions
        .into_iter()
        .filter_map(TransactionGasCost::try_from_balance_transaction)
        .collect::<Vec<_>>();
    Ok(deposit_gas_data_vec)
}

pub async fn get_serial_batches_gas_data(
    from_block: i64,
    to_block: i64,
) -> Result<Vec<TransactionGasCost>> {
    let serial_batches = get_serial_batches(from_block, to_block).await?;
    let mut serial_batches_gas_data_vec: Vec<TransactionGasCost> = vec![];
    for serial_batch in serial_batches {
        serial_batches_gas_data_vec.push(TransactionGasCost::from_serial_batch(serial_batch));
    }
    Ok(serial_batches_gas_data_vec)
}
