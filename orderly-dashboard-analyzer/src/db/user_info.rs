use crate::db::user_token_summary::DBException;
use crate::db::user_token_summary::DBException::InsertError;
use crate::db::{DB_CONN_ERR_MSG, POOL};
use crate::schema::user_info;
use crate::sync_broker::{cal_account_id, cal_broker_hash, get_sol_account_id};
use diesel::pg::upsert::on_constraint;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use orderly_dashboard_indexer::sdk::solana::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Insertable, Queryable, Debug, Clone)]
#[diesel(table_name = user_info)]
pub struct UserInfo {
    pub account_id: String,
    pub broker_id: String,
    pub broker_hash: String,
    pub address: String,
}

impl UserInfo {
    pub fn try_new(broker_id: String, address: String) -> anyhow::Result<UserInfo> {
        let broker_hash = cal_broker_hash(&broker_id);
        let account_id = if address.starts_with("0x") {
            cal_account_id(&broker_id, &address)?
        } else {
            get_sol_account_id(&Pubkey::from_str(&address)?, &broker_id)?
        };
        Ok(UserInfo {
            account_id,
            broker_id,
            broker_hash,
            address,
        })
    }
}

pub async fn create_user_info(p_user: &UserInfo) -> Result<usize, DBException> {
    use crate::schema::user_info::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let update_result = diesel::insert_into(user_info)
        .values(p_user.clone())
        .on_conflict(on_constraint("pr_account_id"))
        .do_nothing()
        .execute(&mut conn)
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

pub async fn get_user_info(account_id_: String) -> anyhow::Result<Option<UserInfo>> {
    use crate::schema::user_info::dsl::*;
    let mut conn = POOL.get().await.expect(DB_CONN_ERR_MSG);
    let result = user_info
        .filter(account_id.eq(account_id_))
        .limit(1)
        .get_result::<UserInfo>(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(Some(user)),
        Err(error) => match error {
            diesel::NotFound => return Ok(None),
            _ => return Err(anyhow::anyhow!(error)),
        },
    }
}
