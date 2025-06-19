
#[derive(Clone, Debug, PartialEq)]
pub enum Side {
    Long,
    Short,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub id: u64,
    pub user: String,
    pub market: String,
    pub side: Side,
    pub price: f64,
    pub size: f64,
    pub ts: u64,
}

#[derive(Clone, Debug)]
pub struct MatchResult {
    pub taker: Order,
    pub maker: Order,
    pub executed_size: f64,
    pub execution_price: f64,
}