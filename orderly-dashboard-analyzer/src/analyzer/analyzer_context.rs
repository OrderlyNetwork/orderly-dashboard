use std::collections::HashMap;

use chrono::NaiveDateTime;

use crate::db::hourly_orderly_perp::{
    create_or_update_hourly_orderly_perp, find_hourly_orderly_perp, HourlyOrderlyPerp,
    HourlyOrderlyPerpKey,
};
use crate::db::hourly_orderly_token::{
    find_hourly_orderly_token, HourlyOrderlyToken, HourlyOrderlyTokenKey,
};
use crate::db::hourly_user_perp::{
    create_or_update_hourly_user_perp, find_hourly_user_perp, HourlyUserPerp, HourlyUserPerpKey,
};
use crate::db::hourly_user_token::{
    create_or_update_hourly_user_token, find_hourly_user_token, HourlyUserToken, HourlyUserTokenKey,
};
use crate::db::orderly_perp_summary::{
    create_or_update_orderly_perp_summary, find_orderly_perp_summary, OrderlyPerpSummary,
};
use crate::db::orderly_token_summary::{
    create_or_update_orderly_token_summary, find_orderly_token_summary, OrderlyTokenSummary,
    OrderlyTokenSummaryKey,
};
use crate::db::user_perp_summary::{
    create_or_update_user_perp_summary, find_user_perp_summary, UserPerpSummary, UserPerpSummaryKey,
};
use crate::db::user_token_summary::{
    create_or_update_user_token_summary, find_user_token_summary, UserTokenSummary,
    UserTokenSummaryKey,
};

pub struct AnalyzeContext {
    pub hourly_orderly_perp_cache: HashMap<HourlyOrderlyPerpKey, HourlyOrderlyPerp>,
    pub hourly_orderly_token_cache: HashMap<HourlyOrderlyTokenKey, HourlyOrderlyToken>,

    pub hourly_user_perp_cache: HashMap<HourlyUserPerpKey, HourlyUserPerp>,
    pub hourly_user_token_cache: HashMap<HourlyUserTokenKey, HourlyUserToken>,

    pub orderly_perp_cache: HashMap<String, OrderlyPerpSummary>,
    pub user_perp_cache: HashMap<UserPerpSummaryKey, UserPerpSummary>,

    pub orderly_token_cache: HashMap<OrderlyTokenSummaryKey, OrderlyTokenSummary>,
    pub user_token_cache: HashMap<UserTokenSummaryKey, UserTokenSummary>,

    #[allow(dead_code)]
    symbol_cache: HashMap<String, String>,
}

impl AnalyzeContext {
    pub fn new_context() -> Self {
        AnalyzeContext {
            hourly_orderly_perp_cache: HashMap::new(),
            hourly_orderly_token_cache: HashMap::new(),
            hourly_user_perp_cache: HashMap::new(),
            hourly_user_token_cache: HashMap::new(),
            orderly_perp_cache: HashMap::new(),
            user_perp_cache: HashMap::new(),
            orderly_token_cache: HashMap::new(),
            user_token_cache: HashMap::new(),
            symbol_cache: HashMap::new(),
        }
    }

    pub async fn save_analyze_result(
        &mut self,
        pulled_block_height: i64,
        pulled_block_time: NaiveDateTime,
    ) {
        self.hourly_user_perp_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });
        create_or_update_hourly_user_perp(Vec::from_iter(self.hourly_user_perp_cache.values()))
            .await
            .unwrap();

        self.hourly_orderly_perp_cache
            .iter_mut()
            .for_each(|(_k, v)| {
                if pulled_block_height != 0 {
                    v.pulled_block_height = pulled_block_height;
                }
                v.pulled_block_time = pulled_block_time;
            });
        create_or_update_hourly_orderly_perp(Vec::from_iter(
            self.hourly_orderly_perp_cache.values(),
        ))
        .await
        .unwrap();

        self.orderly_perp_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });
        create_or_update_orderly_perp_summary(Vec::from_iter(self.orderly_perp_cache.values()))
            .await
            .unwrap();

        self.user_perp_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });
        create_or_update_user_perp_summary(Vec::from_iter(self.user_perp_cache.values()))
            .await
            .unwrap();

        // self.hourly_orderly_token_cache
        //     .iter_mut()
        //     .for_each(|(_k, v)| {
        //         if pulled_block_height != 0 {
        //             v.pulled_block_height = pulled_block_height;
        //         }
        //         v.pulled_block_time = pulled_block_time;
        //     });
        // create_or_update_hourly_orderly_token(Vec::from_iter(
        //     self.hourly_orderly_token_cache.values(),
        // ))
        // .await
        // .unwrap();

        self.hourly_user_token_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });
        create_or_update_hourly_user_token(Vec::from_iter(self.hourly_user_token_cache.values()))
            .await
            .unwrap();

        self.user_token_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });
        create_or_update_user_token_summary(Vec::from_iter(self.user_token_cache.values()))
            .await
            .unwrap();

        self.orderly_token_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });
        create_or_update_orderly_token_summary(Vec::from_iter(self.orderly_token_cache.values()))
            .await
            .unwrap();
    }
}

