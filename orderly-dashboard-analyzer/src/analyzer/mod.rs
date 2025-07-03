use std::str::FromStr;

use bigdecimal::BigDecimal;

pub mod adl_analyzer;
pub mod analyzer_context;
pub mod analyzer_gas_context;
pub mod analyzer_gas_job;
pub mod analyzer_job;
pub mod calc;
pub mod liquidation_analyzer;
pub mod perp_analyzer;
pub mod settlement_analyzer;
pub mod transaction_analyzer;

pub const INSURANCE_FUNDS: [&str; 2] = [
    "0xe042ae0d7f1cb85245360af73a383d643e43401f64fa56c2c072dbbf200554d7",
    "0xd22bfed15458474d0d4a85dda2b889f47169c0adfca0be5cca0303537b87cd40",
];

pub const STG_FUTURES_FEE_COLLECTOR: &str =
    "0xf56214af975ac69f8519f6e0237cb98a5c121edd0c2444e841eea68d424b5515";
pub const PROD_FUTURES_FEE_COLLECTOR: &str =
    "0x9af93b238feedcf6d53c8578a63b37c4fe6feed486b9c85dc3f79f72b976dbb8";

pub fn get_qty_prec() -> BigDecimal {
    BigDecimal::from(100_000_000)
}

pub fn get_symbol_prec() -> i64 {
    8
}

pub fn get_price_prec() -> BigDecimal {
    BigDecimal::from(100_000_000)
}

pub fn get_cost_position_prec() -> BigDecimal {
    BigDecimal::from(1_000_000)
}

pub fn get_unitary_prec() -> BigDecimal {
    BigDecimal::from_str("1_000_000_000_000_000").unwrap()
}

