use serde::{Deserialize, Serialize};

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
    pub index_price: f64,
    pub mark_price: f64,
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
            "reqwest get_brokers faield with err: {:?}",
            err
        )),
    }
}
