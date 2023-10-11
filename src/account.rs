use token::Token;

// We want to track id, and all the balances he has in every market
pub struct Account{
    pub id: i32,
    pub balances: HashMap<Token, u32>,
}

impl Account{
    pub fn new(id: i32) -> Account{
        Account{
            id: id,
            balances: HashMap::new(),
        }
    }

    pub fn update_balance(&mut self, token: Token, amount: i32){
        let balance = self.balances.entry(token).or_insert(0);
        *balance += amount;
    }
}
