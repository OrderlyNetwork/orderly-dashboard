use crate::db::settings::get_db_network_info;
use crate::formats_external::NetworkInfo;
use crate::formats_external::{Response, SuccessResponse};
use anyhow::Result;

pub async fn get_network_info() -> Result<Response<NetworkInfo>> {
    let info = get_db_network_info().await?;
    if let Some(height) = info.finalized_height {
        return Ok(Response::Success(SuccessResponse::new(NetworkInfo {
            finalized_height: Some(height),
        })));
    }
    Ok(Response::Success(SuccessResponse::new(
        NetworkInfo::default(),
    )))
}
