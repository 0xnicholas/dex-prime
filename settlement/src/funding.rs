use std::collections::HashMap;
use common::types::Side;
use crate::position::Position;

pub struct FundingRate {
    pub market: String,
    pub rate: f64,  // hourly funding rate
    pub timestamp: u64,
}

pub fn apply_funding(positions: &mut [Position], funding_rates: &HashMap<String, FundingRate>) {
    for pos in positions.iter_mut() {
        if let Some(rate) = funding_rates.get(&pos.market) {
            let funding_payment = pos.size * pos.entry_price * rate.rate;
            match pos.side {
                Side::Long => pos.margin -= funding_payment,
                Side::Short => pos.margin += funding_payment,
            }
        }
    }
}