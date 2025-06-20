use std::collections::HashMap;
use crate::position::Position;

pub struct Account {
    pub user: String,
    pub balance: f64,
    pub positions: HashMap<String, Position>,
}

impl Account {
    pub fn new(user: &str, initial_balance: f64) -> Self {
        Self {
            user: user.to_string(),
            balance: initial_balance,
            positions: HashMap::new(),
        }
    }
    
    pub fn update_balance(&mut self, delta: f64) {
        self.balance += delta;
    }
    
    pub fn get_equity(&self, mark_prices: &HashMap<String, f64>) -> f64 {
        let mut equity = self.balance;
        for (market, pos) in &self.positions {
            if let Some(mark) = mark_prices.get(market) {
                equity = pos.unrealized_pnl(*mark);
            }
        }
        equity
    }
}