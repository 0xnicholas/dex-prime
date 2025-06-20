use std::collections::{BTreeMap, VecDeque, HashMap};
use ordered_float::OrderedFloat;
use common::types::{Order, Side};

pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, VecDeque<Order>>,  // price FIFO
    pub asks: BTreeMap<OrderedFloat<f64>, VecDeque<Order>>,
    pub index: HashMap<u64, (Side, f64)>, // order_id (side, price)
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            index: HashMap::new(),
        }
    }

    pub fn insert(&mut self, order: Order) {
        let price_key = OrderedFloat(order.price);
        let book = match order.side {
            Side::Long => &mut self.bids,
            Side::Short => &mut self.asks,
        };
        book.entry(price_key).or_default().push_back(order.clone());
        self.index.insert(order.id, (order.side, order.price));
    }

    pub fn remove_order(&mut self, order_id: u64) -> bool {
        if let Some((side, price)) = self.index.remove(&order_id) {
            let book = match side {
                Side::Long => &mut self.bids,
                Side::Short => &mut self.asks,
            };
            let price_key = OrderedFloat(price);
            if let Some(queue) = book.get_mut(&price_key) {
                if let Some(pos) = queue.iter().position(|o| o.id == order_id) {
                    queue.remove(pos);
                    if queue.is_empty() {
                        book.remove(&price_key);
                    }
                    return true;
                }
            }
        }
            false
    }
}