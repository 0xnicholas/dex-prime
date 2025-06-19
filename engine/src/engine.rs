use std::collections::HashMap;
use crate::orderbook::OrderBook;
use crate::types::Order;
use crate::matching::submit_order;
use crate::execution::generate_events;
use crate::event::EngineEvent;

pub struct MatchingEngine {
    books: HashMap<String, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self { books: HashMap::new() }
    }

    pub fn submit(&mut self, order: Order) -> Vec<EngineEvent> {
        let book = self.books.entry(order.market.clone()).or_insert_with(OrderBook::new);
        let results = submit_order(order, book);
        generate_events(results)
    }

    pub fn cancel(&mut self, market: &str, order_id: u64) -> Option<EngineEvent> {
        let book = self.books.get_mut(market)?;
        if book.remove_order(order_id) {
            Some(EngineEvent::OrderCancelled(order_id.to_string()))
        } else {
            None
        }
    }
}

