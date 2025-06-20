use std::collections::HashMap;
use crate::position::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidationMode {
    Isolated,
    CrossMargin
}

#[derive(Debug)]
pub struct LiquidationResult {
    pub liquidated: bool,
    pub realized_loss: f64,  //amount of loss (positive)
    pub liquiation_fee: f64,
    pub remaining_margin: f64,  //margin remaining after liquidation (usually 0)
    pub liquidator_reward: f64,
}

pub fn should_liquidate(pos: &Position, mark_price: f64, maintenance_margin_ratio: f64) -> bool {
    let equity = pos.margin * pos.unrealized_pnl(mark_price);
    let maintenance_margin = pos.size * mark_price * maintenance_margin_ratio;
    equity < maintenance_margin
}

pub fn execute_liquidation(pos: &mut Position, mark_price: f64, liquidation_fee_ratio: f64) -> LiquidationResult {
    let pnl = pos.unrealized_pnl(mark_price);
    let equity = pos.margin + pnl;
    // let realized_loss = if pnl < 0.0 {-pnl} else { 0.0 };
    let realized_loss = -pnl.min(0.0);
    
    let mut fee = equity * liquidation_fee_ratio;
    if fee > equity {
        fee = equity;
    }
    
    let remaining = (equity - fee).max(0.0);
    
    pos.size = 0.0;
    pos.margin = 0.0;
    
    LiquidationResult {
        liquidated: true,
        realized_loss,
        liquiation_fee: fee,
        remaining_margin: remaining,
        liquidator_reward: fee,
    }
}

pub fn cross_margin_liquidation(
    positions: &mut [Position],
    mark_prices: HashMap<String, f64>,
    maintenance_margin_ratio: f64,
    liquidation_fee_ratio: f64,
) -> Vec<LiquidationResult> {
    let mut results = Vec::new();
    for pos in positions.iter_mut() {
        if let Some(&mark_price) = mark_prices.get(&pos.market) {
            if should_liquidate(pos, mark_price, maintenance_margin_ratio) {
                let result = execute_liquidation(pos, mark_price, liquidation_fee_ratio);
                results.push(result);
                // vault.receive_fee(result.liquidation_fee, liquidator_addr);
            }
        }
    }
    results
}