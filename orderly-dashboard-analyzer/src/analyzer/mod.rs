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

pub fn get_qty_prec() -> BigDecimal {
    BigDecimal::from(100_000_000)
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
