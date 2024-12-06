use std::cmp::max;
use std::{collections::VecDeque, future::Future, str::FromStr, sync::Arc, time::Duration};

use base58::ToBase58;
use bigdecimal::BigDecimal;
use borsh::BorshDeserialize;
use num_traits::FromPrimitive;

use crate::db::sol_transaction_events::{
    create_sol_balance_transaction_executions, DbSolTransactionEvent,
};
use crate::{
    config::{get_common_cfg, SolChainConfig},
    contract::sol_events::VaultWithdrawn,
    db::{
        settings::{get_sol_sync_signature, SolSyncSignature},
        transaction_events::{DbTransactionSide, DbTransactionStatus},
    },
    service_base::sdk::solana::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        rpc::{
            configs::RpcTransactionConfig, GetConfirmedSignaturesForAddress2Config, Signature,
            SolRpcClient,
        },
        transaction_status::EncodedConfirmedTransactionWithStatusMeta,
        utils::{
            decode_confirm_transaction_log_messages, split_program_event, split_program_exit_log,
            split_program_from_log,
        },
    },
    utils::hex_bytes,
};

const SOL_LOG_PROCESSOR: &str = "sol_log_processor";
const SOL_SIG_GET_LIMIT: usize = 1000;
const SOL_SIG_FETCH_LIMIT: usize = 100_000;
const SOL_API_CALL_INTERVAL_MS: u64 = 600;
const SOL_GET_TX_RETRY_LIMIT: usize = 3;

pub(crate) struct SolanaProgramLogData {
    pub program_address: String,
    pub tx_signature: String,
    pub slot: u64,
    pub block_time: i64,
    pub logs: Vec<String>,
}

