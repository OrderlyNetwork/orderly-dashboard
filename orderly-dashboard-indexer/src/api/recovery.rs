use crate::config::get_common_cfg;
use crate::consume_data_task::{consume_data_inner, pull_target_block};
use crate::formats_external::{
    FailureResponse, IndexerQueryResponse, RecoveryBlockRequest, RecoverySolEventRequest,
};
use crate::handler::solana::solana_program_log_processor::{
    handle_sol_program_logs, SolanaProgramLogProcessor,
};
use anyhow::Result;
use std::cmp::min;
use std::sync::atomic::{AtomicBool, Ordering};

#[allow(dead_code)]
const RECOVERY: &str = "recovery_block";
static IS_RECOVER_FLIGHT: AtomicBool = AtomicBool::new(false);
static IS_SOL_RECOVER_FLIGHT: AtomicBool = AtomicBool::new(false);

pub struct RecoverGuard;

impl Drop for RecoverGuard {
    fn drop(&mut self) {
        IS_RECOVER_FLIGHT.store(false, Ordering::Relaxed);
    }
}

pub struct SolRecoverGuard;

impl Drop for SolRecoverGuard {
    fn drop(&mut self) {
        IS_SOL_RECOVER_FLIGHT.store(false, Ordering::Relaxed);
    }
}

pub(crate) async fn recovery_block(
    request: RecoveryBlockRequest,
) -> Result<IndexerQueryResponse<()>> {
    if IS_RECOVER_FLIGHT.swap(true, Ordering::Relaxed) {
        tracing::warn!(
            target: RECOVERY,
            "recovery_block is on the flight, pls wait"
        );
        return Ok(IndexerQueryResponse::Failure(FailureResponse::new(
            1000,
            "recovery_block is on the flight, pls wait".to_string(),
        )));
    }
    let _guard = RecoverGuard;
    let newest_block = pull_target_block().await?;
    let end_block = min(newest_block, request.end_block_height);
    tracing::info!(
        target: RECOVERY,
        "recovery_block, start:{}, end:{}",
        request.start_block_height,
        end_block
    );
    if let Err(err) = consume_data_inner(
        request.start_block_height,
        end_block,
        Some(end_block),
        false,
    )
    .await
    {
        tracing::warn!(target: RECOVERY, "recovery_block with error: {:?}", err);
        return Ok(IndexerQueryResponse::Failure(FailureResponse::new(
            0,
            "recovery_block failed".to_owned(),
        )));
    }

    Ok(IndexerQueryResponse::empty_success())
}

pub(crate) async fn recovery_sol_events(
    request: RecoverySolEventRequest,
) -> Result<IndexerQueryResponse<()>> {
    tracing::info!(
        target: RECOVERY,
        "recovery_sol_events, start sig:{}, end slot:{}",
        request.start_sigature,
        request.end_slot,
    );
    if IS_SOL_RECOVER_FLIGHT.swap(true, Ordering::Relaxed) {
        tracing::warn!(
            target: RECOVERY,
            "recovery_block is on the flight, pls wait"
        );
        return Ok(IndexerQueryResponse::Failure(FailureResponse::new(
            1000,
            "recovery_block is on the flight, pls wait".to_string(),
        )));
    }
    let _guard = SolRecoverGuard;
    let sol_config = &get_common_cfg().sol_chain_config;

    SolanaProgramLogProcessor::new(
        sol_config,
        sol_config.program_address.clone(),
        || {
            let start_signature_clone = request.start_sigature.clone();
            async move { Ok(start_signature_clone) }
        },
        handle_sol_program_logs,
        |_| async move { Ok(()) },
    )
    .recover_logs(request.end_slot)
    .await?;

    Ok(IndexerQueryResponse::empty_success())
}

pub(crate) async fn recover_sol_deposit_events(
    request: RecoveryBlockRequest,
) -> Result<IndexerQueryResponse<()>> {
    let cefi_cli = std::sync::Arc::new(crate::cefi_client::CefiClient::new("".to_string()));
    crate::contract::simple_recover_deposit_sol_logs(
        request.start_block_height,
        request.end_block_height,
        cefi_cli,
    )
    .await?;
    Ok(IndexerQueryResponse::empty_success())
}

pub(crate) async fn recover_sol_withdraw_approve_rebalance_events(
    request: RecoveryBlockRequest,
) -> Result<IndexerQueryResponse<()>> {
    crate::contract::simple_recover_sol_deposit_withdraw_approve_and_rebalance_logs(
        request.start_block_height,
        request.end_block_height,
    )
    .await?;
    Ok(IndexerQueryResponse::empty_success())
}

pub(crate) async fn recover_swap_result_uploaded_events(
    request: RecoveryBlockRequest,
) -> Result<IndexerQueryResponse<()>> {
    crate::contract::simple_recover_swap_result_uploded_logs(
        request.start_block_height,
        request.end_block_height,
    )
    .await?;
    Ok(IndexerQueryResponse::empty_success())
}
