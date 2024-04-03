use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::QueryError;
use crate::db::POOL;
use crate::schema::broker_info;

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name = "broker_info"]
pub struct BrokerInfo {
    pub broker_id: String,
    pub broker_hash: String,
}

pub async fn find_by_broker_hash(p_broker_hash: String) -> Result<BrokerInfo, DBException> {
    use crate::schema::broker_info::dsl::*;
    let select_result = broker_info
        .filter(broker_hash.eq(p_broker_hash.clone()))
        .first_async::<BrokerInfo>(&POOL)
        .await;

    match select_result {
        Ok(result) => Ok(result),
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => Err(QueryError),
            _ => Err(QueryError),
        },
    }
}
