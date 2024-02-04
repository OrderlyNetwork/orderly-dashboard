use bigdecimal::BigDecimal;

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
            open_cost_diff = (quoted_diff.clone() * current_holding.clone()) / holding_diff.clone()
                - last_opening_cost.clone();
            pnl_diff = (quoted_diff.clone() * (-latest_holding.clone())) / holding_diff.clone()
                + last_opening_cost.clone();
        } else {
            let reduce = is_reduce(latest_holding.clone(), holding_diff.clone());
            if reduce {
                open_cost_diff =
                    last_opening_cost.clone() * holding_diff.clone() / latest_holding.clone();
                pnl_diff = last_opening_cost.clone() * (-holding_diff.clone())
                    / latest_holding.clone()
                    + quoted_diff.clone();
            } else {
                open_cost_diff = quoted_diff.clone();
                pnl_diff = BigDecimal::from(0);
            }
        }

        (open_cost_diff.with_prec(6), pnl_diff.with_prec(6))
    }
}
