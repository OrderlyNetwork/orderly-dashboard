use crate::db::symbols_config::DbSymbolConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SymbolConfig {
    pub symbol: String,
    pub symbol_hash: String,
    pub base_maintenance_margin: Option<i32>,
    pub base_initial_margin: Option<i32>,
    pub mark_price: Option<String>,
    pub index_price_orderly: Option<String>,
    pub sum_unitary_fundings: Option<String>,
    pub last_mark_price_updated: Option<String>,
    pub last_funding_updated: Option<String>,
}

impl From<DbSymbolConfig> for SymbolConfig {
    fn from(value: DbSymbolConfig) -> Self {
        SymbolConfig {
            symbol: value.symbol,
            symbol_hash: value.symbol_hash,
            base_maintenance_margin: value.base_maintenance_margin,
            base_initial_margin: value.base_initial_margin,
            mark_price: value.mark_price.map(|v| v.to_string()),
            index_price_orderly: value.index_price_orderly.map(|v| v.to_string()),
            sum_unitary_fundings: value.sum_unitary_fundings.map(|v| v.to_string()),
            last_mark_price_updated: value.last_mark_price_updated.map(|v| v.to_string()),
            last_funding_updated: value.last_funding_updated.map(|v| v.to_string()),
        }
    }
}
