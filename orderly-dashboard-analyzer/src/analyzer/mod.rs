use bigdecimal::BigDecimal;
use num_traits::FromPrimitive;
use std::ops::Div;

pub mod adl_analyzer;
pub mod analyzer_context;
pub mod analyzer_job;
pub mod calc;
pub mod liquidation_analyzer;
pub mod perp_analyzer;
pub mod settlement_analyzer;
pub mod transaction_analyzer;
pub mod analyzer_gas_job;
mod abalyer_gas_context;

pub(crate) fn div_into_real(dividend: i128, divisor: i128) -> BigDecimal {
    let x = BigDecimal::from_i128(dividend).unwrap();
    let k = BigDecimal::from_i128(divisor).unwrap();
    x.div(k)
}

pub(crate) fn to_big_decimal(num: BigDecimal) -> BigDecimal {
    num
}
