use crate::analyzer_db::{ANLYZER_DB_CONN_ERR_MSG, POOL};
use crate::analyzer_db_schema::user_info;
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Insertable, Queryable, Debug, Clone)]
#[diesel(table_name = user_info)]
pub struct UserInfo {
    pub account_id: String,
    pub broker_id: String,
    pub broker_hash: String,
    pub address: String,
}

#[derive(Insertable, Queryable, Debug, Clone)]
#[diesel(table_name = user_info)]
pub struct UserInfoView {
    pub account_id: String,
    pub broker_id: String,
    pub broker_hash: String,
    pub address: String,
    pub id: i64,
}

pub async fn create_user_info(p_user: &UserInfo) -> anyhow::Result<usize> {
    use crate::analyzer_db_schema::user_info::dsl::*;
    let mut conn = POOL.get().await.expect(ANLYZER_DB_CONN_ERR_MSG);
    let update_result = diesel::insert_into(user_info)
        .values(p_user.clone())
        .on_conflict(on_constraint("pr_account_id"))
        .do_nothing()
        .execute(&mut conn)
        .await;

    match update_result {
        Ok(_) => {}
        Err(error) => {
            tracing::warn!("create_user_info err: {}", error);
            return Err(anyhow::anyhow!("create_user_info err: {}", error));
        }
    }
    return Ok(1);
}

pub async fn get_user_info(account_id_: String) -> anyhow::Result<Option<UserInfoView>> {
    use crate::analyzer_db_schema::user_info::dsl::*;
    let mut conn = POOL.get().await.expect(ANLYZER_DB_CONN_ERR_MSG);
    let result = user_info
        .filter(account_id.eq(account_id_))
        .limit(1)
        .get_result::<UserInfoView>(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(Some(user)),
        Err(error) => match error {
            diesel::NotFound => return Ok(None),
            _ => return Err(anyhow::anyhow!(error)),
        },
    }
}

#[allow(dead_code)]
pub async fn get_user_infos(account_ids: Vec<String>) -> anyhow::Result<Vec<UserInfoView>> {
    use crate::analyzer_db_schema::user_info::dsl::*;
    let mut conn = POOL.get().await.expect(ANLYZER_DB_CONN_ERR_MSG);
    let result = user_info
        .filter(account_id.eq_any(account_ids))
        .get_results::<UserInfoView>(&mut conn)
        .await?;

    Ok(result)
}
