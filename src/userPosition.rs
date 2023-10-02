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

    pub fn update_balance(&mut self, market: Market, balance: i32) {
        account_balance = self.balances.get(&market);
        if (account_balance == None) {
            self.balances.insert(market, balance);
        } else {
            self.balances.insert(market, account_balance + balance);
        }
    }
}
