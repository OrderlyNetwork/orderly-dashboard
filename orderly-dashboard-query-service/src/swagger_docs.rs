use utoipa::{
    openapi::{
        self,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
    Modify, OpenApi,
};

use crate::{
    events::events_api::{GetAccountEventsRequest, GetAccountEventsV2Request},
    format_extern::{
        trading_metrics::{
            AccountVolumeStatistic, AccountVolumeStatisticRequest, BrokerVolumeStatistic,
            BrokerVolumeStatisticRequest, DailyData, DailyTradingFeeExtern, DailyVolumeExtern,
            OrderlyPerpDaily, TokenAmountRanking, TradingPnlRanking, TradingVolumeRanking,
        },
        QeuryServiceResponse,
    },
    trading_metrics::{
        DailyRequest, DepositWithdrawRankingRequest, PnlRankingRequest, VolumeRankingRequest,
    },
};
use crate::{
    format_extern::rank_metrics::UserSummaryRankExtern,
    trading_metrics::{PositionRankingRequest, RealizedPnlRankingRequest},
};

use orderly_dashboard_indexer::formats_external::trading_events::AccountTradingEventsResponse;
use orderly_dashboard_indexer::formats_external::IndexerQueryExternResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::trading_metrics::get_daily_orderly_perp,
        crate::trading_metrics::daily_volume,
        crate::trading_metrics::daily_trading_fee,
        crate::trading_metrics::get_trading_volume_rank,
        crate::trading_metrics::get_position_rank,
        crate::trading_metrics::get_realized_pnl_rank,
        // crate::trading_metrics::get_perp_recent_days_pnl_rank,
        crate::trading_metrics::get_token_deposit_rank,
        crate::trading_metrics::get_token_withdraw_rank,
        crate::events::events_api::list_events,
        crate::events::events_api::list_events_v2,
        crate::trading_metrics::volume_statistic::get_account_volume_statistic,
        crate::trading_metrics::volume_statistic::get_broker_volume_statistic,
    ),
    components(
        schemas(
            DailyRequest,
            VolumeRankingRequest,
            PositionRankingRequest,
            QeuryServiceResponse<UserSummaryRankExtern>,
            RealizedPnlRankingRequest,
            GetAccountEventsRequest,
            PnlRankingRequest,
            DepositWithdrawRankingRequest,
            GetAccountEventsV2Request,
            AccountVolumeStatisticRequest,
            BrokerVolumeStatisticRequest,
            IndexerQueryExternResponse<AccountTradingEventsResponse>,
            QeuryServiceResponse<DailyData<OrderlyPerpDaily>>,
            QeuryServiceResponse<TradingVolumeRanking>,
            QeuryServiceResponse<DailyVolumeExtern>,
            QeuryServiceResponse<DailyTradingFeeExtern>,
            QeuryServiceResponse<Vec<TradingPnlRanking>>,
            QeuryServiceResponse<Vec<TokenAmountRanking>>,
            QeuryServiceResponse<AccountVolumeStatistic>,
            QeuryServiceResponse<BrokerVolumeStatistic>,
        )
    ),
    tags((name = "Orderly DashboardBasicAPI", description = "Orderly Dashboard Basic API")),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        // NOTE: we can unwrap safely since there already is components registered.
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}
