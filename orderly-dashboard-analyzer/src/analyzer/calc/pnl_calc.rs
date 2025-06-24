use bigdecimal::{BigDecimal, RoundingMode};

use crate::analyzer::get_symbol_prec;

pub struct RealizedPnl {}

fn is_reverse(holding_diff: BigDecimal, latest_holding: BigDecimal) -> bool {
    if latest_holding.clone() != BigDecimal::from(0) {
        let current_holding = latest_holding.clone() + holding_diff.clone();
        if (current_holding.clone() == BigDecimal::from(0))
            || current_holding.clone().sign() != latest_holding.clone().sign()
        {
            return true;
        }
    }
    false
}

fn is_reduce(latest_holding: BigDecimal, holding_diff: BigDecimal) -> bool {
    latest_holding.clone() != BigDecimal::from(0) && latest_holding.sign() != holding_diff.sign()
}

/**
 * public class SymbolPnlDiff {
    private String symbol;
    private BigDecimal openCostDiff;
    private BigDecimal realizedPnlDiff;
}
 *
*/

// CeFi+avg_open_price+calculation
impl RealizedPnl {
    pub fn calc_realized_pnl(
        holding_diff: BigDecimal,
        quoted_diff: BigDecimal,
        latest_holding: BigDecimal,
        last_opening_cost: BigDecimal,
    ) -> (BigDecimal, BigDecimal) {
        let reverse = is_reverse(holding_diff.clone(), latest_holding.clone());
        let current_holding = latest_holding.clone() + holding_diff.clone();
        let open_cost_diff;
        let pnl_diff;

        if reverse {
            open_cost_diff = ((quoted_diff.clone() * current_holding.clone())
                / holding_diff.clone())
            .with_scale_round(get_symbol_prec(), RoundingMode::HalfUp)
                - last_opening_cost.clone();
            pnl_diff = ((quoted_diff.clone() * (-latest_holding.clone())) / holding_diff.clone())
                .with_scale_round(get_symbol_prec(), RoundingMode::HalfUp)
                + last_opening_cost.clone();
        } else {
            let reduce = is_reduce(latest_holding.clone(), holding_diff.clone());
            if reduce {
                open_cost_diff = (last_opening_cost.clone() * holding_diff.clone()
                    / latest_holding.clone())
                .with_scale_round(get_symbol_prec(), RoundingMode::HalfUp);
                pnl_diff = (last_opening_cost.clone() * (-holding_diff.clone())
                    / latest_holding.clone()
                    + quoted_diff.clone())
                .with_scale_round(get_symbol_prec(), RoundingMode::HalfUp);
            } else {
                open_cost_diff = quoted_diff.clone();
                pnl_diff = BigDecimal::from(0);
            }
        }

        (open_cost_diff, pnl_diff)
    }
}

#[derive(Debug)]
pub struct SymbolPnlSummary {
    pub holding: BigDecimal,
    pub opening_cost: BigDecimal,
    pub day_realized_pnl: BigDecimal,
    pub total_realized_pnl: BigDecimal,
}

