use crate::db::POOL;
use crate::formats_external::NetworkInfo;
use crate::schema::settings;
use actix_diesel::dsl::AsyncRunQueryDsl;
use actix_diesel::AsyncError;
use anyhow::Result;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone)]
pub enum SettingsKey {
    LastRpcProcessHeight = 1,
    NetworkInfo = 2,
    LastRpcProcessTimeStamp = 3,
    SolSyncSignature = 4,
    SolSyncBlockTime = 5,
}

#[derive(Insertable, Queryable, Debug)]
#[table_name = "settings"]
pub struct DbSettings {
    pub id: i32,
    pub value: String,
}

impl DbSettings {
    pub fn new(key: SettingsKey, value: String) -> DbSettings {
        DbSettings {
            id: key as i32,
            value,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SolSyncSignature {
    pub signature: String,
    pub slot: u64,
}

impl SolSyncSignature {
    pub fn new(signature: String, slot: u64) -> SolSyncSignature {
        SolSyncSignature { signature, slot }
    }
}

pub async fn get_last_rpc_processed_height() -> Result<Option<u64>> {
    let key = SettingsKey::LastRpcProcessHeight as i32;
    let result = get_setting(key).await?;
    match result {
        Some(settings) => Ok(Some(settings.value.parse::<u64>()?)),
        None => Ok(None),
    }
}

pub async fn update_last_rpc_processed_height(block_height: u64) -> Result<()> {
    let height = get_last_rpc_processed_height().await?.unwrap_or_default();
    if height >= block_height {
        return Ok(());
    }
    update_setting(SettingsKey::LastRpcProcessHeight, block_height.to_string()).await?;
    Ok(())
}

pub async fn get_last_rpc_processed_timestamp() -> Result<Option<i64>> {
    let key = SettingsKey::LastRpcProcessTimeStamp as i32;
    let result = get_setting(key).await?;
    match result {
        Some(settings) => Ok(Some(settings.value.parse::<i64>()?)),
        None => Ok(None),
    }
}

pub async fn update_last_rpc_processed_timestamp(block_timestamp: i64) -> Result<()> {
    let timestamp = get_last_rpc_processed_timestamp()
        .await?
        .unwrap_or_default();
    if timestamp > block_timestamp {
        return Ok(());
    }
    update_setting(
        SettingsKey::LastRpcProcessTimeStamp,
        block_timestamp.to_string(),
    )
    .await?;
    Ok(())
}

pub async fn update_network_info(network_info: NetworkInfo) -> Result<()> {
    let info = serde_json::to_string(&network_info)?;
    update_setting(SettingsKey::NetworkInfo, info).await?;

    Ok(())
}

pub async fn get_db_network_info() -> Result<NetworkInfo> {
    match get_setting(SettingsKey::NetworkInfo as i32).await? {
        Some(settings) => Ok(serde_json::from_str(&settings.value)?),
        None => Ok(NetworkInfo::default()),
    }
}

pub async fn get_sol_sync_signature() -> Result<Option<SolSyncSignature>> {
    let key = SettingsKey::SolSyncSignature as i32;
    let result = get_setting(key).await?;
    match result {
        Some(settings) => Ok(Some(serde_json::from_str::<SolSyncSignature>(
            &settings.value,
        )?)),
        None => Ok(None),
    }
}

pub async fn update_last_sol_syn_signature(info: SolSyncSignature) -> Result<()> {
    let last_sync = get_sol_sync_signature().await?.unwrap_or_default();
    if last_sync.slot > info.slot {
        return Ok(());
    }
    update_setting(SettingsKey::SolSyncSignature, serde_json::to_string(&info)?).await?;

    Ok(())
}

pub async fn get_sol_sync_block_time() -> Result<Option<i64>> {
    let key = SettingsKey::SolSyncBlockTime as i32;
    let result = get_setting(key).await?;
    match result {
        Some(settings) => Ok(Some(settings.value.parse::<i64>()?)),
        None => Ok(None),
    }
}

pub async fn update_last_sol_syn_block_time(block_time: i64) -> Result<()> {
    let last_block_time = get_sol_sync_block_time().await?.unwrap_or_default();
    if last_block_time > block_time {
        return Ok(());
    }
    update_setting(SettingsKey::SolSyncBlockTime, block_time.to_string()).await?;

    Ok(())
}

async fn get_setting(key: i32) -> Result<Option<DbSettings>> {
    use crate::schema::settings::dsl::*;
    let result = settings
        .filter(id.eq(key))
        .first_async::<DbSettings>(&POOL)
        .await;

    let event = match result {
        Ok(event) => Some(event),
        Err(error) => match error {
            AsyncError::Execute(Error::NotFound) => None,
            _ => Err(error)?,
        },
    };

    Ok(event)
}

async fn update_setting(key: SettingsKey, value_: String) -> Result<()> {
    use crate::schema::settings::dsl::*;
    let model = DbSettings::new(key, value_.clone());

    diesel::insert_into(crate::schema::settings::table)
        .values(model)
        .on_conflict(id)
        .do_update()
        .set(value.eq(value_))
        .execute_async(&POOL)
        .await?;

    Ok(())
}
