pub struct Market {
    pub base: &str,
    pub quote: &str,
    pub order_book: OrderBook,
}

impl Market {
    pub fn new(base: &str, quote: &str) -> Self {
        Market {
            base,
            quote,
            order_book: OrderBook::new(),
        }
    }
}
