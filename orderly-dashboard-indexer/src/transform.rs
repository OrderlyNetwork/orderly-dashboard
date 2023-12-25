use crate::bindings::market_manager::PerpMarketCfg;
use crate::db::symbols_config::DbSymbolConfig;
use crate::utils::convert_amount;

impl From<PerpMarketCfg> for DbSymbolConfig {
    fn from(cfg: PerpMarketCfg) -> Self {
        let last_mark_price_updated = cfg.last_mark_price_updated.as_u128();
        let last_funding_updated = cfg.last_funding_updated.as_u128();
        DbSymbolConfig {
            symbol: "".to_string(),
            symbol_hash: "".to_string(),
            base_maintenance_margin: if cfg.base_maintenance_margin != 0 {
                Some(cfg.base_maintenance_margin as i32)
            } else {
                None
            },
            base_initial_margin: if cfg.base_initial_margin != 0 {
                Some(cfg.base_initial_margin as i32)
            } else {
                None
            },
            mark_price: if cfg.mark_price != 0 {
                Some(convert_amount(cfg.mark_price as i128).unwrap_or_default())
            } else {
                None
            },
            index_price_orderly: if cfg.index_price_orderly != 0 {
                Some(convert_amount(cfg.index_price_orderly as i128).unwrap_or_default())
            } else {
                None
            },
            sum_unitary_fundings: if cfg.sum_unitary_fundings != 0 {
                Some(convert_amount(cfg.sum_unitary_fundings).unwrap_or_default())
            } else {
                None
            },
            last_mark_price_updated: if last_mark_price_updated != 0 {
                Some(convert_amount(last_mark_price_updated as i128).unwrap_or_default())
            } else {
                None
            },
            last_funding_updated: if last_funding_updated != 0 {
                Some(convert_amount(last_funding_updated as i128).unwrap_or_default())
            } else {
                None
            },
        }
    }
}
