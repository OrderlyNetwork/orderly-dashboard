use crate::db::user_token_summary::DBException;
use crate::schema::symbols;
#[allow(dead_code, unused_imports)]
use diesel::QueryDsl;

#[derive(Debug, Insertable, Queryable, Clone)]
#[diesel(table_name = symbols)]
pub struct Symbol {
    pub symbol: String,
    pub symbol_hash: String,
}

#[allow(dead_code, unused_imports)]
pub async fn find_symbol_by_hash(_p_symbol_hash: String) -> Result<Symbol, DBException> {
    use crate::schema::symbols::dsl::*;
    Ok(Symbol {
        symbol: "".to_string(),
        symbol_hash: "".to_string(),
    })
}
