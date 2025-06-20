
pub fn initial_margin(size: f64, price: f64, leverage: f64) -> f64 {
    size * price / leverage
}

pub fn maintenance_margin(size: f64, price: f64, mmr: f64) -> f64 {
    size * price / mmr
}