use common::types::{Order, MatchResult};
use crate::orderbook::OrderBook;
use crate::matcher::match_order;

pub fn submit_order(order: Order, book: &mut OrderBook) -> Vec<MatchResult> {
    let result = match_order(order.clone(), book);
    if result.is_empty() {
        book.insert(order);
    }
    result
}