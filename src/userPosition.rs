use market::Market;

// We want to track id, and all the balances he has in every market
pub struct UserPosition {
    pub id: i32,
    pub balances: HashMap<Market, i32>,
}

impl UserPosition {
    pub fn new(id: i32) -> Self {
        UserPosition {
            id,
            balances: HashMap::new(),
        }
    }
}
