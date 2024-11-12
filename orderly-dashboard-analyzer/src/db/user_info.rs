use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::InsertError;
use crate::db::POOL;
use crate::schema::user_info;
use crate::sync_broker::{cal_account_id, cal_broker_hash};
use actix_diesel::dsl::AsyncRunQueryDsl;
use diesel::pg::upsert::on_constraint;

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name = "user_info"]
pub struct UserInfo {
    pub account_id: String,
    pub broker_id: String,
    pub broker_hash: String,
    pub address: String,
}

impl UserInfo {
    pub fn try_new(broker_id: String, address: String) -> anyhow::Result<UserInfo> {
        let broker_hash = cal_broker_hash(&broker_id);
        Ok(UserInfo {
            account_id: cal_account_id(&broker_id, &address)?,
            broker_id,
            broker_hash,
            address,
        })
    }
}

pub async fn create_user_info(p_user: &UserInfo) -> Result<usize, DBException> {
    use crate::schema::user_info::dsl::*;
    let update_result = diesel::insert_into(user_info)
        .values(p_user.clone())
        .on_conflict(on_constraint("pr_account_id"))
        .do_nothing()
        .execute_async(&POOL)
        .await;

    match update_result {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
            return Err(InsertError);
        }
    }
    return Ok(1);
}
