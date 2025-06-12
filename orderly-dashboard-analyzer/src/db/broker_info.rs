use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::QueryError;
use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::broker_info;

#[derive(Insertable, Queryable, Debug, Clone)]
#[diesel(table_name = broker_info)]
pub struct BrokerInfo {
    pub broker_id: String,
    pub broker_hash: String,
}

pub async fn find_by_broker_hash(p_broker_hash: String) -> Result<BrokerInfo, DBException> {
    use crate::schema::broker_info::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let select_result = broker_info
        .filter(broker_hash.eq(p_broker_hash.clone()))
        .first::<BrokerInfo>(&mut conn)
        .await;

    match select_result {
        Ok(result) => Ok(result),
        Err(error) => match error {
            diesel::NotFound => Err(QueryError),
            _ => Err(QueryError),
        },
    }
}

#[allow(dead_code)]
pub async fn create_or_update_broker_info(broker_vec: Vec<BrokerInfo>) {
    use crate::schema::broker_info::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    for broker_data in broker_vec {
        let update_result = diesel::insert_into(broker_info)
            .values(broker_data.clone())
            .on_conflict(on_constraint("pr_broker_id"))
            .do_nothing()
            .execute(&mut conn)
            .await;

        match update_result {
            Ok(_) => {}
            Err(erro) => {
                tracing::warn!("create_or_update_broker_info: {}", erro);
            }
        }
    }
}
