use serde::{Deserialize, Serialize};
use typescript_type_def::TypeDef;

use crate::trading_metrics::VolumeRankingData;

#[derive(Debug, Deserialize, Serialize, Default, TypeDef)]
pub struct PositionRankExtern {
    pub rows: Vec<VolumeRankingData>,
}

impl PositionRankExtern {
    pub fn new(rows: Vec<VolumeRankingData>) -> PositionRankExtern {
        PositionRankExtern { rows }
    }
}
