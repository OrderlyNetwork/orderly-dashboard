use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ResponseData<T> {
    pub success: bool,
    pub data: T,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MarketDataInfos {
    pub rows: Vec<MarketDataInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MarketDataInfo {
    pub symbol: String,
    pub index_price: Value,
    pub mark_price: Value,
    pub sum_unitary_funding: f64,
    pub est_funding_rate: f64,
    pub last_funding_rate: f64,
    pub next_funding_time: i64,
    pub open_interest: f64,
    #[serde(rename = "24h_open")]
    pub _24h_open: f64,
    #[serde(rename = "24h_close")]
    pub _24h_close: f64,
    #[serde(rename = "24h_high")]
    pub _24h_high: f64,
    #[serde(rename = "24h_low")]
    pub _24h_low: f64,
    #[serde(rename = "24h_volume")]
    pub _24h_volume: f64,
    #[serde(rename = "24h_amount")]
    pub _24h_amount: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CefiAccountInfo {
    pub address: String,
    pub broker_id: String,
}

pub async fn list_market_infos(base_url: &str) -> anyhow::Result<ResponseData<MarketDataInfos>> {
    let res = get_market_infos(base_url).await?;

    let response_data: ResponseData<MarketDataInfos> = serde_json::from_str(&res)?;
    Ok(response_data)
}

async fn get_market_infos(base_url: &str) -> anyhow::Result<String> {
    let response = reqwest::get(format!("{}/v1/public/futures", base_url)).await;
    match response {
        Ok(res) => Ok(res.text().await?),
        Err(err) => Err(anyhow::anyhow!(
            "reqwest get_market_infos faield with err: {:?}",
            err
        )),
    }
}

pub async fn cefi_get_account_info(
    base_url: &str,
    account_id: &str,
) -> anyhow::Result<ResponseData<CefiAccountInfo>> {
    let res = _get_account_info(base_url, account_id).await?;

    let response_data: ResponseData<CefiAccountInfo> = serde_json::from_str(&res)?;
    Ok(response_data)
}

async fn _get_account_info(base_url: &str, account_id: &str) -> anyhow::Result<String> {
    let response = reqwest::get(format!(
        "{}/v1/public/account?account_id={}",
        base_url, account_id
    ))
    .await;
    match response {
        Ok(res) => Ok(res.text().await?),
        Err(err) => Err(anyhow::anyhow!(
            "reqwest _get_account_info faield with err: {:?} for account_id: {}",
            err,
            account_id
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[actix_web::test]
    async fn test_list_market_infos() {
        let res = list_market_infos("https://api.orderly.org").await.unwrap();
        println!("{:?}", res);
    }

    #[ignore]
    #[actix_web::test]
    async fn test_get_account_info() {
        let res = cefi_get_account_info(
            "https://api.orderly.org",
            "0x459171fde490477c0bcaea14f20d1d3037eb6bca0a67347a473ce5a894a2057b",
        )
        .await
        .unwrap();
        println!("account info: {:?}", res);
    }
}
