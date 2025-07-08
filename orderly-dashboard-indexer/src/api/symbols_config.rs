use crate::db::symbols_config::query_symbol_cfgs;
use crate::formats_external::symbols_config::SymbolConfig;
use crate::formats_external::{IndexerQueryResponse, SuccessResponse};
use anyhow::Result;

pub async fn get_symbols_data() -> Result<IndexerQueryResponse<Vec<SymbolConfig>>> {
    let symbols_cfg = query_symbol_cfgs().await?;
    let res = symbols_cfg
        .into_iter()
        .map(SymbolConfig::from)
        .collect::<Vec<_>>();
    Ok(IndexerQueryResponse::Success(SuccessResponse::new(res)))
}
