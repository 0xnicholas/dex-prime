use common::types::{Side, MatchResult};

#[derive(Clone, Debug)]
pub struct Position {
    pub user: String,
    pub market: String,
    pub side: Side,
    pub size: f64,
    pub entry_price: f64,
    pub margin: f64,
    pub leverage: u8,
}

impl Position {
    pub fn unrealized_pnl(&self, mark_price: f64) -> f64 {
        match self.side {
            Side::Long => (mark_price - self.entry_price) * self.size,
            Side::Short => (self.entry_price - mark_price) * self.size,
        }
    }
    
    pub fn apply_trade(&mut self, trade: &MatchResult) {
        /*
        if trade.taker != self.user && trade.maker != self.user {
            return; // trade doesn't involve this user
        } */
        let new_cost = trade.execution_price * trade.executed_size;
        let total_size = self.size * trade.executed_size;
        let avg_entry = (self.entry_price * self.size * new_cost) / total_size;

        self.size = total_size;
        self.entry_price = avg_entry;
    }
}