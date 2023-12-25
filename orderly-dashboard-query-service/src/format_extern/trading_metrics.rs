use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct DailyVolumeExtern {
    pub daytime: Vec<String>,
    pub volume: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct DailyTradingFeeExtern {
    pub daytime: Vec<String>,
    pub fee_amount: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct TradingVolumeRanking {
    pub account_ids: Vec<String>,
    pub volume: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct UserPerpHoldingRanking {
    pub account_ids: Vec<String>,
    pub holding: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct CountAverageExtern {
    pub latest_day_metric: f64,
    pub latest_week_metric: f64,
    pub latest_month_metric: f64,
}
