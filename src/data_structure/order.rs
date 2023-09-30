#[derive(Debug, Copy, Clone)]
pub enum OrderType {
    Bid,
    Ask,
}

#[derive(Debug, Copy, Clone)]
pub enum OrderStatus {
    Open,
    Filled,
    Cancelled,
}

#[derive(Debug)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub side: OrderType,
    pub status: OrderStatus,
    pub price: i32,
    pub quantity: i32,
}

impl Order {
    pub fn new(
        id: i32,
        user_id: i32,
        side: OrderType,
        status: OrderStatus,
        price: i32,
        quantity: i32,
    ) -> Self {
        Order {
            id,
            user_id,
            side,
            status,
            price,
            quantity,
        }
    }

    pub fn is_bid(&self) -> bool {
        match self.side {
            OrderType::Bid => true,
            _ => false,
        }
    }

    pub fn is_ask(&self) -> bool {
        match self.side {
            OrderType::Ask => true,
            _ => false,
        }
    }

    pub fn is_open(&self) -> bool {
        match self.status {
            OrderStatus::Open => true,
            _ => false,
        }
    }

    pub fn is_filled(&self) -> bool {
        match self.status {
            OrderStatus::Filled => true,
            _ => false,
        }
    }

    pub fn is_cancelled(&self) -> bool {
        match self.status {
            OrderStatus::Cancelled => true,
            _ => false,
        }
    }

    pub fn fill(&mut self, quantity: i32) {
        self.quantity -= quantity;
        if self.quantity == 0 {
            self.status = OrderStatus::Filled;
        }
    }

    pub fn cancel(&mut self) {
        self.status = OrderStatus::Cancelled;
    }

    pub fn to_string(&self) -> String {
        format!(
            "id: {}, user_id: {}, side: {:?}, status: {:?}, price: {}, quantity: {}",
            self.id, self.user_id, self.side, self.status, self.price, self.quantity
        )
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    pub fn get_side(&self) -> OrderType {
        self.side
    }

    pub fn get_status(&self) -> OrderStatus {
        self.status
    }

    pub fn get_price(&self) -> i32 {
        self.price
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }
}