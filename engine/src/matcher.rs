use crate::types::{Order, MatchResult, Side};
use ordered_float::OrderedFloat;
use crate::orderbook::OrderBook;

pub fn match_order(taker: Order, book: &mut OrderBook) -> Vec<MatchResult> {
    let mut results = vec![];

    // 选择对手盘，asks和bids用OrderedFloat包裹价格key
    let levels = match taker.side {
        Side::Long => &mut book.asks,
        Side::Short => &mut book.bids,
    };

    // 价格匹配条件（买单能接受价格 >= 对手价，卖单 <= 对手价）
    let price_check = |level_price: f64| match taker.side {
        Side::Long => taker.price >= level_price,
        Side::Short => taker.price <= level_price,
    };

    let mut remaining = taker.size;

    // 获取对手盘所有价格档，转换 OrderedFloat<f64> -> f64
    // 注意asks升序，bids降序需要特殊处理
    let mut keys: Vec<f64> = levels.keys().map(|p| p.into_inner()).collect();

    // bids价格降序，asks价格升序，确保遍历顺序正确
    match taker.side {
        Side::Long => keys.sort_by(|a, b| a.partial_cmp(b).unwrap()), // asks Ascending order
        Side::Short => keys.sort_by(|a, b| b.partial_cmp(a).unwrap()), // bids Descending order
    };

    for price in keys {
        if !price_check(price) { break; }

        let price_key = OrderedFloat(price);
        if let Some(queue) = levels.get_mut(&price_key) {
            while let Some(mut maker) = queue.pop_front() {
                let executed = f64::min(remaining, maker.price);
                results.push(MatchResult {
                    taker: taker.clone(),
                    maker: maker.clone(),
                    executed_size: executed,
                    execution_price: price,
                });

                remaining -= executed;
                maker.size -= executed;
                
                if maker.size > 0.0 {
                    queue.push_front(maker);
                }

                if remaining <= 0.0 { break; }
            }
        }

        if remaining <= 0.0 { break; }
    }

    results
}