impl AnalyzeContext {
    pub async fn get_orderly_token(
        &mut self,
        token_key: &OrderlyTokenSummaryKey,
    ) -> &mut OrderlyTokenSummary {
        if !self.orderly_token_cache.contains_key(&token_key.clone()) {
            let orderly_token =
                find_orderly_token_summary(token_key.token.clone(), token_key.chain_id.clone())
                    .await
                    .unwrap();
            self.orderly_token_cache
                .insert(token_key.clone(), orderly_token);
        }
        self.orderly_token_cache
            .get_mut(&token_key.clone())
            .unwrap()
    }

    pub async fn get_user_token(
        &mut self,
        token_key: &UserTokenSummaryKey,
    ) -> &mut UserTokenSummary {
        if !self.user_token_cache.contains_key(&token_key.clone()) {
            let user_token = find_user_token_summary(
                token_key.account_id.clone(),
                token_key.token.clone(),
                token_key.chain_id.clone(),
            )
            .await
            .unwrap();
            self.user_token_cache.insert(token_key.clone(), user_token);
        }
        self.user_token_cache.get_mut(&token_key.clone()).unwrap()
    }

    pub async fn get_hourly_user_token(
        &mut self,
        token_key: &HourlyUserTokenKey,
    ) -> &mut HourlyUserToken {
        if !self
            .hourly_user_token_cache
            .contains_key(&token_key.clone())
        {
            let hourly_orderly_token = find_hourly_user_token(
                token_key.account_id.clone(),
                token_key.token.clone(),
                token_key.block_hour.clone(),
                token_key.chain_id.clone(),
            )
            .await
            .unwrap();
            self.hourly_user_token_cache
                .insert(token_key.clone(), hourly_orderly_token);
        }
        self.hourly_user_token_cache
            .get_mut(&token_key.clone())
            .unwrap()
    }

    pub async fn get_hourly_orderly_token(
        &mut self,
        token_key: &HourlyOrderlyTokenKey,
    ) -> &mut HourlyOrderlyToken {
        if !self
            .hourly_orderly_token_cache
            .contains_key(&token_key.clone())
        {
            let hourly_orderly_token = find_hourly_orderly_token(
                token_key.token.clone(),
                token_key.block_hour.clone(),
                token_key.chain_id.clone(),
            )
            .await
            .unwrap();
            self.hourly_orderly_token_cache
                .insert(token_key.clone(), hourly_orderly_token);
        }
        self.hourly_orderly_token_cache
            .get_mut(&token_key.clone())
            .unwrap()
    }

    pub async fn get_hourly_orderly_perp(
        &mut self,
        perp_key: &HourlyOrderlyPerpKey,
    ) -> &mut HourlyOrderlyPerp {
        if !self
            .hourly_orderly_perp_cache
            .contains_key(&perp_key.clone())
        {
            let perp_key_clone = perp_key.clone();
            let saved_hourly =
                find_hourly_orderly_perp(perp_key_clone.symbol, perp_key_clone.block_hour)
                    .await
                    .unwrap();

            self.hourly_orderly_perp_cache
                .insert(perp_key.clone(), saved_hourly);
        }
        self.hourly_orderly_perp_cache
            .get_mut(&perp_key.clone())
            .unwrap()
    }

    pub async fn get_hourly_user_perp(
        &mut self,
        perp_key: &HourlyUserPerpKey,
    ) -> &mut HourlyUserPerp {
        if !self.hourly_user_perp_cache.contains_key(&perp_key.clone()) {
            let perp_key_clone = perp_key.clone();
            let saved_data = find_hourly_user_perp(
                perp_key_clone.account_id.clone(),
                perp_key_clone.symbol.clone(),
                perp_key.block_hour.clone(),
            )
            .await
            .unwrap();
            self.hourly_user_perp_cache
                .insert(perp_key.clone(), saved_data);
        }
        self.hourly_user_perp_cache
            .get_mut(&perp_key.clone())
            .unwrap()
    }

    pub async fn get_orderly_perp(&mut self, symbol: &String) -> &mut OrderlyPerpSummary {
        if !self.orderly_perp_cache.contains_key(&symbol.clone()) {
            let saved_summary = find_orderly_perp_summary(symbol.clone()).await.unwrap();

            self.orderly_perp_cache
                .insert(symbol.clone(), saved_summary);
        }
        self.orderly_perp_cache.get_mut(&symbol.clone()).unwrap()
    }

    pub async fn get_user_perp(&mut self, perp_key: &UserPerpSummaryKey) -> &mut UserPerpSummary {
        if !self.user_perp_cache.contains_key(&perp_key.clone()) {
            let perp_key_clone = perp_key.clone();
            let saved_summary = find_user_perp_summary(
                perp_key_clone.account_id.clone(),
                perp_key_clone.symbol.clone(),
            )
            .await
            .unwrap();
            self.user_perp_cache.insert(perp_key.clone(), saved_summary);
        }
        self.user_perp_cache.get_mut(&perp_key.clone()).unwrap()
    }
}
