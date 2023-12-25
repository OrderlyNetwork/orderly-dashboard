use crate::bindings::market_manager::market_manager as MarketManagerContract;
use crate::config::COMMON_CONFIGS;
use crate::db::symbols_config::{update_symbol_cfgs, DbSymbolConfig};
use crate::eth_rpc::clone_provider;
use crate::service_base::runtime::spawn_future;
use crate::utils::to_hex_format;
use anyhow::Result;
use ethers::core::utils::keccak256;
use ethers::prelude::H160;
use std::str::FromStr;
use std::sync::Arc;
pub const SYNC_PERP_CONFIG_CONTEXT: &str = "sync_perp_config";

pub fn init_sync_perp_config_task() -> Result<()> {
    let config = unsafe { COMMON_CONFIGS.get_unchecked() };

    let symbols = config.perp_symbols_config.clone();
    let market_manager_contract = MarketManagerContract::market_manager::new(
        H160::from_str(&config.l2_config.market_manager_address)?,
        Arc::new(clone_provider()),
    );
    spawn_future(async move {
        loop {
            for symbol in &symbols {
                let hash = keccak256(symbol);

                match market_manager_contract
                    .get_perp_market_cfg(hash)
                    .call()
                    .await
                {
                    Ok(perp_config) => {
                        let mut db_cfg = DbSymbolConfig::from(perp_config);
                        db_cfg.symbol = symbol.to_string();
                        db_cfg.symbol_hash = to_hex_format(&keccak256(symbol));
                        tracing::info!(target: SYNC_PERP_CONFIG_CONTEXT, "syn symbol:{}, symbol hash:{}", db_cfg.symbol, db_cfg.symbol_hash);
                        if let Err(err) = update_symbol_cfgs(vec![db_cfg]).await {
                            tracing::warn!(target: SYNC_PERP_CONFIG_CONTEXT, "update_symbol_cfgs failed with err: {}", err);
                        }
                    }
                    Err(err) => {
                        tracing::warn!(target: SYNC_PERP_CONFIG_CONTEXT, "get_perp_market_cfg err:{:?}", err);
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
        }
        #[allow(unreachable_code)]
        Ok(())
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cal_symbol_hash() {
        let symbol_hash = to_hex_format(&keccak256("PERP_BTC_USDC"));
        assert_eq!(
            symbol_hash,
            "0x5a8133e52befca724670dbf2cade550c522c2410dd5b1189df675e99388f509d"
        );
    }
}
