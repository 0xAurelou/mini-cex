use token::Token;

pub enum MarketType {
    Spot,
    Future,
    Perpetual,
}

pub struct Market {
    pub type : MarketType,
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

    pub fn create_market(&self, base: &str, quote: &str) {
        let market = Market::new(base, quote);
        let base_token = Token::new(base);
        let quote_token = Token::new(quote);
        base_token.create_market(market);
        quote_token.add_market(market);
    }

    pub fn insert(&mut self, order: &Order) {
        self.order_book.insert(order);
    }

    pub fn remove(&mut self, order: &mut Order) {
        self.order_book.remove(order);
    }

    pub fn get_best_bid(&self) -> Option<&Order> {
        self.order_book.get_best_bid()
    }

    pub fn get_best_ask(&self) -> Option<&Order> {
        self.order_book.get_best_ask()
    }

    pub fn get_order_book(&self) -> &OrderBook {
        &self.order_book
    }

    pub fn get_market_type(&self) -> MarketType {
        self.type
    }

}
