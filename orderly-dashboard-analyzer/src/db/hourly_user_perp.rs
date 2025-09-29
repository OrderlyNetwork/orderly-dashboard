use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::pg::upsert::{excluded, on_constraint};
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel_async::RunQueryDsl;

use crate::db::{PrimaryKey, BATCH_UPSERT_LEN, DB_CONN_ERR_MSG, DB_CONTEXT, POOL};
use crate::schema::hourly_user_perp;

#[derive(Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = hourly_user_perp)]
pub struct HourlyUserPerp {
    account_id: String,
    symbol: String,
    block_hour: NaiveDateTime,

    trading_fee: BigDecimal,
    trading_volume: BigDecimal,
    trading_count: i64,

    realized_pnl: BigDecimal,
    un_realized_pnl: BigDecimal,
    latest_sum_unitary_funding: BigDecimal,

    liquidation_amount: BigDecimal,
    liquidation_count: i64,

    pub pulled_block_height: i64,
    pub pulled_block_time: NaiveDateTime,
}

#[derive(Debug, QueryableByName, Clone)]
pub struct AccountVolume {
    #[diesel(sql_type = Text)]
    pub account_id: String,
    #[diesel(sql_type = Numeric)]
    pub volume: BigDecimal,
}

impl HourlyUserPerp {
    pub fn new_emtpy_hourly_user_perp(
        account_id: &str,
        symbol: &str,
        block_hour: NaiveDateTime,
    ) -> HourlyUserPerp {
        HourlyUserPerp {
            account_id: account_id.to_string(),
            symbol: symbol.to_string(),
            block_hour: block_hour,
            trading_fee: Default::default(),
            trading_volume: Default::default(),
            trading_count: 0,
            realized_pnl: Default::default(),
            un_realized_pnl: Default::default(),
            latest_sum_unitary_funding: Default::default(),
            liquidation_amount: Default::default(),
            liquidation_count: 0,
            pulled_block_height: 0,
            pulled_block_time: Default::default(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct HourlyUserPerpKey {
    pub account_id: String,
    pub symbol: String,
    pub block_hour: NaiveDateTime,
}

impl HourlyUserPerpKey {
    pub fn new_key(account_id: String, symbol: String, block_hour: NaiveDateTime) -> Self {
        HourlyUserPerpKey {
            account_id,
            symbol,
            block_hour,
        }
    }
}

impl HourlyUserPerp {
    pub fn new_trade(
        &mut self,
        fee: BigDecimal,
        amount: BigDecimal,
        pulled_block_height: i64,
        realized_pnl: BigDecimal,
    ) -> bool {
        if pulled_block_height <= self.pulled_block_height {
            // already processed this block events
            return false;
        }
        let new_hourly_user = self.trading_count == 0;

        self.trading_fee += fee;
        self.trading_volume += amount.abs();
        self.trading_count += 1;
        self.realized_pnl += realized_pnl;

        new_hourly_user
    }

    #[allow(duplicate_macro_attributes)]
    pub fn new_liquidation(
        &mut self,
        liquidation_amount: BigDecimal,
        block_num: i64,
        realized_pnl: BigDecimal,
    ) {
        if block_num <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.liquidation_count += 1;
        self.liquidation_amount += liquidation_amount.abs();
        self.realized_pnl += realized_pnl;
    }

    pub fn new_realized_pnl(&mut self, pnl: BigDecimal, block_num: i64) {
        if block_num <= self.pulled_block_height {
            // already processed this block events
            return;
        }
        self.realized_pnl += pnl;
    }
}

impl PrimaryKey for HourlyUserPerpKey {}

pub async fn find_hourly_user_perp(
    p_account_id: String,
    p_symbol: String,
    p_block_hour: NaiveDateTime,
) -> anyhow::Result<HourlyUserPerp> {
    use crate::schema::hourly_user_perp::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result = hourly_user_perp
        .filter(account_id.eq(p_account_id.clone()))
        .filter(symbol.eq(p_symbol.clone()))
        .filter(block_hour.eq(p_block_hour.clone()))
        .first::<HourlyUserPerp>(&mut conn)
        .await;

    match select_result {
        Ok(perp_data) => Ok(perp_data),
        Err(error) => match error {
            diesel::NotFound => {
                let new_perp = HourlyUserPerp {
                    account_id: p_account_id.clone(),
                    symbol: p_symbol.clone(),
                    block_hour: p_block_hour,
                    trading_fee: Default::default(),
                    trading_volume: Default::default(),
                    trading_count: 0,
                    realized_pnl: Default::default(),
                    un_realized_pnl: Default::default(),
                    latest_sum_unitary_funding: Default::default(),
                    liquidation_amount: Default::default(),
                    liquidation_count: 0,
                    pulled_block_height: 0,
                    pulled_block_time: Default::default(),
                };

                Ok(new_perp)
            }
            _ => Err(anyhow::anyhow!(
                "find_hourly_user_perp for account_id {}, symbol {}, p_block_hour: {}, err: {}",
                p_account_id,
                p_symbol,
                p_block_hour,
                error
            )),
        },
    }
}

pub async fn create_or_update_hourly_user_perp(
    p_hourly_user_perp_vec: Vec<&HourlyUserPerp>,
) -> anyhow::Result<usize> {
    if p_hourly_user_perp_vec.is_empty() {
        return Ok(0);
    }

    let length = p_hourly_user_perp_vec.len();
    tracing::info!(target: DB_CONTEXT, "dbwrite create_or_update_hourly_user_perp start length: {}", length);
    let inst = std::time::Instant::now();

    let mut row_nums = 0;
    let mut p_hourly_user_perp_vec_ref = p_hourly_user_perp_vec.as_slice();
    loop {
        if p_hourly_user_perp_vec_ref.len() >= BATCH_UPSERT_LEN {
            let values1: &[&HourlyUserPerp];
            (values1, p_hourly_user_perp_vec_ref) =
                p_hourly_user_perp_vec_ref.split_at(BATCH_UPSERT_LEN);
            #[allow(suspicious_double_ref_op)]
            let values1 = values1
                .iter()
                .map(|v| v.clone().clone())
                .collect::<Vec<HourlyUserPerp>>();

            let update_result = _create_or_update_hourly_user_perp(values1).await;

            match update_result {
                Ok(len) => {
                    row_nums += len;
                }
                Err(err) => {
                    return Err(anyhow::anyhow!("update hourly_user_perp failed: {}", err));
                }
            }
        } else {
            #[allow(suspicious_double_ref_op)]
            let values1 = p_hourly_user_perp_vec_ref
                .iter()
                .map(|v| v.clone().clone())
                .collect::<Vec<HourlyUserPerp>>();
            let update_result = _create_or_update_hourly_user_perp(values1).await;

            match update_result {
                Ok(len) => {
                    row_nums += len;
                    break;
                }
                Err(err) => {
                    return Err(anyhow::anyhow!("update hourly_user_perp failed: {}", err));
                }
            }
        }
    }

    tracing::info!(target: DB_CONTEXT, "dbwrite create_or_update_hourly_user_perp end length: {}, time cost: {:?}", length, inst.elapsed());

    Ok(row_nums)
}

async fn _create_or_update_hourly_user_perp(
    hourly_user_perp_vec: Vec<HourlyUserPerp>,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::hourly_user_perp::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let update_result = diesel::insert_into(hourly_user_perp)
        .values(hourly_user_perp_vec)
        .on_conflict(on_constraint("hourly_user_perp_uq"))
        .do_update()
        .set((
            trading_fee.eq(excluded(trading_fee)),
            trading_volume.eq(excluded(trading_volume)),
            trading_count.eq(excluded(trading_count)),
            realized_pnl.eq(excluded(realized_pnl)),
            un_realized_pnl.eq(excluded(un_realized_pnl)),
            latest_sum_unitary_funding.eq(excluded(latest_sum_unitary_funding)),
            liquidation_amount.eq(excluded(liquidation_amount)),
            liquidation_count.eq(excluded(liquidation_count)),
            pulled_block_height.eq(excluded(pulled_block_height)),
            pulled_block_time.eq(excluded(pulled_block_time)),
        ))
        .execute(&mut conn)
        .await;
    update_result
}

pub async fn get_user_trading_volume_in_time_range(
    account_ids: Vec<String>,
    from_time: i64,
    to_time: i64,
) -> anyhow::Result<Vec<AccountVolume>> {
    if account_ids.is_empty() {
        return Ok(vec![]);
    }
    #[allow(unused_imports)]
    use crate::{
        db::{hourly_user_perp::HourlyUserPerp, POOL},
        schema::hourly_user_perp,
        schema::hourly_user_perp::dsl::*,
    };

    let conditions = account_ids
        .into_iter()
        .map(|a| format!("'{}'", a))
        .collect::<Vec<_>>()
        .join(",");
    let query = format!(
        "select account_id,sum(trading_volume) as volume from hourly_user_perp \
    where block_hour>=$1 and block_hour<$2 and account_id in ({}) group by account_id",
        conditions
    );

    let from_time = NaiveDateTime::from_timestamp_opt(from_time, 0).unwrap_or_default();
    let to_time = NaiveDateTime::from_timestamp_opt(to_time, 0).unwrap_or_default();
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let select_result = diesel::sql_query(&query)
        .bind::<Timestamp, _>(from_time)
        .bind::<Timestamp, _>(to_time)
        .get_results::<AccountVolume>(&mut conn)
        .await?;

    Ok(select_result)
}
