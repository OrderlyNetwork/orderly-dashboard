use serde::{Deserialize, Serialize};
use typescript_type_def::TypeDef;

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct DailyVolumeExtern {
    pub daytime: Vec<String>,
    pub volume: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct DailyTradingFeeExtern {
    pub daytime: Vec<String>,
    pub fee_amount: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct TradingVolumeRanking {
    pub account_ids: Vec<String>,
    pub volume: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct TradingPnlRanking {
    pub account_ids: Vec<String>,
    pub volume: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct TokenAmountRanking {
    pub account_ids: Vec<String>,
    pub volume: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct OrderlyPerpDaily {
    pub trading_volume: f64,
    pub trading_fee: f64,
    pub trading_count: f64,
    pub trading_user_count: f64,
    pub liquidation_amount: f64,
    pub liquidation_count: f64,
    pub opening_count: f64,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct OrderlyGasFee {
    pub avg_gas_fee: f64,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct OrderlyTokenDaily {
    pub withdraw_amount: f64,
    pub withdraw_count: f64,
    pub deposit_amount: f64,
    pub deposit_count: f64,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct DailyData<T: Serialize> {
    pub daytime: Vec<String>,
    pub data: Vec<T>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct UserPerpHoldingRanking {
    pub account_ids: Vec<String>,
    pub holding: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct CountAverageExtern {
    pub latest_day_metric: f64,
    pub latest_week_metric: f64,
    pub latest_month_metric: f64,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct DailyTokenAmountExtern {
    pub daytime: Vec<String>,
    pub amount: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef)]
pub struct DailyTokenRanking {
    pub daytime: Vec<String>,
    pub amount: Vec<f64>,
}

pub type Api = (
    DailyVolumeExtern,
    DailyTradingFeeExtern,
    TradingVolumeRanking,
    TradingPnlRanking,
    TokenAmountRanking,
    DailyData<OrderlyPerpDaily>,
    DailyData<OrderlyGasFee>,
    DailyData<OrderlyTokenDaily>,
    UserPerpHoldingRanking,
    CountAverageExtern,
);
