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
}
