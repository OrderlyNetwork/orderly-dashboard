use serde::{Deserialize, Serialize};
use typescript_type_def::TypeDef;
use utoipa::ToSchema;

use crate::trading_metrics::UserSumaryRankingData;

#[derive(Debug, Deserialize, Serialize, Default, TypeDef, ToSchema)]
pub struct UserSummaryRankExtern {
    pub rows: Vec<UserSumaryRankingData>,
}

impl UserSummaryRankExtern {
    pub fn new(rows: Vec<UserSumaryRankingData>) -> UserSummaryRankExtern {
        UserSummaryRankExtern { rows }
    }
}
