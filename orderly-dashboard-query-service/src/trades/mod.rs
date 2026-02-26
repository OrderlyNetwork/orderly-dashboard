pub mod trades_api;

pub use trades_api::{get_trades_status, query_trades, QueryTradesRequest, QueryTradesResponse, TradesStatusResponse};

use anyhow::Result;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use orderly_dashboard_indexer::db::settings::{get_last_rpc_processed_timestamp, DbSettings, SettingsKey};
use orderly_dashboard_indexer::db::POOL;
use orderly_dashboard_indexer::schema::settings::dsl::{id, settings};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

/// Last RPC processed timestamp from indexer database settings
pub static LAST_RPC_PROCESS_TIMESTAMP: AtomicU64 = AtomicU64::new(0);

/// Contract deploy height from indexer database settings
pub static CONTRACT_DEPLOY_HEIGHT: AtomicU64 = AtomicU64::new(0);

const SETTINGS_UPDATE_CONTEXT: &str = "settings_update_context";

/// Get contract deploy height from settings table
async fn get_contract_deploy_height() -> Result<Option<u64>> {
    let key = SettingsKey::ContarctDeployTimestamp as i32;
    let mut conn = POOL
        .get()
        .await
        .expect("Couldn't get db connection from the pool");
    let result = settings
        .filter(id.eq(key))
        .first::<DbSettings>(&mut conn)
        .await;

    match result {
        Ok(setting) => Ok(Some(setting.value.parse::<u64>()?)),
        Err(diesel::NotFound) => Ok(None),
        Err(err) => Err(anyhow::anyhow!("Failed to get contract deploy height: {}", err)),
    }
}

/// Async task to update settings from indexer database
///
/// This task:
/// - Updates LastRpcProcessTimeStamp every 10 seconds
/// - Fetches ContractDeployHeight on startup and retries if not found until successful
pub async fn update_settings_task() -> Result<()> {
    tracing::info!(
        target: SETTINGS_UPDATE_CONTEXT,
        "Starting settings update task"
    );

    // Fetch ContractDeployHeight with retry until successful
    let mut contract_deploy_height_retry_interval = tokio::time::interval(Duration::from_secs(10));
    
    // Try to fetch ContractDeployHeight immediately first, then retry every 10 seconds if not found
    loop {
        match get_contract_deploy_height().await {
            Ok(Some(height)) => {
                CONTRACT_DEPLOY_HEIGHT.store(height, Ordering::Relaxed);
                tracing::info!(
                    target: SETTINGS_UPDATE_CONTEXT,
                    "ContractDeployHeight initialized: {}",
                    height
                );
                break;
            }
            Ok(None) => {
                tracing::warn!(
                    target: SETTINGS_UPDATE_CONTEXT,
                    "ContractDeployHeight not found in settings table, will retry in 10 seconds"
                );
            }
            Err(err) => {
                tracing::warn!(
                    target: SETTINGS_UPDATE_CONTEXT,
                    "Failed to fetch ContractDeployHeight: {}, will retry in 10 seconds",
                    err
                );
            }
        }
        
        // Wait 10 seconds before retrying
        contract_deploy_height_retry_interval.tick().await;
    }

    // Update LastRpcProcessTimeStamp every 10 seconds
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;

        match get_last_rpc_processed_timestamp().await {
            Ok(Some(timestamp)) => {
                // Convert i64 to u64 (timestamp should be positive)
                let timestamp_u64 = timestamp.max(0) as u64;
                LAST_RPC_PROCESS_TIMESTAMP.store(timestamp_u64, Ordering::Relaxed);
                tracing::debug!(
                    target: SETTINGS_UPDATE_CONTEXT,
                    "LastRpcProcessTimeStamp updated: {}",
                    timestamp_u64
                );
            }
            Ok(None) => {
                tracing::debug!(
                    target: SETTINGS_UPDATE_CONTEXT,
                    "LastRpcProcessTimeStamp not found in settings table"
                );
            }
            Err(err) => {
                tracing::warn!(
                    target: SETTINGS_UPDATE_CONTEXT,
                    "Failed to fetch LastRpcProcessTimeStamp: {}",
                    err
                );
            }
        }
    }
}
