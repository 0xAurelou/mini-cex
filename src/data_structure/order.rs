
#[derive(Debug, Copy,Clone )]
pub enum OrderType {
    Market,
    Limit,
}

impl Default for OrderType {
    fn default() -> Self {
        Self::Market
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OrderSide{
    Bid,
    Ask,
}

impl Default for OrderSide {
    fn default() -> Self {
        Self::Bid
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OrderStatus {
    Open,
    Filled,
    Cancelled,
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::Open
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub order_type: OrderType,
    pub order_side: OrderSide,
    pub order_status: OrderStatus,
    pub price: i32,
    pub quantity: i32,
}

impl Order {
    pub fn new(
        id: i32,
        user_id: i32,
        order_type: OrderType,
        order_side: OrderSide,
        order_status: OrderStatus,
        price: i32,
        quantity: i32,
    ) -> Self {
        Order {
            id,
            user_id,
            order_type,
            order_side,
            order_status,
            price,
            quantity,
        }
    }

    pub fn is_bid(&self) -> bool {
        match self.order_side {
            OrderSide::Bid => true,
            _ => false,
        }
    }

    pub fn is_ask(&self) -> bool {
        match self.order_side {
            OrderSide::Ask => true,
            _ => false,
        }
    }

    pub fn is_open(&self) -> bool {
        match self.order_status {
            OrderStatus::Open => true,
            _ => false,
        }
    }

    pub fn is_filled(&self) -> bool {
        match self.order_status{
            OrderStatus::Filled => true,
            _ => false,
        }
    }

    pub fn is_cancelled(&self) -> bool {
        match self.order_status{
            OrderStatus::Cancelled => true,
            _ => false,
        }
    }

    pub fn fill(&mut self) {
        self.order_status= OrderStatus::Filled;
    }

    pub fn cancel(&mut self) {
        self.order_status= OrderStatus::Cancelled;
    }

    pub fn to_string(&self) -> String {
        format!(
            "id: {}, user_id: {}, side: {:?}, status: {:?}, price: {}, quantity: {}",
            self.id, self.user_id, self.order_side, self.order_status, self.price, self.quantity
        )
    }
}
