use diesel::sql_types::Text;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use serde::Serialize;

use orderly_dashboard_analyzer::db::POOL;

use crate::db::DB_CONN_ERR_MSG;

#[derive(Debug, QueryableByName, Clone, Serialize)]
pub struct TokenEntry {
    #[diesel(sql_type = Text)]
    pub token: String,
    #[diesel(sql_type = Text)]
    pub token_hash: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokensResponse {
    pub rows: Vec<TokenEntry>,
}

pub async fn get_all_tokens() -> anyhow::Result<TokensResponse> {
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let results: Vec<TokenEntry> =
        diesel::sql_query("SELECT token, token_hash FROM collateral_info ORDER BY token ASC")
            .get_results::<TokenEntry>(&mut conn)
            .await?;
    Ok(TokensResponse { rows: results })
}
