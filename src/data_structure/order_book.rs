use super::order::Order;
use std::collections::{BTreeMap, VecDeque};

trait OrderBookTrait {
    fn insert(&mut self, order: &Order);
    fn remove(&mut self, price: i32);
    fn get_best_bid(&self) -> Option<&Order>;
    fn get_best_ask(&self) -> Option<&Order>;
}

struct OrderBook {
    bids: BTreeMap<i32, VecDeque<Order>>, // Bids sorted in descending order
    asks: BTreeMap<i32, VecDeque<Order>>, // Asks sorted in ascending order
}

impl OrderBook {
    fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }
}

impl OrderBookTrait for OrderBook {
    fn insert(&mut self, order: &Order) {}
    fn remove(&mut self, price: i32) {}
    fn get_best_bid(&self) -> Option<&Order> {
        return self
            .bids
            .iter()
            .next_back()
            .map(|(_, v)| v.front().unwrap());
    }
    fn get_best_ask(&self) -> Option<&Order> {
        return self.asks.iter().next().map(|(_, v)| v.front().unwrap());
    }
}
