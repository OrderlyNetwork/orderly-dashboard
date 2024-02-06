use crate::consume_data_task::{consume_data_inner, pull_target_block};
use crate::formats_external::{FailureResponse, RecoveryBlockRequest, Response};
use anyhow::Result;
use std::cmp::min;
use std::sync::atomic::{AtomicBool, Ordering};

#[allow(dead_code)]
const RECOVERY: &str = "recovery_block";
static IS_RECOVER_FLIGHT: AtomicBool = AtomicBool::new(false);

pub struct RecoverGuard;

impl Drop for RecoverGuard {
    fn drop(&mut self) {
        IS_RECOVER_FLIGHT.store(false, Ordering::Relaxed);
    }
}

pub(crate) async fn recovery_block(request: RecoveryBlockRequest) -> Result<Response<()>> {
    if IS_RECOVER_FLIGHT.swap(true, Ordering::Relaxed) {
        tracing::warn!(
            target: RECOVERY,
            "recovery_block is on the flight, pls wait"
        );
        return Ok(Response::Failure(FailureResponse::new(
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
        return Ok(Response::Failure(FailureResponse::new(
            0,
            "recovery_block failed".to_owned(),
        )));
    }

    Ok(Response::empty_success())
}
