use utoipa::{
    openapi::{
        self,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
    Modify, OpenApi,
};

use crate::{events::events_api::GetAccountEventsRequest, format_extern::QeuryServiceResponse};
use crate::{
    format_extern::rank_metrics::UserSummaryRankExtern,
    trading_metrics::{PositionRankingRequest, RealizedPnlRankingRequest},
};

use orderly_dashboard_indexer::formats_external::trading_events::AccountTradingEventsResponse;
use orderly_dashboard_indexer::formats_external::IndexerQueryResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::trading_metrics::get_position_rank,
        crate::trading_metrics::get_realized_pnl_rank,
        crate::events::events_api::list_events,
    ),
    components(
        schemas(
            PositionRankingRequest,
            QeuryServiceResponse<UserSummaryRankExtern>,
            RealizedPnlRankingRequest,
            GetAccountEventsRequest,
            IndexerQueryResponse<AccountTradingEventsResponse>,
        )
    ),
    tags((name = "BasicAPI", description = "A very Basic API")),
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
