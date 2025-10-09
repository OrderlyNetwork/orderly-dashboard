use crate::db::trading_metrics::volume_statistic::DbBrokerVolumeStatistic;
use orderly_dashboard_analyzer::db::user_volume_statistics::DBUserVolumeStatistics;
use serde::{Deserialize, Serialize};
use typescript_type_def::TypeDef;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef, ToSchema)]
pub struct DailyVolumeExtern {
    pub daytime: Vec<String>,
    pub volume: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef, ToSchema)]
pub struct DailyTradingFeeExtern {
    pub daytime: Vec<String>,
    pub fee_amount: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef, ToSchema)]
pub struct TradingVolumeRanking {
    pub account_ids: Vec<String>,
    pub volume: Vec<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef, ToSchema)]
pub struct TradingPnlRanking {
    pub account_id: String,
    pub symbol: String,
    pub realized_pnl: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef, ToSchema)]
pub struct TokenAmountRanking {
    pub account_id: String,
    pub token_hash: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef, ToSchema)]
pub struct OrderlyPerpDaily {
    pub trading_volume: f64,
    pub trading_fee: f64,
    pub trading_count: f64,
    pub trading_user_count: f64,
    pub liquidation_amount: f64,
    pub liquidation_count: f64,
    pub opening_count: f64,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AccountVolumeStatisticRequest {
    pub account_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef, ToSchema)]
pub struct AccountVolumeStatistic {
    pub account_id: String,
    pub broker_id: String,
    pub address: String,
    pub perp_volume_ytd: String,
    pub perp_volume_ltd: String,
    pub perp_volume_last_1_day: String,
    pub perp_volume_last_7_days: String,
    pub perp_volume_last_30_days: String,
}

impl From<DBUserVolumeStatistics> for AccountVolumeStatistic {
    fn from(value: DBUserVolumeStatistics) -> Self {
        AccountVolumeStatistic {
            account_id: value.account_id,
            broker_id: value.broker_id,
            address: value.address,
            perp_volume_ytd: value.perp_volume_ytd.to_string(),
            perp_volume_ltd: value.perp_volume_ltd.to_string(),
            perp_volume_last_1_day: value.perp_volume_last_1_day.to_string(),
            perp_volume_last_7_days: value.perp_volume_last_7_days.to_string(),
            perp_volume_last_30_days: value.perp_volume_last_30_days.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct BrokerVolumeStatisticRequest {
    pub broker_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef, ToSchema)]
pub struct BrokerVolumeStatistic {
    pub perp_volume_ytd: String,
    pub perp_volume_ltd: String,
    pub perp_volume_last_1_day: String,
    pub perp_volume_last_7_days: String,
    pub perp_volume_last_30_days: String,
}

impl From<DbBrokerVolumeStatistic> for BrokerVolumeStatistic {
    fn from(value: DbBrokerVolumeStatistic) -> Self {
        BrokerVolumeStatistic {
            perp_volume_ytd: value.perp_volume_ytd.to_string(),
            perp_volume_ltd: value.perp_volume_ltd.to_string(),
            perp_volume_last_1_day: value.perp_volume_last_1_day.to_string(),
            perp_volume_last_7_days: value.perp_volume_last_7_days.to_string(),
            perp_volume_last_30_days: value.perp_volume_last_30_days.to_string(),
        }
    }
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

#[derive(Debug, Deserialize, PartialEq, Serialize, Default, TypeDef, ToSchema)]
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
    pub last_24hours_metric: f64,
    pub last_7days_metric: f64,
    pub last_30days_metric: f64,
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
