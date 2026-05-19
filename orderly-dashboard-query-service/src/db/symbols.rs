use diesel::sql_types::Text;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use serde::Serialize;

use orderly_dashboard_analyzer::db::POOL;

use crate::db::DB_CONN_ERR_MSG;

#[derive(Debug, QueryableByName, Clone, Serialize)]
pub struct SymbolEntry {
    #[diesel(sql_type = Text)]
    pub symbol: String,
    #[diesel(sql_type = Text)]
    pub symbol_hash: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SymbolsResponse {
    pub rows: Vec<SymbolEntry>,
}

pub async fn get_all_symbols() -> anyhow::Result<SymbolsResponse> {
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let results: Vec<SymbolEntry> = diesel::sql_query(
        "SELECT symbol, symbol_hash FROM market_info ORDER BY symbol ASC",
    )
    .get_results::<SymbolEntry>(&mut conn)
    .await?;
    Ok(SymbolsResponse { rows: results })
}
