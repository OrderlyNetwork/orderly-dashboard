# How to indexing data
Indexer will pull block with transaction and block receipts when new blocks generated, and then consume those data.

Indexer can consume blocks in parallel to accelerate sync block speed
```rust
pub async fn parallel_consume_blocks(start_height: u64, gap: u64) -> Result<()> {
    let mut futs = Vec::with_capacity(gap as usize);
    (start_height..=(start_height + gap)).for_each(|block_height| {
        futs.push(consume_data_on_block(block_height));
    });
    let res = raw_spawn_future(join_all(futs)).await?;
    return if res.iter().any(|r| {
        r.as_ref()
            .map_err(|r| {
                tracing::warn!(target: ORDERLY_DASHBOARD_INDEXER, "consume block task err:{}", r);
                r
            })
            .is_err()
    }) {
        Err(anyhow::anyhow!("Some task failed to be executed."))
    } else {
        Ok(())
    };
}
```

The `consume_data_on_block` function will consume data in one block
```rust
pub(crate) async fn consume_data_on_block(block_height: u64) -> anyhow::Result<()> {
    tracing::info!(
        target: HANDLE_LOG,
        "consume_data_on_block block_height: {}",
        block_height
    );
    let (block, tx_receipt_vec) = query_and_filter_block_data_info(block_height).await?;
    consume_logs_from_tx_receipts(block.clone(), &tx_receipt_vec).await?;

    Ok(())
}
```

The `consume_logs_from_tx_receipts` function will deserialize log or transaction tx params to extract trading data
```rust
pub(crate) async fn consume_logs_from_tx_receipts(
    block: Block<Transaction>,
    tx_receipts: &Vec<(Transaction, TransactionReceipt)>,
) -> Result<()> {
    for tx_receipt in tx_receipts.iter() {
        let (tx, receipt) = &tx_receipt;
        // cache
        let mut liquidation_result_log_index_queue: VecDeque<i32> = VecDeque::new();
        let mut settlement_result_log_index_queue: VecDeque<i32> = VecDeque::new();
        let mut liquidation_trasfers_cache: Vec<DbLiquidationTransfer> = Vec::new();
        let mut settlement_exectutions_cahce: Vec<DbSettlementExecution> = Vec::new();
        // 1 (success)
        if receipt.status.unwrap_or_default().as_u64() == 1 {
            for log in &receipt.logs {
                if let Err(err) = handle_log(
                    &mut liquidation_result_log_index_queue,
                    &mut settlement_result_log_index_queue,
                    &mut liquidation_trasfers_cache,
                    &mut settlement_exectutions_cahce,
                    log.clone(),
                    receipt,
                    Some(block.timestamp.as_u64()),
                )
                .await
                {
                    tracing::warn!(target: HANDLE_LOG, "handle_log meet err:{:?}", err);
                }
            }
            if block.number.unwrap_or_default().as_u64()
                < unsafe { COMMON_CONFIGS.get_unchecked().l2_config.upgrade_height }
            {
                if let Err(err) = handle_tx_params(
                    &mut liquidation_result_log_index_queue,
                    &mut settlement_result_log_index_queue,
                    tx,
                    Some(block.timestamp.as_u64()),
                )
                .await
                {
                    tracing::warn!(target: HANDLE_LOG, "handle_tx_params meet err:{:?}, block: {}", err, block.number.unwrap_or_default().as_u64());
                }
            }
        }
        // excute
        // clean cache
        liquidation_result_log_index_queue.clear();
    }
    Ok(())
}
```

Indexer module use [ether-rs]() to decode contract events, when contracts upgrading they may emit new events, deprecated events will reserve the event type in latest contract, so we can indexing and deserialize contract events from contract deployment time to now.
```rust
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum operator_managerEvents {
        ChangeCefiUploadFilter(ChangeCefiUploadFilter),
        ChangeLedgerFilter(ChangeLedgerFilter),
        ChangeMarketManagerFilter(ChangeMarketManagerFilter),
        ChangeOperatorFilter(ChangeOperatorFilter),
        EventUpload1Filter(EventUpload1Filter),
        EventUpload2Filter(EventUpload2Filter),
        FuturesTradeUpload1Filter(FuturesTradeUpload1Filter),
        FuturesTradeUpload2Filter(FuturesTradeUpload2Filter),
    }

    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum user_ledgerEvents {
        AccountDeposit1Filter(AccountDeposit1Filter),
        AccountDeposit2Filter(AccountDeposit2Filter),
        AccountRegister1Filter(AccountRegister1Filter),
        AccountRegister2Filter(AccountRegister2Filter),
        AccountWithdrawApprove1Filter(AccountWithdrawApprove1Filter),
        AccountWithdrawApprove2Filter(AccountWithdrawApprove2Filter),
        AccountWithdrawFail1Filter(AccountWithdrawFail1Filter),
        AccountWithdrawFail2Filter(AccountWithdrawFail2Filter),
        AccountWithdrawFinish1Filter(AccountWithdrawFinish1Filter),
        AccountWithdrawFinish2Filter(AccountWithdrawFinish2Filter),
        AdlResultFilter(AdlResultFilter),
        AdlResultV2Filter(AdlResultV2Filter),
        ChangeCrossChainManagerFilter(ChangeCrossChainManagerFilter),
        ChangeFeeManagerFilter(ChangeFeeManagerFilter),
        ChangeLedgerImplAFilter(ChangeLedgerImplAFilter),
        ChangeMarketManagerFilter(ChangeMarketManagerFilter),
        ChangeOperatorManagerFilter(ChangeOperatorManagerFilter),
        ChangeVaultManagerFilter(ChangeVaultManagerFilter),
        DelegateSignerFilter(DelegateSignerFilter),
        FeeDistributionFilter(FeeDistributionFilter),
        InitializedFilter(InitializedFilter),
        LiquidationResultFilter(LiquidationResultFilter),
        LiquidationResultV2Filter(LiquidationResultV2Filter),
        LiquidationTransferFilter(LiquidationTransferFilter),
        LiquidationTransferV2Filter(LiquidationTransferV2Filter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProcessValidatedFutures1Filter(ProcessValidatedFutures1Filter),
        ProcessValidatedFutures2Filter(ProcessValidatedFutures2Filter),
        SettlementExecutionFilter(SettlementExecutionFilter),
        SettlementResultFilter(SettlementResultFilter),
    }
```