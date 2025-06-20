use common::types::{Side, MatchResult};
use crate::position::Position;
// use crate::margin::{initial_margin, maintenance_margin};

pub struct SettlementOutcome {
    pub realized_pnl: f64,
    pub new_margin: f64,
    pub was_liquidated: bool,
    pub new_size: f64,
}

pub fn settle_trade(position: &mut Position, trade: &MatchResult, mark_price: f64, maintenance_margin_ratio: f64) -> SettlementOutcome {
    // let prev_margin = position.margin;
    let prev_size = position.size;
    let prev_entry_price = position.entry_price;
    let prev_side = position.side.clone();
    
    position.apply_trade(trade);
    
    // realized_PnL -> when reducing a position, settle part of the position to the current price
    let mut realized_pnl = 0.0;
    if position.size < prev_size {
        let closed_size = prev_size - position.size;
        realized_pnl = match prev_side {
            Side::Long => (mark_price - prev_entry_price) * closed_size,
            Side::Short => (prev_entry_price - mark_price) * closed_size,
        };
    }
    
    let was_liquidated = super::liquidation::should_liquidate(position, mark_price, maintenance_margin_ratio);
    
    SettlementOutcome {
        realized_pnl,
        new_margin: position.margin,
        was_liquidated,
        new_size: position.size,
    }
}