use crate::{
    bindings::{
        market_manager::{FundingDataFilter, MarketDataFilter},
        operator_manager::{
            EventUpload1Filter, EventUpload2Filter, FuturesTradeUpload1Filter,
            FuturesTradeUpload2Filter,
        },
        user_ledger::{
            AccountDeposit1Filter, AccountDeposit2Filter, AccountDepositSolFilter,
            AccountWithdrawApprove1Filter, AccountWithdrawApprove2Filter,
            AccountWithdrawFail1Filter, AccountWithdrawFail2Filter, AccountWithdrawFinish1Filter,
            AccountWithdrawFinish2Filter, AccountWithdrawSolApproveFilter,
            AccountWithdrawSolFailFilter, BalanceTransferFilter, FeeDistributionFilter,
            LiquidationResultFilter, LiquidationResultV2Filter, LiquidationTransferFilter,
            LiquidationTransferV2Filter, ProcessValidatedFutures1Filter,
            ProcessValidatedFutures2Filter, SettlementExecutionFilter, SettlementResultFilter,
            SwapResultUploadedFilter,
        },
        vault_manager::{
            RebalanceBurnFilter, RebalanceBurnResultFilter, RebalanceMintFilter,
            RebalanceMintResultFilter,
        },
    },
    config::COMMON_CONFIGS,
    contract::TARGET_ADDRS,
};
use anyhow::Result;
use ethers::prelude::{
    Block, BlockId, BlockNumber, Filter, Log, Transaction, TransactionReceipt, H256,
};
use ethers::providers::{Http, Middleware, Provider};
use ethers_contract::EthEvent;
use once_cell::sync::OnceCell;
use std::time::Duration;
use tokio::time::timeout;

use crate::consume_data_task::ORDERLY_DASHBOARD_INDEXER;
pub(crate) static PROVIDER: OnceCell<Provider<Http>> = OnceCell::new();

pub(crate) fn init_provider() -> Result<()> {
    let rpc = if let Ok(orderly_rpc) = std::env::var("ORDERLY_RPC") {
        orderly_rpc
    } else {
        unsafe { &COMMON_CONFIGS.get_unchecked().l2_config.rpc_url }.clone()
    };
    tracing::info!(target: ORDERLY_DASHBOARD_INDEXER, "rpc wrapped: {}", rpc[0..8].to_string() + "***" + &rpc[rpc.len() - 5..]);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let http_client = Http::new_with_client(url::Url::parse(&rpc)?, client);
    let provider = Provider::new(http_client);
    PROVIDER.set(provider).ok();
    Ok(())
}

pub(crate) fn clone_provider() -> Provider<Http> {
    unsafe { PROVIDER.get_unchecked() }.clone()
}

pub async fn get_latest_block_num() -> Result<u64> {
    Ok(get_blocknumber_with_timeout().await?)
}

pub async fn get_blocknumber_with_timeout() -> Result<u64> {
    let provider = unsafe { PROVIDER.get_unchecked() };

    let result = timeout(Duration::from_secs(3), provider.get_block_number()).await;
    match result {
        Err(_) => {
            return Err(anyhow::anyhow!("request elapsed"));
        }
        Ok(Ok(number)) => {
            return Ok(number.as_u64());
        }
        Ok(Err(err)) => {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "get_blocknumber query err: {}",
                err
            );
            return Err(anyhow::anyhow!("query err"));
        }
    }
}

pub async fn get_block_with_txs(block_num: u64) -> Result<Block<Transaction>> {
    let provider = unsafe { PROVIDER.get_unchecked() };

    let result = timeout(
        Duration::from_secs(8),
        provider.get_block_with_txs(BlockId::Number(BlockNumber::Number(block_num.into()))),
    )
    .await;
    match result {
        Err(_) => {
            return Err(anyhow::anyhow!("get_block_with_txs request elapsed for 8s"));
        }
        Ok(Ok(block)) => {
            return Ok(
                block.ok_or_else(|| anyhow::anyhow!("block not found in calling block api"))?
            );
        }
        Ok(Err(err)) => {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "get_block_with_txs query err: {}",
                err
            );
            return Err(anyhow::anyhow!("query err"));
        }
    }
}

