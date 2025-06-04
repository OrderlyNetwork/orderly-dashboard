use std::collections::HashMap;
use std::time::Instant;

use chrono::NaiveDateTime;
use futures::FutureExt;

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
        let inst = Instant::now();
        self.hourly_user_perp_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });

        self.hourly_orderly_perp_cache
            .iter_mut()
            .for_each(|(_k, v)| {
                if pulled_block_height != 0 {
                    v.pulled_block_height = pulled_block_height;
                }
                v.pulled_block_time = pulled_block_time;
            });
        self.orderly_perp_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });
        self.user_perp_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });
        self.hourly_user_token_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });
        self.user_token_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });

        self.orderly_token_cache.iter_mut().for_each(|(_k, v)| {
            if pulled_block_height != 0 {
                v.pulled_block_height = pulled_block_height;
            }
            v.pulled_block_time = pulled_block_time;
        });

        let mut futs = Vec::with_capacity(8);
        futs.push(
            create_or_update_hourly_user_perp(
                self.hourly_user_perp_cache.values().cloned().collect(),
            )
            .boxed(),
        );

        futs.push(
            create_or_update_hourly_orderly_perp(
                self.hourly_orderly_perp_cache.values().cloned().collect(),
            )
            .boxed(),
        );

        futs.push(
            create_or_update_orderly_perp_summary(
                self.orderly_perp_cache.values().cloned().collect(),
            )
            .boxed(),
        );

        futs.push(
            create_or_update_user_perp_summary(self.user_perp_cache.values().cloned().collect())
                .boxed(),
        );

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

        futs.push(
            create_or_update_hourly_user_token(
                self.hourly_user_token_cache.values().cloned().collect(),
            )
            .boxed(),
        );

        futs.push(
            create_or_update_user_token_summary(self.user_token_cache.values().cloned().collect())
                .boxed(),
        );

        futs.push(
            create_or_update_orderly_token_summary(
                self.orderly_token_cache.values().cloned().collect(),
            )
            .boxed(),
        );

        futures::future::join_all(futs)
            .await
            .into_iter()
            .enumerate()
            .for_each(|(index, result)| {
                if let Err(err) = &result {
                    tracing::error!("save_analyze_result task {} error:{}", index, err);
                }
                result.unwrap();
            });

        let elapsed_ms = inst.elapsed().as_millis();
        if elapsed_ms > 7_000 {
            tracing::info!("save_analyze_result cost: {:?} ms", elapsed_ms);
        }
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

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::analyzer::analyzer_context::AnalyzeContext;
    use crate::analyzer::tests::*;
    use crate::db::hourly_orderly_perp::{HourlyOrderlyPerp, HourlyOrderlyPerpKey};
    use crate::db::hourly_orderly_token::{HourlyOrderlyToken, HourlyOrderlyTokenKey};
    use crate::db::hourly_user_perp::{HourlyUserPerp, HourlyUserPerpKey};
    use crate::db::hourly_user_token::{HourlyUserToken, HourlyUserTokenKey};
    use crate::db::orderly_perp_summary::OrderlyPerpSummary;
    use crate::db::orderly_token_summary::{OrderlyTokenSummary, OrderlyTokenSummaryKey};
    use crate::db::user_token_summary::{UserTokenSummary, UserTokenSummaryKey};

    use chrono::NaiveDateTime;

    fn init_log() {
        tracing_subscriber::fmt::Subscriber::builder()
            .with_writer(std::io::stderr)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
    }

    #[ignore]
    // #[actix_web::test(worker_threads = 6)]
    #[test]
    fn test_analyze_context1() {
        init_log();
        let system = actix::System::new();
        system.block_on(async {
            let mut context = AnalyzeContext::new_context();
            let block_time_ = 1748943453;
            let block_time = NaiveDateTime::from_timestamp_opt(block_time_, 0).unwrap();
            let block_hour = convert_block_hour(block_time_);
            for i in 0..10000 {
                context.hourly_orderly_perp_cache.insert(
                    HourlyOrderlyPerpKey {
                        symbol: format!("0xaaaaa{}", i),
                        block_hour: block_hour,
                    },
                    HourlyOrderlyPerp::new_empty_hourly_orderly_perp(
                        &format!("0xaaaaa{}", i),
                        block_hour,
                    ),
                );
                context.hourly_orderly_token_cache.insert(
                    HourlyOrderlyTokenKey {
                        token: format!("0xtoken{}", i),
                        block_hour,
                        chain_id: "1".to_string(),
                    },
                    HourlyOrderlyToken::new_empty_hourly_orderly_perp(
                        &format!("0xtoken{}", i),
                        block_hour,
                        "1",
                    ),
                );

                context.hourly_user_perp_cache.insert(
                    HourlyUserPerpKey {
                        account_id: format!("0xuser{}", i),
                        symbol: format!("0xsymbol{}", i),
                        block_hour,
                    },
                    HourlyUserPerp::new_emtpy_hourly_user_perp(
                        &format!("0xuser{}", i),
                        &format!("0xsymbol{}", i),
                        block_hour,
                    ),
                );

                context.hourly_user_token_cache.insert(
                    HourlyUserTokenKey {
                        account_id: format!("0xuser{}", i),
                        token: format!("0xtoken{}", i),
                        block_hour,
                        chain_id: "1".to_string(),
                    },
                    HourlyUserToken::new_emtpy_hourly_user_token(
                        &format!("0xuser{}", i),
                        &format!("0xtoken{}", i),
                        block_hour,
                        "1",
                    ),
                );

                context.orderly_perp_cache.insert(
                    format!("0xsymbol{}", i),
                    OrderlyPerpSummary::new_empty_orderly_perp_summary(&format!("0xsymbol{}", i)),
                );

                context.orderly_token_cache.insert(
                    OrderlyTokenSummaryKey {
                        token: format!("0xtoken{}", i),
                        chain_id: "1".to_string(),
                    },
                    OrderlyTokenSummary::new_empty_orderly_token_summary(
                        &format!("0xtoken{}", i),
                        "1",
                    ),
                );

                context.user_token_cache.insert(
                    UserTokenSummaryKey {
                        account_id: format!("0xuser{}", i),
                        token: format!("0xtoken{}", i),
                        chain_id: "1".to_string(),
                    },
                    UserTokenSummary::new_empty_token_summary(
                        &format!("0xuser{}", i),
                        &format!("0xtoken{}", i),
                        "1",
                    ),
                );
            }
            let inst = Instant::now();
            context.save_analyze_result(10000000, block_time).await;
            tracing::info!(
                "save_analyze_result elapse ms: {}",
                inst.elapsed().as_millis()
            );
        });

        system.run().unwrap();
    }

    #[ignore]
    #[actix_rt::test]
    async fn test_analyze_context2() {
        init_log();
        let mut context = AnalyzeContext::new_context();
        let block_time_ = 1748943453;
        let block_time = NaiveDateTime::from_timestamp_opt(block_time_, 0).unwrap();
        let block_hour = convert_block_hour(block_time_);
        for i in 0..10000 {
            // 6s
            // context.hourly_orderly_perp_cache.insert(
            //     HourlyOrderlyPerpKey { symbol: format!("0xaaaaa{}", i), block_hour: block_hour },
            //     HourlyOrderlyPerp::new_empty_hourly_orderly_perp(&format!("0xaaaaa{}", i), block_hour));

            // 0s
            // context.hourly_orderly_token_cache.insert(
            //     HourlyOrderlyTokenKey { token: format!("0xtoken{}", i), block_hour, chain_id: "1".to_string() }
            // , HourlyOrderlyToken::new_empty_hourly_orderly_perp(&format!("0xtoken{}", i), block_hour, "1"));

            // 10s
            // context.hourly_user_perp_cache.insert(
            //     HourlyUserPerpKey { account_id: format!("0xuser{}", i), symbol: format!("0xsymbol{}", i), block_hour },
            //     HourlyUserPerp::new_emtpy_hourly_user_perp(&format!("0xuser{}", i), &format!("0xsymbol{}", i), block_hour)
            // );

            // 6.6s
            // context.hourly_user_token_cache.insert(
            //     HourlyUserTokenKey { account_id: format!("0xuser{}", i), token: format!("0xtoken{}", i), block_hour, chain_id: "1".to_string() },
            //     HourlyUserToken::new_emtpy_hourly_user_token(&format!("0xuser{}", i), &format!("0xtoken{}", i), block_hour, "1")
            // );

            // 8s
            // context.orderly_perp_cache.insert(
            //     format!("0xsymbol{}", i),
            //     OrderlyPerpSummary::new_empty_orderly_perp_summary(&format!("0xsymbol{}", i))
            // );

            // 6s
            // context.orderly_token_cache.insert(
            //     OrderlyTokenSummaryKey{token: format!("0xtoken{}", i), chain_id: "1".to_string()},
            //     OrderlyTokenSummary::new_empty_orderly_token_summary(&format!("0xtoken{}", i), "1")
            // );

            // 7.5
            context.user_token_cache.insert(
                UserTokenSummaryKey {
                    account_id: format!("0xuser{}", i),
                    token: format!("0xtoken{}", i),
                    chain_id: "1".to_string(),
                },
                UserTokenSummary::new_empty_token_summary(
                    &format!("0xuser{}", i),
                    &format!("0xtoken{}", i),
                    "1",
                ),
            );
        }
        let inst = Instant::now();
        context.save_analyze_result(10000000, block_time).await;
        tracing::info!(
            "save_analyze_result elapse ms: {}",
            inst.elapsed().as_millis()
        );
    }
}
