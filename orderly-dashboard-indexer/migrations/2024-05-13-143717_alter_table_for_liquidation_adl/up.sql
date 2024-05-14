-- Your SQL goes here
-- v2 removed insuranceAccountId
alter table adl_result add version smallint;

-- v2 removed liquidatedAccountId,insuranceAccountId; add accountId(reuse liquidatedAccountId)
alter table liquidation_result add version smallint;

-- v2 remove liquidationTransferId, liquidatorAccountId, liquidatorFee, insuranceFee, liquidationFee; add accountId(reused liquidatorAccountId), fee(reuse liquidationFee)
alter table liquidation_transfer add version smallint;