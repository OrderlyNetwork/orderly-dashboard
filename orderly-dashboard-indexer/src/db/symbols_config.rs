use crate::db::{DB_CONTEXT, POOL};
use crate::schema::symbols_config;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use bigdecimal::BigDecimal;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::{Insertable, Queryable};

#[derive(Insertable, Queryable, Debug)]
#[table_name = "symbols_config"]
pub struct DbSymbolConfig {
    pub symbol: String,
    pub symbol_hash: String,
    pub base_maintenance_margin: Option<i32>,
    pub base_initial_margin: Option<i32>,
    pub mark_price: Option<BigDecimal>,
    pub index_price_orderly: Option<BigDecimal>,
    pub sum_unitary_fundings: Option<BigDecimal>,
    pub last_mark_price_updated: Option<BigDecimal>,
    pub last_funding_updated: Option<BigDecimal>,
}

pub async fn update_symbol_cfgs(symbols_conf: Vec<DbSymbolConfig>) -> Result<usize> {
    use crate::schema::symbols_config::dsl::*;
    use diesel::pg::upsert::excluded;

    let num_rows = diesel::insert_into(symbols_config)
        .values(symbols_conf)
        .on_conflict(symbol)
        .do_update()
        .set((
            base_maintenance_margin.eq(excluded(base_maintenance_margin)),
            base_initial_margin.eq(excluded(base_initial_margin)),
            mark_price.eq(excluded(mark_price)),
            index_price_orderly.eq(excluded(index_price_orderly)),
            sum_unitary_fundings.eq(excluded(sum_unitary_fundings)),
            last_mark_price_updated.eq(excluded(last_mark_price_updated)),
            last_funding_updated.eq(excluded(last_funding_updated)),
        ))
        .execute_async(&POOL)
        .await?;
    Ok(num_rows)
}

pub async fn query_symbol_cfgs() -> Result<Vec<DbSymbolConfig>> {
    use crate::schema::symbols_config::dsl::*;
    tracing::info!(
        target: DB_CONTEXT,
        "query_symbol_cfgs start",
    );

    let result = symbols_config.load_async::<DbSymbolConfig>(&POOL).await;

    let symbols_cfgs = match result {
        Ok(symbols_cfgs) => {
            tracing::info!(
                target: DB_CONTEXT,
                "query_symbol_cfgs success. length:{}",
                symbols_cfgs.len(),
            );
            symbols_cfgs
        }
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => {
                tracing::info!(
                    target: DB_CONTEXT,
                    "query_symbol_cfgs success. length: 0",
                );
                vec![]
            }
            _ => {
                tracing::warn!(
                    target: DB_CONTEXT,
                    "query_symbol_cfgs fail. err:{:?}",
                    error,
                );
                Err(error)?
            }
        },
    };

    Ok(symbols_cfgs)
}
