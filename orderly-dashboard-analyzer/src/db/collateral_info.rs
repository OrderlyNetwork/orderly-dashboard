use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::collateral_info;
use diesel::pg::upsert::excluded;

#[derive(Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = collateral_info)]
pub struct DBCollateralInfo {
    pub token: String,
    pub token_hash: String,
    pub decimals: i16,
    pub minimum_withdraw_amount: BigDecimal,
    pub base_weight: BigDecimal,
    pub discount_factor: Option<BigDecimal>,
    pub haircut: BigDecimal,
    pub user_max_qty: BigDecimal,
    pub is_collateral: bool,
    pub update_time: NaiveDateTime,
}

impl DBCollateralInfo {
    pub fn new(
        token: String,
        token_hash: String,
        decimals: i16,
        minimum_withdraw_amount: BigDecimal,
        base_weight: BigDecimal,
        discount_factor: Option<BigDecimal>,
        haircut: BigDecimal,
        user_max_qty: BigDecimal,
        is_collateral: bool,
        update_time: NaiveDateTime,
    ) -> DBCollateralInfo {
        DBCollateralInfo {
            token,
            token_hash,
            decimals,
            minimum_withdraw_amount,
            base_weight,
            discount_factor,
            haircut,
            user_max_qty,
            is_collateral,
            update_time,
        }
    }
}

pub async fn create_or_update_collateral_infos(
    collateral_infos: Vec<DBCollateralInfo>,
) -> anyhow::Result<()> {
    use crate::schema::collateral_info::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    diesel::insert_into(collateral_info)
        .values(collateral_infos)
        .on_conflict(token)
        .do_update()
        .set((
            minimum_withdraw_amount.eq(excluded(minimum_withdraw_amount)),
            base_weight.eq(excluded(base_weight)),
            discount_factor.eq(excluded(discount_factor)),
            haircut.eq(excluded(haircut)),
            is_collateral.eq(excluded(is_collateral)),
            update_time.eq(excluded(update_time)),
        ))
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn find_collateral_info_by_hash(
    token_hash_: String,
) -> anyhow::Result<Option<DBCollateralInfo>> {
    use crate::schema::collateral_info::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result = collateral_info
        .filter(token_hash.eq(token_hash_))
        .first::<DBCollateralInfo>(&mut conn)
        .await;

    match select_result {
        Ok(collateral_data) => Ok(Some(collateral_data)),
        Err(error) => match error {
            diesel::NotFound => Ok(None),
            _ => Err(anyhow::anyhow!(
                "find_collateral_info_by_hash execute err: {}",
                error
            )),
        },
    }
}

#[cfg(test)]
mod tests {
    fn init_log() {
        tracing_subscriber::fmt::Subscriber::builder()
            .with_writer(std::io::stderr)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
    }
}
