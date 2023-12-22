use std::ops::Neg;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

use crate::db::hourly_orderly_perp::{
    create_or_update_hourly_orderly_perp, find_hourly_orderly_perp,
};
use crate::db::hourly_user_perp::{create_or_update_hourly_user_perp, find_hourly_user_perp};
use crate::db::user_perp_summary::{create_or_update_user_perp_summary, find_user_perp_summary};

pub async fn analyzer_adl(
    account_id: String,
    insurance_account_id: String,
    symbol_hash: String,
    position_qty_transfer: String,
    cost_position_transfer: String,
    adl_price: String,
    sum_unitary_fundings: String,
    block_hour: NaiveDateTime,
    pulled_block_time: NaiveDateTime,
    block_num: i64,
) {
    let mut hourly_orderly_perp = find_hourly_orderly_perp(symbol_hash.clone(), block_hour.clone())
        .await
        .unwrap();
    hourly_orderly_perp.new_adl(
        BigDecimal::from_str(&*position_qty_transfer.clone()).unwrap(),
        BigDecimal::from_str(&*adl_price.clone()).unwrap(),
        block_num,
        pulled_block_time.clone(),
    );

    let mut hourly_user_perp =
        find_hourly_user_perp(account_id.clone(), symbol_hash.clone(), block_hour.clone())
            .await
            .unwrap();
    hourly_user_perp.new_adl(
        BigDecimal::from_str(&*position_qty_transfer.clone()).unwrap(),
        BigDecimal::from_str(&*adl_price.clone()).unwrap(),
        block_num,
        pulled_block_time.clone(),
    );

    let mut user_perp_summary = find_user_perp_summary(account_id.clone(), symbol_hash.clone())
        .await
        .unwrap();
    user_perp_summary.new_adl(
        BigDecimal::from_str(&*position_qty_transfer.clone()).unwrap(),
        BigDecimal::from_str(&*adl_price.clone()).unwrap(),
        block_num,
        pulled_block_time.clone(),
        cost_position_transfer.clone(),
        sum_unitary_fundings.clone(),
    );

    let mut insurance_perp_summary =
        find_user_perp_summary(insurance_account_id.clone(), symbol_hash.clone())
            .await
            .unwrap();
    insurance_perp_summary.new_adl(
        BigDecimal::from_str(&*position_qty_transfer).unwrap().neg(),
        BigDecimal::from_str(&*adl_price).unwrap(),
        block_num,
        pulled_block_time.clone(),
        cost_position_transfer,
        sum_unitary_fundings,
    );

    create_or_update_hourly_user_perp(vec![&hourly_user_perp])
        .await
        .unwrap();
    create_or_update_hourly_orderly_perp(vec![&hourly_orderly_perp])
        .await
        .unwrap();
    create_or_update_user_perp_summary(vec![&user_perp_summary, &insurance_perp_summary])
        .await
        .unwrap();
}