impl SymbolPnlSummary {
    pub fn init() -> SymbolPnlSummary {
        SymbolPnlSummary {
            holding: bigdecimal::Zero::zero(),
            opening_cost: bigdecimal::Zero::zero(),
            day_realized_pnl: bigdecimal::Zero::zero(),
            total_realized_pnl: bigdecimal::Zero::zero(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolPnlChange {
    pub holding_diff: BigDecimal,
    pub quote_diff: BigDecimal,
}

impl SymbolPnlChange {
    pub fn init() -> SymbolPnlChange {
        SymbolPnlChange {
            holding_diff: bigdecimal::Zero::zero(),
            quote_diff: bigdecimal::Zero::zero(),
        }
    }
}

pub struct SymbolPnlDiff {
    pub open_cost_diff: BigDecimal,
    pub realized_pnl_diff: BigDecimal,
}

pub struct SymbolPnlCalculator {}

impl SymbolPnlCalculator {
    // todo: finish today and new day
    pub fn calc_new_symbol_pnl_summary(
        symbol_pnl_summary: SymbolPnlSummary,
        symbol_pnl_change: SymbolPnlChange,
        symbol_pnl_diff: SymbolPnlDiff,
    ) {
        let current_holding: BigDecimal =
            symbol_pnl_summary.holding + symbol_pnl_change.holding_diff;
        let mut new_summary = SymbolPnlSummary::init();
        new_summary.holding = current_holding;
        new_summary.opening_cost = symbol_pnl_summary.opening_cost + symbol_pnl_diff.open_cost_diff;
        new_summary.total_realized_pnl =
            symbol_pnl_summary.total_realized_pnl + symbol_pnl_diff.realized_pnl_diff.clone();
        // as new day
        new_summary.day_realized_pnl = symbol_pnl_diff.realized_pnl_diff;
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::RealizedPnl;
    use bigdecimal::{BigDecimal, RoundingMode};

    #[test]
    fn test_bigdecimal_prec() {
        let val = BigDecimal::from_str("100.123456789012345678901").unwrap();
        assert_eq!("100.123456789012345678901", val.to_string());
        assert_eq!(val.with_prec(8).to_string(), "100.12346");
    }

    #[test]
    fn test_bigdecimal_rounding_mod() {
        // let mut ctx = Context::new(NonZeroU64::new(4).unwrap(), RoundingMode::HalfUp);
        let a = BigDecimal::from_str("10").unwrap();
        let b = BigDecimal::from_str("3").unwrap();
        // 使用 Context 进行除法运算
        let result = a / b;
        let scale_round = result.with_scale_round(4, RoundingMode::HalfUp);
        println!(
            "10 / 3 with precision 4, HalfUp: {}, scale_round: {}",
            result, scale_round
        );
        assert_eq!(scale_round.to_string(), "3.3333");
    }

    #[test]
    fn test_calculate_pnl_diff_default_case() {
        let holding_diff: BigDecimal = BigDecimal::from_str("10").unwrap();
        let quoted_diff: BigDecimal = BigDecimal::from_str("-500").unwrap();
        let latest_holding: BigDecimal = BigDecimal::from_str("5").unwrap();
        let last_opening_cost: BigDecimal = BigDecimal::from_str("-500").unwrap();
        let pnl_diff = RealizedPnl::calc_realized_pnl(
            latest_holding,
            last_opening_cost,
            holding_diff,
            quoted_diff,
        );
        assert_eq!(pnl_diff.0.to_string(), "-500");
        assert_eq!(pnl_diff.1.to_string(), "0");
    }

    #[test]
    fn test_calculate_pnl_diff_reverse_case() {
        let holding_diff: BigDecimal = BigDecimal::from_str("10").unwrap();
        let quoted_diff: BigDecimal = BigDecimal::from_str("-500").unwrap();
        let latest_holding: BigDecimal = BigDecimal::from_str("-15").unwrap();
        let last_opening_cost: BigDecimal = BigDecimal::from_str("600").unwrap();
        let pnl_diff = RealizedPnl::calc_realized_pnl(
            latest_holding,
            last_opening_cost,
            holding_diff,
            quoted_diff,
        );
        assert_eq!(pnl_diff.0.to_string(), "700.00000000");
        assert_eq!(pnl_diff.1.to_string(), "-100.00000000");
    }

    #[test]
    fn test_calculate_pnl_diff_close_case() {
        let holding_diff: BigDecimal = BigDecimal::from_str("10").unwrap();
        let quoted_diff: BigDecimal = BigDecimal::from_str("-500").unwrap();
        let latest_holding: BigDecimal = BigDecimal::from_str("-5").unwrap();
        let last_opening_cost: BigDecimal = BigDecimal::from_str("300").unwrap();
        let pnl_diff = RealizedPnl::calc_realized_pnl(
            latest_holding,
            last_opening_cost,
            holding_diff,
            quoted_diff,
        );
        assert_eq!(pnl_diff.0.to_string(), "250.00000000");
        assert_eq!(pnl_diff.1.to_string(), "50.00000000");
    }

    #[test]
    fn test_calc_new_symbol_pnl_summary_when_reverse_then_correct() {}
}
