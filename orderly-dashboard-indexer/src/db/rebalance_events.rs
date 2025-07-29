use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::rebalance_events;
use diesel_async::RunQueryDsl;

use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};

#[derive(Insertable, Queryable, Debug)]
#[diesel(table_name = rebalance_events)]
pub struct DBRebalanceEvent {
    pub rebalance_id: i64,
    pub token_hash: String,
    pub amount: BigDecimal,
    pub src_chain_id: BigDecimal,
    pub dst_chain_id: BigDecimal,
    pub burn_tx_id: String,
    pub burn_result_tx_id: Option<String>,
    pub mint_tx_id: Option<String>,
    pub mint_result_tx_id: Option<String>,
    pub burn_result_block_time: Option<i64>,
    pub mint_result_block_time: Option<i64>,
    pub burn_result_block_number: Option<i64>,
    pub mint_result_block_number: Option<i64>,
    pub burn_success: Option<bool>,
    pub mint_success: Option<bool>,
}

impl DBRebalanceEvent {
    pub fn new(
        rebalance_id: i64,
        token_hash: String,
        amount: BigDecimal,
        src_chain_id: BigDecimal,
        dst_chain_id: BigDecimal,
        burn_tx_id: String,
    ) -> DBRebalanceEvent {
        DBRebalanceEvent {
            rebalance_id,
            token_hash,
            amount,
            src_chain_id,
            dst_chain_id,
            burn_tx_id,
            burn_result_tx_id: None,
            mint_tx_id: None,
            mint_result_tx_id: None,
            burn_result_block_time: None,
            mint_result_block_time: None,
            burn_result_block_number: None,
            mint_result_block_number: None,
            mint_success: None,
            burn_success: None,
        }
    }
}

pub(crate) async fn create_rebalance(rebalance_event: DBRebalanceEvent) -> Result<usize> {
    use crate::schema::rebalance_events::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);

    let num_rows = diesel::insert_into(rebalance_events)
        .values(rebalance_event)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(num_rows)
}

pub(crate) async fn update_rebalance_burn_status(
    rebalance_id_: i64,
    burn_result_tx_id_: String,
    burn_result_block_time_: i64,
    burn_result_block_number_: i64,
    is_success_: bool,
) -> Result<()> {
    use crate::schema::rebalance_events::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let target = rebalance_events.filter(rebalance_id.eq(rebalance_id_));

    diesel::update(target)
        .set((
            burn_result_tx_id.eq(burn_result_tx_id_),
            burn_result_block_time.eq(burn_result_block_time_),
            burn_result_block_number.eq(burn_result_block_number_),
            burn_success.eq(is_success_),
        ))
        .execute(&mut conn)
        .await?;
    Ok(())
}

pub(crate) async fn update_rebalance_mint(rebalance_id_: i64, mint_tx_id_: String) -> Result<()> {
    use crate::schema::rebalance_events::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let target = rebalance_events.filter(rebalance_id.eq(rebalance_id_));

    diesel::update(target)
        .set(mint_tx_id.eq(mint_tx_id_))
        .execute(&mut conn)
        .await?;
    Ok(())
}

pub(crate) async fn update_rebalance_mint_status(
    rebalance_id_: i64,
    mint_result_tx_id_: String,
    mint_result_block_time_: i64,
    mint_result_block_number_: i64,
    is_success_: bool,
) -> Result<()> {
    use crate::schema::rebalance_events::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let target = rebalance_events.filter(rebalance_id.eq(rebalance_id_));

    diesel::update(target)
        .set((
            mint_result_tx_id.eq(mint_result_tx_id_),
            mint_result_block_time.eq(mint_result_block_time_),
            mint_result_block_number.eq(mint_result_block_number_),
            mint_success.eq(is_success_),
        ))
        .execute(&mut conn)
        .await?;
    Ok(())
}
