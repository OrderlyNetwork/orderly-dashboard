use std::collections::HashMap;

use crate::db::hourly_gas_fee::{
    create_or_update_hourly_gas_fee, find_hourly_gas_fee, HourlyGasFee, HourlyGasFeeKey,
};

pub struct GasFeeContext {
    hourly_gas_fee_cache: HashMap<HourlyGasFeeKey, HourlyGasFee>,
}

impl GasFeeContext {
    pub fn new_context() -> Self {
        GasFeeContext {
            hourly_gas_fee_cache: HashMap::new(),
        }
    }

    pub async fn save_analyze_result(&mut self) {
        create_or_update_hourly_gas_fee(Vec::from_iter(
            self.hourly_gas_fee_cache.values().into_iter(),
        ))
        .await
        .unwrap();
    }
}

impl GasFeeContext {
    pub async fn get_hourly_gas(&mut self, p_key: &HourlyGasFeeKey) -> &mut HourlyGasFee {
        if !self.hourly_gas_fee_cache.contains_key(&p_key.clone()) {
            let hourly_gas =
                find_hourly_gas_fee(p_key.event_type.clone(), p_key.block_hour.clone())
                    .await
                    .unwrap();
            self.hourly_gas_fee_cache.insert(p_key.clone(), hourly_gas);
        }
        self.hourly_gas_fee_cache.get_mut(&p_key.clone()).unwrap()
    }
}