pub(crate) async fn process_solana_logs<F, FutF, H, FutH, K, FutK>(
    solana_program_log_processor: SolanaProgramLogProcessor<F, FutF, H, FutH, K, FutK>,
) where
    F: Fn() -> FutF,
    FutF: Future<Output = anyhow::Result<String>> + Send + 'static,
    H: Fn(SolanaProgramLogData) -> FutH,
    FutH: Future<Output = anyhow::Result<()>> + Send + 'static,
    K: Fn(SolSyncSignature) -> FutK,
    FutK: Future<Output = anyhow::Result<()>> + Send + 'static,
{
    while let Err(err) = solana_program_log_processor.process_logs().await {
        tracing::error!(target: SOL_LOG_PROCESSOR, "process_solana_logs err: {}, restarting logs processing", err);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

pub(crate) async fn get_starting_signature() -> anyhow::Result<String> {
    if let Some(sync_info) = get_sol_sync_signature().await? {
        Ok(sync_info.signature)
    } else {
        Ok(get_common_cfg().sol_chain_config.start_sig.clone())
    }
}

pub(crate) struct SolanaProgramLogProcessor<F, FutF, H, FutH, K, FutK>
where
    F: Fn() -> FutF,
    FutF: Future<Output = anyhow::Result<String>> + Send + 'static,
    H: Fn(SolanaProgramLogData) -> FutH,
    FutH: Future<Output = anyhow::Result<()>> + Send + 'static,
    K: Fn(SolSyncSignature) -> FutK,
    FutK: Future<Output = anyhow::Result<()>> + Send + 'static,
{
    /// The RPC client used to interact with the Solana blockchain.
    pub rpc_client: Arc<SolRpcClient>,
    /// The address of the Solana program to fetch transactions for.
    pub program_address: String,
    /// Function to get the starting signature.
    pub get_starting_signature_fn: F,
    /// Handler function to process transactions.
    pub handle_logs_fn: H,
    /// Function to update the starting signature.
    pub update_starting_signature_fn: K,
}

impl<F, FutF, H, FutH, K, FutK> SolanaProgramLogProcessor<F, FutF, H, FutH, K, FutK>
where
    F: Fn() -> FutF,
    FutF: Future<Output = anyhow::Result<String>> + Send + 'static,
    H: Fn(SolanaProgramLogData) -> FutH,
    FutH: Future<Output = anyhow::Result<()>> + Send + 'static,
    K: Fn(SolSyncSignature) -> FutK,
    FutK: Future<Output = anyhow::Result<()>> + Send + 'static,
{
    pub fn new(
        sol_config: &SolChainConfig,
        program_address: String,
        get_starting_signature_fn: F,
        handle_logs_fn: H,
        update_starting_signature_fn: K,
    ) -> Self {
        Self {
            rpc_client: Arc::new(SolRpcClient::new(
                &sol_config.rpc_url,
                CommitmentConfig::confirmed(),
            )),
            program_address,
            get_starting_signature_fn,
            handle_logs_fn,
            update_starting_signature_fn,
        }
    }

    pub async fn process_logs(&self) -> anyhow::Result<()> {
        let mut start_sig = (self.get_starting_signature_fn)().await?;

        tracing::info!(target: SOL_LOG_PROCESSOR, "Start fetching old transactions for program: {}, starting signature: {}", self.program_address, start_sig);
        loop {
            let block_height = self.rpc_client.get_block_height().await?;
            tokio::time::sleep(Duration::from_millis(SOL_API_CALL_INTERVAL_MS)).await;
            let block_time = self.rpc_client.get_block_time(block_height).await?;
            tokio::time::sleep(Duration::from_millis(SOL_API_CALL_INTERVAL_MS)).await;
            let signatures = self.fetch_signatures(start_sig.clone()).await?;
            let mut process_block_time = 0_i64;
            for sig in &signatures {
                tracing::info!(target: SOL_LOG_PROCESSOR, "Process history signature: {}", sig.signature);
                let tx = self.get_transaction_with_retry(&sig.signature).await?;
                let block_time = tx.block_time.unwrap_or_default();

                (self.handle_logs_fn)(SolanaProgramLogData {
                    program_address: self.program_address.clone(),
                    tx_signature: sig.signature.clone(),
                    slot: sig.slot,
                    block_time,
                    logs: decode_confirm_transaction_log_messages(tx),
                })
                .await?;

                (self.update_starting_signature_fn)(sig.clone()).await?;
                process_block_time = max(process_block_time, block_time);

                tokio::time::sleep(Duration::from_millis(SOL_API_CALL_INTERVAL_MS)).await;
            }

            if !signatures.is_empty() {
                // Safely clone the last signature since we know signatures is not empty
                start_sig.clone_from(&signatures.back().unwrap().signature);
                // write sync height to db
                crate::db::settings::update_last_sol_syn_block_time(process_block_time - 1).await?;
                tokio::time::sleep(Duration::from_millis(SOL_API_CALL_INTERVAL_MS)).await;
            } else {
                crate::db::settings::update_last_sol_syn_block_time(block_time - 1).await?;
                tokio::time::sleep(Duration::from_secs(4)).await;
            }
        }
    }

    pub async fn recover_logs(&self, end_slot: u64) -> anyhow::Result<()> {
        let mut start_sig = (self.get_starting_signature_fn)().await?;
        let mut signatures_count = 0;

        tracing::info!(target: SOL_LOG_PROCESSOR, "Start recover transactions for program: {}, starting signature: {}, end slot: {}", self.program_address, start_sig, end_slot);

        'fetching: loop {
            let signatures = self.fetch_signatures(start_sig.clone()).await?;
            signatures_count += signatures.len();

            for sig in &signatures {
                if sig.slot >= end_slot {
                    tracing::info!(target: SOL_LOG_PROCESSOR, "Recover signatures for program {} finished at signature: {}, slot: {}, total signatures processed: {}", self.program_address, sig.signature, sig.slot, signatures_count);
                    break 'fetching;
                }
                tracing::info!(target: SOL_LOG_PROCESSOR, "Process signature: {}", sig.signature);
                let tx = self.get_transaction_with_retry(&sig.signature).await?;

                (self.handle_logs_fn)(SolanaProgramLogData {
                    program_address: self.program_address.clone(),
                    tx_signature: sig.signature.clone(),
                    slot: sig.slot,
                    block_time: tx.block_time.unwrap_or_default(),
                    logs: decode_confirm_transaction_log_messages(tx),
                })
                .await?;

                tokio::time::sleep(Duration::from_millis(SOL_API_CALL_INTERVAL_MS)).await;
            }

            if !signatures.is_empty() {
                // Safely clone the last signature since we know signatures is not empty
                start_sig.clone_from(&signatures.back().unwrap().signature);
            } else {
                // No more signatures to fetch; Probably end_slot is in the future
                break;
            }
        }

        Ok(())
    }

    async fn get_transaction_with_retry(
        &self,
        sig: &str,
    ) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
        let mut retry_count = 0;
        loop {
            match self
                .rpc_client
                .get_transaction_with_config(
                    &Signature::from_str(sig)?,
                    RpcTransactionConfig::new_json_confirmed_v0(),
                )
                .await
            {
                Ok(tx) => return Ok(tx),
                Err(err) => {
                    retry_count += 1;
                    if retry_count > SOL_GET_TX_RETRY_LIMIT {
                        return Err(err);
                    }
                    tracing::warn!(target: SOL_LOG_PROCESSOR, "Failed to fetch transaction: {}. Retrying...", err);
                    tokio::time::sleep(Duration::from_millis(SOL_API_CALL_INTERVAL_MS)).await;
                }
            }
        }
    }

    /// Fetches signatures for the program address from last fetched signature
    /// up to the latest signature or limit of SOL_SIG_FETCH_LIMIT is reached.
    ///
    /// Returns a VecDeque of SolSyncSignature in ascending order from oldest to newest.
    ///
    /// The sign, that current latest signature is reached is that length of the returned
    /// VecDeque is less than SOL_SIG_FETCH_LIMIT.
    pub async fn fetch_signatures(
        &self,
        start_sig: String,
    ) -> anyhow::Result<VecDeque<SolSyncSignature>> {
        tracing::info!(target: SOL_LOG_PROCESSOR, "Start fetching signatures for program: {} from signature: {}", self.program_address, start_sig);

        let mut signatures = VecDeque::<SolSyncSignature>::new();
        let mut end_sig: Option<String> = None;
        let address = Pubkey::from_str(&self.program_address)?;

        loop {
            let until = Some(Signature::from_str(&start_sig)?);
            let before = if let Some(sig) = end_sig.as_ref() {
                Some(Signature::from_str(sig)?)
            } else {
                None
            };

            tracing::debug!(target: SOL_LOG_PROCESSOR, "Fetching signatures for program: {} from signature: {:?} to signature: {:?}", self.program_address, before, until);

            let config = GetConfirmedSignaturesForAddress2Config {
                before,
                until,
                limit: Some(SOL_SIG_GET_LIMIT),
                commitment: Some(self.rpc_client.commitment()),
            };
            let sigs = self
                .rpc_client
                .get_signatures_for_address_with_config(&address, config)
                .await?;

            sigs.iter()
                .filter(|sig| sig.err.is_none())
                .map(|sig| SolSyncSignature::new(sig.signature.clone(), sig.slot))
                .for_each(|sync_sig| {
                    signatures.push_front(sync_sig);
                    if signatures.len() > SOL_SIG_FETCH_LIMIT {
                        signatures.pop_back(); // Remove the earliest signature if limit is exceeded. It will be fetched on the next iteration.
                    }
                });

            if let Some(last_sig) = sigs.last() {
                end_sig = Some(last_sig.signature.clone());
            } else {
                break;
            }

            tokio::time::sleep(Duration::from_millis(SOL_API_CALL_INTERVAL_MS)).await;
        }

        tracing::info!(target: SOL_LOG_PROCESSOR, "Finish fetching signatures for program: {}, found {} signatures", self.program_address, signatures.len());
        Ok(signatures)
    }
}

