use data_structure::{Order, OrderBook};
use market::Market;

pub struct Token {
    pub name: &str,
    pub market: HashMap<&str, Market>,
}

impl Token {
    pub fn new(name: String) -> Self {
        Token {
            name,
            market: HashMap::new(),
        }
    }

    pub fn add_market(&mut self, market: Market) {
        self.market.insert(&market.quote, market);
    }
}
