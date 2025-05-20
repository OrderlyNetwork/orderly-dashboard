use actix_diesel::dsl::AsyncRunQueryDsl;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::db::POOL;
use crate::schema::market_info;
use crate::sync_broker::cal_symbol_hash;
use diesel::pg::upsert::excluded;

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "market_info"]
pub struct DBMarketInfo {
    pub symbol: String,
    pub symbol_hash: String,
    pub index_price: BigDecimal,
    pub mark_price: BigDecimal,
    pub sum_unitary_funding: BigDecimal,
    pub open_interest: BigDecimal,
    pub update_time: NaiveDateTime,
}

impl DBMarketInfo {
    pub fn new(
        symbol: String,
        index_price: BigDecimal,
        mark_price: BigDecimal,
        sum_unitary_funding: BigDecimal,
        open_interest: BigDecimal,
        update_time: NaiveDateTime,
    ) -> DBMarketInfo {
        let symbol_hash = cal_symbol_hash(&symbol);
        DBMarketInfo {
            symbol,
            symbol_hash,
            index_price,
            mark_price,
            sum_unitary_funding,
            open_interest,
            update_time,
        }
    }
}

pub async fn create_or_update_market_infos(market_infos: Vec<DBMarketInfo>) -> anyhow::Result<()> {
    use crate::schema::market_info::dsl::*;
    diesel::insert_into(market_info)
        .values(market_infos)
        .on_conflict(symbol_hash)
        .do_update()
        .set((
            index_price.eq(excluded(index_price)),
            mark_price.eq(excluded(mark_price)),
            sum_unitary_funding.eq(excluded(sum_unitary_funding)),
            open_interest.eq(excluded(open_interest)),
            update_time.eq(excluded(update_time)),
        ))
        .execute_async(&POOL)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::list_market_infos;
    use num_traits::FromPrimitive;

    fn init_log() {
        tracing_subscriber::fmt::Subscriber::builder()
            .with_writer(std::io::stderr)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
    }
    #[ignore]
    #[actix_web::test]
    async fn test_upsert_market_infos() {
        dotenv::dotenv().ok();
        init_log();
        let base_url = "https://api.orderly.org";
        let data = list_market_infos(base_url).await.unwrap();
        tracing::info!("market infos: {:?}", data);
        let nsecs = data.timestamp % 1000 * 1_000_000;
        let update_time =
            NaiveDateTime::from_timestamp_opt(data.timestamp / 1000, nsecs as u32).unwrap();
        let market_infos = data
            .data
            .rows
            .into_iter()
            .map(|v| {
                DBMarketInfo::new(
                    v.symbol,
                    BigDecimal::from_f64(v.index_price).unwrap(),
                    BigDecimal::from_f64(v.mark_price).unwrap(),
                    BigDecimal::from_f64(v.sum_unitary_funding).unwrap(),
                    BigDecimal::from_f64(v.open_interest).unwrap(),
                    update_time,
                )
            })
            .collect::<Vec<_>>();
        create_or_update_market_infos(market_infos).await.unwrap();
    }
}
