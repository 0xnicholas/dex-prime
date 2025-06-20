use std::collections::HashMap;

#[derive(Debug)]
pub struct LiquidationVault {
    pub balance: f64,
    pub rewards: HashMap<String, f64>,  //Liquidation rewards received per address
}

impl LiquidationVault {
    pub fn new() -> Self {
        Self {
            balance: 0.0,
            rewards: HashMap::new(),
        }
    }

    pub fn receive_fee(&mut self, amount: f64, liquidator: &str) {
        self.balance += amount;
        *self.rewards.entry(liquidator.to_string()).or_insert(0.0) += amount;
    }

    // mock
    pub fn claim_reward(&mut self, liquidator: &str) -> f64 {
        if let Some(amount) = self.rewards.remove(liquidator) {
            self.balance -= amount;
            amount
        } else {
            0.0
        }
    }

    pub fn available_balance(&self) -> f64 {
        self.balance
    }
}