#[allow(dead_code)]
pub async fn get_tx_receipt(tx_hash: H256) -> Result<TransactionReceipt> {
    let provider = unsafe { PROVIDER.get_unchecked() };

    let result = timeout(
        Duration::from_secs(3),
        provider.get_transaction_receipt(tx_hash),
    )
    .await;
    match result {
        Err(_) => {
            return Err(anyhow::anyhow!("get_tx_receipt request elapsed"));
        }
        Ok(Ok(receipt)) => {
            return Ok(
                receipt.ok_or_else(|| anyhow::anyhow!("receipt not found in calling block api"))?
            );
        }
        Ok(Err(err)) => {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "get_tx_receipt query err: {}",
                err
            );
            return Err(anyhow::anyhow!("get_tx_receipt query err:{}", err));
        }
    }
}

pub async fn get_block_receipts(block_num: u64) -> Result<Vec<TransactionReceipt>> {
    let provider = unsafe { PROVIDER.get_unchecked() };

    let result = timeout(
        Duration::from_secs(8),
        provider.get_block_receipts(BlockNumber::Number(block_num.into())),
    )
    .await;
    match result {
        Err(_) => {
            return Err(anyhow::anyhow!("get_block_receipts request elapsed"));
        }
        Ok(Ok(receipts)) => {
            return Ok(receipts);
        }
        Ok(Err(err)) => {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "get_block_receipts query err: {}",
                err
            );
            return Err(anyhow::anyhow!("get_block_receipts query err:{}", err));
        }
    }
}

pub async fn get_block_logs(block_num: u64) -> Result<Vec<Log>> {
    let provider = unsafe { PROVIDER.get_unchecked() };
    let topic0 = vec![
        AccountDeposit1Filter::signature(),
        AccountDeposit2Filter::signature(),
        AccountWithdrawFinish1Filter::signature(),
        AccountWithdrawFinish2Filter::signature(),
        AccountWithdrawApprove1Filter::signature(),
        AccountWithdrawApprove2Filter::signature(),
        AccountWithdrawSolFailFilter::signature(),
        AccountWithdrawFail1Filter::signature(),
        AccountWithdrawFail2Filter::signature(),
        ProcessValidatedFutures1Filter::signature(),
        ProcessValidatedFutures2Filter::signature(),
        EventUpload1Filter::signature(),
        EventUpload2Filter::signature(),
        FuturesTradeUpload1Filter::signature(),
        FuturesTradeUpload2Filter::signature(),
        MarketDataFilter::signature(),
        FundingDataFilter::signature(),
        FeeDistributionFilter::signature(),
        SettlementResultFilter::signature(),
        SettlementExecutionFilter::signature(),
        LiquidationTransferFilter::signature(),
        LiquidationResultFilter::signature(),
        LiquidationResultV2Filter::signature(),
        LiquidationTransferV2Filter::signature(),
        BalanceTransferFilter::signature(),
        AccountDepositSolFilter::signature(),
        AccountWithdrawSolApproveFilter::signature(),
        RebalanceBurnFilter::signature(),
        RebalanceBurnResultFilter::signature(),
        RebalanceMintFilter::signature(),
        RebalanceMintResultFilter::signature(),
        SwapResultUploadedFilter::signature(),
    ];
    let address = unsafe { TARGET_ADDRS.get_unchecked() }.clone();
    let filter = Filter::new()
        .from_block(block_num)
        .to_block(block_num)
        .address(address)
        .topic0(topic0);
    let result = timeout(Duration::from_secs(8), provider.get_logs(&filter)).await;
    match result {
        Err(_) => {
            return Err(anyhow::anyhow!("get_block_receipts request elapsed"));
        }
        Ok(Ok(receipts)) => {
            return Ok(receipts);
        }
        Ok(Err(err)) => {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "get_block_receipts query err: {}",
                err
            );
            return Err(anyhow::anyhow!("get_block_receipts query err:{}", err));
        }
    }
}

pub async fn get_blockheader_by_number(number: u64) -> Result<Block<H256>> {
    let provider = unsafe { PROVIDER.get_unchecked() };

    let result = timeout(
        Duration::from_secs(3),
        provider.get_block(BlockNumber::Number(number.into())),
    )
    .await;
    match result {
        Err(_) => {
            return Err(anyhow::anyhow!("request elapsed"));
        }
        Ok(Ok(Some(block))) => {
            return Ok(block);
        }
        Ok(Ok(None)) => {
            return Err(anyhow::anyhow!("block number found for number: {}", number));
        }
        Ok(Err(err)) => {
            tracing::warn!(
                target: ORDERLY_DASHBOARD_INDEXER,
                "get_blockheader_by_number query err: {}",
                err
            );
            return Err(anyhow::anyhow!("query err"));
        }
    }
}