pub fn get_gas_prec() -> BigDecimal {
    BigDecimal::from_str("1_000_000_000_000_000_000").unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::analyzer::analyzer_context::AnalyzeContext;
    use crate::analyzer::calc::USDC_CHAIN_ID;
    use crate::db::hourly_orderly_perp::HourlyOrderlyPerpKey;
    use crate::db::hourly_user_perp::HourlyUserPerpKey;
    use crate::db::user_perp_summary::UserPerpSummaryKey;
    use crate::db::user_token_summary::UserTokenSummaryKey;
    use bigdecimal::BigDecimal;
    use chrono::NaiveDateTime;
    use chrono::Timelike;
    use std::ops::Div;

    use crate::db::{
        hourly_orderly_perp::HourlyOrderlyPerp,
        hourly_orderly_token::{HourlyOrderlyToken, HourlyOrderlyTokenKey},
        hourly_user_perp::HourlyUserPerp,
        hourly_user_token::{HourlyUserToken, HourlyUserTokenKey},
        orderly_perp_summary::OrderlyPerpSummary,
        orderly_token_summary::{OrderlyTokenSummary, OrderlyTokenSummaryKey},
        user_perp_summary::UserPerpSummary,
        user_token_summary::UserTokenSummary,
    };

    use super::*;
    pub const BROKER_HASH: &str =
        "0x083098c593f395bea1de45dda552d9f14e8fcb0be3faaa7a1903c5477d7ba7fd";
    pub const TOKEN_HASH: &str =
        "0xd6aca1be9729c13d677335161321649cccae6a591554772516700f986f942eaa";
    pub const SYMBOL_HASH_BTC_USDC: &str =
        "0x1111101010101010101010101010101010101010101010101010101010101010";
    pub const SYMBOL_HASH_ETH_USDC: &str =
        "0x2222101010101010101010101010101010101010101010101010101010101010";
    pub const LIQUIDATED_ACCOUNT_ID: &str =
        "0xa11ce00000000000000000000000000000000000000000000000000000000000";
    pub const LIQUIDATOR_ACCOUNT_ID: &str =
        "0xb0b0000000000000000000000000000000000000000000000000000000000000";
    pub const INSURANCE_FUND: &str =
        "0x1234123412341234123412341234123412341234123412341234123412341234";

    pub fn convert_block_hour(block_timestamp: i64) -> NaiveDateTime {
        let date_time = NaiveDateTime::from_timestamp_opt(block_timestamp, 0).unwrap();
        return date_time.with_second(0).unwrap().with_minute(0).unwrap();
    }

    pub trait TestAnalyzeContextUtils {
        fn set_user_perp_cache(
            &mut self,
            perp_key: &UserPerpSummaryKey,
            position_qty: BigDecimal,
            cost_position: BigDecimal,
            sum_unitary_fundings: BigDecimal,
        );
        fn set_user_token_cache(&mut self, token_key: &UserTokenSummaryKey);
        fn get_user_perp_cache(&mut self, perp_key: &UserPerpSummaryKey) -> &mut UserPerpSummary;
        fn get_user_token_cache(
            &mut self,
            token_key: &UserTokenSummaryKey,
        ) -> &mut UserTokenSummary;
        fn init_orderly_context(
            &mut self,
            block_time: i64,
            symbols: Vec<String>,
            tokens: Vec<String>,
            account_symbols: Vec<(String, String)>,
            account_tokens: Vec<(String, String)>,
        );
    }

    impl TestAnalyzeContextUtils for AnalyzeContext {
        fn set_user_perp_cache(
            &mut self,
            perp_key: &UserPerpSummaryKey,
            position_qty: BigDecimal,
            cost_position: BigDecimal,
            sum_unitary_fundings: BigDecimal,
        ) {
            let mut data = UserPerpSummary::new_empty_user_perp_summary(
                &perp_key.account_id,
                &perp_key.symbol,
            );
            data.holding = position_qty.div(get_qty_prec());
            data.cost_position = cost_position.clone().div(get_cost_position_prec());
            data.sum_unitary_fundings = sum_unitary_fundings.div(get_unitary_prec());
            self.user_perp_cache.insert(perp_key.clone(), data);
        }
        fn set_user_token_cache(&mut self, _token_key: &UserTokenSummaryKey) {}
        fn get_user_perp_cache(&mut self, perp_key: &UserPerpSummaryKey) -> &mut UserPerpSummary {
            if !self.user_perp_cache.contains_key(&perp_key.clone()) {
                let saved_summary = UserPerpSummary::new_empty_user_perp_summary(
                    &perp_key.account_id,
                    &perp_key.symbol,
                );
                self.user_perp_cache.insert(perp_key.clone(), saved_summary);
            }
            self.user_perp_cache.get_mut(&perp_key.clone()).unwrap()
        }

        fn get_user_token_cache(
            &mut self,
            token_key: &UserTokenSummaryKey,
        ) -> &mut UserTokenSummary {
            self.user_token_cache.get_mut(&token_key.clone()).unwrap()
        }

        fn init_orderly_context(
            &mut self,
            block_time: i64,
            symbols: Vec<String>,
            tokens: Vec<String>,
            account_symbols: Vec<(String, String)>,
            account_tokens: Vec<(String, String)>,
        ) {
            let block_hour = convert_block_hour(block_time);
            let block_time = NaiveDateTime::from_timestamp_opt(block_time, 0).unwrap();
            for symbol in &symbols {
                self.hourly_orderly_perp_cache.insert(
                    HourlyOrderlyPerpKey {
                        symbol: symbol.clone(),
                        block_hour: block_hour,
                    },
                    HourlyOrderlyPerp::new_empty_hourly_orderly_perp(symbol.as_str(), block_hour),
                );
                self.orderly_perp_cache.insert(
                    symbol.to_string(),
                    OrderlyPerpSummary::new_empty_orderly_perp_summary(symbol),
                );
            }

            for token in &tokens {
                self.hourly_orderly_token_cache.insert(
                    HourlyOrderlyTokenKey {
                        token: token.clone(),
                        block_hour: block_hour,
                        chain_id: USDC_CHAIN_ID.to_string(),
                    },
                    HourlyOrderlyToken::new_empty_hourly_orderly_perp(
                        token,
                        block_hour,
                        USDC_CHAIN_ID,
                    ),
                );

                self.orderly_token_cache.insert(
                    OrderlyTokenSummaryKey {
                        token: token.clone(),
                        chain_id: USDC_CHAIN_ID.to_string(),
                    },
                    OrderlyTokenSummary::new_empty_orderly_token_summary(token, USDC_CHAIN_ID),
                );
            }

            for (account, symbol) in &account_symbols {
                self.hourly_user_perp_cache.insert(
                    HourlyUserPerpKey {
                        account_id: account.to_string(),
                        symbol: symbol.to_string(),
                        block_hour,
                    },
                    HourlyUserPerp::new_emtpy_hourly_user_perp(&account, &symbol, block_time),
                );
                let saved_summary = UserPerpSummary::new_empty_user_perp_summary(&account, &symbol);
                self.user_perp_cache.insert(
                    UserPerpSummaryKey {
                        account_id: account.to_string(),
                        symbol: symbol.to_string(),
                    },
                    saved_summary,
                );
            }

            for (account, token) in &account_tokens {
                self.hourly_user_token_cache.insert(
                    HourlyUserTokenKey {
                        account_id: account.to_string(),
                        token: token.to_string(),
                        block_hour,
                        chain_id: USDC_CHAIN_ID.to_string(),
                    },
                    HourlyUserToken::new_emtpy_hourly_user_token(
                        &account,
                        &token,
                        block_hour,
                        USDC_CHAIN_ID,
                    ),
                );
                let saved_summary =
                    UserTokenSummary::new_empty_token_summary(&account, &token, USDC_CHAIN_ID);
                self.user_token_cache.insert(
                    UserTokenSummaryKey {
                        account_id: account.to_string(),
                        token: token.to_string(),
                        chain_id: USDC_CHAIN_ID.to_string(),
                    },
                    saved_summary,
                );
            }
        }
    }
}
