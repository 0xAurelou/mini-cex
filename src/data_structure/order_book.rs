use super::order::{Order};
use std::collections::{BTreeMap, VecDeque};

trait OrderBookTrait {
    fn insert(&mut self, order: &Order);
    fn remove(&mut self, price: &mut Order);
    fn fill(&mut self, order: &Order) -> Order;
    fn get_best_bid(&self) -> Option<&Order>;
    fn get_best_ask(&self) -> Option<&Order>;
    fn get_spread(&self) -> Option<i32>;
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
    fn insert(&mut self, order: &Order) {
        if order.is_bid() {
            self.bids
                .entry(order.price)
                .or_insert_with(VecDeque::new)
                .push_back(*order);
        } else {
            self.asks
                .entry(order.price)
                .or_insert_with(VecDeque::new)
                .push_back(*order);
        }
    }
    fn remove(&mut self, order: &mut Order) {
        // Check if the order exist in the order book
        if order.is_bid() {
            if let Some(orders) = self.bids.get_mut(&order.price) {
                orders.retain(|x| x.id != order.id);
            }
        } else {
            if let Some(orders) = self.asks.get_mut(&order.price) {
                orders.retain(|x| x.id != order.id);
            }
        }
    }

    fn fill(&mut self, order: &Order) -> Order {
        let mut order = *order;
        if order.is_bid() {
            if let Some(orders) = self.asks.get_mut(&order.price) {
                while order.quantity > 0 {
                    if let Some(mut ask) = orders.pop_front() {
                        if ask.quantity <= order.quantity {
                            order.quantity -= ask.quantity;
                            ask.quantity = 0;
                            ask.fill();
                        } else {
                            ask.quantity -= order.quantity;
                            order.quantity = 0;
                            orders.push_front(ask);
                        }
                    } else {
                        break;
                    }
                }
            }
        } else {
            if let Some(orders) = self.bids.get_mut(&order.price) {
                while order.quantity > 0 {
                    if let Some(mut bid) = orders.pop_front() {
                        if bid.quantity <= order.quantity {
                            order.quantity -= bid.quantity;
                            bid.quantity = 0;
                            bid.fill();
                        } else {
                            bid.quantity -= order.quantity;
                            order.quantity = 0;
                            orders.push_front(bid);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        return order;
    }

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

    fn get_spread(&self) -> Option<i32> {
        match (self.get_best_bid(), self.get_best_ask()) {
            (Some(bid), Some(ask)) => Some(ask.price - bid.price),
            _ => None,
        }
    }
}