pub(crate) async fn handle_sol_program_logs(log_data: SolanaProgramLogData) -> anyhow::Result<()> {
    let mut current_contracts: Vec<String> = Vec::new();

    let vault_withdrawn_sig = {
        let vault_withdrawn_discriminator_preimage =
            "event:VaultWithdrawn".to_string().into_bytes();
        let vault_withdrawn_discriminator =
            anchor_syn::hash::hash(&vault_withdrawn_discriminator_preimage);
        vault_withdrawn_discriminator.0[..8].to_vec()
    };

    for (log_idx, log) in log_data.logs.into_iter().enumerate() {
        if let Some(program) = split_program_from_log(&log) {
            if program == log_data.program_address {
                tracing::debug!(target: SOL_LOG_PROCESSOR, "push program {:?}", program);
            }
            current_contracts.push(program.clone());
            continue;
        }
        if let Some(_program) = split_program_exit_log(&log) {
            let program = current_contracts.pop();
            if program != Some(_program.clone()) {
                tracing::warn!(target: SOL_LOG_PROCESSOR, "exit program should equal but split program: {:?}, exit program: {:?}", _program, program);
            }
            if _program == log_data.program_address {
                tracing::debug!(target: SOL_LOG_PROCESSOR, "exit and pop program: {:?}", _program);
            }
            continue;
        }
        if let Some(raw_event_data) = split_program_event(&log) {
            let data = base64::decode(raw_event_data)?;
            let (discriminator_decode, data) = data.split_at(8);
            let current_contract = current_contracts
                .last()
                .map_or_else(String::new, |s| s.clone());
            if current_contract == log_data.program_address
                && discriminator_decode == vault_withdrawn_sig
            {
                let slot = i64::try_from(log_data.slot)?;
                let block_time = log_data.block_time;
                let withdraw_data = VaultWithdrawn::deserialize(&mut &data[..])?;
                tracing::info!(target: SOL_LOG_PROCESSOR, "signature: {}, decoded vault withdrawn: {:?}", log_data.tx_signature, withdraw_data);
                let withdraw_event = DbSolTransactionEvent {
                    block_number: slot,
                    transaction_index: 0,
                    log_index: log_idx as i32,
                    transaction_id: log_data.tx_signature.clone(),
                    block_time: BigDecimal::from_i64(block_time).unwrap_or_default(),
                    account_id: hex_bytes(&withdraw_data.account_id),
                    sender: Some(withdraw_data.sender.to_base58()),
                    receiver: withdraw_data.receiver.to_base58(),
                    token_hash: hex_bytes(&withdraw_data.token_hash),
                    broker_hash: hex_bytes(&withdraw_data.broker_hash),
                    chain_id: BigDecimal::from_str(&withdraw_data.chain_id.to_string())?,
                    side: DbTransactionSide::Withdraw.value(),
                    amount: BigDecimal::from_str(&withdraw_data.token_amount.to_string())?,
                    fee: BigDecimal::from_str(&withdraw_data.fee.to_string())?,
                    status: DbTransactionStatus::Succeed.value(),
                    withdraw_nonce: Some(i64::try_from(withdraw_data.withdraw_nonce)?),
                    fail_reason: None,
                };

                create_sol_balance_transaction_executions(vec![withdraw_event]).await?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::{
        process_solana_logs, SolanaProgramLogData, SolanaProgramLogProcessor, SOL_LOG_PROCESSOR,
    };
    use crate::{
        config::{get_common_cfg, init_config, CommonConfigs},
        db::settings::SolSyncSignature,
        init::init_log,
    };

    #[ignore]
    #[tokio::test]
    async fn test_fetch_solana_transactions() {
        let raw_common_config = std::fs::read_to_string("./config.example-dev.json")
            .expect("missing_common_config_file");
        let mut config: CommonConfigs =
            serde_json::from_str(&raw_common_config).expect("unable_to_deserialize_common_configs");
        config.sol_chain_config.program_address =
            "SysvarRent111111111111111111111111111111111".to_string();
        config.sol_chain_config.start_sig = "5sPVvy4wVhsUxzu76crTnLAiBH618aNjy6k9YSHdPjtD8tvkouA59R7wZwENLc6112uQVt3bA5fmr5pruTbmh2r3".to_string();

        init_config(config.clone());
        env::set_var("RUST_LOG", "sol_sdk=error,sol_log_processor=debug");
        init_log();

        async fn get_starting_signature_config_only() -> anyhow::Result<String> {
            Ok(get_common_cfg().sol_chain_config.start_sig.clone())
        }

        async fn test_handle_sol_program_logs(
            log_data: SolanaProgramLogData,
        ) -> anyhow::Result<()> {
            tracing::debug!(target: SOL_LOG_PROCESSOR, "Handling transaction. Signature: {}, Program address: {}", log_data.tx_signature, log_data.program_address);

            Ok(())
        }

        async fn skip_signature_updating(_info: SolSyncSignature) -> anyhow::Result<()> {
            Ok(())
        }

        let sol_config = config.sol_chain_config.clone();
        let solana_log_processor = SolanaProgramLogProcessor::new(
            &sol_config,
            sol_config.program_address.clone(),
            get_starting_signature_config_only,
            test_handle_sol_program_logs,
            skip_signature_updating,
        );

        process_solana_logs(solana_log_processor).await;
    }
}
